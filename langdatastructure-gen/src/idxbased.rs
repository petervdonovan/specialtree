use langspec::{
    flat::LangSpecFlat,
    humanreadable::LangSpecHuman,
    langspec::{AlgebraicSortId, LangSpec, Name, SortId, TerminalLangSpec},
};
use syn::{parse_quote, ItemEnum, ItemStruct};

use crate::{asi2rs_type, name_as_camel_ident, name_as_snake_ident, sort2rs_ident};

pub struct GenResult {
    pub db: ItemStruct,
    pub prods: Vec<ItemStruct>,
    pub sums: Vec<ItemEnum>,
}

impl GenResult {
    pub fn to_items(self) -> Vec<syn::Item> {
        let mut items = vec![syn::Item::Struct(self.db)];
        items.extend(self.prods.into_iter().map(syn::Item::Struct));
        items.extend(self.sums.into_iter().map(syn::Item::Enum));
        items
    }
}

pub fn gen<L: LangSpec>(l: &L) -> GenResult {
    fn sort2rs_idx_type<L: LangSpec>(_l: &L, sort: &SortId<L::AlgebraicSortId>) -> syn::Type {
        match sort {
            SortId::NatLiteral | SortId::Algebraic(_) => parse_quote!(usize),
            SortId::Set(_) | SortId::Sequence(_) => {
                parse_quote!(Vec<usize>)
            }
        }
    }
    fn gen_db<L: LangSpec>(l: &L) -> ItemStruct {
        let name = name_as_camel_ident(l.name());
        let prod_names = l
            .products()
            .map(|id| name_as_snake_ident(l.product_name(id)));
        let sum_names = l.sums().map(|id| name_as_snake_ident(l.sum_name(id)));
        let prod_tys = l
            .products()
            .map(|id| asi2rs_type(l, AlgebraicSortId::Product(id)));
        let sum_tys = l.sums().map(|id| asi2rs_type(l, AlgebraicSortId::Sum(id)));
        parse_quote!(
            pub struct #name {
                #(pub #prod_names: Vec<#prod_tys>,)*
                #(pub #sum_names: Vec<#sum_tys>,)*
            }
        )
    }
    fn struct_item<L: LangSpec>(
        name: &Name,
        ls: &L,
        sorts: &[SortId<L::AlgebraicSortId>],
    ) -> ItemStruct {
        let name = name_as_camel_ident(name);
        let fields = sorts
            .iter()
            .map(|sort| -> syn::Type { sort2rs_idx_type(ls, sort) });
        parse_quote!(
            pub struct #name(#(pub #fields),*);
        )
    }
    fn enum_item<L: LangSpec>(
        name: &Name,
        ls: &L,
        sorts: &[SortId<L::AlgebraicSortId>],
    ) -> ItemEnum {
        let name = name_as_camel_ident(name);
        let variant_tys = sorts
            .iter()
            .map(|sort| -> syn::Type {
                let rs_type = sort2rs_idx_type(ls, sort);
                if let SortId::Algebraic(_) = sort {
                    parse_quote!(Box<#rs_type>)
                } else {
                    rs_type
                }
            })
            .collect::<Vec<_>>();
        let variant_names = sorts
            .iter()
            .map(|sort| -> syn::Ident { sort2rs_ident(ls, ls.sid_convert(sort.clone())) })
            .collect::<Vec<_>>();
        parse_quote!(
            pub enum #name {
                #(#variant_names(#variant_tys)),*
            }
        )
    }
    let db = gen_db(l);
    let prods = l
        .products()
        .map(|id| {
            let name = l.product_name(id.clone());
            let sorts = l.product_sorts(id).collect::<Vec<_>>();
            struct_item(name, l, &sorts)
        })
        .collect::<Vec<_>>();
    let sums = l
        .sums()
        .map(|id| {
            let name = l.sum_name(id.clone());
            let sorts = l.sum_sorts(id).collect::<Vec<_>>();
            enum_item(name, l, &sorts)
        })
        .collect::<Vec<_>>();
    GenResult { db, prods, sums }
}

pub fn formatted(lsh: &LangSpecHuman) -> String {
    let lsf: LangSpecFlat = LangSpecFlat::canonical_from(lsh);
    let gen_result = gen(&lsf);
    let items = gen_result.to_items();
    prettyplease::unparse(&syn::parse_quote! {
        #(#items)*
    })
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_data_structure() {
        let formatted = formatted(&langspec_examples::fib());
        let expected = expect_test::expect![[r#"
            pub struct Fib {
                pub f: Vec<F>,
                pub nat: Vec<Nat>,
            }
            pub struct F(pub usize, pub usize);
            pub enum Nat {
                NatLit(usize),
                F(Box<usize>),
            }
        "#]];
        expected.assert_eq(&formatted);
    }
}
