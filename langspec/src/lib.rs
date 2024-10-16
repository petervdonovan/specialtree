use flat::{Name, SortId};
use humanreadable::LangSpecHuman;

pub mod flat;
pub mod humanreadable;

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

impl LangSpec for humanreadable::LangSpecHuman {
    type ProductId = String;

    type SumId = String;

    type AlgebraicSortId = String;

    fn name(&self) -> &Name {
        &self.name
    }

    fn products(&self) -> impl Iterator<Item = Self::ProductId> {
        self.products.iter().map(|p| p.name.name.clone())
    }

    fn sums(&self) -> impl Iterator<Item = Self::SumId> {
        self.sums.iter().map(|s| s.name.name.clone())
    }

    fn product_name(&self, id: Self::ProductId) -> &Name {
        self.products
            .iter()
            .find(|p| p.name.name == id)
            .map(|p| &p.name)
            .unwrap()
    }

    fn sum_name(&self, id: Self::SumId) -> &Name {
        self.sums
            .iter()
            .find(|s| s.name.name == id)
            .map(|s| &s.name)
            .unwrap()
    }

    fn product_sorts(
        &self,
        id: Self::ProductId,
    ) -> impl Iterator<Item = SortId<Self::AlgebraicSortId>> {
        self.products
            .iter()
            .find(|p| p.name.name == id)
            .map(|p| p.sorts.iter().cloned())
            .unwrap()
    }

    fn sum_sorts(&self, id: Self::SumId) -> impl Iterator<Item = SortId<Self::AlgebraicSortId>> {
        self.sums
            .iter()
            .find(|s| s.name.name == id)
            .map(|s| s.sorts.iter().cloned())
            .unwrap()
    }

    fn prod_to_unique_nat(&self, id: Self::ProductId) -> usize {
        self.products
            .iter()
            .position(|p| p.name.name == id)
            .unwrap()
    }

    fn prod_from_unique_nat(&self, nat: usize) -> Self::ProductId {
        self.products[nat].name.name.clone()
    }

    fn sum_to_unique_nat(&self, id: Self::SumId) -> usize {
        self.sums.iter().position(|s| s.name.name == id).unwrap()
    }

    fn sum_from_unique_nat(&self, nat: usize) -> Self::SumId {
        self.sums[nat].name.name.clone()
    }
}
impl LangSpec for flat::LangSpecFlat {
    type ProductId = flat::ProductId;

    type SumId = flat::SumId;

    type AlgebraicSortId = flat::FlatAlgebraicSortId;

    fn name(&self) -> &Name {
        &self.name
    }

    fn products(&self) -> impl Iterator<Item = Self::ProductId> {
        (0..self.products.len()).map(flat::ProductId)
    }

    fn sums(&self) -> impl Iterator<Item = Self::SumId> {
        (0..self.sums.len()).map(flat::SumId)
    }

    fn product_name(&self, id: Self::ProductId) -> &Name {
        &self.products[id].name
    }

    fn sum_name(&self, id: Self::SumId) -> &Name {
        &self.sums[id].name
    }

    fn product_sorts(
        &self,
        id: Self::ProductId,
    ) -> impl Iterator<Item = SortId<Self::AlgebraicSortId>> {
        self.products[id].sorts.iter().cloned()
    }

    fn sum_sorts(&self, id: Self::SumId) -> impl Iterator<Item = SortId<Self::AlgebraicSortId>> {
        self.sums[id].sorts.iter().cloned()
    }

    fn prod_to_unique_nat(&self, id: Self::ProductId) -> usize {
        id.0
    }

    fn prod_from_unique_nat(&self, nat: usize) -> Self::ProductId {
        flat::ProductId(nat)
    }

    fn sum_to_unique_nat(&self, id: Self::SumId) -> usize {
        id.0
    }

    fn sum_from_unique_nat(&self, nat: usize) -> Self::SumId {
        flat::SumId(nat)
    }
}
