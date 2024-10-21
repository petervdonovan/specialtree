use langspec::langspec::{AlgebraicSortId, LangSpec, Name, SortId};
use syn::parse_quote;

pub fn name_as_snake_ident(name: &Name) -> syn::Ident {
    syn::Ident::new(&name.snake, proc_macro2::Span::call_site())
}
pub fn name_as_camel_ident(name: &Name) -> syn::Ident {
    syn::Ident::new(&name.camel, proc_macro2::Span::call_site())
}

pub trait LangSpecGen {
    type ProductId: Clone + Eq;
    type SumId: Clone + Eq;
    fn asi2rs_type(&self, asi: AlgebraicSortId<Self::ProductId, Self::SumId>) -> syn::Type;
    fn sort2rs_type(
        &self,
        sort: SortId<AlgebraicSortId<Self::ProductId, Self::SumId>>,
    ) -> syn::Type;
    fn asi2rs_ident(&self, asi: AlgebraicSortId<Self::ProductId, Self::SumId>) -> syn::Ident;
    fn sort2rs_ident(
        &self,
        sort: SortId<AlgebraicSortId<Self::ProductId, Self::SumId>>,
    ) -> syn::Ident;
}

impl<L: LangSpec> LangSpecGen for L {
    type ProductId = <Self as LangSpec>::ProductId;
    type SumId = <Self as LangSpec>::SumId;
    fn asi2rs_type(&self, asi: AlgebraicSortId<Self::ProductId, Self::SumId>) -> syn::Type {
        let name = self.algebraic_sort_name(asi);
        let ty_name = name_as_camel_ident(name);
        parse_quote!(#ty_name)
    }
    fn sort2rs_type(
        &self,
        sort: SortId<AlgebraicSortId<Self::ProductId, Self::SumId>>,
    ) -> syn::Type {
        match sort {
            SortId::NatLiteral => parse_quote!(usize),
            SortId::Algebraic(asi) => self.asi2rs_type(asi),
            SortId::Set(asi) => {
                let inner_ty = self.asi2rs_type(asi);
                parse_quote!(Vec<#inner_ty>)
            }
            SortId::Sequence(asi) => {
                let inner_ty = self.asi2rs_type(asi);
                parse_quote!(Vec<#inner_ty>)
            }
        }
    }
    fn asi2rs_ident(&self, asi: AlgebraicSortId<Self::ProductId, Self::SumId>) -> syn::Ident {
        let name = self.algebraic_sort_name(asi);
        name_as_camel_ident(name)
    }
    fn sort2rs_ident(
        &self,
        sort: SortId<AlgebraicSortId<Self::ProductId, Self::SumId>>,
    ) -> syn::Ident {
        match sort {
            SortId::NatLiteral => syn::Ident::new("NatLit", proc_macro2::Span::call_site()),
            SortId::Algebraic(asi) => self.asi2rs_ident(asi),
            SortId::Set(asi) => syn::Ident::new(
                &format!("NatSetOf{}", self.asi2rs_ident(asi)),
                proc_macro2::Span::call_site(),
            ),
            SortId::Sequence(asi) => syn::Ident::new(
                &format!("NatSeqOf{}", self.asi2rs_ident(asi)),
                proc_macro2::Span::call_site(),
            ),
        }
    }
}
