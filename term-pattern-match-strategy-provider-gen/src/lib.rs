use langspec::langspec::{LangSpec, SortIdOf};
use langspec_gen_util::{
    AlgebraicsBasePath, CanonicallyConstructibleFromGenData, HeapType, LsGen, cons_list,
};

pub struct BasePaths {
    pub term_trait: syn::Path,
    pub words: syn::Path,
    pub strategy_provider: syn::Path,
}

pub fn generate<L: LangSpec>(base_paths: &BasePaths, lg: &LsGen<L>) -> syn::ItemMod {
    let byline = langspec_gen_util::byline!();
    let impls = lg.ty_gen_datas(Some(base_paths.words.clone())).map(|tgd| {
        let camel_ident = tgd.camel_ident;
        impl_has_pattern_match_strategy_for(
            base_paths,
            lg,
            &camel_ident,
            tgd.ccf_sortses.as_slice(),
            tgd.ccf,
        )
    });
    // let term_trait = &base_paths.term_trait;
    // let words = &base_paths.words;
    // let data_structure = &base_paths.data_structure;
    syn::parse_quote! {
        #byline
        pub mod pattern_match_strategy {
            // impl<Heap: #term_trait::Heap, T> term::case_split::HasPatternMatchStrategyFor<T> for PatternMatchStrategyProvider<Heap>
            // where
            //     T: words::Implements<Heap, #words::L>,
            //     T::LWord: pmsp::NamesPatternMatchStrategyGivenContext<Heap>,
            // {
            //     type Strategy = <T::LWord as pmsp::NamesPatternMatchStrategyGivenContext<Heap>>::Strategy;
            // }
            // pub struct PatternMatchStrategyProvider<Heap>(std::marker::PhantomData<Heap>);
            #(#impls)*
        }
    }
}

pub(crate) fn impl_has_pattern_match_strategy_for<L: LangSpec>(
    BasePaths {
        term_trait,
        words,
        strategy_provider: _,
    }: &BasePaths,
    lg: &LsGen<L>,
    camel_ident: &syn::Ident,
    ccf_sortses: &[Vec<SortIdOf<L>>],
    ccf: CanonicallyConstructibleFromGenData<'_>,
) -> syn::ItemImpl {
    let byline = langspec_gen_util::byline!();
    let ht = HeapType(syn::parse_quote! {Heap});
    let abp = AlgebraicsBasePath::new(quote::quote! {Heap::});
    let strategy: syn::Type = cons_list(
        (ccf.ccf_sort_tyses)(ht.clone(), abp.clone())
            .into_iter()
            .map(|it| cons_list(it.into_iter())),
    );
    let tymetadatas: syn::Type = cons_list(
        ccf_sortses
            .iter()
            .map(|ccf_sorts| tymetadatas(lg, ccf_sorts.as_slice(), &ht, words)),
    );
    syn::parse_quote! {
        #byline
        impl<Heap: #term_trait::Heap> pmsp::NamesPatternMatchStrategyGivenContext<Heap> for #words::sorts::#camel_ident {
            type Strategy = #strategy;
            type TyMetadatas = #tymetadatas;
        }
    }
}

fn tymetadatas<L: LangSpec>(
    lg: &LsGen<L>,
    sids: &[SortIdOf<L>],
    ht: &HeapType,
    // abp: &AlgebraicsBasePath,
    words_path: &syn::Path,
) -> syn::Type {
    cons_list(sids.iter().map(|sid| match sid {
        langspec::langspec::SortId::Algebraic(_) => syn::parse_quote! {
            pmsp::AdtMetadata
        },
        langspec::langspec::SortId::TyMetaFunc(mappedtype) => {
            let ccf_sortses_tmfs = tymetadatas(lg, mappedtype.a.as_slice(), ht, words_path);
            let rs_ty = lg.sort2externbehavioral_from_word_rs_ty(sid.clone(), ht, words_path);
            syn::parse_quote! {
                pmsp::TmfMetadata<#rs_ty, #ccf_sortses_tmfs>
            }
        }
    }))
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
                let term_trait = codegen_deps.add(term_trait_gen::targets::default(
                    arena,
                    codegen_deps.subtree(),
                    l,
                ));
                Box::new(move |c2sp, sp| {
                    let lg = super::LsGen::from(l);
                    super::generate(
                        &crate::BasePaths {
                            term_trait: term_trait(c2sp),
                            words: words(c2sp),
                            strategy_provider: sp,
                        },
                        &lg,
                    )
                })
            },
            external_deps: vec![],
            workspace_deps: vec![
                ("term-pattern-match-strategy-provider-gen", Path::new(".")),
                ("words", Path::new(".")),
                ("pmsp", Path::new(".")),
            ],
            codegen_deps,
        }
    }
}
