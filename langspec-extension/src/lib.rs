use std::any::TypeId;

use either_id::Either;
use langspec::{
    langspec::{AsLifetime, LangSpec, MappedType, SortId, SortIdOf},
    sublang::{AspectImplementors, Sublang, reflexive_sublang},
    tymetafunc::TyMetaFuncSpec,
};
use tmfs_join::TmfsJoin;
use tree_identifier::Identifier;

pub struct LsExtension<'a, 'b, L0, L1, L0M> {
    pub name: Identifier,
    pub l0: &'a L0,
    pub l1: &'b L1,
    pub l0m: L0M,
}
pub trait L0Map<'a, 'b, L0: LangSpec, L1: LangSpec>: Sized {
    type SelfAsLifetime<'c>: L0Map<'c, 'c, L0::AsLifetime<'c>, L1::AsLifetime<'c>> + 'c;
    fn l0_map(
        this: &LsExtension<'a, 'b, L0, L1, Self>,
        sid: SortIdOf<L0>,
    ) -> SortIdOfExtension<L0, L1>;
}
pub fn l0_as_my_sid<L0: LangSpec, L1: LangSpec>(
    sid: join_boilerplate::langspec::langspec::SortIdOf<L0>,
) -> SortIdOfExtension<L0, L1> {
    sid.fmap_p(Either::Left)
        .fmap_s(Either::Left)
        .fmap_f(|it| Either::Left(Either::Left(it)))
}
pub fn l1_as_my_sid<L0: LangSpec, L1: LangSpec>(
    sid: join_boilerplate::langspec::langspec::SortIdOf<L1>,
) -> SortIdOfExtension<L0, L1> {
    sid.fmap_p(Either::Right)
        .fmap_s(Either::Right)
        .fmap_f(|it| Either::Left(Either::Right(it)))
}
#[allow(type_alias_bounds)]
pub type SortIdOfExtension<L0: LangSpec, L1: LangSpec> = SortId<
    Either<L0::ProductId, L1::ProductId>,
    Either<L0::SumId, L1::SumId>,
    <TmfsJoin<TmfsJoin<L0::Tmfs, L1::Tmfs>, tymetafuncspec_core::Core> as TyMetaFuncSpec>::TyMetaFuncId,
>;

impl<'a, 'b, L0: LangSpec, L1: LangSpec, L0M> AsLifetime for LsExtension<'a, 'b, L0, L1, L0M>
where
    L0M: L0Map<'a, 'b, L0, L1>,
{
    type AsLifetime<'c> =
        LsExtension<'c, 'c, L0::AsLifetime<'c>, L1::AsLifetime<'c>, L0M::SelfAsLifetime<'c>>;
}

impl<'a, 'b, L0: LangSpec, L1: LangSpec, L0M> LangSpec for LsExtension<'a, 'b, L0, L1, L0M>
where
    L0M: L0Map<'a, 'b, L0, L1>,
{
    type Tmfs = TmfsJoin<TmfsJoin<L0::Tmfs, L1::Tmfs>, tymetafuncspec_core::Core>;

    join_boilerplate::lsjoin!();

    fn product_sorts(
        &self,
        id: Self::ProductId,
    ) -> impl Iterator<Item = langspec::langspec::SortIdOf<Self>> {
        match id {
            Either::Left(id) => {
                Box::new(self.l0.product_sorts(id).map(|sid| L0M::l0_map(self, sid)))
                    as Box<dyn Iterator<Item = langspec::langspec::SortIdOf<Self>>>
            }
            Either::Right(id) => Box::new(self.l1.product_sorts(id).map(l1_as_my_sid::<L0, L1>))
                as Box<dyn Iterator<Item = langspec::langspec::SortIdOf<Self>>>,
        }
    }

    fn sum_sorts(
        &self,
        id: Self::SumId,
    ) -> impl Iterator<Item = langspec::langspec::SortIdOf<Self>> {
        match id {
            Either::Left(id) => Box::new(self.l0.sum_sorts(id).map(|sid| L0M::l0_map(self, sid)))
                as Box<dyn Iterator<Item = langspec::langspec::SortIdOf<Self>>>,
            Either::Right(id) => Box::new(self.l1.sum_sorts(id).map(l1_as_my_sid::<L0, L1>))
                as Box<dyn Iterator<Item = langspec::langspec::SortIdOf<Self>>>,
        }
    }

    fn tmf_roots(&self) -> impl Iterator<Item = langspec::langspec::MappedTypeOf<Self>> {
        self.l0
            .tmf_roots()
            .map(|mt| L0M::l0_map(self, SortId::TyMetaFunc(mt)))
            .map(|sid| match sid {
                SortId::Algebraic(_) => panic!(),
                SortId::TyMetaFunc(mapped_type) => mapped_type,
            })
    }

    fn sublang<'lsub: 'this, 'this, LSub: LangSpec>(
        &'this self,
        lsub: &'lsub LSub,
    ) -> Option<Sublang<'this, LSub::AsLifetime<'this>, SortIdOf<Self>>> {
        if TypeId::of::<LSub::AsLifetime<'static>>() == TypeId::of::<Self::AsLifetime<'static>>() {
            unsafe {
                Some(std::mem::transmute::<
                    Sublang<Self, SortIdOf<Self>>,
                    Sublang<LSub::AsLifetime<'this>, SortIdOf<Self>>,
                >(reflexive_sublang(self)))
            }
        } else {
            self.l0.sublang::<LSub>(lsub).map(
                |sublang: Sublang<'this, LSub::AsLifetime<'this>, SortIdOf<L0>>| Sublang {
                    lsub: sublang.lsub,
                    map: Box::new(move |name| {
                        let id = (sublang.map)(name);
                        L0M::l0_map(self, id)
                    }),
                    aspect_implementors: sublang
                        .aspect_implementors
                        .into_iter()
                        .map(|tem| AspectImplementors::<SortIdOf<Self>> {
                            from_extern_behavioral: match tem.from_extern_behavioral {
                                SortId::Algebraic(_) => panic!(),
                                SortId::TyMetaFunc(mapped_type) => SortId::TyMetaFunc(MappedType {
                                    f: Either::Left(Either::Left(mapped_type.f.clone())),
                                    a: mapped_type
                                        .a
                                        .into_iter()
                                        .map(|sid| L0M::l0_map(self, sid.clone()))
                                        .collect(),
                                }),
                            },
                            // fromrec: l0_as_my_sid::<L0, L1>(tem.fromrec),
                            to_structural: L0M::l0_map(self, tem.to_structural.clone()),
                        })
                        .collect(),
                },
            )
        }
    }

    // fn sublangs(&self) -> Vec<langspec::sublang::Sublang<SortIdOf<Self>>> {
    //     self.l0
    //         .sublangs()
    //         .into_iter()
    //         .map(
    //             |sublang: Sublang<SortIdOf<L0>>| langspec::sublang::Sublang::<SortIdOf<Self>> {
    //                 name: sublang.name,
    //                 image: sublang
    //                     .image
    //                     .clone()
    //                     .into_iter()
    //                     .map(|sid| L0M::l0_map(self, sid))
    //                     .collect(),
    //                 ty_names: sublang.ty_names,
    //                 map: Box::new(move |name| {
    //                     let id = (sublang.map)(name);
    //                     L0M::l0_map(self, id)
    //                 }),
    //                 aspect_implementors: sublang
    //                     .aspect_implementors
    //                     .into_iter()
    //                     .map(|tem| TmfEndoMapping::<SortIdOf<Self>> {
    //                         fromshallow: match tem.fromshallow {
    //                             SortId::Algebraic(_) => panic!(),
    //                             SortId::TyMetaFunc(mapped_type) => SortId::TyMetaFunc(MappedType {
    //                                 f: Either::Left(Either::Left(mapped_type.f.clone())),
    //                                 a: mapped_type
    //                                     .a
    //                                     .into_iter()
    //                                     .map(|sid| L0M::l0_map(self, sid.clone()))
    //                                     .collect(),
    //                             }),
    //                         },
    //                         fromrec: l0_as_my_sid::<L0, L1>(tem.fromrec),
    //                         to: L0M::l0_map(self, tem.to.clone()),
    //                     })
    //                     .collect(),
    //             },
    //         )
    //         .chain(std::iter::once(reflexive_sublang(self)))
    //         .collect()
    //     // todo!()
    // }
}
