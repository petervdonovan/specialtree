use langspec::langspec::{AlgebraicSortId, LangSpec, Name, SortId};
use syn::parse_quote;

pub mod idxbased;
pub mod refbased;

pub fn name_as_snake_ident(name: &Name) -> syn::Ident {
    syn::Ident::new(&name.snake, proc_macro2::Span::call_site())
}
pub fn name_as_camel_ident(name: &Name) -> syn::Ident {
    syn::Ident::new(&name.camel, proc_macro2::Span::call_site())
}

pub fn asi2rs_type<L: LangSpec>(
    lsf: &L,
    asi: AlgebraicSortId<L::ProductId, L::SumId>,
) -> syn::Type {
    let name = lsf.algebraic_sort_name(asi);
    let ty_name = name_as_camel_ident(name);
    parse_quote!(#ty_name)
}

pub fn sort2rs_type<L: LangSpec>(
    lsf: &L,
    sort: SortId<AlgebraicSortId<L::ProductId, L::SumId>>,
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

pub fn asi2rs_ident<L: LangSpec>(
    lsf: &L,
    asi: AlgebraicSortId<L::ProductId, L::SumId>,
) -> syn::Ident {
    let name = lsf.algebraic_sort_name(asi);
    name_as_camel_ident(name)
}
pub fn sort2rs_ident<L: LangSpec>(
    lsf: &L,
    sort: SortId<AlgebraicSortId<L::ProductId, L::SumId>>,
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
