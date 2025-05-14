use serde::{Deserialize, Serialize};

use crate::{langspec::Name, sublang::reflexive_sublang, tymetafunc::TyMetaFuncSpec};

#[derive(Debug, Serialize, Deserialize)]
#[serde(bound = "")]
pub struct LangSpecHuman<Tmfs: TyMetaFuncSpec> {
    pub name: Name,
    pub products: Vec<Product<Tmfs>>,
    pub sums: Vec<Sum<Tmfs>>,
    #[serde(skip)]
    _phantom: std::marker::PhantomData<Tmfs>,
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(bound = "")]
pub struct Product<Tmfs: TyMetaFuncSpec> {
    pub name: Name,
    pub sorts: Vec<SortId<Tmfs>>,
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(bound = "")]
pub struct Sum<Tmfs: TyMetaFuncSpec> {
    pub name: Name,
    pub sorts: Vec<SortId<Tmfs>>,
}

pub type SortId<Tmfs> = crate::langspec::SortIdOf<LangSpecHuman<Tmfs>>;

impl<Tmfs: TyMetaFuncSpec> crate::langspec::LangSpec for crate::humanreadable::LangSpecHuman<Tmfs> {
    type ProductId = String;

    type SumId = String;

    type Tmfs = Tmfs;

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
    ) -> impl Iterator<Item = crate::langspec::SortIdOf<Self>> {
        self.products
            .iter()
            .find(|p| p.name.human == id)
            .map(|p| p.sorts.iter().cloned())
            .unwrap()
    }

    fn sum_sorts(&self, id: Self::SumId) -> impl Iterator<Item = crate::langspec::SortIdOf<Self>> {
        self.sums
            .iter()
            .find(|s| s.name.human == id)
            .map(|s| s.sorts.iter().cloned())
            .unwrap()
    }

    fn sublangs(&self) -> Vec<crate::sublang::Sublang<crate::langspec::SortIdOf<Self>>> {
        vec![reflexive_sublang(self)]
    }
}
