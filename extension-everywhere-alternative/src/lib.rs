use either_id::Either;
use langspec::langspec::{LangSpec, Name};
use tmfs_join::TmfsJoin;

pub struct EverywhereAlternative<L0: LangSpec, L1: LangSpec> {
    name: Name,
    l0: L0,
    l1: L1,
    l1_root: langspec::langspec::SortIdOf<L1>,
}
impl<L0: LangSpec, L1: LangSpec> LangSpec for EverywhereAlternative<L0, L1> {
    type ProductId = Either<L0::ProductId, L1::ProductId>;

    type SumId = Either<L0::SumId, L1::SumId>;

    type Tmfs = TmfsJoin<L0::Tmfs, L1::Tmfs>;

    fn name(&self) -> &langspec::langspec::Name {
        &self.name
    }

    fn products(&self) -> impl Iterator<Item = Self::ProductId> {
        self.l0
            .products()
            .map(Either::Left)
            .chain(self.l1.products().map(Either::Right))
    }

    fn sums(&self) -> impl Iterator<Item = Self::SumId> {
        self.l0
            .sums()
            .map(Either::Left)
            .chain(self.l1.sums().map(Either::Right))
    }

    fn product_name(&self, id: Self::ProductId) -> &Name {
        match id {
            Either::Left(id) => self.l0.product_name(id),
            Either::Right(id) => self.l1.product_name(id),
        }
    }

    fn sum_name(&self, id: Self::SumId) -> &Name {
        match id {
            Either::Left(id) => self.l0.sum_name(id),
            Either::Right(id) => self.l1.sum_name(id),
        }
    }

    fn product_sorts(
        &self,
        id: Self::ProductId,
    ) -> impl Iterator<Item = langspec::langspec::SortIdOf<Self>> {
        match id {
            Either::Left(id) => Box::new(
                self.l0
                    .product_sorts(id)
                    .map(|sid| {
                        sid.fmap_p(Either::Left)
                            .fmap_s(Either::Left)
                            .fmap_f(Either::Left)
                    })
                    .chain(std::iter::once(
                        self.l1_root
                            .clone()
                            .fmap_p(Either::Right)
                            .fmap_s(Either::Right)
                            .fmap_f(Either::Right),
                    )),
            )
                as Box<dyn Iterator<Item = langspec::langspec::SortIdOf<Self>>>,
            Either::Right(id) => Box::new(self.l1.product_sorts(id).map(|sid| {
                sid.fmap_p(Either::Right)
                    .fmap_s(Either::Right)
                    .fmap_f(Either::Right)
            }))
                as Box<dyn Iterator<Item = langspec::langspec::SortIdOf<Self>>>,
        }
    }

    fn sum_sorts(
        &self,
        id: Self::SumId,
    ) -> impl Iterator<Item = langspec::langspec::SortIdOf<Self>> {
        match id {
            Either::Left(id) => Box::new(self.l0.sum_sorts(id).map(|sid| {
                sid.fmap_p(Either::Left)
                    .fmap_s(Either::Left)
                    .fmap_f(Either::Left)
            }))
                as Box<dyn Iterator<Item = langspec::langspec::SortIdOf<Self>>>,
            Either::Right(id) => Box::new(self.l1.sum_sorts(id).map(|sid| {
                sid.fmap_p(Either::Right)
                    .fmap_s(Either::Right)
                    .fmap_f(Either::Right)
            }))
                as Box<dyn Iterator<Item = langspec::langspec::SortIdOf<Self>>>,
        }
    }

    fn prod_to_unique_nat(&self, id: Self::ProductId) -> usize {
        let max = prods_max(&self.l0);
        match id {
            Either::Left(id) => self.l0.prod_to_unique_nat(id),
            Either::Right(id) => self.l1.prod_to_unique_nat(id) + max + 1,
        }
    }

    fn prod_from_unique_nat(&self, nat: usize) -> Self::ProductId {
        let l0_max = prods_max(&self.l0);
        if nat <= l0_max {
            Either::Left(self.l0.prod_from_unique_nat(nat))
        } else {
            Either::Right(self.l1.prod_from_unique_nat(nat - l0_max - 1))
        }
    }

    fn sum_to_unique_nat(&self, id: Self::SumId) -> usize {
        let max = sums_max(&self.l0);
        match id {
            Either::Left(id) => self.l0.sum_to_unique_nat(id),
            Either::Right(id) => self.l1.sum_to_unique_nat(id) + max + 1,
        }
    }

    fn sum_from_unique_nat(&self, nat: usize) -> Self::SumId {
        let l0_max = sums_max(&self.l0);
        if nat <= l0_max {
            Either::Left(self.l0.sum_from_unique_nat(nat))
        } else {
            Either::Right(self.l1.sum_from_unique_nat(nat - l0_max - 1))
        }
    }
}

fn prods_max<L0: LangSpec>(l0: &L0) -> usize {
    l0.products()
        .map(|pid| l0.prod_to_unique_nat(pid))
        .max()
        .unwrap()
}
fn sums_max<L0: LangSpec>(l0: &L0) -> usize {
    l0.sums()
        .map(|sid| l0.sum_to_unique_nat(sid))
        .max()
        .unwrap()
}
