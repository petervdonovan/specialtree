use serde::{Deserialize, Serialize};

use crate::humanreadable::LangSpecHuman;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Name {
    /// Must be a valid identifier in the Rust language; must be unique within this LangSpec
    pub name: String,
    /// Optional alias which may be more human-readable and can be an arbitrary UTF-8 string
    pub alias: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum SortId<AlgebraicSortId> {
    NatLiteral,
    Algebraic(AlgebraicSortId),
    Set(AlgebraicSortId),
    Sequence(AlgebraicSortId),
}

pub trait LangSpec
where
    LangSpecHuman: From<Self>,
    Self: From<LangSpecHuman>,
{
    type ProductId;
    type SumId;
    type AlgebraicSortId;

    fn name(&self) -> &Name;
    fn products(&self) -> impl Iterator<Item = Self::ProductId>;
    fn sums(&self) -> impl Iterator<Item = Self::SumId>;
    fn product_name(&self, id: Self::ProductId) -> &Name;
    fn sum_name(&self, id: Self::SumId) -> &Name;
    fn product_sorts(
        &self,
        id: Self::ProductId,
    ) -> impl Iterator<Item = SortId<Self::AlgebraicSortId>>;
    fn sum_sorts(&self, id: Self::SumId) -> impl Iterator<Item = SortId<Self::AlgebraicSortId>>;
    fn prod_to_unique_nat(&self, id: Self::ProductId) -> usize;
    fn prod_from_unique_nat(&self, nat: usize) -> Self::ProductId;
    fn sum_to_unique_nat(&self, id: Self::SumId) -> usize;
    fn sum_from_unique_nat(&self, nat: usize) -> Self::SumId;
}
