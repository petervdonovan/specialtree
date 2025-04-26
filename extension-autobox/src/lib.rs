use derivative::Derivative;
use either_id::Either;
use langspec::{
    flat::LangSpecFlat,
    langspec::{AlgebraicSortId, LangSpec, MappedType, Name, SortId, SortIdOf},
    tymetafunc::TyMetaFuncSpec,
};
use langspec_extension::{L0Map, LsExtension, l0_as_my_sid};
use tymetafuncspec_core::Core;

pub struct L0M<L0: LangSpec> {
    breaks: Vec<CycleBreak<L0>>,
}

static EMPTY_LANG: once_cell::sync::Lazy<(LangSpecFlat<Core>,)> =
    once_cell::sync::Lazy::new(|| {
        (LangSpecFlat::empty(Name {
            human: "EmptyLang".into(),
            camel: "EmptyLang".into(),
            snake: "empty_lang".into(),
        }),)
    });

pub fn autobox<L0>(l0: &'_ L0) -> impl LangSpec
where
    L0: LangSpec,
{
    let breaks = find_cycle_breaks(l0);
    println!("Cycle breaks: {:#?}", breaks);
    let empty: &'static LangSpecFlat<Core> = &EMPTY_LANG.0;
    let name = Name {
        human: format!("Autoboxed {}", l0.name().human),
        camel: format!("Autoboxed{}", l0.name().camel),
        snake: format!("autoboxed_{}", l0.name().snake),
    };
    LsExtension {
        name,
        l0,
        l1: empty,
        l0m: L0M { breaks },
    }
}

type EmptyLang = LangSpecFlat<Core>;

impl<'a, 'b, L0> L0Map<'a, 'b, L0, LangSpecFlat<Core>> for L0M<L0>
where
    L0: LangSpec,
{
    fn l0_map(
        this: &langspec_extension::LsExtension<'a, 'b, L0, EmptyLang, Self>,
        sid: SortIdOf<L0>,
    ) -> langspec_extension::SortIdOfExtension<L0, EmptyLang> {
        if this.l0m.breaks.iter().any(|break_| break_.from == sid) {
            langspec::langspec::SortId::TyMetaFunc(MappedType {
                f: Either::Right(tymetafuncspec_core::IDXBOX),
                a: vec![l0_as_my_sid::<L0, EmptyLang>(sid)],
            })
        } else {
            l0_as_my_sid::<L0, EmptyLang>(sid)
        }
    }
}

fn find_cycle_breaks<L: LangSpec>(ls: &L) -> Vec<CycleBreak<L>> {
    let mut breaks = vec![];
    while let Some(cycle) = find_size_depends_on_cycle(ls, &breaks) {
        breaks.push(find_best_cycle_break(cycle.as_slice()));
    }
    breaks
}

fn find_best_cycle_break<L: LangSpec>(cycle: &[SortIdOf<L>]) -> CycleBreak<L> {
    for (to, from) in cycle.iter().zip(cycle.iter().cycle().skip(1)) {
        match to {
            SortId::Algebraic(AlgebraicSortId::Sum(_)) => {
                return CycleBreak {
                    to: to.clone(),
                    from: from.clone(),
                };
            }
            _ => {
                // only try to break at sums for now
            }
        }
    }
    panic!("No cycle break found")
}

#[derive(Derivative)]
#[derivative(PartialEq(bound = ""))]
#[derivative(Eq(bound = ""))]
#[derivative(Debug(bound = ""))]
struct CycleBreak<L: LangSpec> {
    to: SortIdOf<L>,
    from: SortIdOf<L>,
}

fn find_size_depends_on_cycle<L: LangSpec>(
    ls: &L,
    breaks: &[CycleBreak<L>],
) -> Option<Vec<SortIdOf<L>>> {
    for starting_point in ls.all_sort_ids() {
        if let Some(cycle) = find_size_depends_on_cycle_from(ls, starting_point.clone(), breaks) {
            return Some(cycle);
        }
    }
    None
}

fn find_size_depends_on_cycle_from<L: LangSpec>(
    ls: &L,
    starting_point: SortIdOf<L>,
    breaks: &[CycleBreak<L>],
) -> Option<Vec<SortIdOf<L>>> {
    let mut visited = std::collections::HashSet::new();
    let mut stack = vec![starting_point.clone()];
    visited.insert(starting_point.clone());
    find_size_depends_on_cycle_from_rec(ls, breaks, &mut stack, &mut visited)
}

fn find_size_depends_on_cycle_from_rec<L: LangSpec>(
    ls: &L,
    breaks: &[CycleBreak<L>],
    stack: &mut Vec<SortIdOf<L>>,
    visited: &mut std::collections::HashSet<SortIdOf<L>>,
) -> Option<Vec<SortIdOf<L>>> {
    dbg!(&stack);
    if let Some(current) = stack.last() {
        for sid in size_depends_on(ls, current)
            .into_iter()
            .filter(|it| {
                !breaks.contains(&CycleBreak {
                    to: current.clone(),
                    from: it.clone(),
                })
            })
            .collect::<Vec<_>>()
        {
            if visited.contains(&sid) {
                for idx in 0..stack.len() {
                    if stack[idx] == sid {
                        stack.push(sid.clone());
                        return Some(stack[idx..].to_vec());
                    }
                }
            } else {
                visited.insert(sid.clone());
                stack.push(sid.clone());
                if let Some(cycle) = find_size_depends_on_cycle_from_rec(ls, breaks, stack, visited)
                {
                    return Some(cycle);
                }
            }
        }
        stack.pop();
    }
    None
}

fn size_depends_on<L: LangSpec>(ls: &L, sid: &SortIdOf<L>) -> Vec<SortIdOf<L>> {
    match sid {
        SortId::Algebraic(AlgebraicSortId::Product(pid)) => ls.product_sorts(pid.clone()).collect(),
        SortId::Algebraic(AlgebraicSortId::Sum(sid)) => ls.sum_sorts(sid.clone()).collect(),
        SortId::TyMetaFunc(MappedType { f, a }) => {
            <L::Tmfs as TyMetaFuncSpec>::ty_meta_func_data(f)
                .size_depends_on
                .iter()
                .map(|argid| a[argid.0].clone())
                .collect()
        }
    }
}
