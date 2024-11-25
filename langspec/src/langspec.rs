use functor_derive::Functor;
use serde::{Deserialize, Serialize};

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

#[derive(Debug, Serialize, Deserialize, Clone, Copy, Functor)]
pub enum SortId<AlgebraicSortId> {
    NatLiteral,
    Algebraic(AlgebraicSortId),
    Set(AlgebraicSortId),
    Sequence(AlgebraicSortId),
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Functor)]
#[functor(ProductId as p, SumId as s)]
pub enum AlgebraicSortId<ProductId, SumId> {
    Product(ProductId),
    Sum(SumId),
}

pub type UnpackedSortId<L> =
    SortId<AlgebraicSortId<<L as LangSpec>::ProductId, <L as LangSpec>::SumId>>;
pub type UnpackedAlgebraicSortId<L> =
    AlgebraicSortId<<L as LangSpec>::ProductId, <L as LangSpec>::SumId>;
pub type SortShape = SortId<AlgebraicSortId<(), ()>>;
impl SortShape {
    pub fn project<L: LangSpec>(l: &L, sid: SortId<L::AlgebraicSortId>) -> Self {
        sid.fmap(|it| l.asi_convert(it).fmap_p(|_| ()).fmap_s(|_| ()))
    }
}

pub trait LangSpec {
    type ProductId: Clone + Eq + ToLiteral;
    type SumId: Clone + Eq + ToLiteral;
    type AlgebraicSortId: Clone + Eq;

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
    fn product_sorts(
        &self,
        id: Self::ProductId,
    ) -> impl Iterator<Item = SortId<Self::AlgebraicSortId>>;
    fn sum_sorts(&self, id: Self::SumId) -> impl Iterator<Item = SortId<Self::AlgebraicSortId>>;
    fn prod_to_unique_nat(&self, id: Self::ProductId) -> usize;
    fn prod_from_unique_nat(&self, nat: usize) -> Self::ProductId;
    fn sum_to_unique_nat(&self, id: Self::SumId) -> usize;
    fn sum_from_unique_nat(&self, nat: usize) -> Self::SumId;
    fn asi_convert(&self, id: Self::AlgebraicSortId) -> UnpackedAlgebraicSortId<Self>;
    fn asi_unconvert(&self, id: UnpackedAlgebraicSortId<Self>) -> Self::AlgebraicSortId;
    fn sid_convert(&self, sid: SortId<Self::AlgebraicSortId>) -> UnpackedSortId<Self> {
        match sid {
            SortId::NatLiteral => SortId::NatLiteral,
            SortId::Algebraic(asi) => SortId::Algebraic(self.asi_convert(asi)),
            SortId::Set(asi) => SortId::Set(self.asi_convert(asi)),
            SortId::Sequence(asi) => SortId::Sequence(self.asi_convert(asi)),
        }
    }
    fn product_datas(
        &self,
    ) -> impl Iterator<Item = (&Name, impl Iterator<Item = SortId<Self::AlgebraicSortId>>)> {
        self.products().map(move |pid| {
            let name = self.product_name(pid.clone());
            let sorts = self.product_sorts(pid);
            (name, sorts)
        })
    }
    fn sum_datas(
        &self,
    ) -> impl Iterator<Item = (&Name, impl Iterator<Item = SortId<Self::AlgebraicSortId>>)> {
        self.sums().map(move |sid| {
            let name = self.sum_name(sid.clone());
            let sorts = self.sum_sorts(sid);
            (name, sorts)
        })
    }
    fn ty_names(&self) -> impl Iterator<Item = &Name> {
        self.product_datas()
            .map(|(n, _)| n)
            .chain(self.sum_datas().map(|(n, _)| n))
    }

    fn canonical_into<Bot: TerminalLangSpec>(&self) -> Bot {
        Bot::canonical_from(self)
    }
}
/// Marks a langspec as an element of the iso class of terminal objects in the category of [LangSpec]s.
/// Needed because a [From] impl would conflict with the blanket impl for [From] for all types.
pub trait TerminalLangSpec: LangSpec {
    fn canonical_from<L: LangSpec + ?Sized>(l: &L) -> Self;
}

pub trait ToLiteral {
    fn to_literal(&self) -> syn::Expr;
}
