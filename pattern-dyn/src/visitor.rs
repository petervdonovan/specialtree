use derivative::Derivative;
use names_langspec_sort::NamesLangspecSort;
use thiserror::Error;
use visit::visiteventsink::VisitEventSink;
use words::Implements;

use crate::{CompositePattern, DynPattern};
#[derive(Derivative)]
#[derivative(Debug(bound = "SortId: std::fmt::Debug"))]
pub struct PatternBuilder<L, SortId> {
    int2sid: Vec<SortId>,
    stack: Vec<DynPattern<SortId>>,
    phantom: std::marker::PhantomData<L>,
}
#[derive(Debug, Error)]
pub enum PbResultError {
    #[error("there are no patterns")]
    Empty,
    #[error("not all patterns are complete")]
    Ambiguous,
}
impl<L, SortId: Clone> PatternBuilder<L, SortId> {
    pub(crate) fn new(int2sid: Vec<SortId>) -> Self {
        Self {
            int2sid,
            stack: vec![],
            phantom: std::marker::PhantomData,
        }
    }
    pub fn literal<Heap, CurrentNode>(&mut self, literal: syn::Expr)
    where
        CurrentNode: Implements<Heap, L>,
        <CurrentNode as Implements<Heap, L>>::LWord: NamesLangspecSort<L>,
    {
        self.stack.push(DynPattern::Literal(crate::LiteralPattern {
            sid: self
                .int2sid
                .get(
                    <<CurrentNode as Implements<Heap, L>>::LWord as NamesLangspecSort<L>>::sort_idx(
                    ) as usize,
                )
                .unwrap()
                .clone(),
            equal_to: literal,
        }))
    }
    pub fn variable<Heap, CurrentNode>(&mut self, name: String)
    where
        CurrentNode: Implements<Heap, L>,
        <CurrentNode as Implements<Heap, L>>::LWord: NamesLangspecSort<L>,
    {
        self.stack.push(DynPattern::Variable(crate::Variable {
            sid: self
                .int2sid
                .get(
                    <<CurrentNode as Implements<Heap, L>>::LWord as NamesLangspecSort<L>>::sort_idx(
                    ) as usize,
                )
                .unwrap()
                .clone(),
            ident: name,
        }))
    }
    pub fn vzom<Heap, CurrentNode>(&mut self, name: String)
    where
        CurrentNode: Implements<Heap, L>,
        <CurrentNode as Implements<Heap, L>>::LWord: NamesLangspecSort<L>,
    {
        self.stack.push(DynPattern::ZeroOrMore(crate::Variable {
            sid: self
                .int2sid
                .get(
                    <<CurrentNode as Implements<Heap, L>>::LWord as NamesLangspecSort<L>>::sort_idx(
                    ) as usize,
                )
                .unwrap()
                .clone(),
            ident: name,
        }))
    }
    pub fn ignored<Heap, CurrentNode>(&mut self)
    where
        CurrentNode: Implements<Heap, L>,
        <CurrentNode as Implements<Heap, L>>::LWord: NamesLangspecSort<L>,
    {
        self.stack.push(DynPattern::Ignored(
            self.int2sid
                .get(
                    <<CurrentNode as Implements<Heap, L>>::LWord as NamesLangspecSort<L>>::sort_idx(
                    ) as usize,
                )
                .unwrap()
                .clone(),
        ))
    }
    pub(crate) fn result(mut self) -> Result<DynPattern<SortId>, PbResultError> {
        match self.stack.len() {
            0 => Err(PbResultError::Empty),
            1 => Ok(self.stack.pop().unwrap()),
            _ => Err(PbResultError::Ambiguous),
        }
    }
}
impl<L, CurrentNode, Heap, SortId> VisitEventSink<CurrentNode, Heap> for PatternBuilder<L, SortId>
where
    CurrentNode: Implements<Heap, L>,
    <CurrentNode as Implements<Heap, L>>::LWord: NamesLangspecSort<L>,
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
                .get(
                    <<CurrentNode as Implements<Heap, L>>::LWord as NamesLangspecSort<L>>::sort_idx(
                    ) as usize,
                )
                .unwrap()
                .clone(),
            components,
        }));
    }

    fn deconstruction_failure(&mut self) {
        unimplemented!()
    }
}
