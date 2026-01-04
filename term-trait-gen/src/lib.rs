use langspec::langspec::{LangSpec, SortIdOf};
use langspec_rs_syn::{AlgebraicsBasePath, HeapType, heapbak_gen_datas, sort2word_rs_ty};
use rustgen_utils::byline;
use syn::parse_quote;

use langspec::langspec::SortId;

pub fn generate<L: LangSpec>(
    base_path: &syn::Path,
    words_path: &syn::Path,
    ls: &L,
) -> syn::ItemMod {
    // let owned = owned::generate(base_path, words_path, ls);
    let heap_trait = heap_trait(base_path, words_path, ls);
    let byline = byline!();
    parse_quote!(
        #byline
        pub mod term_trait {
            #heap_trait
            // #owned
        }
    )
}
pub(crate) fn heap_trait<L: LangSpec>(
    base_path: &syn::Path,
    words_path: &syn::Path,
    ls: &L,
) -> syn::ItemTrait {
    // transpose!(
    //     ls.ty_gen_datas(Some(syn::parse_quote! {#words_path})),
    //     camel_ident
    // );
    let byline = byline!();
    let superheap_bounds = generate_superheap_bounds(ls, words_path);
    let inverse_implements_bounds = generate_inverse_implements_bounds(words_path, ls);
    parse_quote! {
        #byline
        pub trait Heap: Sized #(+ #inverse_implements_bounds)* #(+ #superheap_bounds)* {
            // #(
            //     type #camel_ident: #base_path::owned::#camel_ident<Self>;
            // )*
        }
    }
}

fn generate_superheap_bounds<L: LangSpec>(
    ls: &L,
    words_path: &syn::Path,
) -> Vec<syn::TraitBound> {
    heapbak_gen_datas(ls)
        .iter()
        .map(|hgd| -> syn::Type {
            // let ty_func = &hgd.ty_func.ty_func;
            let heapbak = (hgd.heapbak)(
                &HeapType(syn::parse_quote! {Self}),
                &AlgebraicsBasePath::new(quote::quote! {Self::}),
                Some(words_path),
            );
            syn::parse_quote! {
                #heapbak
            }
        })
        .map(|ty| {
            syn::parse_quote! {
                term::SuperHeap<#ty>
            }
        })
        .collect()
}

fn generate_inverse_implements_bounds<L: LangSpec>(
    words_path: &syn::Path,
    ls: &L,
) -> impl Iterator<Item = syn::TraitBound> {
    ls.all_sort_ids().map(move |sid| {
        let lword = sort2word_rs_ty(ls, sid.clone(), words_path);
        match sid {
            SortId::Algebraic(asi) => {
                let camel_ident = ls.algebraic_sort_name(asi).camel_ident();
                syn::parse_quote! {
                    words::InverseImplements<
                        #words_path::L,
                        #lword,
                        // StructuralImplementor: #base_path::owned::#camel_ident<Self>
                    >
                }
            }
            SortId::TyMetaFunc(_) => syn::parse_quote! {
                words::InverseImplements<
                    #words_path::L,
                    #lword,
                    ExternBehavioralImplementor: term::TyMetaFunc
                >
            },
        }
    })
}

// mod owned {

//     use super::*;
//     pub fn generate<L: LangSpec>(
//         base_path: &syn::Path,
//         words_path: &syn::Path,
//         ls: &LsGen<L>,
//     ) -> syn::ItemMod {
//         let traits = ls.ty_gen_datas(Some(syn::parse_quote! {#words_path})).map(
//             |TyGenData {
//                  camel_ident,
//                  ccf: CanonicallyConstructibleFromGenData { ccf_sort_tys, .. },
//                  ..
//              }|
//              -> syn::ItemTrait {
//                 let path = quote::quote! {
//                     <Heap as #base_path::Heap>
//                 };
//                 let ccf_sort_tys = ccf_sort_tys(
//                     HeapType(syn::parse_quote! {Heap}),
//                     AlgebraicsBasePath::new(quote::quote! { #path:: }),
//                 );
//                 let ccf_bounds = ccf_sort_tys.iter().map(|ccf| -> syn::TraitBound {
//                     syn::parse_quote! {
//                         ccf::CanonicallyConstructibleFrom<Heap, #ccf>
//                     }
//                 });
//                 parse_quote! {
//                     pub trait #camel_ident<Heap>: #(#ccf_bounds )+*
//                     where Heap: #base_path::Heap,
//                     {
//                     }
//                 }
//             },
//         );
//         let byline = byline!();
//         parse_quote! {
//             #byline
//             pub mod owned {
//                 #(
//                     #traits
//                 )*
//             }
//         }
//     }
// }

pub mod targets {
    use std::path::Path;

    use codegen_component::{CgDepList, CodegenInstance, bumpalo};
    use rustgen_utils::kebab_id;

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
