use langspec::langspec::{LangSpec, Name};

pub struct SumExtendAll<L: LangSpec> {
    name: Name,
    l: L,
    extend_with: L::ProductId,
}

impl<L: LangSpec> LangSpec for SumExtendAll<L> {
    type ProductId = L::ProductId;

    type SumId = L::SumId;

    type AlgebraicSortId = L::AlgebraicSortId;

    fn name(&self) -> &Name {
        &self.name
    }

    fn products(&self) -> impl Iterator<Item = Self::ProductId> {
        self.l.products()
    }

    fn sums(&self) -> impl Iterator<Item = Self::SumId> {
        self.l.sums()
    }

    fn product_name(&self, id: Self::ProductId) -> &Name {
        self.l.product_name(id)
    }

    fn sum_name(&self, id: Self::SumId) -> &Name {
        self.l.sum_name(id)
    }

    fn product_sorts(
        &self,
        id: Self::ProductId,
    ) -> impl Iterator<Item = langspec::langspec::SortId<Self::AlgebraicSortId>> {
        self.l.product_sorts(id)
    }

    fn sum_sorts(
        &self,
        id: Self::SumId,
    ) -> impl Iterator<Item = langspec::langspec::SortId<Self::AlgebraicSortId>> {
        self.l
            .sum_sorts(id)
            .chain(std::iter::once(langspec::langspec::SortId::Algebraic(
                self.l
                    .asi_unconvert(langspec::langspec::AlgebraicSortId::Product(
                        self.extend_with.clone(),
                    )),
            ))) // This is the only change
    }

    fn prod_to_unique_nat(&self, id: Self::ProductId) -> usize {
        self.l.prod_to_unique_nat(id)
    }

    fn prod_from_unique_nat(&self, nat: usize) -> Self::ProductId {
        self.l.prod_from_unique_nat(nat)
    }

    fn sum_to_unique_nat(&self, id: Self::SumId) -> usize {
        self.l.sum_to_unique_nat(id)
    }

    fn sum_from_unique_nat(&self, nat: usize) -> Self::SumId {
        self.l.sum_from_unique_nat(nat)
    }

    fn asi_convert(
        &self,
        id: Self::AlgebraicSortId,
    ) -> langspec::langspec::UnpackedAlgebraicSortId<Self> {
        self.l.asi_convert(id)
    }

    fn asi_unconvert(
        &self,
        id: langspec::langspec::UnpackedAlgebraicSortId<Self>,
    ) -> Self::AlgebraicSortId {
        self.l.asi_unconvert(id)
    }
}
