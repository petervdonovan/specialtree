use langspec::{
    langspec::{LangSpec, SortId, SortIdOf},
    sublang::Sublang,
};
use langspec_gen_util::{AlgebraicsBasePath, HeapType, LsGen};
// use words::words_impls;

pub struct BasePaths {
    pub ext_data_structure: syn::Path,
    pub og_term_trait: syn::Path,
    pub og_words_base_path: syn::Path,
}

pub fn generate<'a, L: LangSpec, LSub: LangSpec>(
    ext_lg: &LsGen<L>,
    sublang: &Sublang<'a, LSub, SortIdOf<L>>,
    bps: &BasePaths,
) -> syn::ItemMod {
    let BasePaths {
        ext_data_structure,
        og_term_trait: _,
        og_words_base_path: _,
    } = bps;
    dbg!(sublang.lsub.name());
    let (camel_names, sids): (Vec<_>, Vec<SortIdOf<LSub>>) = LsGen::from(sublang.lsub)
        .ty_gen_datas(None)
        .map(|tgd| (tgd.camel_ident, SortId::Algebraic(tgd.id)))
        .unzip::<_, _, Vec<_>, Vec<_>>();
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
    let heap = generate_heap(&bps.ext_data_structure, &bps.og_term_trait);
    let owned_impls = generate_owned_impls(&camel_names, &image_ty_under_embeddings, bps);
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
        ext_lg,
    );
    syn::parse_quote! {
        mod bridge {
            #heap
            #owned_impls
            // #words_impls
            #inverse_implements_impls
        }
    }
}

pub(crate) fn generate_heap(
    ext_data_structure: &syn::Path,
    og_term_trait: &syn::Path,
) -> syn::ItemImpl {
    let byline = langspec_gen_util::byline!();
    syn::parse_quote! {
        #byline
        impl #og_term_trait::Heap for #ext_data_structure::Heap {}
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
        let ext_lg = super::LsGen::from(l);
        // let oglsg = super::LsGen::from(l);
        dbg!(sl.kebab("term-bridge"));
        CodegenInstance {
            id: codegen_component::KebabCodegenId(
                sl.kebab("term-bridge") + "-bridgeto-" + &l.name().snake.replace("_", "-"),
            ),
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
