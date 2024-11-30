use langspec::langspec::{AlgebraicSortId, LangSpec};

pub struct TyGenData<L: LangSpec, I0: Iterator<Item = syn::Ident>, I1: Iterator<Item = syn::Type>> {
    pub id: L::AlgebraicSortId,
    pub snake_ident: syn::Ident,
    pub camel_ident: syn::Ident,
    pub cmt: CanonicallyMaybeToGenData<I0>,
    pub ccf: CanonicallyConstructibleFromGenData<I1>,
}
pub struct CanonicallyMaybeToGenData<I0: Iterator<Item = syn::Ident>> {
    pub cmt_sort_camels: I0,
}
pub type TyCamelIdent2Ty<'a> = &'a dyn Fn(syn::Ident, AlgebraicSortId<(), ()>) -> syn::Type;
pub struct CanonicallyConstructibleFromGenData<I0: Iterator<Item = syn::Type>> {
    pub ccf_sort_tys: fn(TyCamelIdent2Ty<'_>) -> I0,
}
