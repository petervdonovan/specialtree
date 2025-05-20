use serde::{Deserialize, Serialize};

use crate::langspec::Name;

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
    pub canonical_froms: Box<[Box<[ArgId]>]>,
    pub size_depends_on: Box<[ArgId]>,
    pub is_collection_of: Box<[ArgId]>,
    pub transparency: Transparency,
}
#[derive(Clone, PartialEq, Eq)]
pub enum Transparency {
    Transparent,
    Visible,
}

pub trait TyMetaFuncSpec: Sized + 'static {
    type TyMetaFuncId: Serialize
        + for<'a> Deserialize<'a>
        + std::fmt::Debug
        + Clone
        + Eq
        + std::hash::Hash
        + 'static
        + Ord;
    fn ty_meta_func_data(id: &Self::TyMetaFuncId) -> TyMetaFuncData;
}
