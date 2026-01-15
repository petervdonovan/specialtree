use std::any::TypeId;

use derive_more::{From, Into};
use serde::{Deserialize, Serialize};
use typed_index_collections::TiVec;

use crate::{
    langspec::{AlgebraicSortId, AsLifetime, LangSpec, SortIdOf, TerminalLangSpec},
    sublang::{Sublang, reflexive_sublang},
    tymetafunc::TyMetaFuncSpec,
};
use tree_identifier::Identifier;

#[derive(Debug, Serialize, Deserialize)]
#[serde(bound = "")]
pub struct LangSpecFlat<Tmfs: TyMetaFuncSpec> {
    pub name: Identifier,
    pub products: TiVec<ProductId, Product<Tmfs>>,
    pub sums: TiVec<SumId, Sum<Tmfs>>,
    _phantom: std::marker::PhantomData<Tmfs>,
}

impl<Tmfs: TyMetaFuncSpec> LangSpecFlat<Tmfs> {
    pub fn empty(name: Identifier) -> Self {
        LangSpecFlat {
            name,
            products: TiVec::new(),
            sums: TiVec::new(),
            _phantom: std::marker::PhantomData,
        }
    }
}

#[derive(
    Debug, Serialize, Deserialize, Clone, Copy, From, Into, PartialEq, Eq, Hash, PartialOrd, Ord,
)]
pub struct ProductId(pub usize);
#[derive(
    Debug, Serialize, Deserialize, Clone, Copy, From, Into, PartialEq, Eq, Hash, PartialOrd, Ord,
)]
pub struct SumId(pub usize);
pub type FlatSortId<Tmfs> = SortIdOf<LangSpecFlat<Tmfs>>;
pub type FlatAlgebraicSortId = AlgebraicSortId<ProductId, SumId>;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(bound = "")]
pub struct Product<Tmfs: TyMetaFuncSpec> {
    pub name: Identifier,
    pub sorts: Box<[FlatSortId<Tmfs>]>,
}
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(bound = "")]
pub struct Sum<Tmfs: TyMetaFuncSpec> {
    pub name: Identifier,
    pub sorts: Box<[FlatSortId<Tmfs>]>,
}

impl<Tmfs: TyMetaFuncSpec> std::fmt::Display for LangSpecFlat<Tmfs> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_yml::to_string(self).unwrap())
    }
}

impl<Tmfs: TyMetaFuncSpec + 'static> AsLifetime<Self> for LangSpecFlat<Tmfs> {
    type AsLifetime<'a> = Self;
}

impl<Tmfs: TyMetaFuncSpec> crate::langspec::LangSpec for crate::flat::LangSpecFlat<Tmfs> {
    type ProductId = crate::flat::ProductId;

    type SumId = crate::flat::SumId;

    type Tmfs = Tmfs;

    fn name(&self) -> &Identifier {
        &self.name
    }

    fn products(&self) -> impl Iterator<Item = Self::ProductId> {
        (0..self.products.len()).map(crate::flat::ProductId)
    }

    fn sums(&self) -> impl Iterator<Item = Self::SumId> {
        (0..self.sums.len()).map(crate::flat::SumId)
    }

    fn product_name(&self, id: Self::ProductId) -> &Identifier {
        &self.products[id].name
    }

    fn sum_name(&self, id: Self::SumId) -> &Identifier {
        &self.sums[id].name
    }

    fn product_sorts(&self, id: Self::ProductId) -> impl Iterator<Item = SortIdOf<Self>> {
        self.products[id].sorts.iter().cloned()
    }

    fn sum_sorts(&self, id: Self::SumId) -> impl Iterator<Item = SortIdOf<Self>> {
        self.sums[id].sorts.iter().cloned()
    }

    fn sublang<'lsub: 'this, 'this, LSub: LangSpec>(
        &'this self,
        lsub: &'lsub LSub,
    ) -> Option<crate::sublang::Sublang<'this, LSub::AsLifetime<'this>, SortIdOf<Self>>> {
        if TypeId::of::<LSub::AsLifetime<'static>>() == TypeId::of::<Self>()
            && lsub.name() == self.name()
        {
            unsafe {
                Some(std::mem::transmute::<
                    Sublang<Self, SortIdOf<Self>>,
                    Sublang<LSub::AsLifetime<'this>, SortIdOf<Self>>,
                >(reflexive_sublang(self)))
            }
        } else {
            None
        }
    }

    // fn sublangs(&self) -> Vec<crate::sublang::Sublang<SortIdOf<Self>>> {
    //     vec![reflexive_sublang(self)]
    // }
}

impl<Tmfs: TyMetaFuncSpec> TerminalLangSpec for LangSpecFlat<Tmfs> {
    fn canonical_from<L: LangSpec<Tmfs = Tmfs>>(l: &L) -> Self {
        let name = l.name().clone();
        let mut products_sorted = l.products().collect::<Vec<_>>();
        products_sorted.sort_by_key(|pid| l.product_name(pid.clone()));
        let products_sorted = products_sorted;
        let mut sums_sorted = l.sums().collect::<Vec<_>>();
        sums_sorted.sort_by_key(|sid| l.sum_name(sid.clone()));
        let sums_sorted = sums_sorted;
        let products = products_sorted
            .iter()
            .map(|pid| {
                let name = l.product_name(pid.clone()).clone();
                let sorts = l
                    .product_sorts(pid.clone())
                    .map(|sid| {
                        sid.fmap_p(|p| {
                            ProductId(products_sorted.iter().position(|it| it == &p).unwrap())
                        })
                        .fmap_s(|s| SumId(sums_sorted.iter().position(|it| it == &s).unwrap()))
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
                        sid.fmap_p(|p| {
                            ProductId(products_sorted.iter().position(|it| it == &p).unwrap())
                        })
                        .fmap_s(|s| SumId(sums_sorted.iter().position(|it| it == &s).unwrap()))
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
            _phantom: std::marker::PhantomData,
        }
    }
}
