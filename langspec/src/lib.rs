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

    // fn asi_convert(&self, id: Self::AlgebraicSortId) -> langspec::AlgebraicSortId<(), ()> {
    //     if let Some(pid) = self
    //         .products
    //         .iter()
    //         .enumerate()
    //         .find(|(_idx, prod)| prod.name.name == id)
    //     {
    //         langspec::AlgebraicSortId::Product(())
    //     } else if let Some(sid) = self
    //         .sums
    //         .iter()
    //         .enumerate()
    //         .find(|(_idx, sum)| sum.name.name == id)
    //     {
    //         langspec::AlgebraicSortId::Sum(())
    //     } else {
    //         panic!("Sort not found: {}", id)
    //     }
    // }
}
impl langspec::LangSpec for flat::LangSpecFlat {
    type ProductId = flat::ProductId;

    type SumId = flat::SumId;

    type AlgebraicSortId = flat::FlatAlgebraicSortId;

    fn name(&self) -> &langspec::Name {
        &self.name
    }

    fn products(&self) -> impl Iterator<Item = Self::ProductId> {
        (0..self.products.len()).map(flat::ProductId)
    }

    fn sums(&self) -> impl Iterator<Item = Self::SumId> {
        (0..self.sums.len()).map(flat::SumId)
    }

    fn product_name(&self, id: Self::ProductId) -> &langspec::Name {
        &self.products[id].name
    }

    fn sum_name(&self, id: Self::SumId) -> &langspec::Name {
        &self.sums[id].name
    }

    fn product_sorts(
        &self,
        id: Self::ProductId,
    ) -> impl Iterator<Item = langspec::SortId<Self::AlgebraicSortId>> {
        self.products[id].sorts.iter().cloned()
    }

    fn sum_sorts(
        &self,
        id: Self::SumId,
    ) -> impl Iterator<Item = langspec::SortId<Self::AlgebraicSortId>> {
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

    fn asi_convert(
        &self,
        id: Self::AlgebraicSortId,
    ) -> langspec::AlgebraicSortId<Self::ProductId, Self::SumId> {
        id
    }
}
