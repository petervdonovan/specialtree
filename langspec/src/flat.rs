use derive_more::{From, Into};
use serde::{Deserialize, Serialize};
use typed_index_collections::TiVec;

use crate::{
    langspec::{AlgebraicSortId, LangSpec, Name, SortIdOf, TerminalLangSpec},
    tymetafunc::TyMetaFuncSpec,
};

#[derive(Debug, Serialize, Deserialize)]
#[serde(bound = "")]
pub struct LangSpecFlat<Tmfs: TyMetaFuncSpec> {
    pub name: Name,
    pub products: TiVec<ProductId, Product<Tmfs>>,
    pub sums: TiVec<SumId, Sum<Tmfs>>,
    // ty_meta_funcs: TiVec<FlatTyMetaFuncId, TyMetaFunc>,
    _phantom: std::marker::PhantomData<Tmfs>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, From, Into, PartialEq, Eq)]
pub struct ProductId(pub usize);
#[derive(Debug, Serialize, Deserialize, Clone, Copy, From, Into, PartialEq, Eq)]
pub struct SumId(pub usize);
// #[derive(Debug, Serialize, Deserialize, Clone, Copy, From, Into, PartialEq, Eq)]
// pub struct FlatTyMetaFuncId(pub usize);
pub type FlatSortId<Tmfs> = SortIdOf<LangSpecFlat<Tmfs>>;
pub type FlatAlgebraicSortId = AlgebraicSortId<ProductId, SumId>;

#[derive(Debug, Serialize, Deserialize)]
#[serde(bound = "")]
pub struct Product<Tmfs: TyMetaFuncSpec> {
    pub name: Name,
    pub sorts: Box<[FlatSortId<Tmfs>]>,
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(bound = "")]
pub struct Sum<Tmfs: TyMetaFuncSpec> {
    pub name: Name,
    pub sorts: Box<[FlatSortId<Tmfs>]>,
}

// #[derive(Debug, Serialize, Deserialize)]
// pub struct TyMetaFunc {
//     pub name: Name,
//     pub args: Vec<Name>,
// }

impl<Tmfs: TyMetaFuncSpec> std::fmt::Display for LangSpecFlat<Tmfs> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_yml::to_string(self).unwrap())
    }
}

impl<Tmfs: TyMetaFuncSpec> crate::langspec::LangSpec for crate::flat::LangSpecFlat<Tmfs> {
    type ProductId = crate::flat::ProductId;

    type SumId = crate::flat::SumId;

    type Tmfs = Tmfs;

    // type TyMetaFuncId = crate::flat::FlatTyMetaFuncId;

    // type AlgebraicSortId = crate::flat::FlatAlgebraicSortId;

    fn name(&self) -> &crate::langspec::Name {
        &self.name
    }

    fn products(&self) -> impl Iterator<Item = Self::ProductId> {
        (0..self.products.len()).map(crate::flat::ProductId)
    }

    fn sums(&self) -> impl Iterator<Item = Self::SumId> {
        (0..self.sums.len()).map(crate::flat::SumId)
    }

    // fn ty_meta_funcs(&self) -> impl Iterator<Item = <Self::Tmfs as TyMetaFuncSpec>::TyMetaFuncId> {
    //     (0..self.ty_meta_funcs.len()).map(crate::flat::FlatTyMetaFuncId)
    // }

    fn product_name(&self, id: Self::ProductId) -> &crate::langspec::Name {
        &self.products[id].name
    }

    fn sum_name(&self, id: Self::SumId) -> &crate::langspec::Name {
        &self.sums[id].name
    }

    fn product_sorts(&self, id: Self::ProductId) -> impl Iterator<Item = SortIdOf<Self>> {
        self.products[id].sorts.iter().cloned()
    }

    fn sum_sorts(&self, id: Self::SumId) -> impl Iterator<Item = SortIdOf<Self>> {
        self.sums[id].sorts.iter().cloned()
    }

    // fn ty_meta_func_args(
    //     &self,
    //     id: <Self::Tmfs as TyMetaFuncSpec>::TyMetaFuncId,
    // ) -> impl Iterator<Item = &crate::langspec::Name> {
    //     self.ty_meta_funcs[id].args.iter()
    // }

    fn prod_to_unique_nat(&self, id: Self::ProductId) -> usize {
        id.0
    }

    fn prod_from_unique_nat(&self, nat: usize) -> Self::ProductId {
        crate::flat::ProductId(nat)
    }

    fn sum_to_unique_nat(&self, id: Self::SumId) -> usize {
        id.0
    }

    fn sum_from_unique_nat(&self, nat: usize) -> Self::SumId {
        crate::flat::SumId(nat)
    }

    // fn asi_convert(
    //     &self,
    //     id: Self::AlgebraicSortId,
    // ) -> crate::langspec::AlgebraicSortId<Self::ProductId, Self::SumId> {
    //     id
    // }

    // fn asi_unconvert(
    //     &self,
    //     id: crate::langspec::UnpackedAlgebraicSortId<Self>,
    // ) -> Self::AlgebraicSortId {
    //     id
    // }
}

impl<Tmfs: TyMetaFuncSpec> TerminalLangSpec for LangSpecFlat<Tmfs> {
    fn canonical_from<L: LangSpec<Tmfs = Tmfs> + ?Sized>(l: &L) -> Self {
        let name = l.name().clone();
        let mut products_sorted = l.products().collect::<Vec<_>>();
        products_sorted.sort_by_key(|pid| l.product_name(pid.clone()).human.clone());
        let products_sorted = products_sorted;
        let mut sums_sorted = l.sums().collect::<Vec<_>>();
        sums_sorted.sort_by_key(|sid| l.sum_name(sid.clone()).human.clone());
        let sums_sorted = sums_sorted;
        // let mut ty_meta_funcs_sorted = l.ty_meta_funcs().collect::<Vec<_>>();
        // ty_meta_funcs_sorted.sort_by_key(|fid| l.ty_meta_func_name(fid.clone()).human.clone());
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
                        // .fmap_f(|fid| {
                        //     FlatTyMetaFuncId(
                        //         ty_meta_funcs_sorted
                        //             .iter()
                        //             .position(|it| it == &fid)
                        //             .unwrap(),
                        //     )
                        // })
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
                        // .fmap_f(|fid| {
                        //     FlatTyMetaFuncId(
                        //         ty_meta_funcs_sorted
                        //             .iter()
                        //             .position(|it| it == &fid)
                        //             .unwrap(),
                        //     )
                        // })
                    })
                    .collect::<Vec<_>>()
                    .into_boxed_slice();
                Sum { name, sorts }
            })
            .collect::<Vec<_>>()
            .into();
        // let ty_meta_funcs = ty_meta_funcs_sorted
        //     .iter()
        //     .map(|fid| {
        //         let name = l.ty_meta_func_name(fid.clone()).clone();
        //         let args = l.ty_meta_func_args(fid.clone()).cloned().collect();
        //         TyMetaFunc { name, args }
        //     })
        //     .collect::<Vec<_>>()
        //     .into();
        LangSpecFlat {
            name,
            products,
            sums,
            // ty_meta_funcs,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl crate::langspec::ToLiteral for ProductId {
    fn to_literal(&self) -> syn::Expr {
        let inner = &self.0;
        syn::parse_quote! { langspec::flat::ProductId(#inner) }
    }
}

impl crate::langspec::ToLiteral for SumId {
    fn to_literal(&self) -> syn::Expr {
        let inner = &self.0;
        syn::parse_quote! { langspec::flat::SumId(#inner) }
    }
}

// impl crate::langspec::ToLiteral for FlatTyMetaFuncId {
//     fn to_literal(&self) -> syn::Expr {
//         let inner = &self.0;
//         syn::parse_quote! { langspec::flat::FlatAlgebraicTyMetaFuncId(#inner) }
//     }
// }

#[cfg(test)]
mod test {
    use crate::langspec::ToLiteral as _;

    #[test]
    fn test_to_literal() {
        let s = crate::flat::ProductId(42usize);
        let literal = s.to_literal();
        assert_eq!(
            quote::quote!(#literal).to_string(),
            "langspec :: flat :: ProductId (42usize)"
        );
    }
}
