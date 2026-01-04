use langspec::{
    langspec::{AlgebraicSortId, LangSpec, SortId, SortIdOf},
    sublang::Sublangs,
};
use langspec_rs_syn::{
    AlgebraicsBasePath, HeapType, TyGenData, sort2rs_ty, tmfs_monomorphizations, ty_gen_datas,
};
use memo::memo_cache::thread_local_cache;
use rustgen_utils::{byline, cons_list_index_range};
use transitive_ccf::{CcfPaths, analysis::ccf_paths};

pub struct BasePaths {
    pub data_structure: syn::Path,
    pub term_trait: syn::Path,
    pub words: syn::Path,
}

pub fn generate<L: LangSpec>(
    bps: &BasePaths,
    lsg: &L,
    important_sublangs: impl Sublangs<SortIdOf<L>>,
) -> syn::ItemMod {
    let ccf_mod = gen_ccf_mod(bps, lsg);
    let transitive_ccf_mod = gen_transitive_ccf_mod(&bps.data_structure, lsg, important_sublangs);
    let ccf_auto_impls = gen_ccf_auto_impls(&bps.data_structure, &bps.words, lsg);
    let byline = byline!();
    syn::parse_quote! {
        #byline
        pub mod term_impls {
            #ccf_mod
            #transitive_ccf_mod
            #ccf_auto_impls
        }
    }
}

pub(crate) fn gen_ccf_mod<L: LangSpec>(bps: &BasePaths, term_trait_lsg: &L) -> syn::ItemMod {
    let words = &bps.words;
    let ccf_impls = ty_gen_datas(
        thread_local_cache(),
        term_trait_lsg,
        Some(syn::parse_quote!(#words)),
    )
    .iter()
    .map(|tgd| gen_ccf_impls(term_trait_lsg, bps, tgd));
    let byline = byline!();
    syn::parse_quote! {
        #byline
        pub mod ccf_impls {
            #(#ccf_impls)*
        }
    }
}

pub(crate) fn gen_ccf_auto_impls<L: LangSpec>(
    ds_base_path: &syn::Path,
    words_path: &syn::Path,
    term_trait_lsg: &L,
) -> syn::ItemMod {
    let ccf_impls = ty_gen_datas(
        thread_local_cache(),
        term_trait_lsg,
        Some(syn::parse_quote!(#words_path)),
    )
    .iter()
    .map(|tgd| -> syn::ItemMacro {
        let camel_ident = &tgd.camel_ident;
        syn::parse_quote! {
            term::auto_impl_ccf!(#ds_base_path::Heap, #ds_base_path::#camel_ident);
        }
    });
    let tmf_ccf_impls = tmfs_monomorphizations(thread_local_cache(), term_trait_lsg)
        .iter()
        .map(|tmf| -> syn::ItemMacro {
            let ty = sort2rs_ty(
                term_trait_lsg,
                SortId::TyMetaFunc(tmf.clone()),
                &HeapType(syn::parse_quote! {#ds_base_path::Heap}),
                &AlgebraicsBasePath::new(quote::quote! { #ds_base_path:: }),
            );
            syn::parse_quote! {
                term::auto_impl_ccf!(#ds_base_path::Heap, #ty);
            }
        });
    let byline = byline!();
    syn::parse_quote! {
        #byline
        pub mod ccf_auto_impls {
            #(#ccf_impls)*
            #(#tmf_ccf_impls)*
        }
    }
}

pub(crate) fn gen_transitive_ccf_mod<L: LangSpec>(
    data_structure: &syn::Path,
    term_trait_lsg: &L,
    important_sublangs: impl Sublangs<SortIdOf<L>>,
) -> syn::ItemMod {
    let ccfp = ccf_paths(thread_local_cache(), term_trait_lsg, &important_sublangs);
    let tucs = tuc_impls(data_structure, &ccfp, term_trait_lsg);
    let tacs = tac_impls(data_structure, &ccfp, term_trait_lsg);
    let byline = byline!();
    syn::parse_quote! {
        #byline
        pub mod transitive_ccf {
            #(
                #tucs
            )*
            #(
                #tacs
            )*
        }
    }
}

pub(crate) fn tuc_impls<'a, 'b, 'c, L: LangSpec>(
    ds_base_path: &'b syn::Path,
    ccfp: &'a CcfPaths<SortIdOf<L>>,
    lsg: &'c L,
) -> impl Iterator<Item = syn::ItemImpl> + use<'a, 'b, 'c, L> {
    ccfp.units.iter().map(move |tuc| -> syn::ItemImpl {
        let tuc = tuc.clone();
        let sort2rs_ty = |sid| {
            sort2rs_ty(
                lsg,
                sid,
                &HeapType(syn::parse_quote! {#ds_base_path::Heap}),
                &AlgebraicsBasePath::new(quote::quote! { #ds_base_path:: }),
            )
        };
        let from = sort2rs_ty(tuc.from);
        let to = sort2rs_ty(tuc.to);
        let intermediary = sort2rs_ty(tuc.intermediary);
        let byline = byline!();
        syn::parse_quote! {
            #byline
            impl ccf::transitivity::TransitivelyUnitCcf<#ds_base_path::Heap, #from> for #to {
                type Intermediary = #intermediary;
            }
        }
    })
}

pub(crate) fn tac_impls<'a, 'b, 'c, L: LangSpec>(
    ds_base_path: &'b syn::Path,
    ccfp: &'a CcfPaths<SortIdOf<L>>,
    lsg: &'c L,
) -> impl Iterator<Item = syn::ItemImpl> + use<'a, 'b, 'c, L> {
    ccfp.non_units.iter().map(move |tac| -> syn::ItemImpl {
        let tac = tac.clone();
        let sort2rs_ty = |sid| {
            sort2rs_ty(
                lsg,
                sid,
                &HeapType(syn::parse_quote! {#ds_base_path::Heap}),
                &AlgebraicsBasePath::new(quote::quote! { #ds_base_path:: }),
            )
        };
        let froms_cons_list =
            rustgen_utils::cons_list(tac.from.iter().cloned().map(&sort2rs_ty));
        let to = sort2rs_ty(tac.to);
        let intermediary = sort2rs_ty(tac.intermediary.to);
        let intermediary_cons_list =
            rustgen_utils::cons_list(tac.intermediary.from.iter().cloned().map(sort2rs_ty));
        let byline = byline!();
        syn::parse_quote! {
            #byline
            impl ccf::transitivity::TransitivelyAllCcf<#ds_base_path::Heap, #froms_cons_list> for #to {
                type Intermediary = #intermediary;
                type Intermediaries = #intermediary_cons_list;
            }
        }
    })
}

pub(crate) fn gen_ccf_impls<L: LangSpec>(
    lg: &L,
    bps: &BasePaths,
    tgd: &TyGenData<L>,
) -> syn::ItemMod {
    let byline = byline!();
    let camel = &tgd.camel_ident;
    let snake = &tgd.snake_ident;
    let data_structure = &bps.data_structure;
    let ccf_tys = (tgd.ccf.ccf_sort_tys)(
        HeapType(syn::parse_quote!(<Self as term::Heaped>::Heap)),
        AlgebraicsBasePath::new(syn::parse_quote!(#data_structure::)),
    );
    // let ccf_tys_flattened = (tgd.ccf.ccf_sort_tyses)(
    //     HeapType(syn::parse_quote!(<Self as term::Heaped>::Heap)),
    //     AlgebraicsBasePath::new(syn::parse_quote!(#data_structure::)),
    // )
    // .into_iter()
    // .flatten()
    // .collect::<Vec<_>>();
    let ccf_tys_flattened = tgd
        .ccf_sortses
        .iter()
        .flat_map(|sids| {
            sids.iter().map(|sid| {
                sort2rs_ty(
                    lg,
                    sid.clone(),
                    &HeapType(syn::parse_quote!(<Self as term::Heaped>::Heap)),
                    &AlgebraicsBasePath::new(syn::parse_quote!(#data_structure::)),
                )
            })
        })
        .collect::<Vec<_>>();
    let ccf_sort_snake_idents = (tgd.ccf.ccf_sort_snake_idents)()
        .into_iter()
        .flatten()
        .collect::<Vec<_>>();
    let ccf_sort_camel_idents = (tgd.ccf.ccf_sort_camel_idents)()
        .into_iter()
        .flatten()
        .collect::<Vec<_>>();
    let ccf_impls = match tgd.id {
        AlgebraicSortId::Product(_) => {
            vec![gen_ccf_impl_prod(
                bps,
                camel,
                ccf_tys.first().unwrap(),
                &ccf_tys_flattened,
                &ccf_sort_snake_idents,
            )]
        }
        AlgebraicSortId::Sum(_) => ccf_tys
            .iter()
            .zip(ccf_sort_camel_idents.iter())
            .map(|(ccfty, ccf_camel)| gen_ccf_impl_sum(bps, camel, ccfty, ccf_camel))
            .collect(),
    };
    syn::parse_quote! {
        #byline
        pub mod #snake {
            #(#ccf_impls)*
        }
    }
}

pub(crate) fn gen_ccf_impl_prod(
    BasePaths {
        data_structure,
        term_trait: _,
        words: _,
    }: &BasePaths,
    camel: &syn::Ident,
    ccf_ty: &syn::Type,
    ccf_tys_flattened: &[syn::Type],
    ccf_ty_snakes: &[syn::Ident],
) -> syn::ItemImpl {
    let idxs = cons_list_index_range(ccf_tys_flattened.len(), syn::parse_quote! {t});
    let byline = byline!();
    let deconstruct: syn::Expr =
        rustgen_utils::cons_list(ccf_ty_snakes.iter().map(|it| syn::parse_quote! {self.#it}));
    let ret = quote::quote! {
        #byline
        impl
            ccf::DirectlyCanonicallyConstructibleFrom<<Self as term::Heaped>::Heap, #ccf_ty> for #data_structure::#camel
        {
            fn construct(
                heap: &mut <Self as term::Heaped>::Heap,
                t: #ccf_ty,
            ) -> Self {
                #data_structure::#camel {
                    #(
                        #ccf_ty_snakes: #idxs,
                    )*
                }
            }

            fn deconstruct_succeeds(
                &self,
                heap: &<Self as term::Heaped>::Heap,
            ) -> bool {
                true
            }

            fn deconstruct(
                self,
                heap: &<Self as term::Heaped>::Heap,
            ) -> #ccf_ty {
                #deconstruct
            }
        }
    };
    syn::parse_quote!(#ret)
}
pub(crate) fn gen_ccf_impl_sum(
    BasePaths {
        data_structure,
        term_trait: _,
        words: _,
    }: &BasePaths,
    camel: &syn::Ident,
    ccf_ty: &syn::Type,
    ccf_ty_camel: &syn::Ident,
) -> syn::ItemImpl {
    let byline = byline!();
    syn::parse_quote! {
        #byline
        impl
            ccf::DirectlyCanonicallyConstructibleFrom<<Self as term::Heaped>::Heap, #ccf_ty> for #data_structure::#camel
        {
            fn construct(
                heap: &mut <Self as term::Heaped>::Heap,
                t: #ccf_ty,
            ) -> Self {
                #data_structure::#camel::#ccf_ty_camel(t.0)
            }

            fn deconstruct_succeeds(
                &self,
                heap: &<Self as term::Heaped>::Heap,
            ) -> bool {
                match self {
                    #data_structure::#camel::#ccf_ty_camel(_) => true,
                    _ => false,
                }
            }

            fn deconstruct(
                self,
                heap: &<Self as term::Heaped>::Heap,
            ) -> #ccf_ty {
                match self {
                    #data_structure::#camel::#ccf_ty_camel(t) => (t,()),
                    _ => panic!("conversion failure"),
                }
            }
        }
    }
}

pub mod targets {
    use std::path::Path;

    use codegen_component::{CgDepList, CodegenInstance, KebabCodegenId, bumpalo};
    use extension_autobox::autobox;
    use langspec::{
        langspec::SortIdOf,
        sublang::{Sublang, SublangsList},
    };
    use tree_identifier::Identifier;

    pub fn default<'langs, L: super::LangSpec>(
        arena: &'langs bumpalo::Bump,
        mut codegen_deps: CgDepList<'langs>,
        l: &'langs L,
        sublangs: &'langs impl SublangsList<'langs, SortIdOf<L>>,
    ) -> CodegenInstance<'langs> {
        CodegenInstance {
            id: sublangs.id("term-specialized-impl").into(),
            generate: {
                let data_structure = codegen_deps.add(term_specialized_gen::targets::default(
                    arena,
                    codegen_deps.subtree(),
                    l,
                ));
                let term_trait = codegen_deps.add(term_trait_gen::targets::default(
                    arena,
                    codegen_deps.subtree(),
                    l,
                ));
                let words =
                    codegen_deps.add(words::targets::words_mod(arena, codegen_deps.subtree(), l));
                Box::new(move |c2sp, _| {
                    let lsf_boxed = &*arena.alloc(autobox(l));
                    let lg = lsf_boxed;
                    super::generate(
                        &crate::BasePaths {
                            data_structure: data_structure(c2sp),
                            term_trait: term_trait(c2sp),
                            words: words(c2sp),
                        },
                        lg,
                        sublangs.push_through(lsf_boxed),
                    )
                })
            },
            external_deps: vec![],
            workspace_deps: vec![
                ("term", Path::new(".")),
                ("term-specialized-impl-gen", Path::new(".")),
                ("ccf", Path::new(".")),
            ],
            codegen_deps,
        }
    }
    pub fn words_impls<'langs, LImplFor: super::LangSpec, LSub: super::LangSpec>(
        arena: &'langs bumpalo::Bump,
        mut codegen_deps: CgDepList<'langs>,
        lif: &'langs LImplFor,
        sublang: &'langs Sublang<'langs, LSub, SortIdOf<LImplFor>>,
    ) -> CodegenInstance<'langs> {
        CodegenInstance {
            id: KebabCodegenId::from(Identifier::list(
                vec![
                    Identifier::from_kebab_str("words-impls").unwrap(),
                    Identifier::list(vec![sublang.lsub.name().clone(), lif.name().clone()].into()),
                ]
                .into(),
            )),
            generate: {
                let words_path = codegen_deps.add(words::targets::words_mod::<LSub>(
                    arena,
                    codegen_deps.subtree(),
                    sublang.lsub,
                ));
                let data_structure = codegen_deps.add(term_specialized_gen::targets::default(
                    arena,
                    codegen_deps.subtree(),
                    lif,
                ));
                Box::new(move |c, _| {
                    let words_path = words_path(c);
                    let sorts_path = data_structure(c);
                    words::words_impls(&sorts_path, &words_path, sublang, lif)
                })
            },
            external_deps: vec![],
            workspace_deps: vec![("words", Path::new("."))],
            codegen_deps,
        }
    }
}
