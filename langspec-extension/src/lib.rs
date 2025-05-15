use either_id::Either;
use langspec::{
    langspec::{LangSpec, MappedType, Name, SortId, SortIdOf},
    sublang::{Sublang, TmfEndoMapping, reflexive_sublang},
    tymetafunc::TyMetaFuncSpec,
};
use tmfs_join::TmfsJoin;

pub struct LsExtension<'a, 'b, L0, L1, L0M> {
    pub name: Name,
    pub l0: &'a L0,
    pub l1: &'b L1,
    pub l0m: L0M,
}
pub trait L0Map<'a, 'b, L0: LangSpec, L1: LangSpec>: Sized {
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

    fn sublangs(&self) -> Vec<langspec::sublang::Sublang<SortIdOf<Self>>> {
        self.l0
            .sublangs()
            .into_iter()
            .map(
                |sublang: Sublang<SortIdOf<L0>>| langspec::sublang::Sublang::<SortIdOf<Self>> {
                    name: sublang.name,
                    image: sublang
                        .image
                        .clone()
                        .into_iter()
                        .map(|sid| L0M::l0_map(self, sid))
                        .collect(),
                    ty_names: sublang.ty_names,
                    map: Box::new(move |name| {
                        let id = (sublang.map)(name);
                        L0M::l0_map(self, id)
                    }),
                    tems: sublang
                        .tems
                        .into_iter()
                        .map(|tem| match (tem.from, tem.to) {
                            (SortId::TyMetaFunc(mtfrom), mtto) => {
                                TmfEndoMapping::<SortIdOf<Self>> {
                                    from: SortId::TyMetaFunc(MappedType {
                                        f: Either::Left(Either::Left(mtfrom.f.clone())),
                                        a: mtfrom
                                            .a
                                            .iter()
                                            .map(|sid| L0M::l0_map(self, sid.clone()))
                                            .collect(),
                                    }),
                                    to: L0M::l0_map(self, mtto.clone()),
                                }
                            }
                            _ => panic!(),
                        })
                        .collect(),
                },
            )
            .chain(std::iter::once(reflexive_sublang(self)))
            .collect()
        // todo!()
    }
}
