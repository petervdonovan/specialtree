use crate::DynPattern;

pub struct PatternBuilder<SortId> {
    stack: Vec<DynPattern<SortId>>,
}
