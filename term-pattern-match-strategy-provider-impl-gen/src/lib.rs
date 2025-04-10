use langspec::{
    flat::LangSpecFlat,
    humanreadable::LangSpecHuman,
    langspec::{LangSpec, TerminalLangSpec},
    tymetafunc::TyMetaFuncSpec,
};
use langspec_gen_util::{
    cons_list, AlgebraicsBasePath, CanonicallyConstructibleFromGenData, HeapType, LsGen,
};

pub struct BasePaths {
    pub term_trait: syn::Path,
    pub data_structure: syn::Path,
    pub strategy_provider: syn::Path,
}

pub fn generate<L: LangSpec>(base_paths: &BasePaths, ls: &LsGen<L>) -> syn::ItemMod {
    let byline = langspec_gen_util::byline!();
    let impls = ls
        .ty_gen_datas()
        .map(|tgd| {
            let camel_ident = tgd.camel_ident;
            impl_has_pattern_match_strategy_for(base_paths, &camel_ident, tgd.ccf)
        })
        .chain(ls.ty_gen_datas().map(|tgd| {
            let camel_ident = tgd.camel_ident;
            impl_adt_for(base_paths, &camel_ident)
        }))
        .collect::<Vec<_>>();
    syn::parse_quote! {
        #byline
        pub mod pattern_match_strategy {
            pub struct PatternMatchStrategyProvider;
            #(#impls)*
        }
    }
}

pub(crate) fn impl_has_pattern_match_strategy_for(
    BasePaths {
        term_trait: _,
        data_structure,
        strategy_provider: _,
    }: &BasePaths,
    camel_ident: &syn::Ident,
    ccf: CanonicallyConstructibleFromGenData<'_>,
) -> syn::ItemImpl {
    let byline = langspec_gen_util::byline!();
    let strategy: syn::Type = cons_list(
        (ccf.ccf_sort_tyses)(
            HeapType(syn::parse_quote! {#data_structure::Heap}),
            AlgebraicsBasePath::new(quote::quote! {#data_structure::}),
        )
        .into_iter()
        .map(|it| cons_list(it.into_iter())),
    );
    syn::parse_quote! {
        #byline
        impl term::case_split::HasPatternMatchStrategyFor<#data_structure::#camel_ident> for PatternMatchStrategyProvider {
            type Strategy = #strategy;
        }
    }
}

pub(crate) fn impl_adt_for(
    BasePaths {
        term_trait: _,
        data_structure,
        strategy_provider,
    }: &BasePaths,
    camel_ident: &syn::Ident,
) -> syn::ItemImpl {
    let byline = langspec_gen_util::byline!();
    syn::parse_quote! {
        #byline
        impl term::case_split::Adt for #data_structure::#camel_ident {
            type PatternMatchStrategyProvider = #strategy_provider::PatternMatchStrategyProvider;
        }
    }
}

pub fn formatted<Tmfs: TyMetaFuncSpec>(lsh: &LangSpecHuman<Tmfs>) -> String {
    let lsf: LangSpecFlat<Tmfs> = LangSpecFlat::canonical_from(lsh);
    let lsg = LsGen::from(&lsf);
    let bps = BasePaths {
        strategy_provider: syn::parse_quote!(crate::pattern_match_strategy),
        data_structure: syn::parse_quote!(crate::data_structure),
        term_trait: syn::parse_quote!(crate::extension_of),
    };
    let m = generate(&bps, &lsg);
    let tt = term_trait_gen::generate(&bps.term_trait, &lsf);
    let ds = term_specialized_gen::generate(&bps.data_structure, &lsg, false);
    prettyplease::unparse(&syn::parse_quote! {
        #m
        #tt
        #ds
    })
}
