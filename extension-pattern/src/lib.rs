use std::{any::TypeId, rc::Rc};

use either_id::Either;
use langspec::{
    langspec::{AlgebraicSortId, AsLifetime, LangSpec, MappedType, SortId, SortIdOf},
    sublang::{AspectImplementors, Sublang, reflexive_sublang},
    tymetafunc::TyMetaFuncSpec,
};
use tmfs_join::TmfsJoin;
use tree_identifier::Identifier;

pub struct PatternExtension<'a, L> {
    pub name: Identifier,
    pub l0: &'a L,
}

pub fn patternfy<'a, L: LangSpec>(arena: &'a bumpalo::Bump, l: &'a L) -> impl LangSpec + 'a {
    let patternfied = arena.alloc(PatternExtension {
        name: Identifier::list(
            vec![
                Identifier::from_camel_str("Pattern").unwrap(),
                l.name().clone(),
            ]
            .into(),
        ),
        l0: l,
    });
    extension_file::filefy_all_tmf(
        patternfied,
        Either::Right(pattern_tmf::PatternTmfsId::NamedPattern),
    )
}

fn map_sid_temfrom<'a, L: LangSpec + 'a>(sid: SortIdOf<L>) -> SortIdOf<PatternExtension<'a, L>> {
    sid.fmap_f(Either::Left)
}
fn map_sid<'a, L: LangSpec + 'a>(sid: SortIdOf<L>) -> SortIdOf<PatternExtension<'a, L>> {
    match sid {
        SortId::Algebraic(_) => map_sid_temfrom::<'a, L>(sid),
        SortId::TyMetaFunc(mapped_type) => {
            let data = <L::Tmfs as TyMetaFuncSpec>::ty_meta_func_data(&mapped_type.f);
            let args = mapped_type
                .a
                .iter()
                .enumerate()
                .map(|(idx, sid)| {
                    if data.is_collection_of.iter().any(|it| it.0 == idx) {
                        SortId::TyMetaFunc(MappedType {
                            f: Either::Right(pattern_tmf::PatternTmfsId::OrVariableZeroOrMore),
                            a: vec![map_sid::<'a, L>(sid.clone())],
                        })
                    } else {
                        map_sid::<'a, L>(sid.clone())
                    }
                })
                .collect();
            SortId::TyMetaFunc(MappedType {
                f: Either::Left(mapped_type.f),
                a: args,
            })
        }
    }
}

impl<'a, L: LangSpec> AsLifetime<Self> for PatternExtension<'a, L> {
    type AsLifetime<'this> = PatternExtension<'this, L::AsLifetime<'this>>;
}

impl<'a, L: LangSpec> LangSpec for PatternExtension<'a, L> {
    type ProductId = L::ProductId;

    type SumId = L::SumId;

    type Tmfs = TmfsJoin<L::Tmfs, pattern_tmf::PatternTmfs>;

    fn name(&self) -> &Identifier {
        &self.name
    }

    fn products(&self) -> impl Iterator<Item = Self::ProductId> {
        self.l0.products()
    }

    fn sums(&self) -> impl Iterator<Item = Self::SumId> {
        self.l0.sums()
    }

    fn product_name(&self, id: Self::ProductId) -> &Identifier {
        self.l0.product_name(id)
    }

    fn sum_name(&self, id: Self::SumId) -> &Identifier {
        self.l0.sum_name(id)
    }

    fn product_sorts(
        &self,
        id: Self::ProductId,
    ) -> impl Iterator<Item = langspec::langspec::SortIdOf<Self>> {
        self.l0.product_sorts(id).map(|sid| {
            langspec::langspec::SortId::TyMetaFunc(MappedType {
                f: Either::Right(pattern_tmf::PatternTmfsId::OrVariable),
                a: vec![map_sid::<'a, L>(sid)],
            })
        })
    }

    fn sum_sorts(
        &self,
        id: Self::SumId,
    ) -> impl Iterator<Item = langspec::langspec::SortIdOf<Self>> {
        self.l0.sum_sorts(id).map(map_sid::<'a, L>)
    }

    fn tmf_roots(&self) -> impl Iterator<Item = langspec::langspec::MappedTypeOf<Self>> {
        self.l0
            .tmf_roots()
            .map(|it| it.fmap_f(Either::Left))
            .chain(self.products().map(|pid| MappedType {
                f: Either::Right(pattern_tmf::PatternTmfsId::NamedPattern),
                a: vec![SortId::Algebraic(AlgebraicSortId::Product(pid))],
            }))
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
                |Sublang {
                     lsub,
                     aspect_implementors,
                 }| Sublang {
                    lsub,
                    aspect_implementors: aspect_implementors
                        .into_iter()
                        .map(
                            |tem| AspectImplementors::<LSub::AsLifetime<'this>, SortIdOf<Self>> {
                                aspect_zst: tem.aspect_zst,
                                map: Rc::new(move |sid: &SortIdOf<LSub::AsLifetime<'this>>| {
                                    map_sid::<'a, L>((tem.map)(sid))
                                }),
                            },
                        )
                        .collect(),
                },
            )
        }
    }
}
