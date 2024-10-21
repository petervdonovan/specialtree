use serde::{Deserialize, Serialize};

use crate::langspec::{Name, TerminalLangSpec};

#[derive(Debug, Serialize, Deserialize)]
pub struct LangSpecHuman {
    pub name: Name,
    pub products: Vec<Product>,
    pub sums: Vec<Sum>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Product {
    pub name: Name,
    pub sorts: Vec<SortId>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Sum {
    pub name: Name,
    pub sorts: Vec<SortId>,
}
pub type SortId = crate::langspec::SortId<String>;

impl TerminalLangSpec for LangSpecHuman {
    fn canonical_from<L: crate::langspec::LangSpec>(l: &L) -> Self {
        let name = l.name().clone();
        let mut products_sorted = l.products().collect::<Vec<_>>();
        products_sorted.sort_by_key(|pid| l.product_name(pid.clone()).human.clone());
        let products_sorted = products_sorted;
        let mut sums_sorted = l.sums().collect::<Vec<_>>();
        sums_sorted.sort_by_key(|sid| l.sum_name(sid.clone()).human.clone());
        let sums_sorted = sums_sorted;
        let products = products_sorted
            .iter()
            .map(|p| {
                let name = l.product_name(p.clone()).clone();
                let sorts = l
                    .product_sorts(p.clone())
                    .map(|sid| {
                        sid.fmap(|asi| match l.asi_convert(asi) {
                            crate::langspec::AlgebraicSortId::Product(p) => {
                                l.product_name(p).human.clone()
                            }
                            crate::langspec::AlgebraicSortId::Sum(s) => l.sum_name(s).human.clone(),
                        })
                    })
                    .collect();
                Product { name, sorts }
            })
            .collect();
        let sums = sums_sorted
            .iter()
            .map(|s| {
                let name = l.sum_name(s.clone()).clone();
                let sorts = l
                    .sum_sorts(s.clone())
                    .map(|sid| {
                        sid.fmap(|asi| match l.asi_convert(asi) {
                            crate::langspec::AlgebraicSortId::Product(p) => {
                                l.product_name(p).human.clone()
                            }
                            crate::langspec::AlgebraicSortId::Sum(s) => l.sum_name(s).human.clone(),
                        })
                    })
                    .collect();
                Sum { name, sorts }
            })
            .collect();
        LangSpecHuman {
            name,
            products,
            sums,
        }
    }
}

impl crate::langspec::LangSpec for crate::humanreadable::LangSpecHuman {
    type ProductId = String;

    type SumId = String;

    type AlgebraicSortId = String;

    fn name(&self) -> &crate::langspec::Name {
        &self.name
    }

    fn products(&self) -> impl Iterator<Item = Self::ProductId> {
        self.products.iter().map(|p| p.name.human.clone())
    }

    fn sums(&self) -> impl Iterator<Item = Self::SumId> {
        self.sums.iter().map(|s| s.name.human.clone())
    }

    fn product_name(&self, id: Self::ProductId) -> &crate::langspec::Name {
        self.products
            .iter()
            .find(|p| p.name.human == id)
            .map(|p| &p.name)
            .unwrap()
    }

    fn sum_name(&self, id: Self::SumId) -> &crate::langspec::Name {
        self.sums
            .iter()
            .find(|s| s.name.human == id)
            .map(|s| &s.name)
            .unwrap()
    }

    fn product_sorts(
        &self,
        id: Self::ProductId,
    ) -> impl Iterator<Item = crate::langspec::SortId<Self::AlgebraicSortId>> {
        self.products
            .iter()
            .find(|p| p.name.human == id)
            .map(|p| p.sorts.iter().cloned())
            .unwrap()
    }

    fn sum_sorts(
        &self,
        id: Self::SumId,
    ) -> impl Iterator<Item = crate::langspec::SortId<Self::AlgebraicSortId>> {
        self.sums
            .iter()
            .find(|s| s.name.human == id)
            .map(|s| s.sorts.iter().cloned())
            .unwrap()
    }

    fn prod_to_unique_nat(&self, id: Self::ProductId) -> usize {
        self.products
            .iter()
            .position(|p| p.name.human == id)
            .unwrap()
    }

    fn prod_from_unique_nat(&self, nat: usize) -> Self::ProductId {
        self.products[nat].name.human.clone()
    }

    fn sum_to_unique_nat(&self, id: Self::SumId) -> usize {
        self.sums.iter().position(|s| s.name.human == id).unwrap()
    }

    fn sum_from_unique_nat(&self, nat: usize) -> Self::SumId {
        self.sums[nat].name.human.clone()
    }

    fn asi_convert(
        &self,
        id: Self::AlgebraicSortId,
    ) -> crate::langspec::AlgebraicSortId<Self::ProductId, Self::SumId> {
        if let Some(pid) = self
            .products
            .iter()
            .map(|p| p.name.human.clone())
            .find(|name| name == &id)
        {
            crate::langspec::AlgebraicSortId::Product(pid)
        } else if let Some(sid) = self
            .sums
            .iter()
            .map(|s| s.name.human.clone())
            .find(|name| name == &id)
        {
            crate::langspec::AlgebraicSortId::Sum(sid)
        } else {
            panic!("Sort not found: {}", id)
        }
    }
}
