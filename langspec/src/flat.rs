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
