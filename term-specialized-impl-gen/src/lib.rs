use langspec::{
    flat::LangSpecFlat,
    humanreadable::LangSpecHuman,
    langspec::{AlgebraicSortId, LangSpec, TerminalLangSpec as _},
    tymetafunc::TyMetaFuncSpec,
};
use langspec_gen_util::{
    byline, cons_list_index_range, number_range, AlgebraicsBasePath, HeapType, LsGen, TyGenData,
};

pub struct BasePaths {
    pub data_structure: syn::Path,
    pub term_trait: syn::Path,
}

pub fn generate<L: LangSpec>(
    bps: &BasePaths,
    data_structure_lsg: &LsGen<L>,
    term_trait_lsg: &LsGen<L>,
) -> syn::ItemMod {
    let owned_mod = gen_owned_mod(bps, data_structure_lsg);
    let ccf_mod = gen_ccf_mod(bps, data_structure_lsg, term_trait_lsg);
    let mct_mod = gen_mct_mod(bps, data_structure_lsg, term_trait_lsg);
    let heap_impl = gen_heap_impl(bps, data_structure_lsg, term_trait_lsg);
    let byline = byline!();
    syn::parse_quote! {
        #byline
        pub mod term_impls {
            #heap_impl
            #owned_mod
            #ccf_mod
            #mct_mod
        }
    }
}

pub(crate) fn gen_owned_mod<L: LangSpec>(
    BasePaths {
        data_structure,
        term_trait,
    }: &BasePaths,
    data_structure_lsg: &LsGen<L>,
) -> syn::ItemMod {
    let camels = data_structure_lsg
        .ty_gen_datas()
        .map(|td| td.camel_ident.clone());
    let byline = byline!();
    syn::parse_quote! {
        #byline
        pub mod owned_impls {
            #(impl #term_trait::owned::#camels for #data_structure::#camels {})*
        }
    }
}

pub(crate) fn gen_ccf_mod<L: LangSpec>(
    bps: &BasePaths,
    data_structure_lsg: &LsGen<L>,
    term_trait_lsg: &LsGen<L>,
) -> syn::ItemMod {
    let ccf_impls = term_trait_lsg
        .ty_gen_datas()
        .map(|tgd| gen_ccf_impls(bps, &tgd));
    let byline = byline!();
    syn::parse_quote! {
        #byline
        pub mod ccf_impls {
            #(#ccf_impls)*
        }
    }
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
        Some(AlgebraicSortId::Product(_)) => vec![gen_ccf_impl_prod(
            bps,
            camel,
            ccf_tys.first().unwrap(),
            &ccf_tys_flattened,
            &ccf_sort_snake_idents,
        )],
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
            term::CanonicallyConstructibleFrom<#ccf_ty> for #data_structure::#camel
        {
            fn construct(
                heap: &mut Self::Heap,
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
                heap: &Self::Heap,
            ) -> bool {
                true
            }

            fn deconstruct(
                self,
                heap: &Self::Heap,
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
    }: &BasePaths,
    camel: &syn::Ident,
    ccf_ty: &syn::Type,
    ccf_ty_camel: &syn::Ident,
) -> syn::ItemImpl {
    let byline = byline!();
    syn::parse_quote! {
        #byline
        impl
            term::CanonicallyConstructibleFrom<#ccf_ty> for #data_structure::#camel
        {
            fn construct(
                heap: &mut Self::Heap,
                t: #ccf_ty,
            ) -> Self {
                #data_structure::#camel::#ccf_ty_camel(t.0)
            }

            fn deconstruct_succeeds(
                &self,
                heap: &Self::Heap,
            ) -> bool {
                match self {
                    #data_structure::#camel::#ccf_ty_camel(_) => true,
                    _ => false,
                }
            }

            fn deconstruct(
                self,
                heap: &Self::Heap,
            ) -> #ccf_ty {
                match self {
                    #data_structure::#camel::#ccf_ty_camel(t) => (t,()),
                    _ => panic!("conversion failure"),
                }
            }
        }
    }
}

pub(crate) fn gen_mct_mod<L: LangSpec>(
    bps: &BasePaths,
    data_structure_lsg: &LsGen<L>,
    term_trait_lsg: &LsGen<L>,
) -> syn::ItemMod {
    let byline = byline!();
    syn::parse_quote! {
        #byline
        pub mod mct_impls {}
    }
}

pub(crate) fn gen_heap_impl<L: LangSpec>(
    BasePaths {
        data_structure,
        term_trait,
    }: &BasePaths,
    _data_structure_lsg: &LsGen<L>,
    term_trait_lsg: &LsGen<L>,
) -> syn::ItemImpl {
    let byline = byline!();
    let camels = term_trait_lsg
        .ty_gen_datas()
        .map(|td| td.camel_ident.clone());
    syn::parse_quote! {
        #byline
        impl #term_trait::Heap for #data_structure::Heap {
            #(type #camels = #data_structure::#camels;)*
        }
    }
}

pub fn formatted<Tmfs: TyMetaFuncSpec>(lsh: &LangSpecHuman<Tmfs>) -> String {
    let lsf: LangSpecFlat<Tmfs> = LangSpecFlat::canonical_from(lsh);
    let lsg = LsGen::from(&lsf);
    let bps = BasePaths {
        data_structure: syn::parse_quote!(crate::data_structure),
        term_trait: syn::parse_quote!(crate::extension_of),
    };
    let m = generate(&bps, &lsg, &lsg);
    let ds = term_specialized_gen::generate(&bps.data_structure, &lsg, false);
    let tpmspi = term_pattern_match_strategy_provider_impl_gen::generate(
        &term_pattern_match_strategy_provider_impl_gen::BasePaths {
            term_trait: syn::parse_quote!(crate::extension_of),
            data_structure: syn::parse_quote!(crate::data_structure),
            strategy_provider: syn::parse_quote!(crate::pattern_match_strategy),
            words: syn::parse_quote!(crate::words),
        },
        &lsg,
    );
    let tt = term_trait_gen::generate(&bps.term_trait, &lsf);
    prettyplease::unparse(&syn::parse_quote! {
        #m
        #ds
        #tpmspi
        #tt
    })
}
