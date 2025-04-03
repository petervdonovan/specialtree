use langspec::{
    flat::LangSpecFlat,
    humanreadable::LangSpecHuman,
    langspec::{LangSpec, TerminalLangSpec},
    tymetafunc::TyMetaFuncSpec,
};
use langspec_gen_util::{cons_list, CanonicallyConstructibleFromGenData, LsGen};

pub struct BasePaths {
    pub term_trait: syn::Path,
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
        .collect::<Vec<_>>();
    syn::parse_quote! {
        #byline
        pub mod pattern_match_strategy {
            struct PatternMatchStrategyProvider<H>(std::marker::PhantomData<H>);
            #(#impls)*
        }
    }
}

pub(crate) fn impl_has_pattern_match_strategy_for(
    BasePaths {
        term_trait,
        strategy_provider: _,
    }: &BasePaths,
    camel_ident: &syn::Ident,
    ccf: CanonicallyConstructibleFromGenData<'_>,
) -> syn::ItemImpl {
    let byline = langspec_gen_util::byline!();
    let strategy = cons_list(
        (ccf.ccf_sort_camel_idents)()
            .into_iter()
            .map(|it| syn::parse_quote! {(#(H::#it,)*)}),
    );
    syn::parse_quote! {
        #byline
        impl<H: #term_trait::extension_of::Heap> term::HasPatternMatchStrategyFor<H::#camel_ident> for PatternMatchStrategyProvider<H> {
            type Strategy = #strategy;
        }
    }
}

pub fn formatted<Tmfs: TyMetaFuncSpec>(lsh: &LangSpecHuman<Tmfs>) -> String {
    let lsf: LangSpecFlat<Tmfs> = LangSpecFlat::canonical_from(lsh);
    let lsg = LsGen::from(&lsf);
    let bps = BasePaths {
        strategy_provider: syn::parse_quote!(crate),
        term_trait: syn::parse_quote!(crate),
    };
    let m = generate(&bps, &lsg);
    let tt = term_trait_gen::generate(&bps.term_trait, &lsf);
    prettyplease::unparse(&syn::parse_quote! {
        #m
        #tt
    })
}
