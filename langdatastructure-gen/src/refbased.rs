use langspec::{
    flat::LangSpecFlat,
    humanreadable::LangSpecHuman,
    langspec::{LangSpec, SortId, TerminalLangSpec},
};
use syn::parse_quote;

use langspec_gen_util::{LangSpecGen, ProdGenData, SumGenData};

pub fn sort2rs_type(_base_path: &syn::Path, sort: SortId<syn::Type>) -> syn::Type {
    match sort {
        SortId::NatLiteral => parse_quote!(u64),
        SortId::Algebraic(asi) => asi,
        SortId::Set(asi) => {
            let inner_ty = asi;
            parse_quote!(Vec<#inner_ty>)
        }
        SortId::Sequence(asi) => {
            let inner_ty = asi;
            parse_quote!(Vec<#inner_ty>)
        }
    }
}
pub fn gen(base_path: &syn::Path, l: &LangSpecFlat, serde: bool) -> syn::ItemMod {
    let lg = LangSpecGen {
        bak: l,
        sort2rs_type,
        type_base_path: parse_quote!(#base_path::refbased),
    };
    let serde = if serde {
        quote::quote!(#[derive(serde::Serialize, serde::Deserialize)])
    } else {
        quote::quote!()
    };
    let prods = lg.prod_gen_datas().map(
        |ProdGenData {
             camel_ident: camel_name,
             sort_rs_types,
             ..
         }|
         -> syn::ItemStruct {
            let ret = quote::quote!(
                #serde
                pub struct #camel_name(#(pub #sort_rs_types),*);
            );
            parse_quote!(#ret)
        },
    );
    let sums = lg.sum_gen_datas().map(
        |SumGenData {
             camel_ident: camel_name,
             sort_rs_camel_idents: sort_rs_idents,
             sort_rs_types,
             sort_shapes,
             ..
         }|
         -> syn::ItemEnum {
            let sort_rs_types =
                sort_rs_types
                    .zip(sort_shapes)
                    .map::<syn::Type, _>(|(ty, shape)| match shape {
                        SortId::Algebraic(_) => parse_quote!(Box<#ty>),
                        _ => ty,
                    });
            parse_quote!(
                #serde
                pub enum #camel_name {
                    #(#sort_rs_idents(#sort_rs_types)),*
                }
            )
        },
    );
    let limpl = gen_limpl(base_path, &lg);
    let byline = langspec_gen_util::byline!();
    parse_quote! {
        #byline
        pub mod refbased {
            #limpl
            pub type NatLit = u64;
            #(#prods)*
            #(#sums)*
        }
    }
}

pub fn gen_limpl<L: LangSpec>(_base_path: &syn::Path, lg: &LangSpecGen<L>) -> syn::Item {
    let ls_camel_ident = lg.camel_ident();
    let byline = langspec_gen_util::byline!();
    parse_quote! {
        #byline
        #[derive(Default)]
        pub struct #ls_camel_ident;
    }
}

pub fn formatted(lsh: &LangSpecHuman) -> String {
    let lsf: LangSpecFlat = LangSpecFlat::canonical_from(lsh);
    let m = gen(&parse_quote!(crate), &lsf, false);
    prettyplease::unparse(&syn::parse_quote! {
        #m
    })
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_data_structure() {
        let formatted = formatted(&langspec_examples::fib());
        let expected = expect_test::expect![[r#"
            pub struct Plus(pub Nat, pub Nat);
            pub struct F(pub Nat);
            pub enum Nat {
                NatLit(usize),
                F(Box<F>),
                Plus(Box<Plus>),
            }
        "#]];
        expected.assert_eq(&formatted);
    }
}
