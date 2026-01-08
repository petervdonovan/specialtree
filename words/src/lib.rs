use aspect::Aspect;
// #![feature(fundamental)]
use conslist::ConsList;
use langspec::{
    langspec::{LangSpec, SortIdOf},
    sublang::Sublang,
};
use langspec_rs_syn::{AlgebraicsBasePath, HeapType, sort2rs_ty, sort2word_rs_ty, ty_gen_datas};
use memo::memo_cache::thread_local_cache;
use rustgen_utils::byline;

pub trait Implements<Heap, L, AspectT: Aspect> {
    type LWord;
}
pub trait InverseImplements<L, LWord, AspectT: Aspect> {
    type Implementor;
}
pub trait InverseImplementsAll<L, LWords: ConsList, AspectT: Aspect>:
    InverseImplements<L, <LWords as ConsList>::Car, AspectT>
{
    type Implementors: ConsList;
}
impl<Heap, L, AspectT: Aspect> InverseImplements<L, (), AspectT> for Heap {
    type Implementor = ();
}
impl<Heap, L, Car, Cdr, AspectT: Aspect> InverseImplementsAll<L, (Car, Cdr), AspectT> for Heap
where
    Cdr: ConsList,
    Heap: InverseImplementsAll<L, Cdr, AspectT> + InverseImplements<L, Car, AspectT>,
{
    type Implementors = (
        <Heap as InverseImplements<L, Car, AspectT>>::Implementor,
        <Heap as InverseImplementsAll<L, Cdr, AspectT>>::Implementors,
    );
}
impl<Heap, L, AspectT: Aspect> InverseImplementsAll<L, (), AspectT> for Heap {
    type Implementors = ();
}

pub fn words_mod<L: LangSpec>(lg: &L) -> syn::ItemMod {
    let sort_camel_idents = ty_gen_datas(thread_local_cache(), lg, None)
        .iter()
        .map(|it| it.camel_ident.clone())
        .collect::<Vec<_>>();
    let byline = rustgen_utils::byline!();
    syn::parse_quote! {
        #byline
        pub mod words {
            pub struct L;
            pub mod sorts {
                #(
                    pub struct #sort_camel_idents;
                )*
            }
            pub mod impls {
                #(
                    impl<A: aspect::Aspect> aspect::Adtishness<A> for super::sorts::#sort_camel_idents {
                        type X = aspect::AdtLike;
                    }
                )*
            }
        }
    }
}

pub fn words_impls<L: LangSpec, LSub: LangSpec>(
    ext_data_structure: &syn::Path,
    og_words_base_path: &syn::Path,
    sublang: &Sublang<'_, LSub, SortIdOf<L>>,
    elsg: &L,
) -> syn::ItemMod {
    let impls: Vec<syn::ItemImpl> = sublang
        .lsub
        .all_sort_ids()
        .flat_map(|sid| {
            let word = sort2word_rs_ty(sublang.lsub, sid.clone(), og_words_base_path);
            sublang.aspect_implementors.iter().map(move |asp| {
                let aspect_path = asp.aspect_zst.zst_path();
                let implementor = sort2rs_ty(
                    elsg,
                    (asp.map)(&sid),
                    &HeapType(syn::parse_quote! { #ext_data_structure::Heap }),
                    &AlgebraicsBasePath::new(syn::parse_quote! { #ext_data_structure:: }),
                );
                syn::parse_quote! {
                    impl words::Implements<#ext_data_structure::Heap, #og_words_base_path::L, #aspect_path> for #implementor {
                        type LWord = #word;
                    }
                }
            })
        })
        .collect::<Vec<_>>();
    let byline = byline!();
    syn::parse_quote! {
        #byline
        pub mod words_impls {
            #(#impls)*
        }
    }
}

pub fn words_inverse_impls<L: LangSpec, LSub: LangSpec>(
    ext_data_structure: &syn::Path,
    og_words_base_path: &syn::Path,
    sublang: &Sublang<'_, LSub, SortIdOf<L>>,
    elsg: &L,
) -> syn::ItemMod {
    let impls: Vec<syn::ItemImpl> = sublang
        .lsub
        .all_sort_ids()
        .flat_map(|sid| {
            let word = sort2word_rs_ty(sublang.lsub, sid.clone(), og_words_base_path);
            sublang.aspect_implementors.iter().map(move |asp| {
                let implementor_sid = (asp.map)(&sid);
                let ht = HeapType(syn::parse_quote! { #ext_data_structure::Heap });
                let abp = AlgebraicsBasePath::new(syn::parse_quote! { #ext_data_structure:: });
                let implementor = sort2rs_ty(elsg, implementor_sid.clone(), &ht, &abp);
                let aspect_path = asp.aspect_zst.zst_path();
                syn::parse_quote! {
                    impl words::InverseImplements<#og_words_base_path::L, #word, #aspect_path> for #ext_data_structure::Heap {
                        type Implementor = #implementor;
                    }
                }
            })
        })
        .collect::<Vec<_>>();
    let byline = byline!();
    syn::parse_quote! {
        #byline
        pub mod words_inverse_impls {
            #(#impls)*
        }
    }
}

pub mod targets {
    use codegen_component::{CgDepList, CodegenInstance, bumpalo};
    use rustgen_utils::kebab_id;

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
            generate: Box::new(move |_, _| super::words_mod(l)),
        }
    }
}
