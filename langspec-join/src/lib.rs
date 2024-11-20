use functor_derive::Functor;
use langspec::langspec::{AlgebraicSortId, LangSpec, Name};

#[derive(Clone, Eq, PartialEq, Debug, Functor)]
#[functor(L as l, R as r)]
pub enum Either<L, R> {
    Left(L),
    Right(R),
}

pub struct Join<L0, L1> {
    name: Name,
    l0: L0,
    l1: L1,
}

impl<L0: LangSpec, L1: LangSpec> LangSpec for Join<L0, L1> {
    type ProductId = Either<L0::ProductId, L1::ProductId>;

    type SumId = Either<L0::SumId, L1::SumId>;

    type AlgebraicSortId = AlgebraicSortId<Self::ProductId, Self::SumId>;

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

    fn product_name(&self, id: Self::ProductId) -> &langspec::langspec::Name {
        match id {
            Either::Left(id) => self.l0.product_name(id),
            Either::Right(id) => self.l1.product_name(id),
        }
    }

    fn sum_name(&self, id: Self::SumId) -> &langspec::langspec::Name {
        match id {
            Either::Left(id) => self.l0.sum_name(id),
            Either::Right(id) => self.l1.sum_name(id),
        }
    }

    fn product_sorts(
        &self,
        id: Self::ProductId,
    ) -> impl Iterator<Item = langspec::langspec::SortId<Self::AlgebraicSortId>> {
        match id {
            Either::Left(id) => {
                let ret: Box<dyn Iterator<Item = _>> =
                    Box::new(self.l0.product_sorts(id).map(|id| {
                        id.fmap(|it| {
                            LangSpec::asi_convert(&self.l0, it)
                                .fmap_p(Either::Left)
                                .fmap_s(Either::Left)
                        })
                    }));
                ret
            }
            Either::Right(id) => {
                let ret: Box<dyn Iterator<Item = _>> =
                    Box::new(self.l1.product_sorts(id).map(|id| {
                        id.fmap(|it| {
                            LangSpec::asi_convert(&self.l1, it)
                                .fmap_p(Either::Right)
                                .fmap_s(Either::Right)
                        })
                    }));
                ret
            }
        }
    }

    fn sum_sorts(
        &self,
        id: Self::SumId,
    ) -> impl Iterator<Item = langspec::langspec::SortId<Self::AlgebraicSortId>> {
        match id {
            Either::Left(id) => {
                let ret: Box<dyn Iterator<Item = _>> = Box::new(self.l0.sum_sorts(id).map(|id| {
                    id.fmap(|it| {
                        LangSpec::asi_convert(&self.l0, it)
                            .fmap_p(Either::Left)
                            .fmap_s(Either::Left)
                    })
                }));
                ret
            }
            Either::Right(id) => {
                let ret: Box<dyn Iterator<Item = _>> = Box::new(self.l1.sum_sorts(id).map(|id| {
                    id.fmap(|it| {
                        LangSpec::asi_convert(&self.l1, it)
                            .fmap_p(Either::Right)
                            .fmap_s(Either::Right)
                    })
                }));
                ret
            }
        }
    }

    fn prod_to_unique_nat(&self, id: Self::ProductId) -> usize {
        match id {
            Either::Left(id) => self.l0.prod_to_unique_nat(id),
            Either::Right(id) => self.l1.prod_to_unique_nat(id) + self.l0.products().count(),
        }
    }

    fn prod_from_unique_nat(&self, nat: usize) -> Self::ProductId {
        if nat < self.l0.products().count() {
            Either::Left(self.l0.prod_from_unique_nat(nat))
        } else {
            Either::Right(
                self.l1
                    .prod_from_unique_nat(nat - self.l0.products().count()),
            )
        }
    }

    fn sum_to_unique_nat(&self, id: Self::SumId) -> usize {
        match id {
            Either::Left(id) => self.l0.sum_to_unique_nat(id),
            Either::Right(id) => self.l1.sum_to_unique_nat(id) + self.l0.sums().count(),
        }
    }

    fn sum_from_unique_nat(&self, nat: usize) -> Self::SumId {
        if nat < self.l0.sums().count() {
            Either::Left(self.l0.sum_from_unique_nat(nat))
        } else {
            Either::Right(self.l1.sum_from_unique_nat(nat - self.l0.sums().count()))
        }
    }

    fn asi_convert(
        &self,
        id: Self::AlgebraicSortId,
    ) -> langspec::langspec::UnpackedAlgebraicSortId<Self> {
        id
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use expect_test::expect;
    use langspec::humanreadable::LangSpecHuman;

    fn hole() -> LangSpecHuman {
        serde_yml::from_str(
            r#"
        name:
            human: hole
            camel: Hole
            snake: hole
        products:
        - name:
            human: …
            camel: Hole
            snake: hole
          sorts:
        sums: []
    "#,
        )
        .unwrap()
    }

    fn join() -> Join<LangSpecHuman, LangSpecHuman> {
        Join {
            name: Name {
                human: "join".to_string(),
                camel: "Join".to_string(),
                snake: "join".to_string(),
            },
            l0: langspec_examples::fib(),
            l1: hole(),
        }
    }

    #[test]
    fn test_join() {
        let lsj = join();
        let expected = expect![[r#"
            {
              "name": {
                "human": "join",
                "camel": "Join",
                "snake": "join"
              },
              "products": [
                {
                  "name": {
                    "human": "+",
                    "camel": "Plus",
                    "snake": "plus"
                  },
                  "sorts": [
                    {
                      "Algebraic": {
                        "Sum": 0
                      }
                    },
                    {
                      "Algebraic": {
                        "Sum": 0
                      }
                    }
                  ]
                },
                {
                  "name": {
                    "human": "f",
                    "camel": "F",
                    "snake": "f"
                  },
                  "sorts": [
                    {
                      "Algebraic": {
                        "Sum": 0
                      }
                    }
                  ]
                },
                {
                  "name": {
                    "human": "…",
                    "camel": "Hole",
                    "snake": "hole"
                  },
                  "sorts": []
                }
              ],
              "sums": [
                {
                  "name": {
                    "human": "ℕ",
                    "camel": "Nat",
                    "snake": "nat"
                  },
                  "sorts": [
                    "NatLiteral",
                    {
                      "Algebraic": {
                        "Product": 1
                      }
                    },
                    {
                      "Algebraic": {
                        "Product": 0
                      }
                    }
                  ]
                }
              ]
            }"#]];
        let lsjf: langspec::flat::LangSpecFlat = lsj.canonical_into();
        expected.assert_eq(&serde_json::to_string_pretty(&lsjf).unwrap());
    }
}
