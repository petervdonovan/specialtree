use langspec::{
    flat::LangSpecFlat,
    humanreadable::LangSpecHuman,
    langspec::{LangSpec, Name, SortId, TerminalLangSpec},
};
use syn::{parse_quote, ItemEnum, ItemStruct};

use langspec_gen_util::{name_as_camel_ident, LangSpecGen};

pub fn gen(l: &LangSpecFlat) -> (Vec<ItemStruct>, Vec<ItemEnum>) {
    fn struct_item(
        name: &Name,
        lsf: &LangSpecFlat,
        sorts: impl Iterator<Item = SortId<<LangSpecFlat as LangSpec>::AlgebraicSortId>>,
    ) -> ItemStruct {
        let name = name_as_camel_ident(name);
        let fields = sorts.map(|sort| -> syn::Type { lsf.sort2rs_type(sort) });
        parse_quote!(
            pub struct #name(#(pub #fields),*);
        )
    }
    fn enum_item(
        name: &Name,
        lsf: &LangSpecFlat,
        sorts: impl Iterator<Item = SortId<<LangSpecFlat as LangSpec>::AlgebraicSortId>>,
    ) -> ItemEnum {
        let name = name_as_camel_ident(name);
        let sorts = sorts.collect::<Vec<_>>();
        let variant_tys = sorts
            .iter()
            .map(|sort| -> syn::Type {
                let rs_type = lsf.sort2rs_type(*sort);
                if let SortId::Algebraic(_) = sort {
                    parse_quote!(Box<#rs_type>)
                } else {
                    rs_type
                }
            })
            .collect::<Vec<_>>();
        let variant_names = sorts
            .iter()
            .map(|sort| -> syn::Ident { lsf.sort2rs_ident(*sort) })
            .collect::<Vec<_>>();
        parse_quote!(
            pub enum #name {
                #(#variant_names(#variant_tys)),*
            }
        )
    }

    let prods = l
        .product_datas()
        .map(|(name, sorts)| struct_item(name, l, sorts))
        .collect::<Vec<_>>();
    let sums = l
        .sum_datas()
        .map(|(name, sorts)| enum_item(name, l, sorts))
        .collect::<Vec<_>>();
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
