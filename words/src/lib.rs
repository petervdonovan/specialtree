#![feature(fundamental)]
use conslist::ConsList;
use langspec::{
    langspec::{LangSpec, SortIdOf},
    sublang::Sublang,
};
use langspec_gen_util::{AlgebraicsBasePath, HeapType, LsGen, byline};

pub trait Implements<Heap, L> {
    type LWord;
}
pub trait InverseImplements<L, LWord> {
    type ExternBehavioralImplementor;
    type StructuralImplementor;
}
pub trait InverseImplementsAll<L, LWords: ConsList>:
    InverseImplements<L, <LWords as ConsList>::Car>
{
    type ExternBehavioralImplementors: ConsList;
    type StructuralImplementors: ConsList;
}
impl<Heap, L> InverseImplements<L, ()> for Heap {
    type ExternBehavioralImplementor = ();
    type StructuralImplementor = ();
}
impl<Heap, L, Car, Cdr> InverseImplementsAll<L, (Car, Cdr)> for Heap
where
    Cdr: ConsList,
    Heap: InverseImplementsAll<L, Cdr> + InverseImplements<L, Car>,
{
    type ExternBehavioralImplementors = (
        <Heap as InverseImplements<L, Car>>::ExternBehavioralImplementor,
        <Heap as InverseImplementsAll<L, Cdr>>::ExternBehavioralImplementors,
    );
    type StructuralImplementors = (
        <Heap as InverseImplements<L, Car>>::StructuralImplementor,
        <Heap as InverseImplementsAll<L, Cdr>>::StructuralImplementors,
    );
}
impl<Heap, L> InverseImplementsAll<L, ()> for Heap {
    type ExternBehavioralImplementors = ();
    type StructuralImplementors = ();
}
// #[fundamental]
// pub trait Adt {}

pub struct AdtLike;
pub struct NotAdtLike;
// pub enum AdtLikeOrNot {
//     AdtLike,
//     NonAdtLike,
// }
pub trait AdtLikeOrNot {}
impl AdtLikeOrNot for AdtLike {}
impl AdtLikeOrNot for NotAdtLike {}
pub trait Aspect {}

pub trait Adtishness<A: Aspect> {
    type X: AdtLikeOrNot;
}

pub fn words_mod<L: LangSpec>(lg: &LsGen<L>) -> syn::ItemMod {
    let sort_camel_idents = lg
        .ty_gen_datas(None)
        .map(|it| it.camel_ident)
        .collect::<Vec<_>>();
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
            pub mod impls {
                #(
                    impl<A: words::Aspect> words::Adtishness<A> for super::sorts::#sort_camel_idents {
                        type X = words::AdtLike;
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
    elsg: &LsGen<L>,
) -> syn::ItemMod {
    let oglsg = LsGen::from(sublang.lsub);
    let impls = oglsg
        .bak()
        .all_sort_ids()
        .map(|sid| {
            let word = oglsg.sort2word_rs_ty(sid.clone(), og_words_base_path);
            let structural_implementor = elsg.sort2rs_ty(
                (sublang.map)(&sid),
                &HeapType(syn::parse_quote! { #ext_data_structure::Heap }),
                &AlgebraicsBasePath::new(syn::parse_quote! { #ext_data_structure:: }),
            );
            (structural_implementor, word)
        })
        .map(|(structural_implementor, word)| -> syn::ItemImpl {
            syn::parse_quote! {
                impl words::Implements<#ext_data_structure::Heap, #og_words_base_path::L> for #structural_implementor {
                    type LWord = #word;
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

pub fn words_inverse_impls<L: LangSpec, LSub: LangSpec>(
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
            let word = oglsg.sort2word_rs_ty(sid.clone(), og_words_base_path);
            let structural_implementor_sid = (sublang.map)(&sid);
            let ht = HeapType(syn::parse_quote! { #ext_data_structure::Heap });
            let abp = AlgebraicsBasePath::new(syn::parse_quote! { #ext_data_structure:: });
            let structural_implementor =
                elsg.sort2rs_ty(structural_implementor_sid.clone(), &ht, &abp);
            let behavioral_implementor = sublang
                .tems
                .iter()
                .find(|it| it.to_structural == structural_implementor_sid)
                .map(|tem| elsg.sort2rs_ty(tem.from_extern_behavioral.clone(), &ht, &abp))
                .unwrap_or_else(|| structural_implementor.clone());
            (structural_implementor, behavioral_implementor, word)
        })
        .map(
            |(structural_implementor, behavioral_implementor, word)| -> syn::ItemImpl {
                syn::parse_quote! {
                    impl words::InverseImplements<#og_words_base_path::L, #word> for #ext_data_structure::Heap {
                        type ExternBehavioralImplementor = #behavioral_implementor;
                        type StructuralImplementor = #structural_implementor;
                    }
                }
            },
        );
    let byline = byline!();
    syn::parse_quote! {
        #byline
        pub mod words_inverse_impls {
            #(#impls)*
        }
    }
}

// pub fn words_inverse_impls<L: LangSpec, LSub: LangSpec>(
//     ext_data_structure: &syn::Path,
//     og_words_base_path: &syn::Path,
//     sublang: &Sublang<'_, LSub, SortIdOf<L>>,
//     elsg: &LsGen<L>,
// ) -> syn::ItemMod {
//     let oglsg = LsGen::from(sublang.lsub);
//     let impls = oglsg
//         .bak()
//         .all_sort_ids()
//         .map(|sid| {
//             let word = oglsg.sort2word_rs_ty(sid.clone(), og_words_base_path);
//             let structural_implementor = elsg.sort2rs_ty(
//                 (sublang.map)(&sid),
//                 &HeapType(syn::parse_quote! { #ext_data_structure::Heap }),
//                 &AlgebraicsBasePath::new(syn::parse_quote! { #ext_data_structure:: }),
//             );
//             (structural_implementor, word)
//         })
//         .map(|(ty, word)| -> syn::ItemImpl {
//             syn::parse_quote! {
//                 impl words::InverseImplements<#word> for #ext_data_structure::Heap {
//                     type ExternBehavioralImplementor = ;
//                     type StructuralImplementor = #structural_implementor;
//                 }
//             }
//         });
//     let byline = byline!();
//     syn::parse_quote! {
//         #byline
//         pub mod words_impls {
//             #(#impls)*
//         }
//     }
// }

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
