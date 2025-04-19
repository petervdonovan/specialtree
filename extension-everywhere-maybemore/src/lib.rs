use either_id::Either;
use langspec::{
    langspec::{LangSpec, MappedType, Name, SortId},
    sublang::reflexive_sublang,
    tymetafunc::{Transparency, TyMetaFuncSpec},
};
use tmfs_join::TmfsJoin;

pub struct EverywhereMaybeMore<'a, 'b, L0: LangSpec, L1: LangSpec> {
    pub name: Name,
    pub l0: &'a L0,
    pub l1: &'b L1,
    pub l1_root: langspec::langspec::SortIdOf<L1>,
}

join_boilerplate::join_over_tmfs_as_my_sid!(EverywhereMaybeMore<'a, 'b, L0, L1>);

fn maybefy<'a, 'b, L0: LangSpec, L1: LangSpec>(
    sid: langspec::langspec::SortIdOf<L1>,
) -> langspec::langspec::SortIdOf<EverywhereMaybeMore<'a, 'b, L0, L1>> {
    langspec::langspec::SortId::TyMetaFunc(langspec::langspec::MappedType {
        f: Either::Right(tymetafuncspec_core::MAYBE),
        a: vec![l1_as_my_sid::<L0, L1>(sid)],
    })
}
fn maybemorefy<'a, 'b, L0: LangSpec, L1: LangSpec>(
    sid: langspec::langspec::SortIdOf<L0>,
    maybe_sid: langspec::langspec::SortIdOf<L1>,
) -> langspec::langspec::SortIdOf<EverywhereMaybeMore<'a, 'b, L0, L1>> {
    langspec::langspec::SortId::TyMetaFunc(langspec::langspec::MappedType {
        f: Either::Right(tymetafuncspec_core::PAIR),
        a: vec![l0_as_my_sid::<L0, L1>(sid), maybefy::<L0, L1>(maybe_sid)],
    })
}
fn maybemorefy_if_product<'a, 'b, L0: LangSpec, L1: LangSpec>(
    sid: langspec::langspec::SortIdOf<L0>,
    maybe_sid: langspec::langspec::SortIdOf<L1>,
) -> langspec::langspec::SortIdOf<EverywhereMaybeMore<'a, 'b, L0, L1>> {
    match sid {
        langspec::langspec::SortId::Algebraic(langspec::langspec::AlgebraicSortId::Product(_)) => {
            maybemorefy::<L0, L1>(sid, maybe_sid)
        }
        langspec::langspec::SortId::TyMetaFunc(MappedType { f, a }) => {
            let rec = langspec::langspec::SortId::TyMetaFunc(langspec::langspec::MappedType {
                f: Either::Left(Either::Left(f.clone())),
                a: a.into_iter()
                    .map(|a| maybemorefy_if_product::<L0, L1>(a, maybe_sid.clone()))
                    .collect(),
            });
            // if <<L0 as LangSpec>::Tmfs as TyMetaFuncSpec>::ty_meta_func_data(&f).transparency
            //     == langspec::tymetafunc::Transparency::Visible
            // {
            //     maybemorefy::<L0, L1>(sid, maybe_sid)
            // } else {
            //     rec
            // }
            match <<L0 as LangSpec>::Tmfs as TyMetaFuncSpec>::ty_meta_func_data(&f).transparency {
                Transparency::Visible => {
                    langspec::langspec::SortId::TyMetaFunc(langspec::langspec::MappedType {
                        f: Either::Right(tymetafuncspec_core::PAIR),
                        a: vec![rec, maybefy::<L0, L1>(maybe_sid)],
                    })
                }
                Transparency::Transparent => rec,
            }
        }
        _ => l0_as_my_sid::<L0, L1>(sid),
    }
}

impl<L0: LangSpec, L1: LangSpec> LangSpec for EverywhereMaybeMore<'_, '_, L0, L1> {
    type Tmfs = TmfsJoin<TmfsJoin<L0::Tmfs, L1::Tmfs>, tymetafuncspec_core::Core>;

    join_boilerplate::lsjoin!();

    fn product_sorts(
        &self,
        id: Self::ProductId,
    ) -> impl Iterator<Item = langspec::langspec::SortIdOf<Self>> {
        match id {
            Either::Left(id) => Box::new(
                self.l0
                    .product_sorts(id)
                    .map(|sid| maybemorefy_if_product::<L0, L1>(sid, self.l1_root.clone())),
            )
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
            Either::Left(id) => Box::new(
                self.l0
                    .sum_sorts(id)
                    .map(|sid| maybemorefy_if_product::<L0, L1>(sid, self.l1_root.clone())),
            )
                as Box<dyn Iterator<Item = langspec::langspec::SortIdOf<Self>>>,
            Either::Right(id) => Box::new(self.l1.sum_sorts(id).map(l1_as_my_sid::<L0, L1>))
                as Box<dyn Iterator<Item = langspec::langspec::SortIdOf<Self>>>,
        }
    }

    // fn sublangs(&self) -> Vec<langspec::sublang::Sublang<SortIdOf<Self>>> {
    //     self.l0
    //         .sublangs()
    //         .into_iter()
    //         .map(
    //             |sublang: Sublang<SortIdOf<L0>>| langspec::sublang::Sublang::<SortIdOf<Self>> {
    //                 name: sublang.name,
    //                 is_in_image: Box::new(move |sid| match sid {
    //                     SortId::TyMetaFunc(MappedType {
    //                         f: Either::Right(fr),
    //                         a,
    //                     }) => {
    //                         (*fr == tymetafuncspec_core::EITHER)
    //                             && match a[0].clone() {
    //                                 SortId::Algebraic(AlgebraicSortId::Product(Either::Left(
    //                                     a,
    //                                 ))) => (sublang.is_in_image)(&SortId::Algebraic(
    //                                     AlgebraicSortId::Product(a.clone()),
    //                                 )),
    //                                 SortId::Algebraic(AlgebraicSortId::Sum(Either::Left(a))) => {
    //                                     (sublang.is_in_image)(&SortId::Algebraic(
    //                                         AlgebraicSortId::Sum(a.clone()),
    //                                     ))
    //                                 }
    //                                 _ => false,
    //                             }
    //                     }
    //                     _ => false,
    //                 }),
    //                 map: Box::new(move |name| {
    //                     let id = (sublang.map)(name);
    //                     let mapped = l0_as_my_sid::<L0, L1>(id);
    //                     let ret: SortIdOf<Self> = self.map_id(mapped);
    //                     ret
    //                 }),
    //             },
    //         )
    //         .collect()
    // }

    fn sublangs(&self) -> Vec<langspec::sublang::Sublang<langspec::langspec::SortIdOf<Self>>> {
        self.l0
            .sublangs()
            .into_iter()
            .map(
                |sublang: langspec::sublang::Sublang<langspec::langspec::SortIdOf<L0>>| {
                    langspec::sublang::Sublang::<langspec::langspec::SortIdOf<Self>> {
                        name: sublang.name,
                        // is_in_image: Box::new(move |sid| match sid {
                        //     SortId::TyMetaFunc(MappedType {
                        //         f: Either::Right(fr),
                        //         a,
                        //     }) => {
                        //         (*fr == tymetafuncspec_core::PAIR)
                        //             && match a[0].clone() {
                        //                 langspec::langspec::SortId::Algebraic(
                        //                     langspec::langspec::AlgebraicSortId::Product(
                        //                         Either::Left(a),
                        //                     ),
                        //                 ) => (sublang.is_in_image)(
                        //                     &langspec::langspec::SortId::Algebraic(
                        //                         langspec::langspec::AlgebraicSortId::Product(
                        //                             a.clone(),
                        //                         ),
                        //                     ),
                        //                 ),
                        //                 _ => false,
                        //             }
                        //     }
                        //     SortId::Algebraic(langspec::langspec::AlgebraicSortId::Sum(
                        //         Either::Left(a),
                        //     )) => (sublang.is_in_image)(&SortId::Algebraic(
                        //         langspec::langspec::AlgebraicSortId::Sum(a.clone()),
                        //     )),
                        //     _ => false,
                        // }),
                        image: sublang
                            .image
                            .into_iter()
                            .map(|sid| maybemorefy_if_product::<L0, L1>(sid, self.l1_root.clone()))
                            .collect(),
                        map: Box::new(move |name| {
                            let id = (sublang.map)(name);
                            let mapped = maybemorefy_if_product::<L0, L1>(id, self.l1_root.clone());
                            mapped
                        }),
                    }
                },
            )
            .chain(std::iter::once(reflexive_sublang(self)))
            .collect()
    }
}
