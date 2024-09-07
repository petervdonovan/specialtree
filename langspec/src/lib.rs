use derive_more::{From, Into};
use serde::{Deserialize, Serialize};
use typed_index_collections::TiVec;

#[derive(Debug, Serialize, Deserialize)]
pub struct Language {
    pub functions: TiVec<FunctionId, Function>,
    pub sort_names: TiVec<SortId, String>,
}
#[derive(Debug, Serialize, Deserialize, Clone, Copy, From, Into)]
pub struct SortId(pub usize);
#[derive(Debug, Serialize, Deserialize, Clone, Copy, From, Into)]
pub struct FunctionId(pub usize);
#[derive(Debug, Serialize, Deserialize)]
pub struct Function {
    pub name: String,
    pub domain: Domain,
    pub codomain: SortId,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Domain(pub Box<[(SortId, Arity)]>);
#[derive(Debug, Serialize, Deserialize)]
pub enum Arity {
    N(u8),
    FiniteSet,
    FiniteSequence,
}
