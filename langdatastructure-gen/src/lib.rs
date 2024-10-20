use langspec::{
    flat::LangSpecFlat,
    humanreadable::LangSpecHuman,
    langspec::{LangSpec, Name, SortId, TerminalLangSpec},
};
use syn::{parse_quote, ItemEnum, ItemStruct};

pub fn name_as_rs_type(name: &Name) -> syn::Ident {
    syn::Ident::new(&name.name, proc_macro2::Span::call_site())
}

pub fn asi2rs_type(
    lsf: &LangSpecFlat,
    asi: &<LangSpecFlat as LangSpec>::AlgebraicSortId,
) -> syn::Type {
    let name = lsf.algebraic_sort_name(*asi);
    let ty_name = name_as_rs_type(name);
    parse_quote!(#ty_name)
}

pub fn sort2rs_type(
    lsf: &LangSpecFlat,
    sort: &SortId<<LangSpecFlat as LangSpec>::AlgebraicSortId>,
) -> syn::Type {
    match sort {
        SortId::NatLiteral => parse_quote!(usize),
        SortId::Algebraic(asi) => asi2rs_type(lsf, asi),
        SortId::Set(asi) => {
            let inner_ty = asi2rs_type(lsf, asi);
            parse_quote!(Vec<#inner_ty>)
        }
        SortId::Sequence(asi) => {
            let inner_ty = asi2rs_type(lsf, asi);
            parse_quote!(Vec<#inner_ty>)
        }
    }
}

pub fn asi2rs_ident(
    lsf: &LangSpecFlat,
    asi: &<LangSpecFlat as LangSpec>::AlgebraicSortId,
) -> syn::Ident {
    let name = lsf.algebraic_sort_name(*asi);
    name_as_rs_type(name)
}
pub fn sort2rs_ident(
    lsf: &LangSpecFlat,
    sort: &SortId<<LangSpecFlat as LangSpec>::AlgebraicSortId>,
) -> syn::Ident {
    match sort {
        SortId::NatLiteral => syn::Ident::new("NatLit", proc_macro2::Span::call_site()),
        SortId::Algebraic(asi) => asi2rs_ident(lsf, asi),
        SortId::Set(asi) => syn::Ident::new(
            &format!("NatSet_{}", asi2rs_ident(lsf, asi)),
            proc_macro2::Span::call_site(),
        ),
        SortId::Sequence(asi) => syn::Ident::new(
            &format!("NatSeq_{}", asi2rs_ident(lsf, asi)),
            proc_macro2::Span::call_site(),
        ),
    }
}

pub fn data_structure(lsf: &LangSpecFlat) -> (Vec<ItemStruct>, Vec<ItemEnum>) {
    fn struct_item(
        name: &Name,
        lsf: &LangSpecFlat,
        sorts: &[SortId<<LangSpecFlat as LangSpec>::AlgebraicSortId>],
    ) -> ItemStruct {
        let name = name_as_rs_type(name);
        let fields = sorts
            .iter()
            .map(|sort| -> syn::Type { sort2rs_type(lsf, sort) });
        parse_quote!(
            pub struct #name(#(pub #fields),*);
        )
    }
    fn enum_item(
        name: &Name,
        lsf: &LangSpecFlat,
        sorts: &[SortId<<LangSpecFlat as LangSpec>::AlgebraicSortId>],
    ) -> ItemEnum {
        let name = name_as_rs_type(name);
        let variant_tys = sorts
            .iter()
            .map(|sort| -> syn::Type {
                let rs_type = sort2rs_type(lsf, sort);
                if let SortId::Algebraic(_) = sort {
                    parse_quote!(Box<#rs_type>)
                } else {
                    rs_type
                }
            })
            .collect::<Vec<_>>();
        let variant_names = sorts
            .iter()
            .map(|sort| -> syn::Ident { sort2rs_ident(lsf, sort) })
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
    let (structs, enums) = data_structure(&lsf);
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

    // #[test]
    // fn compilation() {
    //     let t = trybuild::TestCases::new();
    //     t.pass("tests/*.rs");
    // }
}
