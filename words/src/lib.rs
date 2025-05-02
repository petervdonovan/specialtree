use langspec::langspec::LangSpec;
use langspec_gen_util::LsGen;

pub trait Implements<L> {
    type LWord;
}

pub fn words_mod<L: LangSpec>(lg: &LsGen<L>) -> syn::ItemMod {
    let sort_camel_idents = lg.ty_gen_datas(None).map(|it| it.camel_ident);
    let byline = langspec_gen_util::byline!();
    syn::parse_quote! {
        #byline
        pub mod words {
            pub struct L;
            pub mod sorts {
                #(
                    pub struct #sort_camel_idents;
                )*
            }
        }
    }
}

pub fn words_impls<L: LangSpec>(
    words_path: &syn::Path,
    sorts_path: &syn::Path,
    lg_impl_for: &LsGen<L>,
) -> syn::ItemMod {
    let sort_camel_idents = lg_impl_for
        .ty_gen_datas(None)
        .filter(|it| {
            lg_impl_for
                .ty_gen_datas(None) // FIXME: comparing idents
                .any(|other| other.snake_ident == it.snake_ident)
        })
        .map(|it| it.camel_ident);
    let byline = langspec_gen_util::byline!();
    syn::parse_quote! {
        #byline
        pub mod words_impls {
            #(
                impl words::Implements<#words_path::L> for #sorts_path::#sort_camel_idents {
                    type LWord = #words_path::sorts::#sort_camel_idents;
                }
            )*
        }
    }
}

pub mod targets {
    use codegen_component::{CgDepList, CodegenInstance, bumpalo};
    use langspec_gen_util::kebab_id;

    pub fn words_mod<'langs, L: super::LangSpec>(
        _: &'langs bumpalo::Bump,
        codegen_deps: CgDepList<'langs>,
        l: &'langs L,
    ) -> CodegenInstance<'langs> {
        CodegenInstance {
            id: kebab_id!(l),
            external_deps: vec![],
            workspace_deps: vec![],
            codegen_deps,
            generate: Box::new(move |_| super::words_mod(&super::LsGen::from(l))),
        }
    }
}
