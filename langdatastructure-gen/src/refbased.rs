use langspec::{
    flat::LangSpecFlat,
    humanreadable::LangSpecHuman,
    langspec::{LangSpec, Name, SortId, TerminalLangSpec},
};
use syn::{parse_quote, ItemEnum, ItemStruct};

use langspec_gen_util::{name_as_camel_ident, LangSpecGen, ProdGenData, SumGenData};

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
    // fn struct_item(
    //     name: &Name,
    //     lsf: &LangSpecFlat,
    //     sorts: impl Iterator<Item = SortId<<LangSpecFlat as LangSpec>::AlgebraicSortId>>,
    // ) -> ItemStruct {
    //     let name = name_as_camel_ident(name);
    //     let fields = sorts.map(|sort| -> syn::Type { lsf.sort2rs_type(sort) });
    //     parse_quote!(
    //         pub struct #name(#(pub #fields),*);
    //     )
    // }
    // fn enum_item(
    //     name: &Name,
    //     lsf: &LangSpecFlat,
    //     sorts: impl Iterator<Item = SortId<<LangSpecFlat as LangSpec>::AlgebraicSortId>>,
    // ) -> ItemEnum {
    //     let name = name_as_camel_ident(name);
    //     let sorts = sorts.collect::<Vec<_>>();
    //     let variant_tys = sorts
    //         .iter()
    //         .map(|sort| -> syn::Type {
    //             let rs_type = lsf.sort2rs_type(*sort);
    //             if let SortId::Algebraic(_) = sort {
    //                 parse_quote!(Box<#rs_type>)
    //             } else {
    //                 rs_type
    //             }
    //         })
    //         .collect::<Vec<_>>();
    //     let variant_names = sorts
    //         .iter()
    //         .map(|sort| -> syn::Ident { lsf.sort2rs_ident(*sort) })
    //         .collect::<Vec<_>>();
    //     parse_quote!(
    //         pub enum #name {
    //             #(#variant_names(#variant_tys)),*
    //         }
    //     )
    // }

    // let prods = l
    //     .product_datas()
    //     .map(|(name, sorts)| struct_item(name, l, sorts))
    //     .collect::<Vec<_>>();
    // let sums = l
    //     .sum_datas()
    //     .map(|(name, sorts)| enum_item(name, l, sorts))
    //     .collect::<Vec<_>>();
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
