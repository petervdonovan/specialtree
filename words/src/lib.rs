#![feature(fundamental)]
use langspec::{
    langspec::{LangSpec, SortIdOf},
    sublang::Sublang,
};
use langspec_gen_util::{AlgebraicsBasePath, HeapType, LsGen, byline};

pub trait Implements<Heap, L> {
    type LWord;
}
#[fundamental]
pub trait Adt {}

pub fn words_mod<L: LangSpec>(lg: &LsGen<L>) -> syn::ItemMod {
    let sort_camel_idents = lg.ty_gen_datas(None).map(|it| it.camel_ident);
    let byline = langspec_gen_util::byline!();
    syn::parse_quote! {
        #byline
        pub mod words {
            pub struct L;
            pub mod sorts {
                #(
                    pub struct #sort_camel_idents;
                )*
            }
        }
    }
}

pub fn words_impls<L: LangSpec, LSub: LangSpec>(
    ext_data_structure: &syn::Path,
    og_words_base_path: &syn::Path,
    sublang: &Sublang<'_, LSub, SortIdOf<L>>,
    elsg: &LsGen<L>,
) -> syn::ItemMod {
    let oglsg = LsGen::from(sublang.lsub);
    let impls = oglsg
        .bak()
        .all_sort_ids()
        .map(|sid| {
            let skeleton = oglsg.sort2word_rs_ty(sid.clone(), og_words_base_path);
            let ty = elsg.sort2rs_ty(
                (sublang.map)(&sid),
                &HeapType(syn::parse_quote! { #ext_data_structure::Heap }),
                &AlgebraicsBasePath::new(syn::parse_quote! { #ext_data_structure:: }),
            );
            (ty, skeleton)
        })
        .map(|(ty, skeleton)| -> syn::ItemImpl {
            syn::parse_quote! {
                impl words::Implements<#ext_data_structure::Heap, #og_words_base_path::L> for #ty {
                    type LWord = #skeleton;
                }
            }
        });
    let byline = byline!();
    syn::parse_quote! {
        #byline
        pub mod words_impls {
            #(#impls)*
        }
    }
}

pub mod targets {
    use codegen_component::{CgDepList, CodegenInstance, bumpalo};
    use langspec_gen_util::kebab_id;

    pub fn words_mod<'langs, L: super::LangSpec>(
        _: &'langs bumpalo::Bump,
        codegen_deps: CgDepList<'langs>,
        l: &'langs L,
    ) -> CodegenInstance<'langs> {
        CodegenInstance {
            id: kebab_id!(l),
            external_deps: vec![],
            workspace_deps: vec![],
            codegen_deps,
            generate: Box::new(move |_, _| super::words_mod(&super::LsGen::from(l))),
        }
    }
}
