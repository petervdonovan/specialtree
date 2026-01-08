use std::any::TypeId;

use either_id::Either;
use langspec::{
    langspec::{AlgebraicSortId, AsLifetime, LangSpec, MappedType, SortId, SortIdOf},
    sublang::AspectImplementors,
    sublang::{Sublang, reflexive_sublang},
    tymetafunc::TyMetaFuncSpec,
};
use tmfs_join::TmfsJoin;
use tree_identifier::Identifier;

pub struct LsSortMapped<'a, L, Csm> {
    pub l: &'a L,
    pub name: Identifier,
    pub csm: Csm,
}
pub trait CsmAsLifetime<L: LangSpec> {
    type AsLifetime<'this>: ContextualSortMap<L::AsLifetime<'this>> + 'this;
}
pub trait ContextualSortMap<L: LangSpec>: CsmAsLifetime<L> {
    type Tmfs: TyMetaFuncSpec;
    fn map(
        &self,
        l: &L,
        ctx: &AlgebraicSortId<L::ProductId, L::SumId>,
        sid: &SortIdOf<L>,
    ) -> SortIdOfExtension<L, Self::Tmfs>;
    fn embed_sort_id(sid: SortIdOf<L>) -> SortIdOfExtension<L, Self::Tmfs> {
        sid.fmap_f(Either::Left)
    }
}

#[allow(type_alias_bounds)]
pub type SortIdOfExtension<L: LangSpec, Tmfs> =
    SortId<L::ProductId, L::SumId, <TmfsJoin<L::Tmfs, Tmfs> as TyMetaFuncSpec>::TyMetaFuncId>;
impl<'a, L, Csm, Tmfs> AsLifetime for LsSortMapped<'a, L, Csm>
where
    L: LangSpec,
    Tmfs: TyMetaFuncSpec,
    Csm: ContextualSortMap<L, Tmfs = Tmfs>,
{
    type AsLifetime<'this> = LsSortMapped<'this, L::AsLifetime<'this>, Csm::AsLifetime<'this>>;
}
impl<'a, L, Tmfs, Csm> LangSpec for LsSortMapped<'a, L, Csm>
where
    L: LangSpec,
    Tmfs: TyMetaFuncSpec,
    Csm: ContextualSortMap<L, Tmfs = Tmfs>,
{
    type Tmfs = TmfsJoin<L::Tmfs, Tmfs>;

    type ProductId = L::ProductId;

    type SumId = L::SumId;

    fn name(&self) -> &Identifier {
        &self.name
    }

    fn products(&self) -> impl Iterator<Item = Self::ProductId> {
        self.l.products()
    }

    fn sums(&self) -> impl Iterator<Item = Self::SumId> {
        self.l.sums()
    }

    fn product_name(&self, id: Self::ProductId) -> &Identifier {
        self.l.product_name(id)
    }

    fn sum_name(&self, id: Self::SumId) -> &Identifier {
        self.l.sum_name(id)
    }

    fn product_sorts(&self, id: Self::ProductId) -> impl Iterator<Item = SortIdOf<Self>> {
        self.l.product_sorts(id.clone()).map(move |sid| {
            self.csm
                .map(self.l, &AlgebraicSortId::Product(id.clone()), &sid)
        })
    }

    fn sum_sorts(&self, id: Self::SumId) -> impl Iterator<Item = SortIdOf<Self>> {
        self.l.sum_sorts(id.clone()).map(move |sid| {
            self.csm
                .map(self.l, &AlgebraicSortId::Sum(id.clone()), &sid)
        })
    }

    fn tmf_roots(&self) -> impl Iterator<Item = langspec::langspec::MappedTypeOf<Self>> {
        self.l.tmf_roots().map(|mt| mt.fmap_f(Either::Left))
    }

    fn sublang<'lsub: 'this, 'this, LSub: LangSpec>(
        &'this self,
        lsub: &'lsub LSub,
    ) -> Option<langspec::sublang::Sublang<'this, LSub::AsLifetime<'this>, SortIdOf<Self>>> {
        if TypeId::of::<LSub::AsLifetime<'static>>() == TypeId::of::<Self::AsLifetime<'static>>()
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
        .or_else(|| {
            self.l.sublang::<LSub>(lsub).map(
                |Sublang {
                     lsub,
                     aspect_implementors,
                 }| Sublang {
                    lsub,
                    aspect_implementors: aspect_implementors
                        .into_iter()
                        .map(|it| AspectImplementors {
                            aspect_zst: it.aspect_zst,
                            map: Box::new(move |sid| Csm::embed_sort_id((it.map)(&sid))),
                        })
                        .collect(),
                },
            )
        })
        // self.l
        //     .sublang::<LSub>(lsub)
        //     .map(|Sublang { lsub, map, aspect_implementors }| Sublang {
        //         lsub,
        //         map: Box::new(move |sid| Csm::embed_sort_id(map(sid))),
        //         aspect_implementors: aspect_implementors
        //             .into_iter()
        //             .map(|it| it.fmap(Csm::embed_sort_id))
        //             .collect(),
        //     })
        //     .or_else(|| {
        //         if TypeId::of::<LSub::AsLifetime<'static>>()
        //             == TypeId::of::<Self::AsLifetime<'static>>()
        //             && lsub.name() == self.name()
        //         {
        //             unsafe {
        //                 Some(std::mem::transmute::<
        //                     Sublang<Self, SortIdOf<Self>>,
        //                     Sublang<LSub::AsLifetime<'this>, SortIdOf<Self>>,
        //                 >(reflexive_sublang(self)))
        //             }
        //         } else {
        //             None
        //         }
        //     })
    }

    // fn sublangs(&self) -> Vec<langspec::sublang::Sublang<SortIdOf<Self>>> {
    //     self.l
    //         .sublangs()
    //         .into_iter()
    //         .map(|sublang| langspec::sublang::Sublang::<SortIdOf<Self>> {
    //             name: sublang.name,
    //             image: sublang.image.into_iter().map(Csm::embed_sort_id).collect(),
    //             ty_names: sublang.ty_names,
    //             map: Box::new(move |name| Csm::embed_sort_id((sublang.map)(name))),
    //             aspect_implementors: sublang
    //                 .aspect_implementors
    //                 .into_iter()
    //                 .map(|it| it.fmap(Csm::embed_sort_id))
    //                 .collect(),
    //         })
    //         .chain(std::iter::once(reflexive_sublang(self)))
    //         .collect()
    // }
}
