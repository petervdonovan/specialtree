use either_id::Either;
use langspec::{
    langspec::{LangSpec, MappedType, Name, SortId, SortIdOf},
    sublang::{Sublang, TmfEndoMapping, reflexive_sublang},
    tymetafunc::TyMetaFuncSpec,
};
use tmfs_join::TmfsJoin;

pub struct PatternExtension<'a, L> {
    pub name: Name,
    pub l0: &'a L,
}

pub fn patternfy<'a, L: LangSpec>(l: &'a L) -> impl LangSpec + 'a {
    PatternExtension {
        name: Name {
            human: "Pattern".into(),
            camel: "Pattern".into(),
            snake: "pattern".into(),
        }
        .merge(l.name()),
        l0: l,
    }
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

impl<'a, L: LangSpec> LangSpec for PatternExtension<'a, L> {
    type ProductId = L::ProductId;

    type SumId = L::SumId;

    type Tmfs = TmfsJoin<L::Tmfs, pattern_tmf::PatternTmfs>;

    fn name(&self) -> &langspec::langspec::Name {
        &self.name
    }

    fn products(&self) -> impl Iterator<Item = Self::ProductId> {
        self.l0.products()
    }

    fn sums(&self) -> impl Iterator<Item = Self::SumId> {
        self.l0.sums()
    }

    fn product_name(&self, id: Self::ProductId) -> &langspec::langspec::Name {
        self.l0.product_name(id)
    }

    fn sum_name(&self, id: Self::SumId) -> &langspec::langspec::Name {
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

    fn sublangs(&self) -> Vec<langspec::sublang::Sublang<langspec::langspec::SortIdOf<Self>>> {
        self.l0
            .sublangs()
            .into_iter()
            .map(
                |sublang: Sublang<SortIdOf<L>>| langspec::sublang::Sublang::<SortIdOf<Self>> {
                    name: sublang.name,
                    image: sublang
                        .image
                        .clone()
                        .into_iter()
                        .map(map_sid::<'a, L>)
                        .collect(),
                    ty_names: sublang.ty_names,
                    map: Box::new(move |name| {
                        let id = (sublang.map)(name);
                        map_sid::<'a, L>(id)
                    }),
                    tems: sublang
                        .tems
                        .into_iter()
                        .map(|tem| TmfEndoMapping::<SortIdOf<Self>> {
                            fromshallow: match tem.fromshallow {
                                SortId::Algebraic(_) => panic!(),
                                SortId::TyMetaFunc(mapped_type) => SortId::TyMetaFunc(MappedType {
                                    f: Either::Left(mapped_type.f.clone()),
                                    a: mapped_type
                                        .a
                                        .into_iter()
                                        .map(map_sid_temfrom::<'a, L>)
                                        .collect(),
                                }),
                            },
                            fromrec: map_sid_temfrom::<L>(tem.fromrec),
                            to: map_sid::<'a, L>(tem.to),
                        })
                        .collect(),
                },
            )
            .chain(std::iter::once(reflexive_sublang(self)))
            .collect()
    }
}
