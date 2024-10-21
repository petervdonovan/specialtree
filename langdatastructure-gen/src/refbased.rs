use langspec::{
    flat::LangSpecFlat,
    humanreadable::LangSpecHuman,
    langspec::{LangSpec, Name, SortId, TerminalLangSpec},
};
use syn::{parse_quote, ItemEnum, ItemStruct};

use crate::{name_as_rs_ident, sort2rs_ident, sort2rs_type};

pub fn gen(lsf: &LangSpecFlat) -> (Vec<ItemStruct>, Vec<ItemEnum>) {
    fn struct_item(
        name: &Name,
        lsf: &LangSpecFlat,
        sorts: &[SortId<<LangSpecFlat as LangSpec>::AlgebraicSortId>],
    ) -> ItemStruct {
        let name = name_as_rs_ident(name);
        let fields = sorts
            .iter()
            .map(|sort| -> syn::Type { sort2rs_type(lsf, *sort) });
        parse_quote!(
            pub struct #name(#(pub #fields),*);
        )
    }
    fn enum_item(
        name: &Name,
        lsf: &LangSpecFlat,
        sorts: &[SortId<<LangSpecFlat as LangSpec>::AlgebraicSortId>],
    ) -> ItemEnum {
        let name = name_as_rs_ident(name);
        let variant_tys = sorts
            .iter()
            .map(|sort| -> syn::Type {
                let rs_type = sort2rs_type(lsf, *sort);
                if let SortId::Algebraic(_) = sort {
                    parse_quote!(Box<#rs_type>)
                } else {
                    rs_type
                }
            })
            .collect::<Vec<_>>();
        let variant_names = sorts
            .iter()
            .map(|sort| -> syn::Ident { sort2rs_ident(lsf, *sort) })
            .collect::<Vec<_>>();
        parse_quote!(
            pub enum #name {
                #(#variant_names(#variant_tys)),*
            }
        )
    }
    let mut prods = vec![];
    for prod in lsf.products.iter() {
        let struct_item = struct_item(&prod.name, lsf, &prod.sorts);
        prods.push(struct_item);
    }
    let mut sums = vec![];
    for sum in lsf.sums.iter() {
        let enum_item = enum_item(&sum.name, lsf, &sum.sorts);
        sums.push(enum_item);
    }
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
            pub struct fib(pub Nat, pub Nat);
            pub enum Nat {
                NatLit(usize),
                fib(Box<fib>),
            }
        "#]];
        expected.assert_eq(&formatted);
    }
}
