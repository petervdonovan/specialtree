pub mod flat;
pub mod humanreadable;
pub mod langspec;

impl langspec::LangSpec for humanreadable::LangSpecHuman {
    type ProductId = String;

    type SumId = String;

    type AlgebraicSortId = String;

    fn name(&self) -> &langspec::Name {
        &self.name
    }

    fn products(&self) -> impl Iterator<Item = Self::ProductId> {
        self.products.iter().map(|p| p.name.name.clone())
    }

    fn sums(&self) -> impl Iterator<Item = Self::SumId> {
        self.sums.iter().map(|s| s.name.name.clone())
    }

    fn product_name(&self, id: Self::ProductId) -> &langspec::Name {
        self.products
            .iter()
            .find(|p| p.name.name == id)
            .map(|p| &p.name)
            .unwrap()
    }

    fn sum_name(&self, id: Self::SumId) -> &langspec::Name {
        self.sums
            .iter()
            .find(|s| s.name.name == id)
            .map(|s| &s.name)
            .unwrap()
    }

    fn product_sorts(
        &self,
        id: Self::ProductId,
    ) -> impl Iterator<Item = langspec::SortId<Self::AlgebraicSortId>> {
        self.products
            .iter()
            .find(|p| p.name.name == id)
            .map(|p| p.sorts.iter().cloned())
            .unwrap()
    }

    fn sum_sorts(
        &self,
        id: Self::SumId,
    ) -> impl Iterator<Item = langspec::SortId<Self::AlgebraicSortId>> {
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

    fn asi_convert(
        &self,
        id: Self::AlgebraicSortId,
    ) -> langspec::AlgebraicSortId<Self::ProductId, Self::SumId> {
        if let Some(pid) = self
            .products
            .iter()
            .map(|p| p.name.name.clone())
            .find(|name| name == &id)
        {
            langspec::AlgebraicSortId::Product(pid)
        } else if let Some(sid) = self
            .sums
            .iter()
            .map(|s| s.name.name.clone())
            .find(|name| name == &id)
        {
            langspec::AlgebraicSortId::Sum(sid)
        } else {
            panic!("Sort not found: {}", id)
        }
    }
}
