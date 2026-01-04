use langspec::langspec::LangSpec;
use langspec_rs_syn::ty_gen_datas;
use memo::memo_cache::thread_local_cache;

pub struct BasePaths {
    pub data_structure: syn::Path,
    pub words: syn::Path,
    pub strategy_provider: syn::Path,
}

pub fn generate<L: LangSpec>(data_structure: &syn::Path, ls: &L) -> syn::ItemMod {
    let byline = rustgen_utils::byline!();
    let impls = ty_gen_datas(thread_local_cache(), ls, None)
        .iter()
        .map(|tgd| impl_adt_for(data_structure, &tgd.camel_ident));
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
                Box::new(move |c2sp, _| super::generate(&data_structure(c2sp), l))
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
