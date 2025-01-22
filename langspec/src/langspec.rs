#![allow(clippy::needless_question_mark)]

use functor_derive::Functor;
use serde::{Deserialize, Serialize};

use crate::tymetafunc::TyMetaFuncSpec;

// use crate::humanreadable::SortId;

#[derive(Debug, Clone, Serialize, Deserialize)]
/// All names must be unique within a LangSpec unless they are different fields of the same [Name]
pub struct Name {
    /// Must not contain whitespace (whether operator symbols such as '-' are permitted is TBD)
    pub human: String,
    /// CamelCase alias
    pub camel: String,
    /// snake_case alias
    pub snake: String,
}
#[allow(type_alias_bounds)]
pub type SortIdOf<L: LangSpec + ?Sized> =
    SortId<L::ProductId, L::SumId, <L::Tmfs as TyMetaFuncSpec>::TyMetaFuncId>;

#[derive(Debug, Serialize, Deserialize, Clone, Functor)]
#[functor(ProductId as p, SumId as s, TyMetaFuncId as f)]
pub enum SortId<ProductId, SumId, TyMetaFuncId> {
    Algebraic(AlgebraicSortId<ProductId, SumId>),
    TyMetaFunc(MappedType<ProductId, SumId, TyMetaFuncId>),
}
#[derive(Debug, Serialize, Deserialize, Clone, Functor)]
// #[functor(AlgebraicSortId as a, TyMetaFuncId as f)]
pub struct MappedType<ProductId, SumId, TyMetaFuncId> {
    pub f: TyMetaFuncId,
    pub a: Vec<SortId<ProductId, SumId, TyMetaFuncId>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Functor)]
#[functor(ProductId as p, SumId as s)]
pub enum AlgebraicSortId<ProductId, SumId> {
    Product(ProductId),
    Sum(SumId),
}
pub type SortShape = SortId<(), (), ()>;
impl<P, S, F> SortId<P, S, F> {
    pub fn project(self) -> SortShape {
        self.fmap_p(|_| ()).fmap_s(|_| ()).fmap_f(|_| ())
    }
}

pub trait LangSpec {
    type ProductId: Clone + Eq + ToLiteral + 'static;
    type SumId: Clone + Eq + ToLiteral + 'static;
    type Tmfs: TyMetaFuncSpec;

    fn name(&self) -> &Name;
    fn products(&self) -> impl Iterator<Item = Self::ProductId>;
    fn sums(&self) -> impl Iterator<Item = Self::SumId>;
    fn product_name(&self, id: Self::ProductId) -> &Name;
    fn sum_name(&self, id: Self::SumId) -> &Name;
    // fn ty_meta_func_name(
    //     &self,
    //     id: <Self::TyMetaFuncSpec as TyMetaFuncSpec>::TyMetaFuncId,
    // ) -> &Name;
    fn algebraic_sort_name(&self, id: AlgebraicSortId<Self::ProductId, Self::SumId>) -> &Name {
        match id {
            AlgebraicSortId::Product(pid) => self.product_name(pid),
            AlgebraicSortId::Sum(sid) => self.sum_name(sid),
        }
    }
    fn product_sorts(&self, id: Self::ProductId) -> impl Iterator<Item = SortIdOf<Self>>;
    fn sum_sorts(&self, id: Self::SumId) -> impl Iterator<Item = SortIdOf<Self>>;
    // fn ty_meta_func_args(
    //     &self,
    //     id: <Self::Tmfs as TyMetaFuncSpec>::TyMetaFuncId,
    // ) -> impl Iterator<Item = &Name>;
    // fn ty_meta_funcs(&self) -> impl Iterator<Item = <Self::Tmfs as TyMetaFuncSpec>::TyMetaFuncId>;
    fn prod_to_unique_nat(&self, id: Self::ProductId) -> usize;
    fn prod_from_unique_nat(&self, nat: usize) -> Self::ProductId;
    fn sum_to_unique_nat(&self, id: Self::SumId) -> usize;
    fn sum_from_unique_nat(&self, nat: usize) -> Self::SumId;
    // fn asi_convert(&self, id: Self::AlgebraicSortId) -> UnpackedAlgebraicSortId<Self>;
    // fn asi_unconvert(&self, id: UnpackedAlgebraicSortId<Self>) -> Self::AlgebraicSortId;
    // fn sid_convert(
    //     &self,
    //     sid: SortId<Self::AlgebraicSortId, Self::TyMetaFuncId>,
    // ) -> UnpackedSortId<Self> {
    //     match sid {
    //         SortId::Algebraic(asi) => SortId::Algebraic(self.asi_convert(asi)),
    //         SortId::TyMetaFunc(fid) => SortId::TyMetaFunc(fid),
    //     }
    // }
    // fn product_datas(
    //     &self,
    // ) -> impl Iterator<
    //     Item = (
    //         &Name,
    //         impl Iterator<Item = SortId<Self::AlgebraicSortId, Self::TyMetaFuncId>>,
    //     ),
    // > {
    //     self.products().map(move |pid| {
    //         let name = self.product_name(pid.clone());
    //         let sorts = self.product_sorts(pid);
    //         (name, sorts)
    //     })
    // }
    // fn sum_datas(
    //     &self,
    // ) -> impl Iterator<
    //     Item = (
    //         &Name,
    //         impl Iterator<Item = SortId<Self::AlgebraicSortId, Self::TyMetaFuncId>>,
    //     ),
    // > {
    //     self.sums().map(move |sid| {
    //         let name = self.sum_name(sid.clone());
    //         let sorts = self.sum_sorts(sid);
    //         (name, sorts)
    //     })
    // }
    // fn ty_names(&self) -> impl Iterator<Item = &Name> {
    //     self.product_datas()
    //         .map(|(n, _)| n)
    //         .chain(self.sum_datas().map(|(n, _)| n))
    // }

    fn canonical_into<Bot: TerminalLangSpec<Tmfs = Self::Tmfs>>(&self) -> Bot {
        Bot::canonical_from(self)
    }
}
/// Marks a langspec as an element of the iso class of terminal objects in the category of [LangSpec]s.
/// Needed because a [From] impl would conflict with the blanket impl for [From] for all types.
pub trait TerminalLangSpec: LangSpec {
    fn canonical_from<L: LangSpec<Tmfs = <Self as LangSpec>::Tmfs> + ?Sized>(l: &L) -> Self;
}

pub trait ToLiteral {
    fn to_literal(&self) -> syn::Expr;
}
