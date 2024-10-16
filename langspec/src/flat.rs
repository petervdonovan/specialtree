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

#[derive(Debug, Serialize, Deserialize, Clone, Copy, From, Into)]
pub struct ProductId(pub usize);
#[derive(Debug, Serialize, Deserialize, Clone, Copy, From, Into)]
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

impl TerminalLangSpec for LangSpecFlat {
    fn from<L: LangSpec>(l: &L) -> Self {
        let name = l.name().clone();
        let products = l
            .products()
            .map(|pid| {
                let name = l.product_name(pid.clone()).clone();
                let sorts = l
                    .product_sorts(pid)
                    .map(|sid| {
                        sid.fmap(|asi| match l.asi_convert(asi) {
                            AlgebraicSortId::Product(p) => {
                                FlatAlgebraicSortId::Product(ProductId(l.prod_to_unique_nat(p)))
                            }
                            AlgebraicSortId::Sum(s) => {
                                FlatAlgebraicSortId::Sum(SumId(l.sum_to_unique_nat(s)))
                            }
                        })
                    })
                    .collect::<Vec<_>>()
                    .into_boxed_slice();
                Product { name, sorts }
            })
            .collect::<Vec<_>>()
            .into();
        let sums = l
            .sums()
            .map(|sid| {
                let name = l.sum_name(sid.clone()).clone();
                let sorts = l
                    .sum_sorts(sid)
                    .map(|sid| {
                        sid.fmap(|asi| match l.asi_convert(asi) {
                            AlgebraicSortId::Product(p) => {
                                FlatAlgebraicSortId::Product(ProductId(l.prod_to_unique_nat(p)))
                            }
                            AlgebraicSortId::Sum(s) => {
                                FlatAlgebraicSortId::Sum(SumId(l.sum_to_unique_nat(s)))
                            }
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

// impl From<LangSpecFlat> for LangSpecHuman {
//     fn from(ls: LangSpecFlat) -> Self {
//         fn sortid2string(s: crate::flat::FlatAlgebraicSortId, ls: &LangSpecFlat) -> String {
//             match s {
//                 crate::flat::FlatAlgebraicSortId::Product(pid) => {
//                     ls.products[pid].name.name.clone()
//                 }
//                 crate::flat::FlatAlgebraicSortId::Sum(sid) => ls.sums[sid].name.name.clone(),
//             }
//         }
//         fn map_sorts(
//             sorts: &[crate::flat::FlatSortId],
//             ls: &LangSpecFlat,
//         ) -> Vec<crate::humanreadable::SortId> {
//             sorts
//                 .iter()
//                 .map(|sid| match sid {
//                     crate::flat::FlatSortId::Algebraic(a) => {
//                         crate::humanreadable::SortId::Algebraic(sortid2string(*a, ls))
//                     }
//                     crate::flat::FlatSortId::Set(a) => {
//                         crate::humanreadable::SortId::Set(sortid2string(*a, ls))
//                     }
//                     crate::flat::FlatSortId::Sequence(a) => {
//                         crate::humanreadable::SortId::Algebraic(sortid2string(*a, ls))
//                     }
//                     crate::flat::FlatSortId::NatLiteral => crate::humanreadable::SortId::NatLiteral,
//                 })
//                 .collect()
//         }
//         let name = ls.name.clone();
//         let products = ls
//             .products
//             .iter_enumerated()
//             .map(|(_prodid, prod)| crate::humanreadable::Product {
//                 name: prod.name.clone(),
//                 sorts: map_sorts(prod.sorts.as_ref(), &ls),
//             })
//             .collect::<Vec<_>>();
//         let sums = ls
//             .sums
//             .iter_enumerated()
//             .map(|(_sumid, sum)| crate::humanreadable::Sum {
//                 name: sum.name.clone(),
//                 sorts: map_sorts(sum.sorts.as_ref(), &ls),
//             })
//             .collect::<Vec<_>>();
//         LangSpecHuman {
//             name,
//             products,
//             sums,
//         }
//     }
// }
