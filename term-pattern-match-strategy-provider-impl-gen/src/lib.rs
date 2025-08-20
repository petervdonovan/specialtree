use langspec::langspec::LangSpec;
use langspec_gen_util::LsGen;

pub struct BasePaths {
    pub data_structure: syn::Path,
    pub words: syn::Path,
    pub strategy_provider: syn::Path,
}

pub fn generate<L: LangSpec>(data_structure: &syn::Path, ls: &LsGen<L>) -> syn::ItemMod {
    let byline = rustgen_utils::byline!();
    let impls = ls.ty_gen_datas(None).map(|tgd| {
        let camel_ident = tgd.camel_ident;
        impl_adt_for(data_structure, &camel_ident)
    });
    syn::parse_quote! {
        #byline
        pub mod pattern_match_strategy_impls {
            // #(#impls)*
        }
    }
}

pub(crate) fn impl_adt_for(data_structure: &syn::Path, camel_ident: &syn::Ident) -> syn::ItemImpl {
    let byline = rustgen_utils::byline!();
    syn::parse_quote! {
        #byline
        impl words::Adt for #data_structure::#camel_ident {}
    }
}

pub mod targets {
    use std::path::Path;

    use codegen_component::{CgDepList, CodegenInstance, bumpalo};
    use langspec::sublang::reflexive_sublang;
    use rustgen_utils::kebab_id;

    pub fn default<'langs, L: super::LangSpec>(
        arena: &'langs bumpalo::Bump,
        mut codegen_deps: CgDepList<'langs>,
        l: &'langs L,
    ) -> CodegenInstance<'langs> {
        CodegenInstance {
            id: kebab_id!(l),
            generate: {
                let data_structure = codegen_deps.add(term_specialized_gen::targets::default(
                    arena,
                    codegen_deps.subtree(),
                    l,
                ));
                let _ = codegen_deps.add(term_specialized_impl_gen::targets::words_impls(
                    arena,
                    codegen_deps.subtree(),
                    l,
                    arena.alloc(reflexive_sublang(l)),
                ));
                Box::new(move |c2sp, _| {
                    let lg = super::LsGen::from(l);
                    super::generate(&data_structure(c2sp), &lg)
                })
            },
            external_deps: vec![],
            workspace_deps: vec![(
                "term-pattern-match-strategy-provider-impl-gen",
                Path::new("."),
            )],
            codegen_deps,
        }
    }
}
