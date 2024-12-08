use bounded_slice::BoundedSlice;
use functor_derive::Functor;
use serde::{Deserialize, Serialize};

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

// pub type SortIdOf<L: LangSpec + ?Sized> =
//     SortId<L::ProductId, L::SumId, L::TyMetaFuncId, { L::MAX_TY_FUNC_ARGS as usize }>;
#[macro_export]
macro_rules! sort_id_of {
    ($langspec:ty) => {
        SortId<
            <$langspec as LangSpec>::ProductId,
            <$langspec as LangSpec>::SumId,
            <$langspec as LangSpec>::TyMetaFuncId,
            { <$langspec as LangSpec>::MAX_TY_FUNC_ARGS },
        >
    };
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, Functor)]
#[functor(ProductId as p, SumId as s, TyMetaFuncId as f)]
pub enum SortId<ProductId: Copy, SumId: Copy, TyMetaFuncId: Copy, const MAX_TY_FUNC_ARGS: usize> {
    Algebraic(AlgebraicSortId<ProductId, SumId>),
    TyMetaFunc(MappedType<AlgebraicSortId<ProductId, SumId>, TyMetaFuncId, MAX_TY_FUNC_ARGS>),
}
#[derive(Debug, Serialize, Deserialize, Clone, Copy, Functor)]
#[functor(AlgebraicSortId as a, TyMetaFuncId as f)]
pub struct MappedType<AlgebraicSortId: Copy, TyMetaFuncId: Copy, const MAX_TY_FUNC_ARGS: usize> {
    pub f: TyMetaFuncId,
    pub a: BoundedSlice<AlgebraicSortId, MAX_TY_FUNC_ARGS>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Functor)]
#[functor(ProductId as p, SumId as s)]
pub enum AlgebraicSortId<ProductId, SumId> {
    Product(ProductId),
    Sum(SumId),
}
pub type SortShape<const S: usize> = SortId<(), (), (), S>;
impl<P: Copy, S: Copy, F: Copy, const SA: usize> SortId<P, S, F, SA> {
    pub fn project(self) -> SortShape<SA> {
        self.fmap_p(|_| ()).fmap_s(|_| ()).fmap_f(|_| ())
    }
}

pub trait LangSpec<const MAX_TY_FUNC_ARGS: usize> {
    type ProductId: Copy + Clone + Eq + ToLiteral;
    type SumId: Copy + Clone + Eq + ToLiteral;
    type TyMetaFuncId: Copy + Clone + Eq + ToLiteral;

    fn name(&self) -> &Name;
    fn products(&self) -> impl Iterator<Item = Self::ProductId>;
    fn sums(&self) -> impl Iterator<Item = Self::SumId>;
    fn product_name(&self, id: Self::ProductId) -> &Name;
    fn sum_name(&self, id: Self::SumId) -> &Name;
    fn algebraic_sort_name(&self, id: AlgebraicSortId<Self::ProductId, Self::SumId>) -> &Name {
        match id {
            AlgebraicSortId::Product(pid) => self.product_name(pid),
            AlgebraicSortId::Sum(sid) => self.sum_name(sid),
        }
    }
    fn product_sorts(&self, id: Self::ProductId) -> impl Iterator<Item = sort_id_of!(Self)>;
    fn sum_sorts(&self, id: Self::SumId) -> impl Iterator<Item = sort_id_of!(Self)>;
    fn ty_meta_funcs(&self) -> impl Iterator<Item = Self::TyMetaFuncId>;
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

    fn canonical_into<Bot: TerminalLangSpec>(&self) -> Bot {
        Bot::canonical_from(self)
    }
}
/// Marks a langspec as an element of the iso class of terminal objects in the category of [LangSpec]s.
/// Needed because a [From] impl would conflict with the blanket impl for [From] for all types.
pub trait TerminalLangSpec: LangSpec {
    fn canonical_from<L: LangSpec + ?Sized>(l: &L) -> Self
    where
        [(); L::MAX_TY_FUNC_ARGS]:;
}

pub trait ToLiteral {
    fn to_literal(&self) -> syn::Expr;
}
