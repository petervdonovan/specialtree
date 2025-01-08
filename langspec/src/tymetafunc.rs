use serde::{Deserialize, Serialize};

use crate::langspec::{Name, ToLiteral};
#[derive(Clone)]
pub struct RustGenericTrait<ArgIds> {
    pub tys2trait_func: syn::Path,
    pub args: Box<[ArgIds]>,
}
#[derive(Clone)]
pub struct RustTyMap<ArgIds> {
    pub ty_func: syn::Path,
    pub args: Box<[ArgIds]>,
}
#[derive(Clone)]
pub enum IdentifiedBy {
    Tmf,
    FirstTmfArg,
}
#[derive(Clone)]
pub struct TyMetaFuncData<TyMetaFuncArgId> {
    pub name: Name,
    pub args: Box<[(TyMetaFuncArgId, Name)]>,
    pub imp: RustTyMap<TyMetaFuncArgId>,
    pub idby: IdentifiedBy,
    pub maybe_conversions: Box<[RustGenericTrait<TyMetaFuncArgId>]>,
    pub canonical_froms: Box<[RustGenericTrait<TyMetaFuncArgId>]>,
}

pub trait TyMetaFuncSpec: Sized {
    type TyMetaFuncId: Serialize
        + for<'a> Deserialize<'a>
        + std::fmt::Debug
        + Clone
        + Eq
        + 'static
        + ToLiteral;
    type TyMetaFuncArgId: Serialize
        + for<'a> Deserialize<'a>
        + std::fmt::Debug
        + Clone
        + Eq
        + 'static
        + ToLiteral;
    // fn ty_meta_funcs(&self) -> impl Iterator<Item = Self::TyMetaFuncId>;
    fn ty_meta_func_data(id: &Self::TyMetaFuncId) -> TyMetaFuncData<Self::TyMetaFuncArgId>;
}
