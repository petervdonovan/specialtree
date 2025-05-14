use langspec::langspec::LangSpec;
use langspec_gen_util::{AlgebraicsBasePath, HeapType, LsGen, byline};

pub fn generate<L: LangSpec>(
    words_path: &syn::Path,
    ds_base_path: &syn::Path,
    lg: &LsGen<L>,
) -> syn::ItemMod {
    let byline = byline!();
    let tys = lg.bak().all_sort_ids().map(|sid| {
        lg.sort2rs_ty(
            sid,
            &HeapType(syn::parse_quote! {#ds_base_path::Heap}),
            &AlgebraicsBasePath::new(syn::parse_quote! { #ds_base_path:: }),
        )
    });
    let impls = tys.enumerate().map(|(idx, ty)| -> syn::ItemImpl {
        let idx = idx as u32;
        syn::parse_quote! {
            impl has_own_sort_id::HasOwnSortId<#ds_base_path::Heap, #words_path::L> for #ty {
                fn own_sort_id() -> u32 {
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
                let words_path = codegen_deps.add(words::targets::words_mod::<L>(
                    arena,
                    codegen_deps.subtree(),
                    l,
                ));
                let ds_base_path = codegen_deps.add(term_specialized_gen::targets::default::<L>(
                    arena,
                    codegen_deps.subtree(),
                    l,
                ));
                Box::new(move |c2sp, _sp| {
                    super::generate(&words_path(c2sp), &ds_base_path(c2sp), &lg)
                })
            },
            external_deps: vec![],
            workspace_deps: vec![
                ("has-own-sort-id-gen", Path::new(".")),
                ("has-own-sort-id", Path::new(".")),
            ],
            codegen_deps,
        }
    }
}
