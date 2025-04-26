pub use either_id;
pub use langspec;

#[macro_export]
macro_rules! lsjoin {
    () => {
        type ProductId = join_boilerplate::either_id::Either<L0::ProductId, L1::ProductId>;

        type SumId = join_boilerplate::either_id::Either<L0::SumId, L1::SumId>;

        fn name(&self) -> &langspec::langspec::Name {
            &self.name
        }

        fn products(&self) -> impl Iterator<Item = Self::ProductId> {
            self.l0
                .products()
                .map(join_boilerplate::either_id::Either::Left)
                .chain(
                    self.l1
                        .products()
                        .map(join_boilerplate::either_id::Either::Right),
                )
        }

        fn sums(&self) -> impl Iterator<Item = Self::SumId> {
            self.l0
                .sums()
                .map(join_boilerplate::either_id::Either::Left)
                .chain(
                    self.l1
                        .sums()
                        .map(join_boilerplate::either_id::Either::Right),
                )
        }

        fn product_name(&self, id: Self::ProductId) -> &Name {
            match id {
                join_boilerplate::either_id::Either::Left(id) => self.l0.product_name(id),
                join_boilerplate::either_id::Either::Right(id) => self.l1.product_name(id),
            }
        }

        fn sum_name(&self, id: Self::SumId) -> &Name {
            match id {
                join_boilerplate::either_id::Either::Left(id) => self.l0.sum_name(id),
                join_boilerplate::either_id::Either::Right(id) => self.l1.sum_name(id),
            }
        }
    };
}
#[macro_export]
macro_rules! join_over_tmfs_as_my_sid {
    ($Me:ty) => {
        fn l0_as_my_sid<'a, 'b, L0: LangSpec, L1: LangSpec>(
            sid: join_boilerplate::langspec::langspec::SortIdOf<L0>,
        ) -> join_boilerplate::langspec::langspec::SortIdOf<$Me> {
            sid.fmap_p(Either::Left)
                .fmap_s(Either::Left)
                .fmap_f(|it| Either::Left(Either::Left(it)))
        }
        fn l1_as_my_sid<'a, 'b, L0: LangSpec, L1: LangSpec>(
            sid: join_boilerplate::langspec::langspec::SortIdOf<L1>,
        ) -> join_boilerplate::langspec::langspec::SortIdOf<$Me> {
            sid.fmap_p(Either::Right)
                .fmap_s(Either::Right)
                .fmap_f(|it| Either::Left(Either::Right(it)))
        }
    };
}
