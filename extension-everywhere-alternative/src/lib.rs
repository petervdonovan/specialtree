use aspect::Aspect;
use either_id::Either;
use langspec::langspec::{LangSpec, MappedType, SortIdOf};
use langspec_extension::{L0Map, LsExtension, SortIdOfExtension, l0_as_my_sid, l1_as_my_sid};
use tree_identifier::Identifier;

struct L0M<L1: LangSpec> {
    l1_root: SortIdOf<L1>,
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
        fn eitherfy<'a, 'b, L0, L1>(
            this: &LsExtension<'a, 'b, L0, L1, L0M<L1>>,
            sid: SortIdOfExtension<L0, L1>,
        ) -> SortIdOfExtension<L0, L1>
        where
            L0: LangSpec,
            L1: LangSpec,
        {
            langspec::langspec::SortId::TyMetaFunc(MappedType {
                f: Either::Right(tymetafuncspec_core::EITHER),
                a: vec![sid, l1_as_my_sid::<L0, L1>(this.l0m.l1_root.clone())],
            })
        }

        eitherfy::<L0, L1>(
            this,
            match sid {
                langspec::langspec::SortId::Algebraic(_) => l0_as_my_sid::<L0, L1>(sid.clone()),
                langspec::langspec::SortId::TyMetaFunc(MappedType { f, a }) => {
                    langspec::langspec::SortId::TyMetaFunc(MappedType {
                        f: Either::Left(Either::Left(f)),
                        a: a.into_iter().map(|sid| Self::l0_map(this, sid)).collect(),
                    })
                }
            },
        )
    }

    type SelfAsLifetime<'c> = L0M<L1::AsLifetime<'c>>;
}

pub fn everywhere_alternative<L0: LangSpec, L1: LangSpec>(
    name: Identifier,
    l0: &L0,
    l1: &L1,
    l1_root: SortIdOf<L1>,
    added_aspects: Vec<&'static (dyn Aspect + 'static)>,
) -> impl LangSpec {
    LsExtension {
        name,
        l0,
        l1,
        l0m: L0M { l1_root },
        added_aspects,
    }
}
