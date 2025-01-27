use either_id::Either;
use langspec::langspec::{LangSpec, MappedType, Name, SortIdOf};
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
}

impl<L0: LangSpec, L1: LangSpec> LangSpec for EverywhereAlternative<'_, '_, L0, L1> {
    type Tmfs = TmfsJoin<TmfsJoin<L0::Tmfs, L1::Tmfs>, tymetafuncspec_core::Core>;

    join_boilerplate::lsjoin!();

    fn product_sorts(
        &self,
        id: Self::ProductId,
    ) -> impl Iterator<Item = langspec::langspec::SortIdOf<Self>> {
        match id {
            Either::Left(id) => Box::new(self.l0.product_sorts(id).map(|sid| {
                let mapped = l0_as_my_sid::<L0, L1>(sid);
                let mapped = match mapped {
                    langspec::langspec::SortId::Algebraic(_) => mapped,
                    langspec::langspec::SortId::TyMetaFunc(MappedType { f, a }) => {
                        langspec::langspec::SortId::TyMetaFunc(MappedType {
                            f,
                            a: a.into_iter().map(|sid| self.eitherfy(sid)).collect(),
                        })
                    }
                };
                self.eitherfy(mapped)
            }))
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
            Either::Left(id) => Box::new(self.l0.sum_sorts(id).map(l0_as_my_sid::<L0, L1>))
                as Box<dyn Iterator<Item = langspec::langspec::SortIdOf<Self>>>,
            Either::Right(id) => Box::new(self.l1.sum_sorts(id).map(l1_as_my_sid::<L0, L1>))
                as Box<dyn Iterator<Item = langspec::langspec::SortIdOf<Self>>>,
        }
    }
}
