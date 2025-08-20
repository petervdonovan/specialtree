//! Data types for transitive CCF analysis

use term::CcfRelation;

/// Collection of transitive CCF relationships
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct CcfPaths<SortId> {
    pub units: Vec<TransitiveUnitCcfRelation<SortId>>,
    pub non_units: Vec<TransitiveNonUnitCcfRelation<SortId>>,
}

/// A transitive unit CCF relation represents a path between sorts through a single intermediary
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct TransitiveUnitCcfRelation<SortId> {
    pub to: SortId,
    pub from: SortId,
    pub intermediary: SortId,
}

impl<SortId: Ord> PartialOrd for TransitiveUnitCcfRelation<SortId> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<SortId: Ord> Ord for TransitiveUnitCcfRelation<SortId> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let cmp = self.to.cmp(&other.to);
        match cmp {
            std::cmp::Ordering::Equal => {
                let cmp = self.from.cmp(&other.from);
                if cmp == std::cmp::Ordering::Equal {
                    self.intermediary.cmp(&other.intermediary)
                } else {
                    cmp
                }
            }
            cmp => cmp,
        }
    }
}

/// A transitive non-unit CCF relation represents a path from multiple sorts to a single sort
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct TransitiveNonUnitCcfRelation<SortId> {
    pub from: Vec<SortId>,
    pub to: SortId,
    pub intermediary: CcfRelation<SortId>,
}

/// Pair used for UCP analysis
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct UcpPair<SortId> {
    pub from: SortId,
    pub to: SortId,
}

// Internal data structures used by analysis algorithms

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub(crate) struct UnitCcfRel<SortId> {
    pub from: SortId,
    pub to: SortId,
}

#[derive(Clone, Copy)]
pub(crate) struct Distance(pub usize);
