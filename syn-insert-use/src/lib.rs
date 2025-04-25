use std::collections::HashSet;

use syn::{visit::Visit as _, visit_mut::VisitMut};

pub fn insert_use(f: syn::File) -> syn::File {
    let mut pv = PathVisitor::<Recursive>::new();
    pv.visit_file(&f);
    let pv = pv.sorted();
    let (_, cp) = use_declarations(pv);
    let mut tersified_file = f.clone();
    InsertUseInModuleAsNeeded.visit_file_mut(&mut tersified_file);
    TersifyingPathsVisitor { collisions: cp }.visit_file_mut(&mut tersified_file);
    let (top_level_use, _) = {
        let mut pv = PathVisitor::<NonRecursive>::new();
        pv.visit_file(&f);
        let pv = pv.sorted();
        use_declarations(pv)
    };
    for use_decl in top_level_use {
        tersified_file.items.insert(0, use_decl.to_syn().into());
    }
    tersified_file
}

struct InsertUseInModuleAsNeeded;

impl VisitMut for InsertUseInModuleAsNeeded {
    fn visit_item_mod_mut(&mut self, item: &mut syn::ItemMod) {
        if let Some((_, items)) = &mut item.content {
            let mut pv = PathVisitor::<NonRecursive>::new();
            for item in items.iter() {
                pv.visit_item(item);
            }
            let pv = pv.sorted();
            for use_decl in use_declarations(pv).0 {
                items.insert(0, use_decl.to_syn().into());
            }
        }
        syn::visit_mut::visit_item_mod_mut(self, item);
    }
}
struct PathVisitorSorted {
    crate_paths: Vec<Vec<syn::Ident>>,
    external_paths: Vec<Vec<syn::Ident>>,
}
struct Recursive;
struct NonRecursive;
struct PathVisitor<RecurseMod> {
    crate_paths: std::collections::HashSet<Vec<syn::Ident>>,
    external_paths: std::collections::HashSet<Vec<syn::Ident>>,
    _recurse_mod: std::marker::PhantomData<RecurseMod>,
}

impl<T> PathVisitor<T> {
    fn new() -> Self {
        Self {
            crate_paths: std::collections::HashSet::new(),
            external_paths: std::collections::HashSet::new(),
            _recurse_mod: std::marker::PhantomData,
        }
    }
    fn sorted(self) -> PathVisitorSorted {
        let mut crate_paths = self.crate_paths.into_iter().collect::<Vec<_>>();
        crate_paths.sort_by(|a, b| {
            let a = a.iter().map(|x| x.to_string()).collect::<String>();
            let b = b.iter().map(|x| x.to_string()).collect::<String>();
            a.cmp(&b)
        });
        let mut external_paths = self.external_paths.into_iter().collect::<Vec<_>>();
        external_paths.sort_by(|a, b| {
            let a = a.iter().map(|x| x.to_string()).collect::<String>();
            let b = b.iter().map(|x| x.to_string()).collect::<String>();
            a.cmp(&b)
        });
        PathVisitorSorted {
            crate_paths,
            external_paths,
        }
    }
}

fn to_segments(path: &[syn::PathSegment]) -> Vec<syn::Ident> {
    path.iter().map(|segment| segment.ident.clone()).collect()
}

fn path_visitor_visit_type_path<T>(pv: &mut PathVisitor<T>, type_path: &syn::TypePath)
where
    PathVisitor<T>: for<'ast> syn::visit::Visit<'ast>,
{
    let (head, tail) = split_type_path(type_path);
    let path = to_segments(&head);
    if !path.is_empty() {
        if is_crate(&path) {
            pv.crate_paths.insert(path);
        } else {
            pv.external_paths.insert(to_segments(
                head.iter()
                    .cloned()
                    .chain(std::iter::once(tail.first().unwrap()).cloned())
                    .collect::<Vec<_>>()
                    .as_slice(),
            ));
        }
    }
    syn::visit::visit_type_path(pv, type_path);
}

fn path_visitor_visit_trait_bound<T>(pv: &mut PathVisitor<T>, trait_bound: &syn::TraitBound)
where
    PathVisitor<T>: for<'ast> syn::visit::Visit<'ast>,
{
    let (head, tail) = split_path(&trait_bound.path, trait_bound.path.segments.len());
    let path = to_segments(&head);
    if !path.is_empty() {
        if is_crate(&path) {
            pv.crate_paths.insert(path);
        } else {
            pv.external_paths.insert(to_segments(
                head.iter()
                    .cloned()
                    .chain(std::iter::once(tail.first().unwrap()).cloned())
                    .collect::<Vec<_>>()
                    .as_slice(),
            ));
        }
    }
    syn::visit::visit_trait_bound(pv, trait_bound);
}

impl<'ast> syn::visit::Visit<'ast> for PathVisitor<Recursive> {
    fn visit_type_path(&mut self, item: &'ast syn::TypePath) {
        path_visitor_visit_type_path(self, item);
    }
    fn visit_trait_bound(&mut self, item: &'ast syn::TraitBound) {
        path_visitor_visit_trait_bound(self, item);
    }
}

impl<'ast> syn::visit::Visit<'ast> for PathVisitor<NonRecursive> {
    fn visit_type_path(&mut self, item: &'ast syn::TypePath) {
        path_visitor_visit_type_path(self, item);
    }
    fn visit_trait_bound(&mut self, item: &'ast syn::TraitBound) {
        path_visitor_visit_trait_bound(self, item);
    }
    fn visit_item_mod(&mut self, _: &'ast syn::ItemMod) {
        // don't visit the mod content
    }
    fn visit_file(&mut self, item: &'ast syn::File) {
        for item in item
            .items
            .iter()
            .filter(|item| !matches!(item, syn::Item::Use(_) | syn::Item::Mod(_)))
        {
            self.visit_item(item);
        }
    }
}

fn split_type_path(path: &syn::TypePath) -> (Vec<syn::PathSegment>, Vec<syn::PathSegment>) {
    let end = if let Some(qself) = &path.qself {
        qself.position
    } else {
        path.path.segments.len()
    };
    split_path(&path.path, end)
}

fn split_path(path: &syn::Path, end: usize) -> (Vec<syn::PathSegment>, Vec<syn::PathSegment>) {
    let end = if end == 0 { path.segments.len() } else { end };
    if end == 0 {
        return (vec![], path.segments.iter().cloned().collect());
    }
    for idx in 0..end {
        if path.segments[idx]
            .ident
            .to_string()
            .chars()
            .next()
            .unwrap()
            .is_ascii_uppercase()
        {
            return (
                path.segments.iter().take(idx).cloned().collect(),
                path.segments.iter().skip(idx).cloned().collect(),
            );
        }
    }
    (
        path.segments.iter().take(end - 1).cloned().collect(),
        path.segments.iter().skip(end - 1).cloned().collect(),
    )
}

fn terse_prefix(path: &[syn::Ident]) -> syn::Ident {
    if path.len() == 1 {
        return path[0].clone();
    }
    syn::Ident::new(
        &path
            .iter()
            .skip(1)
            .flat_map(|ident| {
                ident
                    .to_string()
                    .split("_")
                    .map(|word| word.chars().next().unwrap())
                    .collect::<Vec<_>>()
            })
            .fold(String::new(), |mut acc, x| {
                acc.push(x);
                acc
            }),
        proc_macro2::Span::call_site(),
    )
}

fn is_crate(path: &[syn::Ident]) -> bool {
    path.first().unwrap() == "crate"
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct UseDeclaration {
    path: Vec<syn::Ident>,
    alias: Option<syn::Ident>,
}

impl UseDeclaration {
    fn to_syn(&self) -> syn::ItemUse {
        let path_prefix = self.path.iter().take(self.path.len() - 1);
        let path_last = self.path.last().unwrap();
        match &self.alias {
            None => syn::parse_quote! {
                use #(#path_prefix::)*#path_last;
            },
            Some(alias) => syn::parse_quote! {
                use #(#path_prefix::)*#path_last as #alias;
            },
        }
    }
}

impl std::fmt::Display for UseDeclaration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = self.to_syn();
        quote::quote! {#s}.fmt(f)
    }
}
#[derive(Debug, Clone)]
struct CollidingPaths {
    collisions: Vec<Vec<syn::Ident>>,
}

fn use_declarations(paths: PathVisitorSorted) -> (Vec<UseDeclaration>, CollidingPaths) {
    let uses = paths
        .crate_paths
        .iter()
        .filter(|path| path.len() > 1)
        .map(|path| {
            let path_prefix = path.to_vec();
            let tersed = terse_prefix(path);
            (
                UseDeclaration {
                    path: path_prefix.clone(),
                    alias: Some(tersed.clone()),
                },
                tersed.clone(),
                path.clone(),
            )
        })
        .chain(
            paths
                .external_paths
                .iter()
                .filter(|path| path.len() > 1)
                .map(|path| {
                    (
                        UseDeclaration {
                            path: path.clone(),
                            alias: None,
                        },
                        path.last().unwrap().clone(),
                        path.clone(),
                    )
                }),
        )
        .collect::<Vec<_>>();
    let mut unique_uses = std::collections::HashMap::new();
    let mut cp = CollidingPaths { collisions: vec![] };
    let mut deduplicated = HashSet::new();
    for use_decl in &uses {
        if unique_uses.contains_key(&use_decl.1) && unique_uses[&use_decl.1] != use_decl.2 {
            cp.collisions.push(use_decl.2.clone());
            continue;
        }
        unique_uses.insert(use_decl.1.clone(), use_decl.2.clone());
        deduplicated.insert(use_decl.0.clone());
    }
    (deduplicated.into_iter().collect(), cp)
}

struct TersifyingPathsVisitor {
    collisions: CollidingPaths,
}

impl VisitMut for TersifyingPathsVisitor {
    fn visit_type_path_mut(&mut self, type_path: &mut syn::TypePath) {
        let (head, tail) = split_type_path(type_path);
        let segments = to_segments(&head);
        if self
            .collisions
            .collisions
            .iter()
            .any(|path| path == &segments)
            || head.is_empty()
        {
            syn::visit_mut::visit_type_path_mut(self, type_path);
            return;
        }
        let new_path: Vec<syn::Path> = if is_crate(&segments) {
            let terse = terse_prefix(&segments);
            vec![syn::parse_quote! {#terse}]
        } else {
            vec![]
        };
        if let Some(qself) = &mut type_path.qself {
            qself.position = new_path.len() + 1;
        }
        type_path.path = syn::parse_quote! {
            #(#new_path::)*#(#tail)::*
        };
        syn::visit_mut::visit_type_path_mut(self, type_path);
    }
    fn visit_trait_bound_mut(&mut self, i: &mut syn::TraitBound) {
        let (head, tail) = split_path(&i.path, i.path.segments.len());
        let segments = to_segments(&head);
        if self
            .collisions
            .collisions
            .iter()
            .any(|path| path == &segments)
            || head.is_empty()
        {
            syn::visit_mut::visit_trait_bound_mut(self, i);
            return;
        }
        let new_path: Vec<syn::Path> = if is_crate(&segments) {
            let terse = terse_prefix(&segments);
            vec![syn::parse_quote! {#terse}]
        } else {
            vec![]
        };
        i.path = syn::parse_quote! {
            #(#new_path::)*#(#tail)::*
        };
        syn::visit_mut::visit_trait_bound_mut(self, i);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use syn::parse_quote;

    #[test]
    fn test_insert_use() {
        let file: syn::File = parse_quote! {
            fn main() {
                let x: std::collections::HashSet<i32> = vec![1, 2, 3].into_iter().collect();
                let x: std::incarnations::HashSet<i32> = vec![1, 2, 3].into_iter().collect();
            }
            pub struct Bak(
                pub EitherHeapBak<
                    crate::Heap,
                    external1::Pair<
                        crate::c_d_s::Heap,
                        crate::c_d_s::LeftOperand,
                        external3::segment::Maybe<crate::c_d_s::Heap, external2::ParseMetadata<crate::c_d_s::Heap>>,
                    >,
                    external3::segment::ParseError<crate::c_d_s::Heap>,
                >,
            );
            mod foo {
                mod bar {
                    fn run() {
                        let x: std::collections::HashSet<i32> = vec![1, 2, 3].into_iter().collect();
                        let y: std::incarnations::HashSet<i32> = vec![1, 2, 3].into_iter().collect();
                        let z: crate::foo::RunFoo = Run::new();
                    }
                }
                fn bar() {
                    let y: std::collections::HashMap<i32, i32> = vec![(1, 1), (2, 1), (3, 1)].into_iter().collect();
                    let z: crate::foo::Bar = Bar::new();
                    let w: crate::faa::Bar::Bar2 = Bar2::new();
                    let w2: external::Bar::Bar2 = Bar2::new();
                    let z: crate::foo::bar::Run = Run::new();
                }
                pub struct Bak(
                    pub EitherHeapBak<
                        crate::Heap,
                        external1::Pair<
                            crate::c_d_s::Heap,
                            crate::c_d_s::LeftOperand,
                            external3::segment::Maybe<crate::c_d_s::Heap, external2::ParseMetadata<crate::c_d_s::Heap>>,
                        >,
                        external3::segment::ParseError<crate::c_d_s::Heap>,
                    >,
                );
            }
        };
        let tersified_file = insert_use(file.clone());
        let tersified_file = prettyplease::unparse(&tersified_file);
        println!("Tersified file = {}", tersified_file);
    }
}
