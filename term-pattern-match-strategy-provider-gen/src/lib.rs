use langspec::langspec::{LangSpec, SortId, SortIdOf};
use langspec_rs_syn::{HeapType, tmf_ccf_sortses, ty_gen_datas, tmfs_monomorphizations, sort2externbehavioral_from_word_rs_ty, sort2word_rs_ty};
use rustgen_utils::cons_list;

pub struct BasePaths {
    pub term_trait: syn::Path,
    pub words: syn::Path,
    pub strategy_provider: syn::Path,
}

pub fn generate<L: LangSpec>(base_paths: &BasePaths, lg: &L) -> syn::ItemMod {
    let byline = rustgen_utils::byline!();
    let implementations = ty_gen_datas(lg, Some(base_paths.words.clone()))
        .map(|tgd| {
            impl_has_pattern_match_strategy_for(
                &base_paths.words,
                lg,
                SortId::Algebraic(tgd.id.clone()),
                tgd.ccf_sortses.as_slice(),
            )
        })
        .chain(tmfs_monomorphizations(lg).map(|mt| {
            let tcs = tmf_ccf_sortses::<L>(&mt);
            impl_has_pattern_match_strategy_for(
                &base_paths.words,
                lg,
                SortId::TyMetaFunc(mt),
                tcs.as_slice(),
            )
        }));
    syn::parse_quote! {
        #byline
        pub mod pattern_match_strategy {
            #(#implementations)*
        }
    }
}

fn impl_has_pattern_match_strategy_for<L: LangSpec>(
    words_path: &syn::Path,
    lg: &L,
    sid: SortId<L::ProductId, L::SumId, <L::Tmfs as langspec::tymetafunc::TyMetaFuncSpec>::TyMetaFuncId>,
    ccf_sortses: &[Vec<SortIdOf<L>>],
) -> syn::ItemImpl {
    let byline = rustgen_utils::byline!();
    let strategy: syn::Type = cons_list(
        ccf_sortses.iter().map(|ccf_sorts| {
            cons_list(
                ccf_sorts
                    .iter()
                    .map(|sid| sort2word_rs_ty(lg, sid.clone(), words_path)),
            )
        }),
    );
    let word = sort2word_rs_ty(lg, sid, words_path);
    syn::parse_quote! {
        #byline
        impl pmsp::NamesPatternMatchStrategy<#words_path::L> for #word {
            type Strategy = #strategy;
        }
    }
}

fn tymetadatas<L: LangSpec>(
    lg: &L,
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
            let rs_ty = sort2externbehavioral_from_word_rs_ty(lg, sid.clone(), ht, words_path);
            syn::parse_quote! {
                pmsp::TmfMetadata<#rs_ty, #ccf_sortses_tmfs>
            }
        }
    }))
}

pub mod targets {
    use std::path::Path;

    use codegen_component::{CgDepList, CodegenInstance, bumpalo};
    use rustgen_utils::kebab_id;

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
                    let lg = l;
                    super::generate(
                        &crate::BasePaths {
                            term_trait: term_trait(c2sp),
                            words: words(c2sp),
                            strategy_provider: sp,
                        },
                        lg,
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
