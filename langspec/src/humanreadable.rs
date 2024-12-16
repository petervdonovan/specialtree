use serde::{Deserialize, Serialize};

use crate::{langspec::Name, tymetafunc::TyMetaFuncSpec};

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
}

impl crate::langspec::ToLiteral for String {
    fn to_literal(&self) -> syn::Expr {
        syn::parse_quote! { #self }
    }
}

#[cfg(test)]
mod test {
    use crate::langspec::ToLiteral as _;

    #[test]
    fn test_to_literal() {
        let s = "foo".to_string();
        let literal = s.to_literal();
        assert_eq!(quote::quote!(#literal).to_string(), "\"foo\"");
    }
}
