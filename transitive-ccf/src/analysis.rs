//! CCF analysis algorithms

use crate::types::*;
use langspec::langspec::{
    AlgebraicSortId, LangSpec, SortId, SortIdOf, call_on_all_tmf_monomorphizations,
};
use langspec::sublang::Sublangs;
use langspec::tymetafunc::TyMetaFuncSpec;
use rustgen_utils::combinations;
use std::collections::{HashMap, HashSet};
use term::CcfRelation;

use memo::memo;
use memo::memo_cache::thread_local_cache;

#[memo('a)]
pub fn ccf_paths<'a, L: LangSpec>(
    ls: &'a L,
    important_sublangs: &'a impl Sublangs<SortIdOf<L>>,
) -> CcfPaths<SortIdOf<L>> {
    let direct_ccf_rels = get_direct_ccf_rels(thread_local_cache(), ls);
    let mut ucp_acc = std::collections::HashSet::new();
    let mut cebup_acc = std::collections::HashSet::new();
    for non_transparent_sorts in important_sublangs.images() {
        let ucp = unit_ccf_paths_quadratically_large_closure(
            thread_local_cache(),
            direct_ccf_rels,
            &non_transparent_sorts,
        );
        let cebup = ccfs_exploded_by_unit_paths(
            thread_local_cache(),
            direct_ccf_rels,
            &ucp,
            &non_transparent_sorts,
        );
        let cebup_filtered = cebup
            .iter()
            .cloned()
            .filter(|it| {
                it.from.iter().all(|it| non_transparent_sorts.contains(it))
                    && non_transparent_sorts.contains(&it.to)
            })
            .collect::<Vec<_>>();
        ucp_acc.extend(ucp.iter().cloned().filter(|it| {
            non_transparent_sorts.contains(&it.from)
                || cebup_filtered
                    .iter()
                    .any(|cebup| cebup.intermediary.to == it.from)
        }));
        cebup_acc.extend(cebup_filtered);
    }
    let all_tems = important_sublangs
        .tems()
        .flatten()
        .filter(|it| it.from_extern_behavioral != it.to_structural)
        .filter(|it| {
            !ucp_acc.iter().any(|existing| {
                existing.from == it.from_extern_behavioral && existing.to == it.to_structural
            })
        })
        .collect::<Vec<_>>();
    for desired_pair in all_tems.into_iter() {
        // todo: iterate over the existing ucps targeting anything under tmfs that don't have a unit ccf,
        // starting of course with the reflexive ucp, until you exhaust the combinations. Only then, panic.
        // Yes I know this is potentially very inefficient. Is it time to rethink this? Is it appropriate, now, to "preserve, don't recover?" Or is it appropriate only to update fromshallow?
        ucp_acc.insert(
            find_ucp(
                direct_ccf_rels.as_slice(),
                UcpPair {
                    from: desired_pair.from_extern_behavioral.clone(),
                    to: desired_pair.to_structural.clone(),
                },
            )
            .unwrap_or_else(|| panic!("Cannot find UCP for {:?}", &desired_pair)),
        );
    }
    let mut units_sorted = ucp_acc.into_iter().collect::<Vec<_>>();
    units_sorted.sort();
    let mut non_units_sorted = cebup_acc.into_iter().collect::<Vec<_>>();
    non_units_sorted.sort();
    CcfPaths {
        units: units_sorted,
        non_units: non_units_sorted,
    }
}

/// Get direct CCF relations from a language specification
#[memo('a)]
pub fn get_direct_ccf_rels<'a, L>(ls: &'a L) -> Vec<CcfRelation<SortIdOf<L>>>
where
    L: LangSpec,
{
    let mut ret: Vec<CcfRelation<SortIdOf<L>>> = ls
        .products()
        .map(|pid| CcfRelation {
            from: ls.product_sorts(pid.clone()).collect(),
            to: SortId::Algebraic(AlgebraicSortId::Product(pid)),
        })
        .chain(ls.sums().flat_map(|sid| {
            ls.sum_sorts(sid.clone()).map(move |argsid| CcfRelation {
                from: vec![argsid.clone()],
                to: SortId::Algebraic(AlgebraicSortId::Sum(sid.clone())),
            })
        }))
        .chain(ls.products().map(|sid| CcfRelation {
            from: vec![SortId::Algebraic(AlgebraicSortId::Product(sid.clone()))],
            to: SortId::Algebraic(AlgebraicSortId::Product(sid)),
        }))
        .chain(ls.sums().map(|sid| CcfRelation {
            from: vec![SortId::Algebraic(AlgebraicSortId::Sum(sid.clone()))],
            to: SortId::Algebraic(AlgebraicSortId::Sum(sid)),
        }))
        .collect();
    call_on_all_tmf_monomorphizations(ls, &mut |mt| {
        for argids in L::Tmfs::ty_meta_func_data(&mt.f).canonical_froms {
            ret.push(CcfRelation {
                from: argids.iter().map(|argid| mt.a[argid.0].clone()).collect(),
                to: SortId::TyMetaFunc(mt.clone()),
            });
        }
        ret.push(CcfRelation {
            from: vec![SortId::TyMetaFunc(mt.clone())],
            to: SortId::TyMetaFunc(mt.clone()),
        });
    });
    ret
}

/// Compute unit CCF paths with quadratically large closure
#[memo('a)]
pub fn unit_ccf_paths_quadratically_large_closure<
    'a,
    SortId: std::fmt::Debug + Clone + Eq + std::hash::Hash,
>(
    direct_ccf_rels: &'a [CcfRelation<SortId>],
    non_transparent_sorts: &'a [SortId],
) -> Vec<TransitiveUnitCcfRelation<SortId>> {
    let unit_ccf_rels: Vec<_> = direct_ccf_rels
        .iter()
        .filter(|rel| rel.from.len() == 1)
        .map(|rel| UnitCcfRel {
            from: rel.from.first().unwrap().clone(),
            to: rel.to.clone(),
        })
        .collect();
    let unit_ccf_tos: HashSet<_> = unit_ccf_rels.iter().map(|rel| rel.to.clone()).collect();
    unit_ccf_tos
        .iter()
        .flat_map(|to| get_tucr_for_to::<SortId>(&unit_ccf_rels, to.clone(), non_transparent_sorts))
        .collect()
}

/// Compute CCFs exploded by unit paths
#[memo('a)]
pub fn ccfs_exploded_by_unit_paths<'a, SortId: Clone + Eq>(
    direct_ccf_rels: &'a [CcfRelation<SortId>],
    unit_ccf_rels: &'a [TransitiveUnitCcfRelation<SortId>],
    non_transparent_sorts: &'a [SortId],
) -> Vec<TransitiveNonUnitCcfRelation<SortId>> {
    fn from_sets<SortId: Eq + Clone>(
        froms: Vec<SortId>,
        unit_ccf_rels: &[TransitiveUnitCcfRelation<SortId>],
    ) -> Vec<Vec<SortId>> {
        froms
            .iter()
            .map(|from| {
                unit_ccf_rels
                    .iter()
                    .filter_map(|it| {
                        if &it.to == from {
                            Some(it.from.clone())
                        } else {
                            None
                        }
                    })
                    .chain(std::iter::once(from.clone())) // also allow no intermediaries
                    .collect()
            })
            .collect()
    }
    let unit_ccf_rels_from_nts: Vec<_> = unit_ccf_rels
        .iter()
        .filter(|&rel| non_transparent_sorts.contains(&rel.from))
        .cloned()
        .collect();
    direct_ccf_rels
        .iter()
        .filter(|direct| direct.from.len() != 1) // we only want non-units
        .flat_map(|direct| {
            unit_ccf_rels
                .iter()
                .filter_map(|it| {
                    if it.from == direct.to && non_transparent_sorts.contains(&it.to) {
                        Some(it.to.clone())
                    } else {
                        None
                    }
                })
                .chain(std::iter::once(direct.to.clone())) // also allow no intermediaries
                .flat_map({
                    let unit_ccf_rels_from_nts = unit_ccf_rels_from_nts.clone();
                    move |to| {
                        combinations(from_sets(direct.from.clone(), &unit_ccf_rels_from_nts))
                            .filter({
                                let to = to.clone();
                                move |from| !from.contains(&to)
                            }) // we don't want cycles
                            .map(move |from| TransitiveNonUnitCcfRelation {
                                from: from.clone(),
                                to: to.clone(),
                                intermediary: direct.clone(),
                            })
                    }
                })
        })
        .collect()
}

// Helper functions

fn unit_ccf_rels<SortId: Clone>(
    direct_ccf_rels: &[CcfRelation<SortId>],
) -> Vec<UnitCcfRel<SortId>> {
    direct_ccf_rels
        .iter()
        .filter(|rel| rel.from.len() == 1)
        .map(|rel| UnitCcfRel {
            from: rel.from.first().unwrap().clone(),
            to: rel.to.clone(),
        })
        .collect()
}

pub fn find_ucp<SortId: Clone + Eq + std::hash::Hash + std::fmt::Debug>(
    direct_ccf_rels: &[CcfRelation<SortId>],
    pair: UcpPair<SortId>,
) -> Option<TransitiveUnitCcfRelation<SortId>> {
    let tucr = get_tucr_for_to(&unit_ccf_rels(direct_ccf_rels), pair.to.clone(), &[]);
    tucr.into_iter()
        .find(|tucr| tucr.from == pair.from && tucr.to == pair.to)
        .clone()
}

fn get_tucr_for_to<SortId: std::fmt::Debug + Clone + Eq + std::hash::Hash>(
    unit_ccf_rels: &[UnitCcfRel<SortId>],
    to: SortId,
    non_transparent_sorts: &[SortId],
) -> Vec<TransitiveUnitCcfRelation<SortId>> {
    let intermediaries = unit_ccf_rels
        .iter()
        .filter(|rel| rel.to == to && rel.from != to)
        .map(|rel| rel.from.clone())
        .collect::<HashSet<_>>();
    enum Ambiguity {
        Ambiguous,
        Unambiguous,
    }
    type CostedTucrs<SortId> =
        HashMap<UcpPair<SortId>, (TransitiveUnitCcfRelation<SortId>, Distance)>;
    type CostedTucrsWithAmbiguity<SortId> =
        HashMap<UcpPair<SortId>, (TransitiveUnitCcfRelation<SortId>, Distance, Ambiguity)>;
    fn get_tucr_for_to_and_intermediary<SortId: Clone + Eq + std::hash::Hash>(
        unit_ccf_rels: &[UnitCcfRel<SortId>],
        to: SortId,
        intermediary: SortId,
        non_transparent_sorts: &[SortId],
    ) -> CostedTucrs<SortId> {
        fn find_all_reachable_from_intermediary<SortId: Clone + Eq + std::hash::Hash>(
            unit_ccf_rels: &[UnitCcfRel<SortId>],
            intermediary: SortId,
            forbidden_node: SortId,
            non_transparent_sorts: &[SortId],
        ) -> HashMap<SortId, Distance> {
            let mut reachable = HashMap::new();
            let mut distance = Distance(0);
            let mut frontier = vec![(
                intermediary.clone(),
                non_transparent_sorts.contains(&forbidden_node),
            )];
            while !frontier.is_empty() {
                distance.0 += 1;
                let mut new_frontier = vec![];
                for sid in frontier.iter() {
                    if reachable.contains_key(&sid.0) || sid.0 == forbidden_node {
                        continue;
                    }
                    reachable.insert(sid.0.clone(), distance);
                    let nt = non_transparent_sorts.contains(&sid.0);
                    if sid.1 && nt {
                        continue;
                    }
                    for ucr in unit_ccf_rels.iter() {
                        if ucr.to == sid.0 {
                            new_frontier.push((ucr.from.clone(), sid.1 || nt));
                        }
                    }
                }
                frontier = new_frontier;
            }
            reachable
        }
        let reachable = find_all_reachable_from_intermediary::<SortId>(
            unit_ccf_rels,
            intermediary.clone(),
            to.clone(),
            non_transparent_sorts,
        );
        reachable
            .into_iter()
            .map(move |(from, distance)| {
                (
                    UcpPair {
                        from: from.clone(),
                        to: to.clone(),
                    },
                    (
                        TransitiveUnitCcfRelation {
                            from: from.clone(),
                            to: to.clone(),
                            intermediary: intermediary.clone(),
                        },
                        distance,
                    ),
                )
            })
            .collect()
    }
    let mut tucrs: CostedTucrsWithAmbiguity<SortId> = HashMap::new();
    for intermediary in intermediaries.iter() {
        let tucrs_intermediary = get_tucr_for_to_and_intermediary::<SortId>(
            unit_ccf_rels,
            to.clone(),
            intermediary.clone(),
            non_transparent_sorts,
        );
        for (pair, (tucr, distance)) in tucrs_intermediary.iter() {
            if let Some(existing) = tucrs.get(pair) {
                if existing.1.0 > distance.0 {
                    tucrs.insert(
                        pair.clone(),
                        (tucr.clone(), *distance, Ambiguity::Unambiguous),
                    );
                } else if existing.1.0 == distance.0 {
                    tucrs.insert(
                        pair.clone(),
                        (tucr.clone(), *distance, Ambiguity::Ambiguous),
                    );
                }
            } else {
                tucrs.insert(
                    pair.clone(),
                    (tucr.clone(), *distance, Ambiguity::Unambiguous),
                );
            }
        }
    }
    tucrs
        .into_values()
        .filter(|it| matches!(it.2, Ambiguity::Unambiguous))
        .map(|v| v.0)
        .collect()
}
