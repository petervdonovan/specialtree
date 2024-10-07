use serde::{Deserialize, Serialize};

use crate::{LangSpec, Name};

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
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SortName {
    name: String,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum SortId {
    Algebraic(SortName),
    Set(SortName),
    Sequence(SortName),
    NatLiteral,
}

impl From<LangSpecHuman> for LangSpec {
    fn from(value: LangSpecHuman) -> Self {
        // fn string2sortid(s: String, ls: &LangSpec) -> crate::AlgebraicSortId {
        //     if let Some(pid) = ls.products.iter_enumerated().find(|(_prodid, prod)| prod.name.name == s) {
        //         crate::AlgebraicSortId::Product(pid.0)
        //     } else if let Some(sid) = ls.sums.iter_enumerated().find(|(_sumid, sum)| sum.name.name == s) {
        //         crate::AlgebraicSortId::Sum(sid.0)
        //     } else {
        //         panic!("Sort not found: {}", s)
        //     }
        // }
        fn string2sortid(s: String, lsh: &LangSpecHuman) -> crate::AlgebraicSortId {
            if let Some(pid) = lsh
                .products
                .iter()
                .enumerate()
                .find(|(_idx, prod)| prod.name.name == s)
            {
                crate::AlgebraicSortId::Product(pid.0.into())
            } else if let Some(sid) = lsh
                .sums
                .iter()
                .enumerate()
                .find(|(_idx, sum)| sum.name.name == s)
            {
                crate::AlgebraicSortId::Sum(sid.0.into())
            } else {
                panic!("Sort not found: {}", s)
            }
        }
        fn map_sorts(sorts: &[SortId], lsh: &LangSpecHuman) -> Box<[crate::SortId]> {
            sorts
                .iter()
                .map(|sid| match sid {
                    SortId::Algebraic(s) => {
                        crate::SortId::Algebraic(string2sortid(s.name.clone(), lsh))
                    }
                    SortId::Set(s) => crate::SortId::Set(string2sortid(s.name.clone(), lsh)),
                    SortId::Sequence(s) => {
                        crate::SortId::Algebraic(string2sortid(s.name.clone(), lsh))
                    }
                    SortId::NatLiteral => crate::SortId::NatLiteral,
                })
                .collect()
        }
        let name = value.name.clone();
        let products = value
            .products
            .iter()
            .map(|prod| crate::Product {
                name: prod.name.clone(),
                sorts: map_sorts(&prod.sorts, &value),
            })
            .collect();
        let sums = value
            .sums
            .iter()
            .map(|sum| crate::Sum {
                name: sum.name.clone(),
                sorts: map_sorts(&sum.sorts, &value),
            })
            .collect();
        LangSpec {
            name,
            products,
            sums,
        }
    }
}

impl From<&LangSpec> for LangSpecHuman {
    fn from(ls: &LangSpec) -> Self {
        fn sortid2string(s: crate::AlgebraicSortId, ls: &LangSpec) -> String {
            match s {
                crate::AlgebraicSortId::Product(pid) => ls.products[pid].name.name.clone(),
                crate::AlgebraicSortId::Sum(sid) => ls.sums[sid].name.name.clone(),
            }
        }
        fn map_sorts(sorts: &[crate::SortId], ls: &LangSpec) -> Vec<SortId> {
            sorts
                .iter()
                .map(|sid| match sid {
                    crate::SortId::Algebraic(a) => SortId::Algebraic(SortName {
                        name: sortid2string(*a, ls),
                    }),
                    crate::SortId::Set(a) => SortId::Set(SortName {
                        name: sortid2string(*a, ls),
                    }),
                    crate::SortId::Sequence(a) => SortId::Algebraic(SortName {
                        name: sortid2string(*a, ls),
                    }),
                    crate::SortId::NatLiteral => SortId::NatLiteral,
                })
                .collect()
        }
        let name = ls.name.clone();
        let products = ls
            .products
            .iter_enumerated()
            .map(|(_prodid, prod)| Product {
                name: prod.name.clone(),
                sorts: map_sorts(prod.sorts.as_ref(), ls),
            })
            .collect::<Vec<_>>();
        let sums = ls
            .sums
            .iter_enumerated()
            .map(|(_sumid, sum)| Sum {
                name: sum.name.clone(),
                sorts: map_sorts(sum.sorts.as_ref(), ls),
            })
            .collect::<Vec<_>>();
        LangSpecHuman {
            name,
            products,
            sums,
        }
    }
}

impl std::fmt::Display for LangSpecHuman {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_yml::to_string(self).unwrap())
    }
}
