use std::path::{Path, PathBuf};

pub use bumpalo;
use tree_identifier::Identifier;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct KebabCodegenId(Identifier);
impl KebabCodegenId {
    pub fn to_snake_ident(&self) -> syn::Ident {
        self.0.snake_ident()
    }
}
impl From<Identifier> for KebabCodegenId {
    fn from(id: Identifier) -> Self {
        KebabCodegenId(id)
    }
}
impl std::fmt::Display for KebabCodegenId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.kebab_str())
    }
}
#[derive(Default)]
pub struct Component2SynPath(std::collections::HashMap<KebabCodegenId, syn::Path>);
// pub struct LanguagesMap(pub std::collections::HashMap<KebabCodegenId, Box<dyn std::any::Any>>);
// pub struct TargetsMap {
//     // pub languages: LanguagesMap,
//     pub targets: Vec<CodegenInstance>,
// }

pub type GenerateFn<'langs> =
    dyn for<'a> Fn(&'a Component2SynPath, syn::Path) -> syn::ItemMod + 'langs;

pub struct CodegenInstance<'langs> {
    pub id: KebabCodegenId,
    pub generate: Box<GenerateFn<'langs>>,
    pub external_deps: Vec<&'static str>,
    pub workspace_deps: Vec<(&'static str, &'static Path)>,
    pub codegen_deps: CgDepList<'langs>,
}

pub struct Crate<'langs> {
    pub id: KebabCodegenId,
    pub provides: Vec<CodegenInstance<'langs>>,
    pub global_workspace_deps: Vec<(&'static str, &'static Path)>,
}

#[must_use]
pub struct CgDepList<'langs> {
    codegen_deps: Vec<CodegenInstance<'langs>>,
}

impl<'langs> CgDepList<'langs> {
    pub fn new() -> Self {
        CgDepList {
            codegen_deps: Vec::new(),
        }
    }
    pub fn add(
        &mut self,
        dep: CodegenInstance<'langs>,
    ) -> Box<dyn for<'a> Fn(&'a Component2SynPath) -> syn::Path + 'static> {
        let id = dep.id.clone();
        self.codegen_deps.push(dep);
        Box::new(move |c2sp| {
            let path = c2sp
                .0
                .get(&id)
                .expect("dependency was not listed in the global dependency list");
            syn::parse_quote! {
                #path
            }
        })
    }
    pub fn subtree(&self) -> CgDepList<'langs> {
        CgDepList {
            codegen_deps: vec![],
        }
    }
}

impl<'langs> Default for CgDepList<'langs> {
    fn default() -> Self {
        Self::new()
    }
}

const WORKSPACE_ROOT_RELPATH: &str = "../..";
const SIBLING_CRATES_RELPATH: &str = "..";

fn write_if_changed(path: &Path, content: &str, files_created: &mut Vec<PathBuf>) {
    if path.exists() {
        let existing_content = std::fs::read_to_string(path).unwrap();
        files_created.push(path.canonicalize().unwrap().to_path_buf());
        if existing_content == content {
            return;
        }
    }
    std::fs::write(path, content).unwrap();
    files_created.push(path.canonicalize().unwrap().to_path_buf());
}
fn delete_files_not_created(base_path: &Path, files_created: &[PathBuf]) {
    let mut files_to_delete = vec![];
    for entry in std::fs::read_dir(base_path).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path().canonicalize().unwrap();
        if path.is_file() && !files_created.contains(&path) {
            files_to_delete.push(path.clone());
        }
        if path.is_dir() {
            delete_files_not_created(&path, files_created);
            if path.read_dir().unwrap().count() == 0 {
                std::fs::remove_dir(path).unwrap();
            }
        }
    }
    for path in files_to_delete {
        std::fs::remove_file(path).unwrap();
    }
}
impl<'langs> Crate<'langs> {
    pub fn generate(&self, base_path: &Path, c2sp: &mut Component2SynPath, other_crates: &[Crate]) {
        let crate_path = base_path.join(self.crate_relpath());
        std::fs::create_dir_all(&crate_path).unwrap();
        let mut files_created = vec![];
        {
            let cargo_toml_path = crate_path.join("Cargo.toml");
            let cargo_toml_content = self.generate_cargo_toml(other_crates);
            write_if_changed(&cargo_toml_path, &cargo_toml_content, &mut files_created);
        }
        {
            let src = crate_path.join("src");
            std::fs::create_dir_all(&src).unwrap();
            let contents = self.generate_dep_contents(c2sp);
            let content_names = contents
                .iter()
                .map(|content| content.ident.clone())
                .collect::<Vec<_>>();
            for content in contents {
                let file = syn::File {
                    shebang: None,
                    attrs: content
                        .attrs
                        .into_iter()
                        .map(|mut it| {
                            it.style = syn::AttrStyle::Inner(syn::token::Not(
                                proc_macro2::Span::call_site(),
                            ));
                            it
                        })
                        .collect(),
                    items: content.content.unwrap().1,
                };
                let file_path = src.join(format!("{}.rs", content.ident));
                write_if_changed(
                    &file_path,
                    &prettyplease::unparse(&syn_insert_use::insert_use(file)),
                    &mut files_created,
                );
            }
            let file_path = src.join("lib.rs");
            let lib: syn::File = syn::parse_quote! {
                #(
                    pub mod #content_names;
                )*
            };
            write_if_changed(
                &file_path,
                &prettyplease::unparse(&syn_insert_use::insert_use(lib)),
                &mut files_created,
            );
        }
        delete_files_not_created(&crate_path, &files_created);
    }
    pub fn provides(&self, id: &KebabCodegenId) -> bool {
        self.provides.iter().any(|it| it.id == *id)
    }
    pub fn plus(
        self,
        provides: Vec<CodegenInstance<'langs>>,
        deps: Vec<(&'static str, &'static Path)>,
    ) -> Self {
        let added_provides = provides.into_iter();
        let added_deps = deps.into_iter();
        let Self {
            id,
            mut provides,
            mut global_workspace_deps,
        } = self;
        provides.extend(added_provides);
        global_workspace_deps.extend(added_deps);
        Self {
            id,
            provides,
            global_workspace_deps,
        }
    }
    fn generate_dep_contents(&self, c2sp: &mut Component2SynPath) -> Vec<syn::ItemMod> {
        let mut current_c2sp = Component2SynPath(c2sp.0.clone());
        let mut contents = vec![];
        for dep in self.provides.iter() {
            self.generate_single_dep_contents(dep, &mut current_c2sp, c2sp, &mut contents);
        }
        contents
    }
    fn generate_single_dep_contents(
        &self,
        dep: &CodegenInstance<'_>,
        current_c2sp: &mut Component2SynPath,
        global_c2sp: &mut Component2SynPath,
        contents: &mut Vec<syn::ItemMod>,
    ) {
        if current_c2sp.0.contains_key(&dep.id) {
            return;
        }
        
        // Mark this dependency as in-progress by inserting into maps FIRST
        let dep_ident = &dep.id.to_snake_ident();
        current_c2sp.0.insert(
            dep.id.clone(),
            syn::parse_quote! {
                crate::#dep_ident
            },
        );
        let crate_ident = self.crate_ident();
        global_c2sp.0.insert(
            dep.id.clone(),
            syn::parse_quote! {
                #crate_ident::#dep_ident
            },
        );
        
        // Then process dependencies (which will now see this as already processed)
        for depdep in dep.codegen_deps.codegen_deps.iter() {
            self.generate_single_dep_contents(depdep, current_c2sp, global_c2sp, contents);
        }
        
        // Finally generate the module
        let mut m = (dep.generate)(current_c2sp, current_c2sp.0.get(&dep.id).unwrap().clone());
        m.ident = dep_ident.clone();
        contents.push(m);
    }
    fn crate_ident(&self) -> syn::Ident {
        self.id.to_snake_ident()
    }
    fn crate_relpath(&self) -> PathBuf {
        self.id.0.kebab_str().into()
    }
    fn all_external_deps(&self) -> Vec<&'static str> {
        fn all_external_deps<'a, 'b>(
            provides: &'a CodegenInstance<'b>,
        ) -> Box<dyn Iterator<Item = &'static str> + 'a> {
            let transitive = provides
                .codegen_deps
                .codegen_deps
                .iter()
                .flat_map(all_external_deps);
            Box::new(provides.external_deps.iter().copied().chain(transitive))
        }
        self.provides.iter().flat_map(all_external_deps).collect()
    }
    fn all_workspace_deps(&self) -> Vec<(&'static str, &'static Path)> {
        fn all_workspace_deps<'a, 'b>(
            provides: &'a CodegenInstance<'b>,
        ) -> Box<dyn Iterator<Item = (&'static str, &'static Path)> + 'a> {
            let transitive = provides
                .codegen_deps
                .codegen_deps
                .iter()
                .flat_map(all_workspace_deps);
            Box::new(provides.workspace_deps.iter().copied().chain(transitive))
        }
        self.provides.iter().flat_map(all_workspace_deps).collect()
    }
    // fn all_codegen_deps<'a, 'b>(
    //     &self,
    //     other_crates: &'a [Crate<'b>],
    //     result: &mut Vec<&'a Crate<'b>>,
    // ) {
    //     // fn all_codegen_deps<'a, 'b>(
    //     //     provides: &'a Crate<'b>,
    //     // ) -> Box<dyn Iterator<Item = (&'static str, &'static Path)> + 'a> {
    //     //     provides.codegen_deps.iter().flat_map(all_codegen_deps)
    //     // }
    //     for dep in other_crates.iter().filter(|it| {
    //         self.provides
    //             .iter()
    //             .flat_map(|it| it.codegen_deps.codegen_deps.iter())
    //             .any(|dep| it.provides(&dep.id))
    //             && it.id != self.id
    //     }) {
    //         if !result.iter().any(|it| std::ptr::eq(*it, dep)) {
    //             result.push(dep);
    //             println!(
    //                 "dbg: adding {} to {}",
    //                 dep.crate_ident(),
    //                 self.crate_ident()
    //             );
    //             dep.all_codegen_deps(other_crates, result);
    //         }
    //     }
    // }
    fn all_codegen_deps(&self) -> Vec<&CodegenInstance<'langs>> {
        fn all_codegen_deps<'a, 'b>(
            provides: &'a CodegenInstance<'b>,
        ) -> Box<dyn Iterator<Item = &'a CodegenInstance<'b>> + 'a> {
            let it = std::iter::once(provides).chain(
                provides
                    .codegen_deps
                    .codegen_deps
                    .iter()
                    .flat_map(all_codegen_deps),
            );
            Box::new(it)
        }
        self.provides.iter().flat_map(all_codegen_deps).collect()
    }
    fn generate_cargo_toml(&self, other_crates: &[Crate]) -> String {
        // let depends_on_crates = other_crates
        //     .iter()
        //     .filter(|it| {
        //         self.provides
        //             .iter()
        //             .flat_map(|it| it.codegen_deps.codegen_deps.iter())
        //             .any(|dep| it.provides(&dep.id))
        //             && it.id != self.id
        //     })
        //     .collect::<Vec<_>>();
        let acd = self.all_codegen_deps();
        let depends_on_crates = other_crates
            .iter()
            .filter(|c| c.id != self.id && acd.iter().any(|dep| c.provides(&dep.id)));
        let workspace_dep2entry = |dep: &(&str, &Path)| {
            (
                dep.0.to_string(),
                toml::Value::Table(
                    [(
                        "path".into(),
                        toml::Value::String(
                            PathBuf::from(WORKSPACE_ROOT_RELPATH)
                                .join(dep.1)
                                .join(dep.0)
                                .to_string_lossy()
                                .into(),
                        ),
                    )]
                    .into_iter()
                    .collect::<toml::Table>(),
                ),
            )
        };
        let cargo_toml = [
            (
                String::from("package"),
                toml::Value::Table(
                    [
                        (
                            "name".into(),
                            toml::Value::String(self.id.0.kebab_str().to_owned()),
                        ),
                        ("version".into(), toml::Value::String("0.0.0".into())),
                        ("edition".into(), toml::Value::String("2024".into())),
                    ]
                    .into_iter()
                    .collect::<toml::Table>(),
                ),
            ),
            (
                String::from("dependencies"),
                toml::Value::Table(
                    self.global_workspace_deps
                        .iter()
                        .map(workspace_dep2entry)
                        .chain(self.all_workspace_deps().iter().map(workspace_dep2entry))
                        .chain(self.all_external_deps().iter().map(|dep| {
                            (
                                dep.to_string(),
                                toml::Value::Table(
                                    [("workspace".into(), toml::Value::Boolean(true))]
                                        .into_iter()
                                        .collect::<toml::Table>(),
                                ),
                            )
                        }))
                        .chain(depends_on_crates.map(|dep| {
                            (
                                dep.id.to_string(),
                                toml::Value::Table(
                                    [(
                                        "path".into(),
                                        toml::Value::String(
                                            PathBuf::from(SIBLING_CRATES_RELPATH)
                                                .join(dep.crate_relpath())
                                                .to_string_lossy()
                                                .into(),
                                        ),
                                    )]
                                    .into_iter()
                                    .collect::<toml::Table>(),
                                ),
                            )
                        }))
                        .collect::<toml::Table>(),
                ),
            ),
        ]
        .into_iter()
        .collect::<toml::Table>();
        toml::to_string(&cargo_toml).unwrap()
    }
}
