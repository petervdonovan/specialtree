use langspec::langspec::{LangSpec, SortId};
use langspec_gen_util::{HeapType, proc_macro2};
use langspec_gen_util::{LsGen, byline, transpose};
use quote::ToTokens;
use syn::parse_quote;

use langspec_gen_util::{AlgebraicsBasePath, CanonicallyConstructibleFromGenData, TyGenData};

pub fn generate<L: LangSpec>(
    base_path: &syn::Path,
    words_path: &syn::Path,
    ls: &L,
) -> syn::ItemMod {
    let lg = LsGen::from(ls);
    let owned = owned::generate(base_path, words_path, &lg);
    let heap_trait = heap_trait(base_path, words_path, &lg);
    let byline = byline!();
    parse_quote!(
        #byline
        pub mod term_trait {
            #heap_trait
            #owned
        }
    )
}
pub(crate) fn heap_trait<L: LangSpec>(
    base_path: &syn::Path,
    words_path: &syn::Path,
    ls: &LsGen<L>,
) -> syn::ItemTrait {
    transpose!(
        ls.ty_gen_datas(Some(syn::parse_quote! {#words_path})),
        camel_ident
    );
    let byline = byline!();
    let superheap_bounds = generate_superheap_bounds(ls);
    let (maps_tmf_bounds, tmf_helper_types, _tmf_helper_type_impls) =
        generate_maps_tmf_bounds(words_path, ls);
    parse_quote! {
        #byline
        pub trait Heap: Sized #(+ #maps_tmf_bounds)* #(+ #superheap_bounds)* {
            #(
                type #camel_ident: #base_path::owned::#camel_ident<Self>;
            )*
            #(
                #tmf_helper_types
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
    words_path: &syn::Path,
    ls: &LsGen<L>,
) -> (
    Vec<syn::TraitBound>,
    Vec<syn::TraitItemType>,
    Vec<syn::ImplItemType>,
) {
    let mut bounds = vec![];
    let mut helper_types = vec![];
    let mut helper_type_impls = vec![];
    langspec::langspec::call_on_all_tmf_monomorphizations(ls.bak(), &mut |mt| {
        // let tmf = ls.sort2rs_ty(
        //     SortId::TyMetaFunc(mt.clone()),
        //     &HeapType(syn::parse_quote! {Self}),
        //     &AlgebraicsBasePath::new(quote::quote! {Self::}),
        //     // words_path,
        // );
        let tmf = ls.sort2rs_ty_with_tmfmapped_args(
            SortId::TyMetaFunc(mt.clone()),
            &HeapType(syn::parse_quote! {Self}),
            &AlgebraicsBasePath::new(quote::quote! {Self::}),
            words_path,
        );
        let helper_type_name = tmf
            .to_token_stream()
            .into_iter()
            .map(|it| it.to_string())
            .filter(|it| it.chars().next().unwrap().is_uppercase())
            .filter(|it| it.chars().all(|char| char.is_alphanumeric() || char == '_'))
            .filter(|it| it != "Heap" && it != "Self")
            .fold(String::new(), |acc, next| acc + &next);
        let helper_type_name = syn::Ident::new(&helper_type_name, proc_macro2::Span::call_site());
        helper_types.push(syn::parse_quote! {
            type #helper_type_name: type_equals::TypeEquals<Other = #tmf>;
        });
        helper_type_impls.push(syn::parse_quote! {
            type #helper_type_name = #tmf;
        });
        bounds.push(syn::parse_quote! {
            term::MapsTmf<#words_path::L, Self::#helper_type_name>
        });
    });
    bounds.sort_by_cached_key(|s: &syn::TraitBound| {
        -(s.to_token_stream().into_iter().count() as isize)
    });
    (bounds, helper_types, helper_type_impls)
}

mod owned {

    use super::*;
    pub fn generate<L: LangSpec>(
        base_path: &syn::Path,
        words_path: &syn::Path,
        ls: &LsGen<L>,
    ) -> syn::ItemMod {
        let traits = ls.ty_gen_datas(Some(syn::parse_quote! {#words_path})).map(
            |TyGenData {
                 camel_ident,
                 ccf: CanonicallyConstructibleFromGenData { ccf_sort_tys, .. },
                 ..
             }|
             -> syn::ItemTrait {
                let path = quote::quote! {
                    <Heap as #base_path::Heap>
                };
                let ccf_sort_tys = ccf_sort_tys(
                    HeapType(syn::parse_quote! {Heap}),
                    AlgebraicsBasePath::new(quote::quote! { #path:: }),
                );
                let ccf_bounds = ccf_sort_tys.iter().map(|ccf| -> syn::TraitBound {
                    syn::parse_quote! {
                        ccf::CanonicallyConstructibleFrom<Heap, #ccf>
                    }
                });
                parse_quote! {
                    pub trait #camel_ident<Heap>: #(#ccf_bounds )+*
                    where Heap: #base_path::Heap,
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
    use std::path::Path;

    use codegen_component::{CgDepList, CodegenInstance, bumpalo};
    use langspec_gen_util::kebab_id;

    pub fn default<'langs, L: super::LangSpec>(
        arena: &'langs bumpalo::Bump,
        mut codegen_deps: CgDepList<'langs>,
        l: &'langs L,
    ) -> CodegenInstance<'langs> {
        CodegenInstance {
            id: kebab_id!(l),
            generate: {
                let words_path =
                    codegen_deps.add(words::targets::words_mod(arena, codegen_deps.subtree(), l));
                Box::new(move |c, sp| super::generate(&sp, &words_path(c), l))
            },
            external_deps: vec![],
            workspace_deps: vec![
                ("term", Path::new(".")),
                ("term-trait-gen", Path::new(".")),
                ("ccf", Path::new(".")),
            ],
            codegen_deps,
        }
    }
}
