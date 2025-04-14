use serde::{Deserialize, Serialize};

use crate::langspec::{Name, ToLiteral};
// #[derive(Clone)]
// pub struct RustGenericTrait {
//     pub tys2trait_func: syn::Path,
// }
#[derive(Clone)]
pub struct RustTyMap {
    pub ty_func: syn::Path,
}
#[derive(Clone)]
pub enum IdentifiedBy {
    Tmf,
    FirstTmfArg,
}
#[derive(Clone)]
pub struct ArgId(pub usize);
#[derive(Clone)]
pub struct TyMetaFuncData {
    pub name: Name,
    pub args: Box<[Name]>,
    pub imp: RustTyMap,
    pub idby: IdentifiedBy,
    pub heapbak: RustTyMap,
    // pub maybe_conversions: Box<[RustGenericTrait]>,
    pub canonical_froms: Box<[Box<[ArgId]>]>,
    pub transparency: Transparency,
}
#[derive(Clone)]
pub enum Transparency {
    Transparent,
    Visible,
}

pub trait TyMetaFuncSpec: Sized {
    type TyMetaFuncId: Serialize
        + for<'a> Deserialize<'a>
        + std::fmt::Debug
        + Clone
        + Eq
        + std::hash::Hash
        + 'static
        + ToLiteral;
    type OpaqueTerm: Serialize + for<'a> Deserialize<'a> + std::fmt::Debug + Clone + 'static;
    fn ty_meta_func_data(id: &Self::TyMetaFuncId) -> TyMetaFuncData;
    fn my_type() -> syn::Type;
}
