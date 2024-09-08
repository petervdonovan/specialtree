use derive_more::{From, Into};
use serde::{Deserialize, Serialize};
use typed_index_collections::TiVec;

#[derive(Debug, Serialize, Deserialize)]
pub struct Language {
    pub products: TiVec<ProductId, Product>,
    pub sums: TiVec<SumId, Sum>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, From, Into)]
pub struct ProductId(pub usize);
#[derive(Debug, Serialize, Deserialize, Clone, Copy, From, Into)]
pub struct SumId(pub usize);
#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum SortId {
    Algebraic(AlgebraicSortId),
    Set(AlgebraicSortId),
    Sequence(AlgebraicSortId),
    NatLiteral,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum AlgebraicSortId {
    Product(ProductId),
    Sum(SumId),
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Product {
    pub name: String,
    pub sorts: Box<[SortId]>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Sum {
    pub name: String,
    pub sorts: Box<[SortId]>,
}
