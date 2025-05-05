use extension_autobox::autobox;
use langspec::langspec::LangSpec;
use langspec_gen_util::{LsGen, byline};
use syn::parse_quote;

struct BasePaths {}

pub fn generate<L: LangSpec>(bps: &BasePaths, lg: &LsGen<L>) -> syn::ItemMod {
    let byline = byline!();
    syn::parse_quote! {
        #byline
        mod unparse {

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
                Box::new(move |c2sp, sp| {
                    let lg = super::LsGen::from(l);
                    super::generate(
                        &crate::BasePaths {
                            // data_structure: data_structure(c2sp),
                            // words: words(c2sp),
                            // parse: sp,
                            // term_trait: term_trait(c2sp),
                            // pattern_match_strategy: pattern_match_strategy(c2sp),
                            // cst_data_structure: cst_data_structure(c2sp),
                            // cst_term_trait: cst_term_trait(c2sp),
                            // cst_words: cst_words(c2sp),
                        },
                        &lg,
                    )
                })
            },
            external_deps: vec!["typenum"],
            workspace_deps: vec![
                ("unparse-gen", Path::new(".")),
                ("unparse-adt", Path::new(".")),
            ],
            codegen_deps,
        }
    }
}
