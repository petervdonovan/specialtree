use serde::{Deserialize, Serialize};

use crate::{
    langspec::{AsLifetime, LangSpec, SortIdOf},
    sublang::Sublang,
    tymetafunc::TyMetaFuncSpec,
};
use tree_identifier::Identifier;

#[derive(Debug, Serialize, Deserialize)]
#[serde(bound = "")]
pub struct LangSpecHuman<Tmfs: TyMetaFuncSpec> {
    pub name: Identifier,
    pub products: Vec<Product<Tmfs>>,
    pub sums: Vec<Sum<Tmfs>>,
    #[serde(skip)]
    _phantom: std::marker::PhantomData<Tmfs>,
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(bound = "")]
pub struct Product<Tmfs: TyMetaFuncSpec> {
    pub name: Identifier,
    pub sorts: Vec<SortId<Tmfs>>,
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(bound = "")]
pub struct Sum<Tmfs: TyMetaFuncSpec> {
    pub name: Identifier,
    pub sorts: Vec<SortId<Tmfs>>,
}

pub type SortId<Tmfs> = crate::langspec::SortIdOf<LangSpecHuman<Tmfs>>;

impl<Tmfs: TyMetaFuncSpec + 'static> AsLifetime for crate::humanreadable::LangSpecHuman<Tmfs> {
    type AsLifetime<'a> = Self;
}

impl<Tmfs: TyMetaFuncSpec + 'static> crate::langspec::LangSpec
    for crate::humanreadable::LangSpecHuman<Tmfs>
{
    type ProductId = String;

    type SumId = String;

    type Tmfs = Tmfs;

    fn name(&self) -> &Identifier {
        &self.name
    }

    fn products(&self) -> impl Iterator<Item = Self::ProductId> {
        self.products.iter().map(|p| p.name.kebab_str())
    }

    fn sums(&self) -> impl Iterator<Item = Self::SumId> {
        self.sums.iter().map(|s| s.name.kebab_str())
    }

    fn product_name(&self, id: Self::ProductId) -> &Identifier {
        self.products
            .iter()
            .find(|p| p.name.kebab_str() == id)
            .map(|p| &p.name)
            .unwrap()
    }

    fn sum_name(&self, id: Self::SumId) -> &Identifier {
        self.sums
            .iter()
            .find(|s| s.name.kebab_str() == id)
            .map(|s| &s.name)
            .unwrap()
    }

    fn product_sorts(
        &self,
        id: Self::ProductId,
    ) -> impl Iterator<Item = crate::langspec::SortIdOf<Self>> {
        self.products
            .iter()
            .find(|p| p.name.kebab_str() == id)
            .map(|p| p.sorts.iter().cloned())
            .unwrap()
    }

    fn sum_sorts(&self, id: Self::SumId) -> impl Iterator<Item = crate::langspec::SortIdOf<Self>> {
        self.sums
            .iter()
            .find(|s| s.name.kebab_str() == id)
            .map(|s| s.sorts.iter().cloned())
            .unwrap()
    }

    fn sublang<'lsub: 'this, 'this, LSub: LangSpec>(
        &'this self,
        _lsub: &'lsub LSub,
    ) -> Option<Sublang<'this, LSub::AsLifetime<'this>, SortIdOf<Self>>> {
        unimplemented!()
    }
}
