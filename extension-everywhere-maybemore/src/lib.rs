use either_id::Either;
use langspec::langspec::{LangSpec, Name};
use tmfs_join::TmfsJoin;

pub struct EverywhereMaybemore<L0: LangSpec, L1: LangSpec> {
    name: Name,
    l0: L0,
    l1: L1,
    l1_root: langspec::langspec::SortIdOf<L1>,
}

join_boilerplate::join_over_tmfs_as_my_sid!(EverywhereMaybemore<L0, L1>);

fn maybefy<L0: LangSpec, L1: LangSpec>(
    sid: langspec::langspec::SortIdOf<L1>,
) -> langspec::langspec::SortIdOf<EverywhereMaybemore<L0, L1>> {
    langspec::langspec::SortId::TyMetaFunc(langspec::langspec::MappedType {
        f: Either::Right(tymetafuncspec_core::MAYBE),
        a: vec![l1_as_my_sid::<L0, L1>(sid)],
    })
}

impl<L0: LangSpec, L1: LangSpec> LangSpec for EverywhereMaybemore<L0, L1> {
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
                    .map(l0_as_my_sid::<L0, L1>)
                    .chain(std::iter::once(maybefy::<L0, L1>(self.l1_root.clone()))),
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
            Either::Left(id) => Box::new(self.l0.sum_sorts(id).map(l0_as_my_sid::<L0, L1>))
                as Box<dyn Iterator<Item = langspec::langspec::SortIdOf<Self>>>,
            Either::Right(id) => Box::new(self.l1.sum_sorts(id).map(l1_as_my_sid::<L0, L1>))
                as Box<dyn Iterator<Item = langspec::langspec::SortIdOf<Self>>>,
        }
    }
}
