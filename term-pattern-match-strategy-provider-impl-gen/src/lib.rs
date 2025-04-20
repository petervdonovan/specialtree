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
    pub words: syn::Path,
    pub strategy_provider: syn::Path,
}

pub fn generate<L: LangSpec>(base_paths: &BasePaths, ls: &LsGen<L>) -> syn::ItemMod {
    let byline = langspec_gen_util::byline!();
    let impls = ls
        .ty_gen_datas(Some(base_paths.words.clone()))
        .map(|tgd| {
            let camel_ident = tgd.camel_ident;
            impl_has_pattern_match_strategy_for(base_paths, &camel_ident, tgd.ccf)
        })
        .chain(ls.ty_gen_datas(Some(base_paths.words.clone())).map(|tgd| {
            let camel_ident = tgd.camel_ident;
            impl_adt_for(base_paths, &camel_ident)
        }))
        .collect::<Vec<_>>();
    let term_trait = &base_paths.term_trait;
    let data_structure = &base_paths.data_structure;
    let words = &base_paths.words;
    syn::parse_quote! {
        #byline
        pub mod pattern_match_strategy {
            impl<Heap: #term_trait::Heap, T> term::case_split::HasPatternMatchStrategyFor<T> for PatternMatchStrategyProvider<Heap>
            where
                T: words::Implements<#words::L>,
                PatternMatchStrategyProvider<Heap>: term::case_split::HasPatternMatchStrategyFor<T::LWord>
            {
                type Strategy = <
                    PatternMatchStrategyProvider<Heap> as term::case_split::HasPatternMatchStrategyFor<
                        T::LWord,
                    >>::Strategy;
            }
            pub struct PatternMatchStrategyProvider<Heap>(std::marker::PhantomData<Heap>);
            #(#impls)*
        }
    }
}

pub(crate) fn impl_has_pattern_match_strategy_for(
    BasePaths {
        term_trait,
        data_structure,
        words,
        strategy_provider: _,
    }: &BasePaths,
    camel_ident: &syn::Ident,
    ccf: CanonicallyConstructibleFromGenData<'_>,
) -> syn::ItemImpl {
    let byline = langspec_gen_util::byline!();
    let strategy: syn::Type = cons_list(
        (ccf.ccf_sort_tyses)(
            HeapType(syn::parse_quote! {Heap}),
            AlgebraicsBasePath::new(quote::quote! {Heap::}),
        )
        .into_iter()
        .map(|it| cons_list(it.into_iter())),
    );
    syn::parse_quote! {
        #byline
        impl<Heap: #term_trait::Heap> term::case_split::HasPatternMatchStrategyFor<#words::sorts::#camel_ident> for PatternMatchStrategyProvider<Heap> {
            type Strategy = #strategy;
        }
    }
}

pub(crate) fn impl_adt_for(
    BasePaths {
        term_trait: _,
        data_structure,
        words: _,
        strategy_provider,
    }: &BasePaths,
    camel_ident: &syn::Ident,
) -> syn::ItemImpl {
    let byline = langspec_gen_util::byline!();
    syn::parse_quote! {
        #byline
        impl term::case_split::Adt for #data_structure::#camel_ident {
            type PatternMatchStrategyProvider = #strategy_provider::PatternMatchStrategyProvider<#data_structure::Heap>;
        }
    }
}

pub fn formatted<Tmfs: TyMetaFuncSpec>(lsh: &LangSpecHuman<Tmfs>) -> String {
    let lsf: LangSpecFlat<Tmfs> = LangSpecFlat::canonical_from(lsh);
    let lsg = LsGen::from(&lsf);
    let bps = BasePaths {
        strategy_provider: syn::parse_quote!(crate::pattern_match_strategy),
        data_structure: syn::parse_quote!(crate::data_structure),
        words: syn::parse_quote!(crate::term_trait::words),
        term_trait: syn::parse_quote!(crate::term_trait),
    };
    let m = generate(&bps, &lsg);
    let tt = term_trait_gen::generate(&bps.term_trait, &lsf);
    let ds = term_specialized_gen::generate(&bps.data_structure, &lsg, false);
    // let words = words::words_mod(&lsg);
    let words_impls = words::words_impls(&bps.words, &bps.data_structure, &lsg, &lsg);
    let ttimpl = term_specialized_impl_gen::generate(
        &term_specialized_impl_gen::BasePaths {
            data_structure: bps.data_structure.clone(),
            term_trait: bps.term_trait.clone(),
        },
        &lsg,
    );
    prettyplease::unparse(&syn::parse_quote! {
        #m
        #tt
        #ds
        // #words
        #words_impls
        #ttimpl
    })
}
