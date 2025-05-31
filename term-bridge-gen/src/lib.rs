use langspec::{
    langspec::{LangSpec, SortId, SortIdOf},
    sublang::Sublang,
};
use langspec_gen_util::{AlgebraicsBasePath, HeapType, LsGen, byline};

pub struct BasePaths {
    pub ext_data_structure: syn::Path,
    pub og_term_trait: syn::Path,
    pub og_words_base_path: syn::Path,
}

pub fn generate<'a, L: LangSpec, LSub: LangSpec>(
    ext_lg: &LsGen<L>,
    // oglsg: &LsGen<LSub>,
    sublang: &Sublang<'a, LSub, SortIdOf<L>>,
    bps: &BasePaths,
) -> syn::ItemMod {
    let BasePaths {
        ext_data_structure,
        og_term_trait: _,
        og_words_base_path: _,
    } = bps;
    // let sublang = ext_lg
    //     .bak()
    //     .sublangs()
    //     .into_iter()
    //     .find(|sublang| sublang.name == *sublang_name)
    //     .expect("Sublang not found");
    dbg!(sublang.lsub.name());
    let (camel_names, sids): (Vec<_>, Vec<SortIdOf<LSub>>) = LsGen::from(sublang.lsub)
        .ty_gen_datas(None)
        .filter_map(|tgd| match tgd.id {
            Some(asi) => Some((tgd.camel_ident, SortId::Algebraic(asi))),
            None => None,
        })
        .unzip::<_, _, Vec<_>, Vec<_>>();
    dbg!(&camel_names);
    let image_ty_under_embeddings = sids
        .iter()
        .map(|sort| {
            ext_lg.sort2rs_ty(
                (sublang.map)(sort),
                &HeapType(syn::parse_quote! {#ext_data_structure::Heap}),
                &AlgebraicsBasePath::new(quote::quote! {#ext_data_structure::}),
            )
        })
        .collect::<Vec<_>>();
    let heap = generate_heap(&camel_names, &image_ty_under_embeddings, bps);
    let owned_impls = generate_owned_impls(&camel_names, &image_ty_under_embeddings, bps);
    let words_impls = bridge_words_impls(bps, sublang, ext_lg);
    let maps_tmf_impls = generate_maps_tmf_impls(ext_lg, sublang, bps);
    syn::parse_quote! {
        mod bridge {
            #heap
            #owned_impls
            #words_impls
            #maps_tmf_impls
        }
    }
}

pub(crate) fn bridge_words_impls<L: LangSpec, LSub: LangSpec>(
    BasePaths {
        ext_data_structure,
        og_term_trait,
        og_words_base_path,
    }: &BasePaths,
    sublang: &Sublang<'_, LSub, SortIdOf<L>>,
    // oglsg: &LsGen<LSub>,
    elsg: &LsGen<L>,
) -> syn::ItemMod {
    // let impls = elsg
    //     .ty_gen_datas(Some(og_words_base_path.clone()))
    //     .map(|tgd| -> syn::ItemImpl {
    //         let camel_ident = &tgd.camel_ident;
    //         let ty: syn::Type = syn::parse_quote! {
    //             <#ext_data_structure::Heap as #og_term_trait::Heap>::#camel_ident
    //         };
    //         syn::parse_quote! {
    //             impl words::Implements<#ext_data_structure::Heap, #og_words_base_path::L> for #ty {
    //                 type LWord = #og_words_base_path::sorts::#camel_ident;
    //             }
    //         }
    //     });
    let oglsg = LsGen::from(sublang.lsub);
    let impls = oglsg
        .bak()
        .all_sort_ids()
        .map(|sid| {
            let skeleton = oglsg.sort2rs_ty(
                sid.clone(),
                &HeapType(syn::parse_quote! { () }),
                &AlgebraicsBasePath::new(syn::parse_quote! { #og_words_base_path::sorts:: }),
            );
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

pub(crate) fn generate_heap(
    camel_names: &[syn::Ident],
    image_ty_under_embeddings: &[syn::Type],
    BasePaths {
        ext_data_structure,
        og_term_trait,
        og_words_base_path: _,
    }: &BasePaths,
) -> syn::ItemImpl {
    dbg!(camel_names);
    let byline = langspec_gen_util::byline!();
    syn::parse_quote! {
        #byline
        impl #og_term_trait::Heap for #ext_data_structure::Heap {
            #(
                type #camel_names = #image_ty_under_embeddings;
            )*
        }
    }
}

pub(crate) fn generate_owned_impls(
    camel_names: &[syn::Ident],
    image_ty_under_embeddings: &[syn::Type],
    BasePaths {
        ext_data_structure,
        og_term_trait,
        og_words_base_path: _,
    }: &BasePaths,
) -> syn::ItemMod {
    let byline = langspec_gen_util::byline!();
    syn::parse_quote! {
        #byline
        pub mod owned_impls {
            #(
                impl #og_term_trait::owned::#camel_names<#ext_data_structure::Heap> for #image_ty_under_embeddings {}
            )*
        }
    }
}

pub(crate) fn generate_maps_tmf_impls<L: LangSpec, LSub: LangSpec>(
    ext_lg: &LsGen<L>,
    sublang: &Sublang<LSub, SortIdOf<L>>,
    BasePaths {
        ext_data_structure,
        og_term_trait,
        og_words_base_path,
    }: &BasePaths,
) -> syn::ItemMod {
    let byline = langspec_gen_util::byline!();
    let (ogtys, mapped_tys) = sublang
        .tems
        .iter()
        .map(|tmf| {
            let ogty_shallow = ext_lg.sort2rs_ty(
                tmf.fromshallow.clone(),
                &HeapType(syn::parse_quote! {#ext_data_structure::Heap}),
                &AlgebraicsBasePath::new(quote::quote! { #ext_data_structure:: }),
            );
            let ogty_rec = ext_lg.sort2rs_ty(
                tmf.fromrec.clone(),
                &HeapType(syn::parse_quote! {#ext_data_structure::Heap}),
                &AlgebraicsBasePath::new(
                    quote::quote! {<#ext_data_structure::Heap as #og_term_trait::Heap>::},
                ),
            );
            let mapped_ty = ext_lg.sort2rs_ty(
                tmf.to.clone(),
                &HeapType(syn::parse_quote! {#ext_data_structure::Heap}),
                &AlgebraicsBasePath::new(quote::quote! {#ext_data_structure::}),
            );
            ((ogty_shallow, ogty_rec), mapped_ty)
        })
        .unzip::<_, _, Vec<_>, Vec<_>>();
    let (ogtys_shallow, ogtys_rec) = ogtys.into_iter().unzip::<_, _, Vec<_>, Vec<_>>();
    syn::parse_quote! {
        #byline
        pub mod maps_tmf_impls {
            #(
                impl term::MapsTmf<#og_words_base_path::L, #ogtys_rec> for #ext_data_structure::Heap {
                    type TmfFrom = #ogtys_shallow;
                    type TmfTo = #mapped_tys;
                }
            )*
        }
    }
}

pub mod targets {
    use std::path::Path;

    use codegen_component::{CgDepList, CodegenInstance, bumpalo};
    use langspec::{
        langspec::SortIdOf,
        sublang::{Sublangs, SublangsList},
    };
    use langspec_gen_util::{byline, kebab_id};

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
        let ext_lg = super::LsGen::from(l);
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
                let _ = codegen_deps.add(rec(arena, codegen_deps.subtree(), l, sl.cdr()));
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
        let ext_lg = super::LsGen::from(l);
        // let oglsg = super::LsGen::from(l);
        CodegenInstance {
            id: codegen_component::KebabCodegenId(sl.kebab("term-bridge")),
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
                // let _ = codegen_deps.add(term_specialized_impl_gen::targets::default(
                //     arena,
                //     codegen_deps.subtree(),
                //     l,
                // ));
                Box::new(move |c2sp, _| {
                    super::generate(
                        &ext_lg,
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
