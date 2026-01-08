use std::any::TypeId;

use either_id::Either;
use file_tmf::FileTmfId;
use langspec::{
    langspec::{AsLifetime, LangSpec, MappedType, SortId, SortIdOf},
    sublang::{AspectImplementors, Sublang, reflexive_sublang},
    tymetafunc::TyMetaFuncSpec,
};
use tmfs_join::TmfsJoin;
use tree_identifier::Identifier;

pub struct FileExtension<'a, L: LangSpec> {
    pub name: Identifier,
    pub l: &'a L,
    pub item_sids: Vec<SortIdOf<L>>,
    item_name: Identifier,
}

pub fn filefy<'a, L: LangSpec>(l: &'a L, item_sids: Vec<SortIdOf<L>>) -> impl LangSpec + 'a {
    FileExtension {
        name: Identifier::list(
            vec![
                Identifier::from_camel_str("File").unwrap(),
                l.name().clone(),
            ]
            .into(),
        ),
        l,
        item_sids,
        item_name: Identifier::from_snake_str("file_item").unwrap(),
    }
}
pub fn filefy_all_tmf<'a, L: LangSpec>(
    l: &'a L,
    f: <<L as LangSpec>::Tmfs as TyMetaFuncSpec>::TyMetaFuncId,
) -> impl LangSpec + 'a {
    filefy::<L>(
        l,
        l.all_sort_ids()
            .filter(|sid| match sid {
                SortId::Algebraic(_) => false,
                SortId::TyMetaFunc(mapped_type) => mapped_type.f == f,
            })
            .collect(),
    )
}

fn embed<'a, L: LangSpec + 'a>(sid: SortIdOf<L>) -> SortIdOf<FileExtension<'a, L>> {
    sid.fmap_f(Either::Left).fmap_s(Either::Left)
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Debug)]
pub struct FileItemSortId;

impl<'a, L: LangSpec> AsLifetime for FileExtension<'a, L> {
    type AsLifetime<'this> = FileExtension<'this, L::AsLifetime<'this>>;
}

impl<'a, L: LangSpec> LangSpec for FileExtension<'a, L> {
    type ProductId = L::ProductId;

    type SumId = Either<L::SumId, FileItemSortId>;

    type Tmfs = TmfsJoin<L::Tmfs, file_tmf::FileTmfs>;

    fn name(&self) -> &Identifier {
        &self.name
    }

    fn products(&self) -> impl Iterator<Item = Self::ProductId> {
        self.l.products()
    }

    fn sums(&self) -> impl Iterator<Item = Self::SumId> {
        self.l
            .sums()
            .map(Either::Left)
            .chain([Either::Right(FileItemSortId)])
    }

    fn product_name(&self, id: Self::ProductId) -> &Identifier {
        self.l.product_name(id)
    }

    fn sum_name(&self, id: Self::SumId) -> &Identifier {
        match id {
            Either::Left(id) => self.l.sum_name(id),
            Either::Right(_) => &self.item_name,
        }
    }

    fn product_sorts(
        &self,
        id: Self::ProductId,
    ) -> impl Iterator<Item = langspec::langspec::SortIdOf<Self>> {
        self.l.product_sorts(id).map(embed::<L>)
    }

    fn sum_sorts(
        &self,
        id: Self::SumId,
    ) -> impl Iterator<Item = langspec::langspec::SortIdOf<Self>> {
        match id {
            Either::Left(id) => self
                .l
                .sum_sorts(id)
                .map(embed::<L>)
                .collect::<Vec<_>>()
                .into_iter(),
            Either::Right(_) => self
                .item_sids
                .iter()
                .cloned()
                .map(embed::<L>)
                .collect::<Vec<_>>()
                .into_iter(),
        }
    }

    fn tmf_roots(&self) -> impl Iterator<Item = langspec::langspec::MappedTypeOf<Self>> {
        std::iter::once(MappedType {
            f: Either::Right(FileTmfId),
            a: vec![SortId::Algebraic(langspec::langspec::AlgebraicSortId::Sum(
                Either::Right(FileItemSortId),
            ))],
        })
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
            self.l.sublang::<LSub>(lsub).map(
                |Sublang {
                     lsub,
                     map,
                     aspect_implementors,
                 }| Sublang {
                    lsub,
                    map: Box::new(move |name| {
                        let id = (map)(name);
                        embed::<L>(id)
                    }),
                    aspect_implementors: aspect_implementors
                        .into_iter()
                        .map(|tem| AspectImplementors::<SortIdOf<Self>> {
                            from_extern_behavioral: embed::<L>(tem.from_extern_behavioral),
                            // fromrec: embed::<L>(tem.fromrec),
                            to_structural: embed::<L>(tem.to_structural),
                        })
                        .collect(),
                },
            )
        }
    }

    // fn sublangs(&self) -> Vec<langspec::sublang::Sublang<langspec::langspec::SortIdOf<Self>>> {
    //     self.l
    //         .sublangs()
    //         .into_iter()
    //         .map(
    //             |sublang: Sublang<SortIdOf<L>>| langspec::sublang::Sublang::<SortIdOf<Self>> {
    //                 name: sublang.name,
    //                 image: sublang.image.clone().into_iter().map(embed::<L>).collect(),
    //                 ty_names: sublang.ty_names,
    //                 map: Box::new(move |name| {
    //                     let id = (sublang.map)(name);
    //                     embed::<L>(id)
    //                 }),
    //                 aspect_implementors: sublang
    //                     .aspect_implementors
    //                     .into_iter()
    //                     .map(|tem| TmfEndoMapping::<SortIdOf<Self>> {
    //                         fromshallow: embed::<L>(tem.fromshallow),
    //                         fromrec: embed::<L>(tem.fromrec),
    //                         to: embed::<L>(tem.to),
    //                     })
    //                     .collect(),
    //             },
    //         )
    //         .chain(std::iter::once(reflexive_sublang(self)))
    //         .collect()
    // }
}
