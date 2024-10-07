use derive_more::{From, Into};
use serde::{Deserialize, Serialize};
use typed_index_collections::TiVec;

pub mod humanreadable;

#[derive(Debug, Clone, Serialize, Deserialize, knuffel::Decode)]
pub struct Name {
    /// Must be a valid identifier in the Rust language; must be unique within this LangSpec
    #[knuffel(argument)]
    pub name: String,
    /// Optional alias which may be more human-readable and can be an arbitrary UTF-8 string
    #[knuffel(argument)]
    pub alias: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LangSpec {
    pub name: Name,
    pub products: TiVec<ProductId, Product>,
    pub sums: TiVec<SumId, Sum>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, From, Into)]
pub struct ProductId(pub usize);
#[derive(Debug, Serialize, Deserialize, Clone, Copy, From, Into)]
pub struct SumId(pub usize);
#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum SortId {
    NatLiteral,
    Algebraic(AlgebraicSortId),
    Set(AlgebraicSortId),
    Sequence(AlgebraicSortId),
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum AlgebraicSortId {
    Product(ProductId),
    Sum(SumId),
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Product {
    pub name: Name,
    pub sorts: Box<[SortId]>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Sum {
    pub name: Name,
    pub sorts: Box<[SortId]>,
}

impl std::fmt::Display for LangSpec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            sexprfmt::fmt_value(&serde_lexpr::to_value(self).unwrap())
        )
    }
}
