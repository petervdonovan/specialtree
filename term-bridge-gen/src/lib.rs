use std::{
    any::{Any as _, TypeId},
    iter::repeat,
};

use aspect::VisitationAspect;
use langspec::{
    langspec::{LangSpec, SortId, SortIdOf},
    sublang::{Sublang, SublangTyMap},
};
use langspec_rs_syn::{
    AlgebraicsBasePath, HeapType, sort2implementor_from_word_rs_ty, sort2rs_ty, sort2word_rs_ty,
    ty_gen_datas,
};
use memo::memo_cache::thread_local_cache;
use rustgen_utils::cons_list;
use transitive_ccf::{TransitiveUnitCcfRelation, ccf_paths};
// use words::words_impls;

pub struct BasePaths {
    pub ext_data_structure: syn::Path,
    pub og_term_trait: syn::Path,
    pub og_words_base_path: syn::Path,
}

pub fn generate<'a, L: LangSpec, LSub: LangSpec>(
    ext_l: &L,
    sublang: &Sublang<'a, LSub, SortIdOf<L>>,
    bps: &BasePaths,
) -> syn::ItemMod {
    let BasePaths {
        ext_data_structure,
        og_term_trait: _,
        og_words_base_path: _,
    } = bps;
    // let (camel_names, sids): (Vec<_>, Vec<SortIdOf<LSub>>) =
    //     ty_gen_datas(thread_local_cache(), sublang.lsub, None)
    //         .iter()
    //         .map(|tgd| (&tgd.camel_ident, SortId::Algebraic(tgd.id.clone())))
    //         .unzip::<_, _, Vec<_>, Vec<_>>();
    // let image_ty_under_embeddings = sids
    //     .iter()
    //     .map(|sort| {
    //         sort2rs_ty(
    //             ext_l,
    //             (sublang.map)(sort),
    //             &HeapType(syn::parse_quote! {#ext_data_structure::Heap}),
    //             &AlgebraicsBasePath::new(quote::quote! {#ext_data_structure::}),
    //         )
    //     })
    //     .collect::<Vec<_>>();
    let heap = generate_heap(&bps.ext_data_structure, &bps.og_term_trait);
    // let owned_impls = generate_owned_impls(&camel_names, &image_ty_under_embeddings, bps);
    // let words_impls = words_impls(
    //     &bps.ext_data_structure,
    //     &bps.og_words_base_path,
    //     sublang,
    //     ext_lg,
    // );
    let inverse_implements_impls = words::words_inverse_impls(
        &bps.ext_data_structure,
        &bps.og_words_base_path,
        sublang,
        ext_l,
    );
    let hdtfwl = generate_hdtfwl_impls(ext_l, sublang, bps);
    syn::parse_quote! {
        mod bridge {
            #heap
            // #owned_impls
            // #words_impls
            #inverse_implements_impls
            #hdtfwl
        }
    }
}

pub(crate) fn generate_heap(
    ext_data_structure: &syn::Path,
    og_term_trait: &syn::Path,
) -> syn::ItemImpl {
    let byline = rustgen_utils::byline!();
    syn::parse_quote! {
        #byline
        impl #og_term_trait::Heap for #ext_data_structure::Heap {}
    }
}

pub(crate) fn generate_hdtfwl_impls<'a, L: LangSpec, LSub: LangSpec>(
    ext_l: &L,
    sublang: &Sublang<'a, LSub, SortIdOf<L>>,
    bps: &BasePaths,
) -> syn::ItemMod {
    let BasePaths {
        ext_data_structure,
        og_term_trait: _,
        og_words_base_path,
    } = bps;
    let byline = rustgen_utils::byline!();
    let hdtfwls = ty_gen_datas(thread_local_cache(), sublang.lsub, None)
        .iter()
        .flat_map(|tgd| {
            repeat(tgd).zip(tgd.ccf_sortses
                .iter())
        }).filter_map(|(tgd, case)| -> Option<syn::ItemImpl> {
            let lwords =
                cons_list(case.iter().map(|sid| {
                    sort2word_rs_ty(sublang.lsub, sid.clone(), og_words_base_path)
                }));
            let visitation_aspect_map = &*sublang
                .aspect_implementors
                .iter()
                .find(|aspect_i| {
                    (*aspect_i.aspect_zst).type_id() == TypeId::of::<VisitationAspect>()
                })
                .unwrap()
                .map;
            let sublangs = (sublang, ());
            let ucps = &ccf_paths(thread_local_cache(), ext_l, &sublangs).units;
            let tgd_sid = SortId::Algebraic(tgd.id.clone());
            let implementor_tgd = ty_gen_datas(thread_local_cache(), ext_l, None)
                .iter()
                .find(|outer_tgd| {
                    visitation_aspect_map(&tgd_sid)
                        == SortId::Algebraic(outer_tgd.id.clone())
                });
            match implementor_tgd {
                Some(implementor_tgd) => {
                    let ht = HeapType(syn::parse_quote! {
                        #ext_data_structure::Heap
                    });
                    let abp = AlgebraicsBasePath(quote::quote!(#ext_data_structure::));
                    let implementors = implementor_tgd.ccf_sortses.iter().find_map(|sorts| maybe_deconstructs_to::<L, LSub>(case, sorts, ucps, ext_l, visitation_aspect_map, &ht, &abp)).unwrap();
                    let context_concrete_ty = sort2rs_ty(ext_l, visitation_aspect_map(&tgd_sid), &ht, &abp);
                    Some(syn::parse_quote! {
                        impl words::HasDeconstructionTargetForWordList<#og_words_base_path::L, #lwords, #context_concrete_ty> for #ext_data_structure::Heap {
                            type Implementors = #implementors;
                        }
                    })
                },
                None => None,
            }
        });
    syn::parse_quote! {
        #byline
        mod hdtfwl {
            #(#hdtfwls)*
        }
    }
}

fn maybe_deconstructs_to<L: LangSpec, LSub: LangSpec>(
    case: &[SortIdOf<LSub>],
    sorts: &[SortIdOf<L>],
    ucps: &[TransitiveUnitCcfRelation<SortIdOf<L>>],
    outerlang: &L,
    visitation_aspect_map: &SublangTyMap<'_, LSub, SortIdOf<L>>,
    ht: &HeapType,
    abp: &AlgebraicsBasePath,
    // words_path: &syn::Path,
) -> Option<syn::Type> {
    let mapped_case: Vec<_> = case.iter().map(visitation_aspect_map).collect();
    let conversion = sorts
        .iter()
        .zip(mapped_case.iter())
        .filter_map(|(sid, mapped_sid)| {
            ucps.iter()
                .find(|ucp| mapped_sid == sid || (&ucp.from == mapped_sid && &ucp.to == sid))
                .map(|_| {
                    sort2rs_ty(outerlang, sid.clone(), ht, abp)
                    // sort2implementor_from_word_rs_ty(
                    //     outerlang,
                    //     sid.clone(),
                    //     ht,
                    //     words_path,
                    //     &VisitationAspect,
                    // )
                })
        })
        .collect::<Vec<_>>();
    if conversion.len() == case.len() {
        Some(cons_list(conversion.into_iter()))
    } else {
        None
    }
}

// pub(crate) fn generate_owned_impls(
//     camel_names: &[syn::Ident],
//     image_ty_under_embeddings: &[syn::Type],
//     BasePaths {
//         ext_data_structure,
//         og_term_trait,
//         og_words_base_path: _,
//     }: &BasePaths,
// ) -> syn::ItemMod {
//     let byline = langspec_gen_util::byline!();
//     syn::parse_quote! {
//         #byline
//         pub mod owned_impls {
//             #(
//                 impl #og_term_trait::owned::#camel_names<#ext_data_structure::Heap> for #image_ty_under_embeddings {}
//             )*
//         }
//     }
// }

pub mod targets {
    use std::path::Path;

    use codegen_component::{CgDepList, CodegenInstance, bumpalo};
    use langspec::{
        langspec::SortIdOf,
        sublang::{Sublangs, SublangsList},
    };
    use rustgen_utils::{byline, kebab_id};

    pub fn default<
        'langs,
        L: super::LangSpec,
        Sl: Sublangs<SortIdOf<L>> + SublangsList<'langs, SortIdOf<L>>,
    >(
        arena: &'langs bumpalo::Bump,
        mut codegen_deps: CgDepList<'langs>,
        l: &'langs L,
        sl: &'langs Sl,
    ) -> CodegenInstance<'langs> {
        // let oglsg = super::LsGen::from(l);
        CodegenInstance {
            id: kebab_id!(l),
            generate: {
                let _ = codegen_deps.add(term_specialized_impl_gen::targets::default(
                    arena,
                    codegen_deps.subtree(),
                    l,
                    sl,
                ));
                // let sublang_name = lsublang.name().clone();
                // let _ = codegen_deps.add(term_specialized_impl_gen::targets::default(
                //     arena,
                //     codegen_deps.subtree(),
                //     l,
                // ));
                let _ = codegen_deps.add(rec(arena, codegen_deps.subtree(), l, sl));
                let byline = byline!();
                Box::new(move |_, _| {
                    // super::generate(
                    //     &ext_lg,
                    //     // &oglsg,
                    //     &l.sublang::<LSub>().unwrap(),
                    //     &crate::BasePaths {
                    //         ext_data_structure: ext_data_structure(c2sp),
                    //         og_term_trait: og_term_trait(c2sp),
                    //         og_words_base_path: og_words_base_path(c2sp),
                    //     },
                    // )
                    syn::parse_quote! {
                        /// intentionally empty
                        #byline
                        pub mod bridge {
                        }
                    }
                })
            },
            external_deps: vec![],
            workspace_deps: vec![
                ("term-bridge-gen", Path::new(".")),
                ("words", Path::new(".")),
            ],
            codegen_deps,
        }
    }
    fn rec<
        'langs,
        L: super::LangSpec,
        Sl: Sublangs<SortIdOf<L>> + SublangsList<'langs, SortIdOf<L>>,
    >(
        arena: &'langs bumpalo::Bump,
        mut codegen_deps: CgDepList<'langs>,
        l: &'langs L,
        sl: &'langs Sl,
    ) -> CodegenInstance<'langs> {
        CodegenInstance {
            id: tree_identifier::Identifier::list(
                vec![
                    sl.id("term-bridge"),
                    tree_identifier::Identifier::from_kebab_str("bridgeto").unwrap(),
                    l.name().clone(),
                ]
                .into(),
            )
            .into(),
            generate: {
                // let _ =
                //     codegen_deps.add(term_specialized_impl_gen::targets::term_specialized_impl(
                //         arena,
                //         codegen_deps.subtree(),
                //         l,
                //         sl,
                //     ));
                // let sublang_name = lsublang.name().clone();
                let car = sl.car();
                let ext_data_structure = codegen_deps.add(term_specialized_gen::targets::default(
                    arena,
                    codegen_deps.subtree(),
                    l,
                ));
                let og_term_trait = codegen_deps.add(term_trait_gen::targets::default(
                    arena,
                    codegen_deps.subtree(),
                    car.lsub,
                ));
                let og_words_base_path = codegen_deps.add(words::targets::words_mod(
                    arena,
                    codegen_deps.subtree(),
                    car.lsub,
                ));
                if Sl::LENGTH > 1 {
                    let _ = codegen_deps.add(rec(arena, codegen_deps.subtree(), l, sl.cdr()));
                }
                let _ = codegen_deps.add(term_specialized_impl_gen::targets::words_impls(
                    arena,
                    codegen_deps.subtree(),
                    l,
                    car,
                ));
                // let _ = codegen_deps.add(term_specialized_impl_gen::targets::default(
                //     arena,
                //     codegen_deps.subtree(),
                //     l,
                // ));
                Box::new(move |c2sp, _| {
                    super::generate(
                        l,
                        // &oglsg,
                        car,
                        &crate::BasePaths {
                            ext_data_structure: ext_data_structure(c2sp),
                            og_term_trait: og_term_trait(c2sp),
                            og_words_base_path: og_words_base_path(c2sp),
                        },
                    )
                })
            },
            external_deps: vec![],
            workspace_deps: vec![
                ("term-bridge-gen", Path::new(".")),
                ("words", Path::new(".")),
            ],
            codegen_deps,
        }
    }
}
