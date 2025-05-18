use langspec::langspec::LangSpec;
use langspec_gen_util::{AlgebraicsBasePath, HeapType, LsGen, byline};

pub fn generate<L: LangSpec>(words_base_path: &syn::Path, lg: &LsGen<L>) -> syn::ItemMod {
    let byline = byline!();
    let tys = lg.bak().all_sort_ids().map(|sid| {
        lg.sort2rs_ty(
            sid,
            &HeapType(syn::parse_quote! {#words_base_path::Heap}),
            &AlgebraicsBasePath::new(syn::parse_quote! { #words_base_path:: }),
        )
    });
    let impls = tys.enumerate().map(|(idx, ty)| -> syn::ItemImpl {
        let idx = idx as u32;
        syn::parse_quote! {
            impl names_langspec_sort::NamesLangspecSort for #ty {
                fn sort_idx() -> u32 {
                    #idx
                }
            }
        }
    });
    syn::parse_quote! {
        #byline
        pub mod has_own_sort {
            #(#impls)*
        }
    }
}

pub mod targets {
    use std::path::Path;

    use codegen_component::{CgDepList, CodegenInstance, bumpalo};
    use langspec_gen_util::{LsGen, kebab_id};

    pub fn default<'langs, L: super::LangSpec>(
        arena: &'langs bumpalo::Bump,
        mut codegen_deps: CgDepList<'langs>,
        l: &'langs L,
    ) -> CodegenInstance<'langs> {
        CodegenInstance {
            id: kebab_id!(l),
            generate: {
                let lg = LsGen::from(l);
                let words_base_path = codegen_deps.add(words::targets::words_mod::<L>(
                    arena,
                    codegen_deps.subtree(),
                    l,
                ));
                Box::new(move |c2sp, _sp| super::generate(&words_base_path(c2sp), &lg))
            },
            external_deps: vec![],
            workspace_deps: vec![
                ("names-langspec-sort-gen", Path::new(".")),
                ("names-langspec-sort", Path::new(".")),
            ],
            codegen_deps,
        }
    }
}
