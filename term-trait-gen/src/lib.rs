use langspec::langspec::{LangSpec, SortId};
use langspec_gen_util::HeapType;
use langspec_gen_util::{LsGen, byline, transpose};
use syn::parse_quote;

use langspec_gen_util::{AlgebraicsBasePath, CanonicallyConstructibleFromGenData, TyGenData};

pub fn generate<L: LangSpec>(base_path: &syn::Path, ls: &L) -> syn::ItemMod {
    let lg = LsGen::from(ls);
    let owned = owned::generate(base_path, &lg);
    let heap_trait = heap_trait(base_path, &lg);
    let byline = byline!();
    let words = words::words_mod(&lg);
    parse_quote!(
        #byline
        pub mod term_trait {
            #heap_trait
            #owned
            #words
        }
    )
}
pub(crate) fn heap_trait<L: LangSpec>(base_path: &syn::Path, ls: &LsGen<L>) -> syn::ItemTrait {
    transpose!(
        ls.ty_gen_datas(Some(syn::parse_quote! {#base_path::words})),
        camel_ident
    );
    let byline = byline!();
    let superheap_bounds = generate_superheap_bounds(ls);
    let maps_tmf_bounds = generate_maps_tmf_bounds(base_path, ls);
    parse_quote! {
        #byline
        pub trait Heap: Sized #(+ #maps_tmf_bounds)* #(+ #superheap_bounds)* {
            #(
                type #camel_ident: #base_path::owned::#camel_ident<Heap = Self>;
            )*
        }
    }
}

fn generate_superheap_bounds<L: LangSpec>(ls: &LsGen<L>) -> Vec<syn::TraitBound> {
    ls.heapbak_gen_datas()
        .iter()
        .map(|hgd| -> syn::Type {
            let ty_func = &hgd.ty_func.ty_func;
            let args = (hgd.ty_args)(
                HeapType(syn::parse_quote! {Self}),
                AlgebraicsBasePath::new(quote::quote! {Self::}),
            );
            syn::parse_quote! {
                #ty_func<Self, #(#args),*>
            }
        })
        .map(|ty| {
            syn::parse_quote! {
                term::SuperHeap<#ty>
            }
        })
        .collect()
}

fn generate_maps_tmf_bounds<L: LangSpec>(
    base_path: &syn::Path,
    ls: &LsGen<L>,
) -> impl Iterator<Item = syn::TraitBound> {
    let mut bounds = vec![];
    langspec::langspec::call_on_all_tmf_monomorphizations(ls.bak(), &mut |mt| {
        let tmf = ls.sort2rs_ty(
            SortId::TyMetaFunc(mt.clone()),
            &HeapType(syn::parse_quote! {Self}),
            &AlgebraicsBasePath::new(quote::quote! {Self::}),
        );
        bounds.push(syn::parse_quote! {
            term::MapsTmf<#base_path::words::L, #tmf>
        });
    });
    bounds.into_iter()
}

mod owned {

    use super::*;
    pub fn generate<L: LangSpec>(base_path: &syn::Path, ls: &LsGen<L>) -> syn::ItemMod {
        let traits = ls
            .ty_gen_datas(Some(syn::parse_quote! {#base_path::words}))
            .map(
                |TyGenData {
                     camel_ident,
                     ccf: CanonicallyConstructibleFromGenData { ccf_sort_tys, .. },
                     ..
                 }|
                 -> syn::ItemTrait {
                    let path = quote::quote! {
                        <<Self as term::Heaped>::Heap as #base_path::Heap>
                    };
                    let ccf_sort_tys = ccf_sort_tys(
                        HeapType(syn::parse_quote! {<Self as term::Heaped>::Heap}),
                        AlgebraicsBasePath::new(quote::quote! { #path:: }),
                    );
                    let ccf_bounds = ccf_sort_tys.iter().map(|ccf| -> syn::TraitBound {
                        syn::parse_quote! {
                            term::CanonicallyConstructibleFrom<<Self as term::Heaped>::Heap, #ccf>
                        }
                    });
                    parse_quote! {
                        pub trait #camel_ident: term::Heaped #(+ #ccf_bounds )*
                        where <Self as term::Heaped>::Heap: #base_path::Heap,
                        {
                        }
                    }
                },
            );
        let byline = byline!();
        parse_quote! {
            #byline
            pub mod owned {
                #(
                    #traits
                )*
            }
        }
    }
}

pub mod targets {
    use codegen_component::{CgDepList, CodegenInstance, bumpalo};
    use langspec_gen_util::kebab_id;

    pub fn default<'langs, L: super::LangSpec>(
        _: &'langs bumpalo::Bump,
        codegen_deps: CgDepList<'langs>,
        l: &'langs L,
    ) -> CodegenInstance<'langs> {
        CodegenInstance {
            id: kebab_id!(l),
            generate: {
                let self_path = codegen_deps.self_path();
                Box::new(move |_| super::generate(&syn::parse_quote! {#self_path::term_trait}, l))
            },
            external_deps: vec![],
            workspace_deps: vec!["term", "term-trait-gen"],
            codegen_deps,
        }
    }
}
