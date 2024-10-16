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
        let products = l
            .products()
            .map(|p| {
                let name = l.product_name(p.clone()).clone();
                let sorts = l
                    .product_sorts(p)
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
        let sums = l
            .sums()
            .map(|s| {
                let name = l.sum_name(s.clone()).clone();
                let sorts = l
                    .sum_sorts(s)
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
