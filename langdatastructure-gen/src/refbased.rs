use langspec::{
    flat::LangSpecFlat,
    humanreadable::LangSpecHuman,
    langspec::{SortId, TerminalLangSpec},
};
use syn::{parse_quote, ItemEnum, ItemStruct};

use langspec_gen_util::{LangSpecGen, ProdGenData, SumGenData};

pub fn gen(l: &LangSpecFlat) -> (Vec<ItemStruct>, Vec<ItemEnum>) {
    pub fn sort2rs_type(sort: SortId<syn::Type>) -> syn::Type {
        match sort {
            SortId::NatLiteral => parse_quote!(usize),
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
    let lg = LangSpecGen {
        bak: l,
        sort2rs_type,
    };
    let prods = lg
        .prod_gen_datas()
        .map(
            |ProdGenData {
                 camel_name,
                 sort_rs_types,
                 ..
             }| {
                let ret = quote::quote!(
                    pub struct #camel_name(#(pub #sort_rs_types),*);
                );
                println!("{}", &ret);
                parse_quote!(#ret)
            },
        )
        .collect();
    let sums =
        lg.sum_gen_datas()
            .map(
                |SumGenData {
                     camel_name,
                     sort_rs_idents,
                     sort_rs_types,
                     sort_shapes,
                     ..
                 }| {
                    let sort_rs_types = sort_rs_types.zip(sort_shapes).map::<syn::Type, _>(
                        |(ty, shape)| match shape {
                            SortId::Algebraic(_) => parse_quote!(Box<#ty>),
                            _ => ty,
                        },
                    );
                    parse_quote!(
                        pub enum #camel_name {
                            #(#sort_rs_idents(#sort_rs_types)),*
                        }
                    )
                },
            )
            .collect();
    (prods, sums)
}

pub fn formatted(lsh: &LangSpecHuman) -> String {
    let lsf: LangSpecFlat = LangSpecFlat::canonical_from(lsh);
    let (structs, enums) = gen(&lsf);
    prettyplease::unparse(&syn::parse_quote! {
        #(#structs)*
        #(#enums)*
    })
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_data_structure() {
        let formatted = formatted(&langspec_examples::fib());
        let expected = expect_test::expect![[r#"
            pub struct F(pub Nat, pub Nat);
            pub enum Nat {
                NatLit(usize),
                F(Box<F>),
            }
        "#]];
        expected.assert_eq(&formatted);
    }
}
