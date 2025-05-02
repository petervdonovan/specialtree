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
    // let data_structure = &base_paths.data_structure;
    syn::parse_quote! {
        #byline
        pub mod pattern_match_strategy {
            impl<Heap: #term_trait::Heap, T> term::case_split::HasPatternMatchStrategyFor<T> for PatternMatchStrategyProvider<Heap>
            where
                T: words::Implements<Heap, #words::L>,
                PatternMatchStrategyProvider<Heap>: term::case_split::HasPatternMatchStrategyForWord<T::LWord>
            {
                type Strategy = <
                    PatternMatchStrategyProvider<Heap> as term::case_split::HasPatternMatchStrategyForWord<
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
        impl<Heap: #term_trait::Heap> term::case_split::HasPatternMatchStrategyForWord<#words::sorts::#camel_ident> for PatternMatchStrategyProvider<Heap> {
            type Strategy = #strategy;
        }
    }
}

pub mod targets {
    use std::path::Path;

    use codegen_component::{CgDepList, CodegenInstance, bumpalo};
    use langspec_gen_util::kebab_id;

    pub fn default<'langs, L: super::LangSpec>(
        arena: &'langs bumpalo::Bump,
        mut codegen_deps: CgDepList<'langs>,
        l: &'langs L,
    ) -> CodegenInstance<'langs> {
        CodegenInstance {
            id: kebab_id!(l),
            generate: {
                let words =
                    codegen_deps.add(words::targets::words_mod(arena, codegen_deps.subtree(), l));
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
                Box::new(move |c2sp, sp| {
                    let lg = super::LsGen::from(l);
                    super::generate(
                        &crate::BasePaths {
                            data_structure: data_structure(c2sp),
                            term_trait: term_trait(c2sp),
                            words: words(c2sp),
                            strategy_provider: sp,
                        },
                        &lg,
                    )
                })
            },
            external_deps: vec![],
            workspace_deps: vec![("term-pattern-match-strategy-provider-gen", Path::new("."))],
            codegen_deps,
        }
    }
}
