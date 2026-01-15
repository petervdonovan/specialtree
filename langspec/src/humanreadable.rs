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

impl<Tmfs: TyMetaFuncSpec + 'static> AsLifetime<Self>
    for crate::humanreadable::LangSpecHuman<Tmfs>
{
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
        // Validate that ProductId references in sorts match actual product names
        let product = self.products.iter().find(|p| p.name.kebab_str() == id);

        if let Some(product) = product {
            // Check if any ProductId references in the sorts are in wrong format
            for sort in &product.sorts {
                self.validate_sort_references(sort);
            }
            product.sorts.iter().cloned()
        } else {
            panic!(
                "Product with id '{}' not found. Available products: [{}]",
                id,
                self.products
                    .iter()
                    .map(|p| format!("'{}'", p.name.kebab_str()))
                    .collect::<Vec<_>>()
                    .join(", ")
            )
        }
    }

    fn sum_sorts(&self, id: Self::SumId) -> impl Iterator<Item = crate::langspec::SortIdOf<Self>> {
        // Validate that ProductId references in sorts match actual product names
        let sum = self.sums.iter().find(|s| s.name.kebab_str() == id);

        if let Some(sum) = sum {
            // Check if any ProductId/SumId references in the sorts are in wrong format
            for sort in &sum.sorts {
                self.validate_sort_references(sort);
            }
            sum.sorts.iter().cloned()
        } else {
            panic!(
                "Sum with id '{}' not found. Available sums: [{}]",
                id,
                self.sums
                    .iter()
                    .map(|s| format!("'{}'", s.name.kebab_str()))
                    .collect::<Vec<_>>()
                    .join(", ")
            )
        }
    }

    fn sublang<'lsub: 'this, 'this, LSub: LangSpec>(
        &'this self,
        _lsub: &'lsub LSub,
    ) -> Option<Sublang<'this, LSub::AsLifetime<'this>, SortIdOf<Self>>> {
        unimplemented!()
    }
}

impl<Tmfs: TyMetaFuncSpec> LangSpecHuman<Tmfs> {
    /// Validates all sort references in the language specification
    /// This should be called after deserialization to catch naming issues early
    pub fn validate_all_sort_references(&self) {
        // Validate all sorts in all products
        for product in &self.products {
            for sort in &product.sorts {
                self.validate_sort_references(sort);
            }
        }

        // Validate all sorts in all sums
        for sum in &self.sums {
            for sort in &sum.sorts {
                self.validate_sort_references(sort);
            }
        }
    }

    /// Validates that ProductId and SumId references in sorts use the correct kebab-case format
    fn validate_sort_references(&self, sort: &SortIdOf<Self>) {
        use crate::langspec::{AlgebraicSortId, SortId};

        match sort {
            SortId::Algebraic(AlgebraicSortId::Product(product_id)) => {
                // Check if this ProductId exists in our products
                let exists = self
                    .products
                    .iter()
                    .any(|p| p.name.kebab_str() == *product_id);
                if !exists {
                    // Check if there's a product with a similar name in CamelCase
                    let camel_matches: Vec<_> = self
                        .products
                        .iter()
                        .filter(|p| p.name.camel_str() == *product_id)
                        .collect();

                    if !camel_matches.is_empty() {
                        panic!(
                            "ProductId '{}' not found. Did you mean '{}' (kebab-case)? \
                               JSON should use kebab-case ProductId references, not CamelCase. \
                               Available products: [{}]",
                            product_id,
                            camel_matches[0].name.kebab_str(),
                            self.products
                                .iter()
                                .map(|p| format!("'{}'", p.name.kebab_str()))
                                .collect::<Vec<_>>()
                                .join(", ")
                        );
                    } else {
                        panic!(
                            "ProductId '{}' not found. Available products: [{}]",
                            product_id,
                            self.products
                                .iter()
                                .map(|p| format!("'{}'", p.name.kebab_str()))
                                .collect::<Vec<_>>()
                                .join(", ")
                        );
                    }
                }
            }
            SortId::Algebraic(AlgebraicSortId::Sum(sum_id)) => {
                // Check if this SumId exists in our sums
                let exists = self.sums.iter().any(|s| s.name.kebab_str() == *sum_id);
                if !exists {
                    // Check if there's a sum with a similar name in CamelCase
                    let camel_matches: Vec<_> = self
                        .sums
                        .iter()
                        .filter(|s| s.name.camel_str() == *sum_id)
                        .collect();

                    if !camel_matches.is_empty() {
                        panic!(
                            "SumId '{}' not found. Did you mean '{}' (kebab-case)? \
                               JSON should use kebab-case SumId references, not CamelCase. \
                               Available sums: [{}]",
                            sum_id,
                            camel_matches[0].name.kebab_str(),
                            self.sums
                                .iter()
                                .map(|s| format!("'{}'", s.name.kebab_str()))
                                .collect::<Vec<_>>()
                                .join(", ")
                        );
                    } else {
                        panic!(
                            "SumId '{}' not found. Available sums: [{}]",
                            sum_id,
                            self.sums
                                .iter()
                                .map(|s| format!("'{}'", s.name.kebab_str()))
                                .collect::<Vec<_>>()
                                .join(", ")
                        );
                    }
                }
            }
            SortId::TyMetaFunc(mapped_type) => {
                // TyMetaFunc sorts can contain ProductId/SumId references in their arguments
                for arg_sort in &mapped_type.a {
                    self.validate_sort_references(arg_sort);
                }
            }
        }
    }
}
