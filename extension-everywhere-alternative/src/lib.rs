use either_id::Either;
use langspec::{
    langspec::{AlgebraicSortId, LangSpec, MappedType, Name, SortId, SortIdOf},
    sublang::{reflexive_sublang, Sublang, TmfEndoMappingNonreflexive},
};
use tmfs_join::TmfsJoin;

pub struct EverywhereAlternative<'a, 'b, L0: LangSpec, L1: LangSpec> {
    pub name: Name,
    pub l0: &'a L0,
    pub l1: &'b L1,
    pub l1_root: SortIdOf<L1>,
}
join_boilerplate::join_over_tmfs_as_my_sid!(EverywhereAlternative<'a, 'b, L0, L1>);
impl<L0: LangSpec, L1: LangSpec> EverywhereAlternative<'_, '_, L0, L1> {
    fn eitherfy(
        &self,
        sid: SortIdOf<EverywhereAlternative<L0, L1>>,
    ) -> SortIdOf<EverywhereAlternative<L0, L1>> {
        langspec::langspec::SortId::TyMetaFunc(MappedType {
            f: Either::Right(tymetafuncspec_core::EITHER),
            a: vec![sid, l1_as_my_sid::<L0, L1>(self.l1_root.clone())],
        })
    }
    fn map_l0(&self, sid: SortIdOf<L0>) -> SortIdOf<EverywhereAlternative<L0, L1>> {
        self.eitherfy(match sid {
            langspec::langspec::SortId::Algebraic(_) => l0_as_my_sid::<L0, L1>(sid.clone()),
            langspec::langspec::SortId::TyMetaFunc(MappedType { f, a }) => {
                langspec::langspec::SortId::TyMetaFunc(MappedType {
                    f: Either::Left(Either::Left(f)),
                    a: a.into_iter().map(|sid| self.map_l0(sid)).collect(),
                })
            }
        })
    }
    // fn map_id(&self, id: langspec::langspec::SortIdOf<Self>) -> langspec::langspec::SortIdOf<Self> {
    //     match id {
    //         langspec::langspec::SortId::Algebraic(
    //             langspec::langspec::AlgebraicSortId::Product(Either::Left(id)),
    //         ) => self.eitherfy(self.map_l0(langspec::langspec::SortId::Algebraic(
    //             langspec::langspec::AlgebraicSortId::Product(id),
    //         ))),
    //         langspec::langspec::SortId::Algebraic(langspec::langspec::AlgebraicSortId::Sum(
    //             Either::Left(id),
    //         )) => self.eitherfy(self.map_l0(langspec::langspec::SortId::Algebraic(
    //             langspec::langspec::AlgebraicSortId::Sum(id),
    //         ))),
    //         langspec::langspec::SortId::TyMetaFunc(MappedType { f, a }) => {
    //             langspec::langspec::SortId::TyMetaFunc(MappedType {
    //                 f,
    //                 a: a.into_iter().map(|sid| self.map_id(sid)).collect(),
    //             })
    //         }
    //         _ => id,
    //     }
    // }
}

impl<L0: LangSpec, L1: LangSpec> LangSpec for EverywhereAlternative<'_, '_, L0, L1> {
    type Tmfs = TmfsJoin<TmfsJoin<L0::Tmfs, L1::Tmfs>, tymetafuncspec_core::Core>;

    join_boilerplate::lsjoin!();

    fn product_sorts(
        &self,
        id: Self::ProductId,
    ) -> impl Iterator<Item = langspec::langspec::SortIdOf<Self>> {
        match id {
            Either::Left(id) => Box::new(self.l0.product_sorts(id).map(|sid| self.map_l0(sid)))
                as Box<dyn Iterator<Item = langspec::langspec::SortIdOf<Self>>>,
            Either::Right(id) => Box::new(self.l1.product_sorts(id).map(l1_as_my_sid::<L0, L1>))
                as Box<dyn Iterator<Item = langspec::langspec::SortIdOf<Self>>>,
        }
    }

    fn sum_sorts(
        &self,
        id: Self::SumId,
    ) -> impl Iterator<Item = langspec::langspec::SortIdOf<Self>> {
        match id {
            Either::Left(id) => Box::new(self.l0.sum_sorts(id).map(|sid| self.map_l0(sid)))
                as Box<dyn Iterator<Item = langspec::langspec::SortIdOf<Self>>>,
            Either::Right(id) => Box::new(self.l1.sum_sorts(id).map(l1_as_my_sid::<L0, L1>))
                as Box<dyn Iterator<Item = langspec::langspec::SortIdOf<Self>>>,
        }
    }

    fn sublangs(&self) -> Vec<langspec::sublang::Sublang<SortIdOf<Self>>> {
        self.l0
            .sublangs()
            .into_iter()
            .map(
                |sublang: Sublang<SortIdOf<L0>>| langspec::sublang::Sublang::<SortIdOf<Self>> {
                    name: sublang.name,
                    // is_in_image: Box::new(move |sid| match sid {
                    //     SortId::TyMetaFunc(MappedType {
                    //         f: Either::Right(fr),
                    //         a,
                    //     }) => {
                    //         (*fr == tymetafuncspec_core::EITHER)
                    //             && match a[0].clone() {
                    //                 SortId::Algebraic(AlgebraicSortId::Product(Either::Left(
                    //                     a,
                    //                 ))) => (sublang.is_in_image)(&SortId::Algebraic(
                    //                     AlgebraicSortId::Product(a.clone()),
                    //                 )),
                    //                 SortId::Algebraic(AlgebraicSortId::Sum(Either::Left(a))) => {
                    //                     (sublang.is_in_image)(&SortId::Algebraic(
                    //                         AlgebraicSortId::Sum(a.clone()),
                    //                     ))
                    //                 }
                    //                 _ => false,
                    //             }
                    //     }
                    //     _ => false,
                    // }),
                    image: sublang
                        .image
                        .clone()
                        .into_iter()
                        .map(|sid| self.map_l0(sid))
                        .collect(),
                    ty_names: sublang.ty_names,
                    map: Box::new(move |name| {
                        let id = (sublang.map)(name);
                        // let mapped = l0_as_my_sid::<L0, L1>(id);
                        // let ret: SortIdOf<Self> = self.eitherfy(self.map_id(mapped));
                        // ret
                        self.map_l0(id)
                    }),
                    tems: sublang
                        .tems
                        .into_iter()
                        .map(|tem| match (tem.from, tem.to) {
                            (SortId::TyMetaFunc(mtfrom), mtto) => {
                                TmfEndoMappingNonreflexive::<SortIdOf<Self>> {
                                    from: SortId::TyMetaFunc(MappedType {
                                        f: Either::Left(Either::Left(mtfrom.f.clone())),
                                        a: mtfrom
                                            .a
                                            .iter()
                                            .map(|sid| self.map_l0(sid.clone()))
                                            .collect(),
                                    }),
                                    to: self.map_l0(mtto.clone()),
                                }
                            }
                            _ => panic!(),
                        })
                        .collect(),
                },
            )
            .chain(std::iter::once(reflexive_sublang(self)))
            .collect()
    }
}
