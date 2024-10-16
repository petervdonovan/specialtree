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
    fn from<L: crate::langspec::LangSpec>(l: &L) -> Self {
        let name = l.name().clone();
        let mut products_sorted = l.products().collect::<Vec<_>>();
        products_sorted.sort_by_key(|pid| l.product_name(pid.clone()).name.clone());
        let products_sorted = products_sorted;
        let mut sums_sorted = l.sums().collect::<Vec<_>>();
        sums_sorted.sort_by_key(|sid| l.sum_name(sid.clone()).name.clone());
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
                                l.product_name(p).name.clone()
                            }
                            crate::langspec::AlgebraicSortId::Sum(s) => l.sum_name(s).name.clone(),
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
                                l.product_name(p).name.clone()
                            }
                            crate::langspec::AlgebraicSortId::Sum(s) => l.sum_name(s).name.clone(),
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
