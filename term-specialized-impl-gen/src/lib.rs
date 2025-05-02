use extension_autobox::autobox;
use langspec::{
    flat::LangSpecFlat,
    humanreadable::LangSpecHuman,
    langspec::{AlgebraicSortId, LangSpec, Name, SortId, SortIdOf, TerminalLangSpec as _},
    tymetafunc::TyMetaFuncSpec,
};
use langspec_gen_util::{
    AlgebraicsBasePath, CcfPaths, HeapType, LsGen, TyGenData, byline, cons_list_index_range,
};

pub struct BasePaths {
    pub data_structure: syn::Path,
    pub term_trait: syn::Path,
    pub words: syn::Path,
}

pub fn generate<L: LangSpec>(
    bps: &BasePaths,
    lsg: &LsGen<L>,
    important_sublangs: &[Name],
) -> syn::ItemMod {
    let owned_mod = gen_owned_mod(bps, lsg);
    let ccf_mod = gen_ccf_mod(bps, lsg);
    let transitive_ccf_mod = gen_transitive_ccf_mod(&bps.data_structure, lsg, important_sublangs);
    let ccf_auto_impls = gen_ccf_auto_impls(&bps.data_structure, &bps.words, lsg);
    let heap_impl = gen_heap_impl(bps, lsg, lsg);
    let maps_tmf_impls = gen_maps_tmf(bps);
    let byline = byline!();
    syn::parse_quote! {
        #byline
        pub mod term_impls {
            #heap_impl
            #owned_mod
            #ccf_mod
            #transitive_ccf_mod
            #ccf_auto_impls
            #maps_tmf_impls
        }
    }
}

pub fn generate_bridge<L: LangSpec>(
    bps: &BasePaths,
    data_structure_lsg: &LsGen<L>,
    term_trait_lsg: &LsGen<L>,
) -> syn::ItemMod {
    let heap_impl = gen_heap_impl(bps, data_structure_lsg, term_trait_lsg);
    let owned_mod = gen_owned_mod(bps, data_structure_lsg);
    let byline = byline!();
    syn::parse_quote! {
        #byline
        pub mod term_impls {
            #heap_impl
            #owned_mod
        }
    }
}

pub(crate) fn gen_maps_tmf(
    BasePaths {
        data_structure,
        term_trait: _,
        words,
    }: &BasePaths,
) -> syn::ItemMod {
    let byline = byline!();
    syn::parse_quote! {
        #byline
        pub mod maps_tmf_impls {
            impl<TmfMonomorphization> term::MapsTmf<#words::L, TmfMonomorphization>
                for #data_structure::Heap
            where
                TmfMonomorphization: term::CanonicallyConstructibleFrom<Self, (TmfMonomorphization, ())>
            {
                type Tmf = TmfMonomorphization;
            }
        }
    }
}

pub(crate) fn gen_owned_mod<L: LangSpec>(
    BasePaths {
        data_structure,
        term_trait,
        words,
    }: &BasePaths,
    data_structure_lsg: &LsGen<L>,
) -> syn::ItemMod {
    let camels = data_structure_lsg
        .ty_gen_datas(Some(syn::parse_quote!(#words)))
        .map(|td| td.camel_ident.clone());
    let byline = byline!();
    syn::parse_quote! {
        #byline
        pub mod owned_impls {
            #(impl #term_trait::owned::#camels<#data_structure::Heap> for #data_structure::#camels {})*
        }
    }
}

pub(crate) fn gen_ccf_mod<L: LangSpec>(bps: &BasePaths, term_trait_lsg: &LsGen<L>) -> syn::ItemMod {
    let words = &bps.words;
    let ccf_impls = term_trait_lsg
        .ty_gen_datas(Some(syn::parse_quote!(#words)))
        .map(|tgd| gen_ccf_impls(bps, &tgd));
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
    term_trait_lsg: &LsGen<L>,
) -> syn::ItemMod {
    let ccf_impls = term_trait_lsg
        .ty_gen_datas(Some(syn::parse_quote!(#words_path)))
        .map(|tgd| -> syn::ItemMacro {
            let camel_ident = &tgd.camel_ident;
            syn::parse_quote! {
                term::auto_impl_ccf!(#ds_base_path::Heap, #ds_base_path::#camel_ident);
            }
        });
    let mut tmf_ccf_impls: Vec<syn::ItemMacro> = Default::default();
    term_trait_lsg.tmfs_monomorphizations(&mut |tmf| {
        let ty = term_trait_lsg.sort2rs_ty(
            SortId::TyMetaFunc(tmf.clone()),
            &HeapType(syn::parse_quote! {#ds_base_path::Heap}),
            &AlgebraicsBasePath::new(quote::quote! { #ds_base_path:: }),
        );
        tmf_ccf_impls.push(syn::parse_quote! {
            term::auto_impl_ccf!(#ds_base_path::Heap, #ty);
        });
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
    ds_base_path: &syn::Path,
    lsg: &LsGen<L>,
    important_sublangs: &[Name],
) -> syn::ItemMod {
    let ccfp = lsg.ccf_paths(important_sublangs);
    let tucs = tuc_impls(ds_base_path, &ccfp, lsg);
    let tacs = tac_impls(ds_base_path, &ccfp, lsg);
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
    lsg: &'c LsGen<L>,
) -> impl Iterator<Item = syn::ItemImpl> + use<'a, 'b, 'c, L> {
    ccfp.units.iter().map(move |tuc| -> syn::ItemImpl {
        let tuc = tuc.clone();
        let sort2rs_ty = |sid| {
            lsg.sort2rs_ty(
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
            impl term::TransitivelyUnitCcf<#ds_base_path::Heap, #from> for #to {
                type Intermediary = #intermediary;
            }
        }
    })
}

pub(crate) fn tac_impls<'a, 'b, 'c, L: LangSpec>(
    ds_base_path: &'b syn::Path,
    ccfp: &'a CcfPaths<SortIdOf<L>>,
    lsg: &'c LsGen<L>,
) -> impl Iterator<Item = syn::ItemImpl> + use<'a, 'b, 'c, L> {
    ccfp.non_units.iter().map(move |tac| -> syn::ItemImpl {
        let tac = tac.clone();
        let sort2rs_ty = |sid| {
            lsg.sort2rs_ty(
                sid,
                &HeapType(syn::parse_quote! {#ds_base_path::Heap}),
                &AlgebraicsBasePath::new(quote::quote! { #ds_base_path:: }),
            )
        };
        let froms_cons_list =
            langspec_gen_util::cons_list(tac.from.iter().cloned().map(&sort2rs_ty));
        let to = sort2rs_ty(tac.to);
        let intermediary = sort2rs_ty(tac.intermediary.to);
        let intermediary_cons_list =
            langspec_gen_util::cons_list(tac.intermediary.from.iter().cloned().map(sort2rs_ty));
        let byline = byline!();
        syn::parse_quote! {
            #byline
            impl term::TransitivelyAllCcf<#ds_base_path::Heap, #froms_cons_list> for #to {
                type Intermediary = #intermediary;
                type Intermediaries = #intermediary_cons_list;
            }
        }
    })
}

pub(crate) fn gen_ccf_impls<L: LangSpec>(bps: &BasePaths, tgd: &TyGenData<L>) -> syn::ItemMod {
    let byline = byline!();
    let camel = &tgd.camel_ident;
    let snake = &tgd.snake_ident;
    let data_structure = &bps.data_structure;
    let ccf_tys = (tgd.ccf.ccf_sort_tys)(
        HeapType(syn::parse_quote!(<Self as term::Heaped>::Heap)),
        AlgebraicsBasePath::new(syn::parse_quote!(#data_structure::)),
    );
    let ccf_tys_flattened = (tgd.ccf.ccf_sort_tyses)(
        HeapType(syn::parse_quote!(<Self as term::Heaped>::Heap)),
        AlgebraicsBasePath::new(syn::parse_quote!(#data_structure::)),
    )
    .into_iter()
    .flatten()
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
        None => vec![],
        Some(AlgebraicSortId::Product(_)) => {
            vec![gen_ccf_impl_prod(
                bps,
                camel,
                ccf_tys.first().unwrap(),
                &ccf_tys_flattened,
                &ccf_sort_snake_idents,
            )]
        }
        Some(AlgebraicSortId::Sum(_)) => ccf_tys
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
        langspec_gen_util::cons_list(ccf_ty_snakes.iter().map(|it| syn::parse_quote! {self.#it}));
    let ret = quote::quote! {
        #byline
        impl
            term::DirectlyCanonicallyConstructibleFrom<<Self as term::Heaped>::Heap, #ccf_ty> for #data_structure::#camel
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
            term::DirectlyCanonicallyConstructibleFrom<<Self as term::Heaped>::Heap, #ccf_ty> for #data_structure::#camel
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

pub(crate) fn gen_heap_impl<L: LangSpec>(
    BasePaths {
        data_structure,
        term_trait,
        words,
    }: &BasePaths,
    _data_structure_lsg: &LsGen<L>,
    term_trait_lsg: &LsGen<L>,
) -> syn::ItemImpl {
    let byline = byline!();
    let camels = term_trait_lsg
        .ty_gen_datas(Some(syn::parse_quote!(#words)))
        .map(|td| td.camel_ident.clone());
    syn::parse_quote! {
        #byline
        impl #term_trait::Heap for #data_structure::Heap {
            #(type #camels = #data_structure::#camels;)*
        }
    }
}

pub mod targets {
    use std::path::Path;

    use codegen_component::{CgDepList, CodegenInstance, bumpalo};
    use extension_autobox::autobox;
    use langspec::langspec::Name;

    pub fn default<'langs, L: super::LangSpec>(
        arena: &'langs bumpalo::Bump,
        codegen_deps: CgDepList<'langs>,
        l: &'langs L,
    ) -> CodegenInstance<'langs> {
        term_specialized_impl(arena, codegen_deps, l, &[l.name().clone()])
    }

    pub fn term_specialized_impl<'langs, L: super::LangSpec>(
        arena: &'langs bumpalo::Bump,
        mut codegen_deps: CgDepList<'langs>,
        l: &'langs L,
        sublang_names: &[Name],
    ) -> CodegenInstance<'langs> {
        let sublang_names_dedup =
            std::iter::once(l.name())
                .chain(sublang_names.iter())
                .fold(Vec::new(), |mut acc, l| {
                    if !acc.contains(l) {
                        acc.push(l.clone());
                    }
                    acc
                });
        let kebab = sublang_names_dedup.iter().fold(String::new(), |acc, l| {
            let kebab = l.snake.replace("_", "-");
            if acc.is_empty() {
                kebab
            } else {
                format!("{}-{}", acc, kebab)
            }
        });
        CodegenInstance {
            id: codegen_component::KebabCodegenId(format!("term-specialized-impl-{}", kebab,)),
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
                    let lsf_boxed = autobox(l);
                    let lg = super::LsGen::from(&lsf_boxed);
                    super::generate(
                        &crate::BasePaths {
                            data_structure: data_structure(c2sp),
                            term_trait: term_trait(c2sp),
                            words: words(c2sp),
                        },
                        &lg,
                        &sublang_names_dedup.as_slice(),
                    )
                })
            },
            external_deps: vec![],
            workspace_deps: vec![
                ("term", Path::new(".")),
                ("term-specialized-impl-gen", Path::new(".")),
            ],
            codegen_deps,
        }
    }
}
