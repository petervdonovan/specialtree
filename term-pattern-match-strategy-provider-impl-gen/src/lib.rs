use langspec::langspec::LangSpec;
use langspec_gen_util::LsGen;

pub struct BasePaths {
    pub data_structure: syn::Path,
    pub words: syn::Path,
    pub strategy_provider: syn::Path,
}

pub fn generate<L: LangSpec>(data_structure: &syn::Path, ls: &LsGen<L>) -> syn::ItemMod {
    let byline = langspec_gen_util::byline!();
    let impls = ls.ty_gen_datas(None).map(|tgd| {
        let camel_ident = tgd.camel_ident;
        impl_adt_for(data_structure, &camel_ident)
    });
    syn::parse_quote! {
        #byline
        pub mod pattern_match_strategy_impls {
            #(#impls)*
        }
    }
}

pub(crate) fn impl_adt_for(data_structure: &syn::Path, camel_ident: &syn::Ident) -> syn::ItemImpl {
    let byline = langspec_gen_util::byline!();
    syn::parse_quote! {
        #byline
        impl words::Adt for #data_structure::#camel_ident {}
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
                let data_structure = codegen_deps.add(term_specialized_gen::targets::default(
                    arena,
                    codegen_deps.subtree(),
                    l,
                ));
                let _ = codegen_deps.add(words_impls(arena, codegen_deps.subtree(), l));
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

    pub fn words_impls<'langs, LImplFor: super::LangSpec>(
        arena: &'langs bumpalo::Bump,
        mut codegen_deps: CgDepList<'langs>,
        lif: &'langs LImplFor,
    ) -> CodegenInstance<'langs> {
        CodegenInstance {
            id: kebab_id!(lif),
            generate: {
                let words_path = codegen_deps.add(words::targets::words_mod::<LImplFor>(
                    arena,
                    codegen_deps.subtree(),
                    lif,
                ));
                let data_structure = codegen_deps.add(term_specialized_gen::targets::default(
                    arena,
                    codegen_deps.subtree(),
                    lif,
                ));
                Box::new(move |c, _| {
                    let words_path = words_path(c);
                    let sorts_path = data_structure(c);
                    words::words_impls(&words_path, &sorts_path, &super::LsGen::from(lif))
                })
            },
            external_deps: vec![],
            workspace_deps: vec![("words", Path::new("."))],
            codegen_deps,
        }
    }
}
