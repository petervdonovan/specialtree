use has_own_sort_id::HasOwnSortId;
use visit::visiteventsink::VisitEventSink;

use crate::{CompositePattern, DynPattern};

pub struct PatternBuilder<L, SortId> {
    int2sid: Vec<SortId>,
    stack: Vec<DynPattern<SortId>>,
    phantom: std::marker::PhantomData<L>,
}
impl<L, CurrentNode, Heap, SortId> VisitEventSink<CurrentNode, Heap> for PatternBuilder<L, SortId>
where
    CurrentNode: HasOwnSortId<Heap, L>,
    SortId: Clone,
{
    fn push(
        &mut self,
        _heap: &Heap,
        _t: &CurrentNode,
        _total: u32,
    ) -> visit::visiteventsink::PopOrProceed {
        visit::visiteventsink::PopOrProceed::Proceed
    }

    fn proceed(&mut self, _idx: u32, _total: u32) -> visit::visiteventsink::PopOrProceed {
        visit::visiteventsink::PopOrProceed::Proceed
    }

    fn pop(&mut self, total: u32) {
        assert!(self.stack.len() >= total as usize);
        let components = self.stack.split_off(self.stack.len() - (total as usize));
        self.stack.push(DynPattern::Composite(CompositePattern {
            rs_ty: self
                .int2sid
                .get(CurrentNode::own_sort_id() as usize)
                .unwrap()
                .clone(),
            components,
        }));
        todo!()
    }

    fn deconstruction_failure(&mut self) {
        unimplemented!()
    }
}
