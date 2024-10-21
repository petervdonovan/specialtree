use derive_more::{From, Into};
use serde::{Deserialize, Serialize};
use typed_index_collections::TiVec;

use crate::langspec::{AlgebraicSortId, LangSpec, Name, SortId, TerminalLangSpec};

#[derive(Debug, Serialize, Deserialize)]
pub struct LangSpecFlat {
    pub name: Name,
    pub products: TiVec<ProductId, Product>,
    pub sums: TiVec<SumId, Sum>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, From, Into, PartialEq, Eq)]
pub struct ProductId(pub usize);
#[derive(Debug, Serialize, Deserialize, Clone, Copy, From, Into, PartialEq, Eq)]
pub struct SumId(pub usize);
pub type FlatSortId = SortId<FlatAlgebraicSortId>;
pub type FlatAlgebraicSortId = AlgebraicSortId<ProductId, SumId>;

#[derive(Debug, Serialize, Deserialize)]
pub struct Product {
    pub name: Name,
    pub sorts: Box<[FlatSortId]>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Sum {
    pub name: Name,
    pub sorts: Box<[FlatSortId]>,
}

impl std::fmt::Display for LangSpecFlat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_yml::to_string(self).unwrap())
    }
}

impl crate::langspec::LangSpec for crate::flat::LangSpecFlat {
    type ProductId = crate::flat::ProductId;

    type SumId = crate::flat::SumId;

    type AlgebraicSortId = crate::flat::FlatAlgebraicSortId;

    fn name(&self) -> &crate::langspec::Name {
        &self.name
    }

    fn products(&self) -> impl Iterator<Item = Self::ProductId> {
        (0..self.products.len()).map(crate::flat::ProductId)
    }

    fn sums(&self) -> impl Iterator<Item = Self::SumId> {
        (0..self.sums.len()).map(crate::flat::SumId)
    }

    fn product_name(&self, id: Self::ProductId) -> &crate::langspec::Name {
        &self.products[id].name
    }

    fn sum_name(&self, id: Self::SumId) -> &crate::langspec::Name {
        &self.sums[id].name
    }

    fn product_sorts(
        &self,
        id: Self::ProductId,
    ) -> impl Iterator<Item = crate::langspec::SortId<Self::AlgebraicSortId>> {
        self.products[id].sorts.iter().cloned()
    }

    fn sum_sorts(
        &self,
        id: Self::SumId,
    ) -> impl Iterator<Item = crate::langspec::SortId<Self::AlgebraicSortId>> {
        self.sums[id].sorts.iter().cloned()
    }

    fn prod_to_unique_nat(&self, id: Self::ProductId) -> usize {
        id.0
    }

    fn prod_from_unique_nat(&self, nat: usize) -> Self::ProductId {
        crate::flat::ProductId(nat)
    }

    fn sum_to_unique_nat(&self, id: Self::SumId) -> usize {
        id.0
    }

    fn sum_from_unique_nat(&self, nat: usize) -> Self::SumId {
        crate::flat::SumId(nat)
    }

    fn asi_convert(
        &self,
        id: Self::AlgebraicSortId,
    ) -> crate::langspec::AlgebraicSortId<Self::ProductId, Self::SumId> {
        id
    }
}

impl TerminalLangSpec for LangSpecFlat {
    fn canonical_from<L: LangSpec>(l: &L) -> Self {
        let name = l.name().clone();
        let mut products_sorted = l.products().collect::<Vec<_>>();
        products_sorted.sort_by_key(|pid| l.product_name(pid.clone()).name.clone());
        let products_sorted = products_sorted;
        let mut sums_sorted = l.sums().collect::<Vec<_>>();
        sums_sorted.sort_by_key(|sid| l.sum_name(sid.clone()).name.clone());
        let sums_sorted = sums_sorted;
        let products = products_sorted
            .iter()
            .map(|pid| {
                let name = l.product_name(pid.clone()).clone();
                let sorts = l
                    .product_sorts(pid.clone())
                    .map(|sid| {
                        sid.fmap(|asi| match l.asi_convert(asi) {
                            AlgebraicSortId::Product(p) => FlatAlgebraicSortId::Product(ProductId(
                                products_sorted.iter().position(|it| it == &p).unwrap(),
                            )),
                            AlgebraicSortId::Sum(s) => FlatAlgebraicSortId::Sum(SumId(
                                sums_sorted.iter().position(|it| it == &s).unwrap(),
                            )),
                        })
                    })
                    .collect::<Vec<_>>()
                    .into_boxed_slice();
                Product { name, sorts }
            })
            .collect::<Vec<_>>()
            .into();
        let sums = sums_sorted
            .iter()
            .map(|sid| {
                let name = l.sum_name(sid.clone()).clone();
                let sorts = l
                    .sum_sorts(sid.clone())
                    .map(|sid| {
                        sid.fmap(|asi| match l.asi_convert(asi) {
                            AlgebraicSortId::Product(p) => FlatAlgebraicSortId::Product(ProductId(
                                products_sorted.iter().position(|it| it == &p).unwrap(),
                            )),
                            AlgebraicSortId::Sum(s) => FlatAlgebraicSortId::Sum(SumId(
                                sums_sorted.iter().position(|it| it == &s).unwrap(),
                            )),
                        })
                    })
                    .collect::<Vec<_>>()
                    .into_boxed_slice();
                Sum { name, sorts }
            })
            .collect::<Vec<_>>()
            .into();
        LangSpecFlat {
            name,
            products,
            sums,
        }
    }
}
