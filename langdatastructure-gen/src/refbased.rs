use abstracted_langspec_gen::{byline, AbstractedLsGen, AlgebraicsBasePath, HeapType, TyGenData};
use langspec::{
    flat::LangSpecFlat,
    humanreadable::LangSpecHuman,
    langspec::{AlgebraicSortId, LangSpec, TerminalLangSpec},
    tymetafunc::TyMetaFuncSpec,
};
use syn::parse_quote;

pub fn generate<Tmfs: TyMetaFuncSpec>(
    base_path: &syn::Path,
    l: &LangSpecFlat<Tmfs>,
    serde: bool,
) -> syn::ItemMod {
    let lg = AbstractedLsGen { bak: l };
    let serde = if serde {
        quote::quote!(#[derive(serde::Serialize, serde::Deserialize)])
    } else {
        quote::quote!()
    };
    let base_path = parse_quote! { #base_path::data_structure };
    let algebraics = lg.ty_gen_datas().filter(|it| it.id.is_some()).map(
        |TyGenData {
             id,
             camel_ident,
             ccf,
             ..
         }|
         -> syn::Item {
            let sort_rs_types = (ccf.flattened_ccf_sort_tys)(
                HeapType(parse_quote!(#base_path::Heap)),
                AlgebraicsBasePath(quote::quote!(#base_path::)),
            );
            let ret = match id.unwrap() {
                AlgebraicSortId::Product(_) => {
                    let sort_rs_snake_idents = (ccf.ccf_sort_snake_idents)();
                    quote::quote! {
                        #serde
                        pub struct #camel_ident {
                            #(pub #sort_rs_snake_idents: #sort_rs_types),*
                        }
                    }
                }
                _ => {
                    let sort_rs_camel_idents = (ccf.ccf_sort_camel_idents)();
                    quote::quote! {
                        #serde
                        pub enum #camel_ident {
                            #(#sort_rs_camel_idents(#sort_rs_types)),*
                        }
                    }
                }
            };
            parse_quote!(#ret)
        },
    );
    let heap = gen_heap(&base_path, &lg);
    let byline = byline!();
    parse_quote! {
        #byline
        mod data_structure {
            #heap
            #(#algebraics)*
        }
    }
}

pub fn gen_heap<L: LangSpec>(_base_path: &syn::Path, lg: &AbstractedLsGen<L>) -> syn::Item {
    let byline = byline!();
    parse_quote! {
        #byline
        #[derive(Default)]
        pub struct Heap();
    }
}

pub fn formatted<Tmfs: TyMetaFuncSpec>(lsh: &LangSpecHuman<Tmfs>) -> String {
    let lsf: LangSpecFlat<Tmfs> = LangSpecFlat::canonical_from(lsh);
    let m = generate(&parse_quote!(crate), &lsf, false);
    prettyplease::unparse(&syn::parse_quote! {
        #m
    })
}
