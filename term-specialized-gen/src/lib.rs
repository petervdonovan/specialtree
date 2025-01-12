use langspec::{
    flat::LangSpecFlat,
    humanreadable::LangSpecHuman,
    langspec::{AlgebraicSortId, LangSpec, TerminalLangSpec},
    tymetafunc::TyMetaFuncSpec,
};
use langspec_gen_util::{byline, transpose, AlgebraicsBasePath, HeapType, LsGen, TyGenData};
use syn::parse_quote;

pub fn generate<Tmfs: TyMetaFuncSpec>(
    base_path: &syn::Path,
    l: &LangSpecFlat<Tmfs>,
    serde: bool,
) -> syn::ItemMod {
    let lg = LsGen::from(l);
    let base_path = parse_quote! { #base_path::data_structure };
    let algebraics = lg
        .ty_gen_datas()
        .filter(|it| it.id.is_some())
        .map(|tgd| alg_dt(serde, &base_path, tgd));
    let heaped_impls = gen_heaped_impls(&base_path, &lg);
    let heap = gen_heap(&base_path, &lg);
    let byline = byline!();
    parse_quote! {
        #byline
        pub mod data_structure {
            #heap
            #(#algebraics)*
            #heaped_impls
        }
    }
}

pub(crate) fn alg_dt<L: LangSpec>(
    serde: bool,
    base_path: &syn::Path,
    TyGenData {
        id,
        camel_ident,
        ccf,
        ..
    }: TyGenData<L>,
) -> syn::Item {
    let serde = if serde {
        quote::quote!(#[derive(serde::Serialize, serde::Deserialize)])
    } else {
        quote::quote!()
    };
    let sort_rs_types = (ccf.flattened_ccf_sort_tys)(
        HeapType(parse_quote!(#base_path::Heap)),
        AlgebraicsBasePath::new(quote::quote!(#base_path::)),
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
}

pub fn gen_heap<L: LangSpec>(base_path: &syn::Path, lg: &LsGen<L>) -> syn::File {
    let alg_snakes = lg.ty_gen_datas().map(|tgd| tgd.snake_ident);
    let alg_camels = lg.ty_gen_datas().map(|tgd| tgd.camel_ident);
    let alg_heapbaks = lg.ty_gen_datas().map(alg_heapbak);
    let byline = byline!();
    parse_quote! {
        #byline
        #[derive(Default)]
        pub struct Heap {
            #(#alg_snakes: #base_path::heap::#alg_camels,)*
        }
        pub mod heap {
            #(#alg_heapbaks)*
        }
    }
}

pub(crate) fn alg_heapbak<L: LangSpec>(tgd: TyGenData<L>) -> syn::ItemStruct {
    let camel_ident = tgd.camel_ident;
    let hst = (tgd.ccf.heap_sort_tys)(
        HeapType(parse_quote!(crate::data_structure::Heap)),
        AlgebraicsBasePath::new(parse_quote!(crate::data_structure::heap::)),
    );
    transpose!(hst, heap_sort_ty, heap_sort_snake_ident);
    let byline = byline!();
    let ret = quote::quote! {
        #byline
        #[derive(Default)]
        pub struct #camel_ident {
            #(pub #heap_sort_snake_ident: #heap_sort_ty,)*
        }
    };
    parse_quote! {
        #ret
    }
}

pub(crate) fn gen_heaped_impls<L: LangSpec>(base_path: &syn::Path, lg: &LsGen<L>) -> syn::ItemMod {
    fn heaped_impl<L: LangSpec>(base_path: &syn::Path, tgd: TyGenData<L>) -> syn::ItemImpl {
        let camel_ident = tgd.camel_ident;
        let byline = byline!();
        let ret = quote::quote! {
            #byline
            impl term::Heaped for #base_path::#camel_ident {
                type Heap = #base_path::Heap;
            }
        };
        parse_quote! {
            #ret
        }
    }
    let byline = byline!();
    let impls = lg.ty_gen_datas().map(|tgd| heaped_impl(base_path, tgd));
    parse_quote! {
        #byline
        pub mod heaped {
            #(#impls)*
        }
    }
}

pub fn formatted<Tmfs: TyMetaFuncSpec>(lsh: &LangSpecHuman<Tmfs>) -> String {
    let lsf: LangSpecFlat<Tmfs> = LangSpecFlat::canonical_from(lsh);
    let m = generate(&parse_quote!(crate), &lsf, false);
    prettyplease::unparse(&syn::parse_quote! {
        #m
    })
}
