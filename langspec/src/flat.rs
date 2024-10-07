use derive_more::{From, Into};
use serde::{Deserialize, Serialize};
use typed_index_collections::TiVec;

use crate::humanreadable::LangSpecHuman;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Name {
    /// Must be a valid identifier in the Rust language; must be unique within this LangSpec
    pub name: String,
    /// Optional alias which may be more human-readable and can be an arbitrary UTF-8 string
    pub alias: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LangSpecFlat {
    pub name: Name,
    pub products: TiVec<ProductId, Product>,
    pub sums: TiVec<SumId, Sum>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, From, Into)]
pub struct ProductId(pub usize);
#[derive(Debug, Serialize, Deserialize, Clone, Copy, From, Into)]
pub struct SumId(pub usize);
#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum SortId {
    NatLiteral,
    Algebraic(AlgebraicSortId),
    Set(AlgebraicSortId),
    Sequence(AlgebraicSortId),
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum AlgebraicSortId {
    Product(ProductId),
    Sum(SumId),
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Product {
    pub name: Name,
    pub sorts: Box<[SortId]>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Sum {
    pub name: Name,
    pub sorts: Box<[SortId]>,
}

impl std::fmt::Display for LangSpecFlat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_yml::to_string(self).unwrap())
    }
}

impl From<LangSpecHuman> for LangSpecFlat {
    fn from(value: LangSpecHuman) -> Self {
        fn string2sortid(s: String, lsh: &LangSpecHuman) -> crate::flat::AlgebraicSortId {
            if let Some(pid) = lsh
                .products
                .iter()
                .enumerate()
                .find(|(_idx, prod)| prod.name.name == s)
            {
                crate::flat::AlgebraicSortId::Product(pid.0.into())
            } else if let Some(sid) = lsh
                .sums
                .iter()
                .enumerate()
                .find(|(_idx, sum)| sum.name.name == s)
            {
                crate::flat::AlgebraicSortId::Sum(sid.0.into())
            } else {
                panic!("Sort not found: {}", s)
            }
        }
        fn map_sorts(
            sorts: &[crate::humanreadable::SortId],
            lsh: &LangSpecHuman,
        ) -> Box<[crate::flat::SortId]> {
            sorts
                .iter()
                .map(|sid| match sid {
                    crate::humanreadable::SortId::Algebraic(s) => {
                        crate::flat::SortId::Algebraic(string2sortid(s.clone(), lsh))
                    }
                    crate::humanreadable::SortId::Set(s) => {
                        crate::flat::SortId::Set(string2sortid(s.clone(), lsh))
                    }
                    crate::humanreadable::SortId::Sequence(s) => {
                        crate::flat::SortId::Algebraic(string2sortid(s.clone(), lsh))
                    }
                    crate::humanreadable::SortId::NatLiteral => crate::flat::SortId::NatLiteral,
                })
                .collect()
        }
        let name = value.name.clone();
        let products = value
            .products
            .iter()
            .map(|prod| crate::flat::Product {
                name: prod.name.clone(),
                sorts: map_sorts(&prod.sorts, &value),
            })
            .collect();
        let sums = value
            .sums
            .iter()
            .map(|sum| crate::flat::Sum {
                name: sum.name.clone(),
                sorts: map_sorts(&sum.sorts, &value),
            })
            .collect();
        LangSpecFlat {
            name,
            products,
            sums,
        }
    }
}

impl From<&LangSpecFlat> for LangSpecHuman {
    fn from(ls: &LangSpecFlat) -> Self {
        fn sortid2string(s: crate::flat::AlgebraicSortId, ls: &LangSpecFlat) -> String {
            match s {
                crate::flat::AlgebraicSortId::Product(pid) => ls.products[pid].name.name.clone(),
                crate::flat::AlgebraicSortId::Sum(sid) => ls.sums[sid].name.name.clone(),
            }
        }
        fn map_sorts(
            sorts: &[crate::flat::SortId],
            ls: &LangSpecFlat,
        ) -> Vec<crate::humanreadable::SortId> {
            sorts
                .iter()
                .map(|sid| match sid {
                    crate::flat::SortId::Algebraic(a) => {
                        crate::humanreadable::SortId::Algebraic(sortid2string(*a, ls))
                    }
                    crate::flat::SortId::Set(a) => {
                        crate::humanreadable::SortId::Set(sortid2string(*a, ls))
                    }
                    crate::flat::SortId::Sequence(a) => {
                        crate::humanreadable::SortId::Algebraic(sortid2string(*a, ls))
                    }
                    crate::flat::SortId::NatLiteral => crate::humanreadable::SortId::NatLiteral,
                })
                .collect()
        }
        let name = ls.name.clone();
        let products = ls
            .products
            .iter_enumerated()
            .map(|(_prodid, prod)| crate::humanreadable::Product {
                name: prod.name.clone(),
                sorts: map_sorts(prod.sorts.as_ref(), ls),
            })
            .collect::<Vec<_>>();
        let sums = ls
            .sums
            .iter_enumerated()
            .map(|(_sumid, sum)| crate::humanreadable::Sum {
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
