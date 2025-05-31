#![allow(clippy::needless_question_mark)]

use functor_derive::Functor;
use serde::{Deserialize, Serialize};

use crate::{sublang::Sublang, tymetafunc::TyMetaFuncSpec};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, PartialOrd, Ord)]
/// All names must be unique within a LangSpec unless they are different fields of the same [Name]
pub struct Name {
    /// Must not contain whitespace (whether operator symbols such as '-' are permitted is TBD)
    pub human: String,
    /// CamelCase alias
    pub camel: String,
    /// snake_case alias
    pub snake: String,
}
impl Name {
    pub fn merge(mut self, other: &Name) -> Name {
        self.human += "-";
        self.human += &other.human;
        self.camel += &other.camel;
        self.snake += "_";
        self.snake += &other.snake;
        self
    }
}
#[allow(type_alias_bounds)]
pub type SortIdOf<L: LangSpec> =
    SortId<L::ProductId, L::SumId, <L::Tmfs as TyMetaFuncSpec>::TyMetaFuncId>;
#[allow(type_alias_bounds)]
pub type MappedTypeOf<L: LangSpec> =
    MappedType<L::ProductId, L::SumId, <L::Tmfs as TyMetaFuncSpec>::TyMetaFuncId>;

#[derive(Debug, Serialize, Deserialize, Clone, Functor, PartialEq, Eq, Hash)]
#[functor(ProductId as p, SumId as s, TyMetaFuncId as f)]
pub enum SortId<ProductId, SumId, TyMetaFuncId> {
    Algebraic(AlgebraicSortId<ProductId, SumId>),
    TyMetaFunc(MappedType<ProductId, SumId, TyMetaFuncId>),
}
impl<ProductId: Ord, SumId: Ord, TyMetaFuncId: Ord> PartialOrd
    for SortId<ProductId, SumId, TyMetaFuncId>
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl<ProductId: Ord, SumId: Ord, TyMetaFuncId: Ord> Ord for SortId<ProductId, SumId, TyMetaFuncId> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (SortId::Algebraic(a), SortId::Algebraic(b)) => a.cmp(b),
            (SortId::TyMetaFunc(a), SortId::TyMetaFunc(b)) => a.cmp(b),
            (SortId::Algebraic(_), SortId::TyMetaFunc(_)) => std::cmp::Ordering::Less,
            (SortId::TyMetaFunc(_), SortId::Algebraic(_)) => std::cmp::Ordering::Greater,
        }
    }
}
#[derive(Debug, Serialize, Deserialize, Clone, Functor, PartialEq, Eq, Hash)]
#[functor(ProductId as p, SumId as s, TyMetaFuncId as f)]
pub struct MappedType<ProductId, SumId, TyMetaFuncId> {
    pub f: TyMetaFuncId,
    pub a: Vec<SortId<ProductId, SumId, TyMetaFuncId>>,
}
impl<ProductId: Ord, SumId: Ord, TyMetaFuncId: Ord> PartialOrd
    for MappedType<ProductId, SumId, TyMetaFuncId>
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl<ProductId: Ord, SumId: Ord, TyMetaFuncId: Ord> Ord
    for MappedType<ProductId, SumId, TyMetaFuncId>
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let (MappedType { f: a_f, a: a_a }, MappedType { f: b_f, a: b_a }) = (self, other);
        if a_f == b_f {
            a_a.cmp(b_a)
        } else {
            a_f.cmp(b_f)
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash, Functor)]
#[functor(ProductId as p, SumId as s)]
pub enum AlgebraicSortId<ProductId, SumId> {
    Product(ProductId),
    Sum(SumId),
}
impl<ProductId: Ord, SumId: Ord> PartialOrd for AlgebraicSortId<ProductId, SumId> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl<ProductId: Ord, SumId: Ord> Ord for AlgebraicSortId<ProductId, SumId> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (AlgebraicSortId::Product(a), AlgebraicSortId::Product(b)) => a.cmp(b),
            (AlgebraicSortId::Sum(a), AlgebraicSortId::Sum(b)) => a.cmp(b),
            (AlgebraicSortId::Product(_), AlgebraicSortId::Sum(_)) => std::cmp::Ordering::Less,
            (AlgebraicSortId::Sum(_), AlgebraicSortId::Product(_)) => std::cmp::Ordering::Greater,
        }
    }
}
pub type SortShape = SortId<(), (), ()>;
impl<P, S, F> SortId<P, S, F> {
    pub fn project(self) -> SortShape {
        self.fmap_p(|_| ()).fmap_s(|_| ()).fmap_f(|_| ())
    }
}

pub trait AsLifetime {
    type AsLifetime<'this>: LangSpec + 'this;
}

pub trait LangSpec: Sized + AsLifetime {
    type ProductId: std::fmt::Debug + Clone + Eq + std::hash::Hash + 'static + Ord;
    type SumId: std::fmt::Debug + Clone + Eq + std::hash::Hash + 'static + Ord;
    type Tmfs: TyMetaFuncSpec;

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
    fn product_sorts(&self, id: Self::ProductId) -> impl Iterator<Item = SortIdOf<Self>>;
    fn sum_sorts(&self, id: Self::SumId) -> impl Iterator<Item = SortIdOf<Self>>;

    fn canonical_into<Bot: TerminalLangSpec<Tmfs = Self::Tmfs>>(&self) -> Bot {
        Bot::canonical_from(self)
    }
    fn sublang<'lsub: 'this, 'this, LSub: LangSpec>(
        &'this self,
        lsub: &'lsub LSub,
    ) -> Option<Sublang<'this, LSub::AsLifetime<'this>, SortIdOf<Self>>>;
    fn tmf_roots(&self) -> impl Iterator<Item = MappedTypeOf<Self>> {
        std::iter::empty()
    }
    fn all_sort_ids(&self) -> impl Iterator<Item = SortIdOf<Self>> {
        let products = self
            .products()
            .map(|pid| SortId::Algebraic(AlgebraicSortId::Product(pid)));
        let sums = self
            .sums()
            .map(|sid| SortId::Algebraic(AlgebraicSortId::Sum(sid)));
        let mut tmfs = vec![];
        call_on_all_tmf_monomorphizations(self, &mut |mt| {
            tmfs.push(SortId::TyMetaFunc(mt.clone()));
        });
        products.chain(sums).chain(tmfs)
    }
}
/// Marks a langspec as an element of the iso class of terminal objects in the category of [LangSpec]s.
/// Needed because a [From] impl would conflict with the blanket impl for [From] for all types.
pub trait TerminalLangSpec: LangSpec {
    fn canonical_from<L: LangSpec<Tmfs = <Self as LangSpec>::Tmfs>>(l: &L) -> Self;
}

pub fn call_on_all_tmf_monomorphizations<
    L: LangSpec,
    F: FnMut(&MappedType<L::ProductId, L::SumId, <L::Tmfs as TyMetaFuncSpec>::TyMetaFuncId>),
>(
    l: &L,
    f: &mut F,
) {
    let mut found: std::collections::HashSet<SortIdOf<L>> = Default::default();
    fn process_tmf<
        L: LangSpec,
        F: FnMut(&MappedType<L::ProductId, L::SumId, <L::Tmfs as TyMetaFuncSpec>::TyMetaFuncId>),
    >(
        found: &mut std::collections::HashSet<SortIdOf<L>>,
        sort: SortIdOf<L>,
        f: &mut F,
    ) {
        match sort.clone() {
            SortId::Algebraic(_) => (),
            SortId::TyMetaFunc(mt) => {
                if found.contains(&sort) {
                    return;
                }
                found.insert(sort);
                f(&mt);
                for arg in mt.a {
                    process_tmf::<L, F>(found, arg, f);
                }
            }
        }
    }
    for sort in l
        .products()
        .flat_map(|pid| l.product_sorts(pid))
        .chain(l.sums().flat_map(|sid| l.sum_sorts(sid)))
        .chain(l.tmf_roots().map(SortId::TyMetaFunc))
    {
        process_tmf::<L, F>(&mut found, sort, f);
    }
}
