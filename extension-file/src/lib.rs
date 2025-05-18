use either_id::Either;
use langspec::{
    langspec::{LangSpec, MappedType, Name, SortId, SortIdOf},
    sublang::{Sublang, TmfEndoMapping, reflexive_sublang},
};
use tmfs_join::TmfsJoin;

pub struct FileExtension<'a, L: LangSpec> {
    pub name: Name,
    pub l: &'a L,
    pub item_sids: Vec<SortIdOf<L>>,
    file_name: Name,
    item_name: Name,
}

pub fn filefy<'a, L: LangSpec>(l: &'a L, item_sids: Vec<SortIdOf<L>>) -> impl LangSpec + 'a {
    FileExtension {
        name: Name {
            human: "file".into(),
            camel: "File".into(),
            snake: "file".into(),
        }
        .merge(l.name()),
        l,
        item_sids,
        file_name: Name {
            human: "file-root".into(),
            camel: "FileRoot".into(),
            snake: "file_root".into(),
        },
        item_name: Name {
            human: "file-item".into(),
            camel: "FileItem".into(),
            snake: "file_item".into(),
        },
    }
}
pub fn filefy_all_products<'a, L: LangSpec>(l: &'a L) -> impl LangSpec + 'a {
    filefy::<L>(
        l,
        l.products()
            .map(|it| SortId::Algebraic(langspec::langspec::AlgebraicSortId::Product(it)))
            .collect(),
    )
}

fn embed<'a, L: LangSpec>(sid: SortIdOf<L>) -> SortIdOf<FileExtension<'a, L>> {
    sid.fmap_f(Either::Left).fmap_s(Either::Left)
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Debug)]
pub enum FileSortId {
    Root,
    Item,
}

impl<'a, L: LangSpec> LangSpec for FileExtension<'a, L> {
    type ProductId = L::ProductId;

    type SumId = Either<L::SumId, FileSortId>;

    type Tmfs = TmfsJoin<L::Tmfs, file_tmf::FileTmfs>;

    fn name(&self) -> &langspec::langspec::Name {
        &self.name
    }

    fn products(&self) -> impl Iterator<Item = Self::ProductId> {
        self.l.products()
    }

    fn sums(&self) -> impl Iterator<Item = Self::SumId> {
        self.l.sums().map(Either::Left).chain([
            Either::Right(FileSortId::Root),
            Either::Right(FileSortId::Item),
        ])
    }

    fn product_name(&self, id: Self::ProductId) -> &langspec::langspec::Name {
        self.l.product_name(id)
    }

    fn sum_name(&self, id: Self::SumId) -> &langspec::langspec::Name {
        match id {
            Either::Left(id) => self.l.sum_name(id),
            Either::Right(fsi) => match fsi {
                FileSortId::Root => &self.file_name,
                FileSortId::Item => &self.item_name,
            },
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
            Either::Right(id) => match id {
                FileSortId::Root => vec![SortId::TyMetaFunc(MappedType {
                    f: Either::Right(file_tmf::FileTmfId),
                    a: vec![SortId::Algebraic(langspec::langspec::AlgebraicSortId::Sum(
                        Either::Right(FileSortId::Item),
                    ))],
                })]
                .into_iter(),
                FileSortId::Item => self
                    .item_sids
                    .iter()
                    .cloned()
                    .map(embed::<L>)
                    .collect::<Vec<_>>()
                    .into_iter(),
            },
        }
    }

    fn sublangs(&self) -> Vec<langspec::sublang::Sublang<langspec::langspec::SortIdOf<Self>>> {
        self.l
            .sublangs()
            .into_iter()
            .map(
                |sublang: Sublang<SortIdOf<L>>| langspec::sublang::Sublang::<SortIdOf<Self>> {
                    name: sublang.name,
                    image: sublang.image.clone().into_iter().map(embed::<L>).collect(),
                    ty_names: sublang.ty_names,
                    map: Box::new(move |name| {
                        let id = (sublang.map)(name);
                        embed::<L>(id)
                    }),
                    tems: sublang
                        .tems
                        .into_iter()
                        .map(|tem| TmfEndoMapping::<SortIdOf<Self>> {
                            fromshallow: embed::<L>(tem.fromshallow),
                            fromrec: embed::<L>(tem.fromrec),
                            to: embed::<L>(tem.to),
                        })
                        .collect(),
                },
            )
            .chain(std::iter::once(reflexive_sublang(self)))
            .collect()
    }
}
