use serde::{Deserialize, Serialize};

use crate::langspec::{Name, ToLiteral};

pub struct RustGenericTrait<ArgIds> {
    ty_func: syn::Path,
    args: Box<[ArgIds]>,
}
pub struct TyMetaFuncData<TyMetaFuncArgId> {
    name: Name,
    args: Box<[(TyMetaFuncArgId, Name)]>,
    maybe_conversions: Box<[RustGenericTrait<TyMetaFuncArgId>]>,
    canonical_froms: Box<[RustGenericTrait<TyMetaFuncArgId>]>,
}

pub trait TyMetaFuncSpec: Sized {
    type TyMetaFuncId: Serialize
        + for<'a> Deserialize<'a>
        + std::fmt::Debug
        + Clone
        + Eq
        + ToLiteral;
    type TyMetaFuncArgId: Serialize
        + for<'a> Deserialize<'a>
        + std::fmt::Debug
        + Clone
        + Eq
        + ToLiteral;
    fn ty_meta_funcs(&self) -> impl Iterator<Item = Self::TyMetaFuncId>;
    fn ty_meta_func_data(&self, id: &Self::TyMetaFuncId) -> TyMetaFuncData<Self::TyMetaFuncArgId>;
}
