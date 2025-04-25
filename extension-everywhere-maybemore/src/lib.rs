use either_id::Either;
use langspec::{
    langspec::{LangSpec, MappedType, Name, SortIdOf},
    tymetafunc::{Transparency, TyMetaFuncSpec},
};

fn maybefy<L0: LangSpec, L1: LangSpec>(
    sid: langspec::langspec::SortIdOf<L1>,
) -> SortIdOfExtension<L0, L1> {
    langspec::langspec::SortId::TyMetaFunc(langspec::langspec::MappedType {
        f: Either::Right(tymetafuncspec_core::MAYBE),
        a: vec![l1_as_my_sid::<L0, L1>(sid)],
    })
}
fn maybemorefy<L0: LangSpec, L1: LangSpec>(
    sid: langspec::langspec::SortIdOf<L0>,
    maybe_sid: langspec::langspec::SortIdOf<L1>,
) -> SortIdOfExtension<L0, L1> {
    langspec::langspec::SortId::TyMetaFunc(langspec::langspec::MappedType {
        f: Either::Right(tymetafuncspec_core::PAIR),
        a: vec![l0_as_my_sid::<L0, L1>(sid), maybefy::<L0, L1>(maybe_sid)],
    })
}
fn maybemorefy_if_product<L0: LangSpec, L1: LangSpec>(
    sid: langspec::langspec::SortIdOf<L0>,
    maybe_sid: langspec::langspec::SortIdOf<L1>,
) -> SortIdOfExtension<L0, L1> {
    match sid {
        langspec::langspec::SortId::Algebraic(langspec::langspec::AlgebraicSortId::Product(_)) => {
            maybemorefy::<L0, L1>(sid, maybe_sid)
        }
        langspec::langspec::SortId::TyMetaFunc(mt) => l0_tmfmap::<L0, L1>(maybe_sid, mt),
        _ => l0_as_my_sid::<L0, L1>(sid),
    }
}
fn l0_tmfmap<L0: LangSpec, L1: LangSpec>(
    maybe_sid: langspec::langspec::SortIdOf<L1>,
    MappedType { f, a }: MappedType<
        L0::ProductId,
        L0::SumId,
        <L0::Tmfs as TyMetaFuncSpec>::TyMetaFuncId,
    >,
) -> SortIdOfExtension<L0, L1> {
    let rec = langspec::langspec::SortId::TyMetaFunc(langspec::langspec::MappedType {
        f: Either::Left(Either::Left(f.clone())),
        a: a.into_iter()
            .map(|a| maybemorefy_if_product::<L0, L1>(a, maybe_sid.clone()))
            .collect(),
    });
    match <<L0 as LangSpec>::Tmfs as TyMetaFuncSpec>::ty_meta_func_data(&f).transparency {
        Transparency::Visible => {
            langspec::langspec::SortId::TyMetaFunc(langspec::langspec::MappedType {
                f: Either::Right(tymetafuncspec_core::PAIR),
                a: vec![rec, maybefy::<L0, L1>(maybe_sid)],
            })
        }
        Transparency::Transparent => rec,
    }
}

use langspec_extension::{L0Map, LsExtension, SortIdOfExtension, l0_as_my_sid, l1_as_my_sid};

pub struct L0M<L1: LangSpec> {
    pub l1_root: SortIdOf<L1>,
}
impl<'a, 'b, L0, L1> L0Map<'a, 'b, L0, L1> for L0M<L1>
where
    L0: LangSpec,
    L1: LangSpec,
{
    fn l0_map(
        this: &LsExtension<'a, 'b, L0, L1, Self>,
        sid: SortIdOf<L0>,
    ) -> SortIdOfExtension<L0, L1> {
        maybemorefy_if_product::<L0, L1>(sid, this.l0m.l1_root.clone())
    }
}

pub fn everywhere_maybemore<'a, 'b, L0: LangSpec, L1: LangSpec>(
    name: Name,
    l0: &'a L0,
    l1: &'b L1,
    l1_root: SortIdOf<L1>,
) -> LsExtension<'a, 'b, L0, L1, L0M<L1>> {
    LsExtension {
        name,
        l0,
        l1,
        l0m: L0M { l1_root },
    }
}
