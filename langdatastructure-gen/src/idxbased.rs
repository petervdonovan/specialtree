use langspec::{
    flat::LangSpecFlat,
    humanreadable::LangSpecHuman,
    langspec::{LangSpec, SortId, TerminalLangSpec},
};
use syn::{parse_quote, ItemEnum, ItemStruct};

use langspec_gen_util::{name_as_camel_ident, SumGenData};
use langspec_gen_util::{LangSpecGen, ProdGenData};

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
    fn sort2rs_idx_type(sort: SortId<syn::Type>) -> syn::Type {
        match sort {
            SortId::NatLiteral | SortId::Algebraic(_) => parse_quote!(usize),
            SortId::Set(_) | SortId::Sequence(_) => {
                parse_quote!(Vec<usize>)
            }
        }
    }
    fn gen_db<L: LangSpec>(l: &LangSpecGen<L>) -> ItemStruct {
        langspec_gen_util::transpose!(l.prod_gen_datas(), snake_name, rs_ty);
        let prod_fields = quote::quote! {
            #(pub #snake_name: Vec<#rs_ty>,)*
        };
        langspec_gen_util::transpose!(l.sum_gen_datas(), snake_name, rs_ty);
        let sum_fields = quote::quote! {
            #(pub #snake_name: Vec<#rs_ty>,)*
        };
        let name = name_as_camel_ident(l.bak.name());
        parse_quote!(
            pub struct #name {
                #prod_fields
                #sum_fields
            }
        )
    }
    let lg = LangSpecGen {
        bak: l,
        sort2rs_type: sort2rs_idx_type,
    };
    let db = gen_db(&lg);
    let prods = lg
        .prod_gen_datas()
        .map(
            |ProdGenData {
                 camel_name,
                 sort_rs_types,
                 ..
             }| {
                parse_quote!(
                    pub struct #camel_name (
                        #(pub #sort_rs_types,)*
                    );
                )
            },
        )
        .collect();
    let sums = lg
        .sum_gen_datas()
        .map(
            |SumGenData {
                 camel_name,
                 sort_rs_idents,
                 sort_rs_types,
                 sort_shapes,
                 ..
             }| {
                let variant_tys = sort_rs_types
                    .zip(sort_shapes)
                    .map(|(ty, shape)| -> syn::Type {
                        if let SortId::Algebraic(_) = shape {
                            parse_quote!(Box<#ty>)
                        } else {
                            ty
                        }
                    });
                parse_quote!(
                    pub enum #camel_name {
                        #(#sort_rs_idents(#variant_tys)),*
                    }
                )
            },
        )
        .collect();
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
