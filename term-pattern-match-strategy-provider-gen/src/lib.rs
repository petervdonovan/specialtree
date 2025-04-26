use langspec::langspec::LangSpec;
use langspec_gen_util::{
    AlgebraicsBasePath, CanonicallyConstructibleFromGenData, HeapType, LsGen, cons_list,
};

pub struct BasePaths {
    pub term_trait: syn::Path,
    pub data_structure: syn::Path,
    pub words: syn::Path,
    pub strategy_provider: syn::Path,
}

pub fn generate<L: LangSpec>(base_paths: &BasePaths, ls: &LsGen<L>) -> syn::ItemMod {
    let byline = langspec_gen_util::byline!();
    let impls = ls.ty_gen_datas(Some(base_paths.words.clone())).map(|tgd| {
        let camel_ident = tgd.camel_ident;
        impl_has_pattern_match_strategy_for(base_paths, &camel_ident, tgd.ccf)
    });
    let term_trait = &base_paths.term_trait;
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
        data_structure: _,
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
