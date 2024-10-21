use functor_derive::Functor;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Name {
    /// Must be a valid identifier in the Rust language; must be unique within this LangSpec
    pub name: String,
    /// Optional alias which may be more human-readable and can be an arbitrary UTF-8 string
    pub alias: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, Functor)]
pub enum SortId<AlgebraicSortId> {
    NatLiteral,
    Algebraic(AlgebraicSortId),
    Set(AlgebraicSortId),
    Sequence(AlgebraicSortId),
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum AlgebraicSortId<ProductId, SumId> {
    Product(ProductId),
    Sum(SumId),
}

pub trait LangSpec {
    type ProductId: Clone + Eq;
    type SumId: Clone + Eq;
    type AlgebraicSortId: Clone + Eq;

    fn name(&self) -> &Name;
    fn products(&self) -> impl Iterator<Item = Self::ProductId>;
    fn sums(&self) -> impl Iterator<Item = Self::SumId>;
    fn product_name(&self, id: Self::ProductId) -> &Name;
    fn sum_name(&self, id: Self::SumId) -> &Name;
    fn algebraic_sort_name(&self, id: AlgebraicSortId<Self::ProductId, Self::SumId>) -> &Name {
        match id {
            AlgebraicSortId::Product(pid) => self.product_name(pid),
            AlgebraicSortId::Sum(sid) => self.sum_name(sid),
        }
    }
    fn product_sorts(
        &self,
        id: Self::ProductId,
    ) -> impl Iterator<Item = SortId<Self::AlgebraicSortId>>;
    fn sum_sorts(&self, id: Self::SumId) -> impl Iterator<Item = SortId<Self::AlgebraicSortId>>;
    fn prod_to_unique_nat(&self, id: Self::ProductId) -> usize;
    fn prod_from_unique_nat(&self, nat: usize) -> Self::ProductId;
    fn sum_to_unique_nat(&self, id: Self::SumId) -> usize;
    fn sum_from_unique_nat(&self, nat: usize) -> Self::SumId;
    fn asi_convert(
        &self,
        id: Self::AlgebraicSortId,
    ) -> AlgebraicSortId<Self::ProductId, Self::SumId>;
    fn sid_convert(
        &self,
        sid: SortId<Self::AlgebraicSortId>,
    ) -> SortId<AlgebraicSortId<Self::ProductId, Self::SumId>> {
        match sid {
            SortId::NatLiteral => SortId::NatLiteral,
            SortId::Algebraic(asi) => SortId::Algebraic(self.asi_convert(asi)),
            SortId::Set(asi) => SortId::Set(self.asi_convert(asi)),
            SortId::Sequence(asi) => SortId::Sequence(self.asi_convert(asi)),
        }
    }
}
/// Marks a langspec as an element of the iso class of terminal objects in the category of [LangSpec]s.
/// Needed because a [From] impl would conflict with the blanket impl for [From] for all types.
pub trait TerminalLangSpec: LangSpec {
    fn canonical_from<L: LangSpec>(l: &L) -> Self;
}
