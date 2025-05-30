use derivative::Derivative;
use names_langspec_sort::NamesLangspecSort;
use thiserror::Error;
use visit::visiteventsink::VisitEventSink;
use words::Implements;

use crate::{CompositePattern, DynPattern};
#[derive(Derivative)]
#[derivative(Debug(bound = "SortId: std::fmt::Debug"))]
pub struct PatternBuilder<L, LSub, SortId> {
    int2sid: Vec<SortId>,
    stack: Vec<DynPattern<SortId>>,
    phantom: std::marker::PhantomData<(L, LSub)>,
}
#[derive(Debug, Error)]
pub enum PbResultError {
    #[error("there are no patterns")]
    Empty,
    #[error("not all patterns are complete")]
    Ambiguous,
}
impl<L, LSub, SortId: Clone> PatternBuilder<L, LSub, SortId> {
    pub(crate) fn new(int2sid: Vec<SortId>) -> Self {
        Self {
            int2sid,
            stack: vec![],
            phantom: std::marker::PhantomData,
        }
    }
    pub fn literal<Heap, CurrentNode>(&mut self, literal: syn::Expr)
    where
        CurrentNode: Implements<Heap, LSub>,
        <CurrentNode as Implements<Heap, LSub>>::LWord: NamesLangspecSort<LSub>,
    {
        self.stack.push(DynPattern::Literal(crate::LiteralPattern {
            sid:
                self.int2sid
                    .get(
                        <<CurrentNode as Implements<Heap, LSub>>::LWord as NamesLangspecSort<
                            LSub,
                        >>::sort_idx() as usize,
                    )
                    .unwrap()
                    .clone(),
            equal_to: literal,
        }))
    }
    pub fn variable<Heap, CurrentNode>(&mut self, name: String)
    where
        CurrentNode: Implements<Heap, LSub>,
        <CurrentNode as Implements<Heap, LSub>>::LWord: NamesLangspecSort<LSub>,
    {
        self.stack.push(DynPattern::Variable(crate::Variable {
            sid:
                self.int2sid
                    .get(
                        <<CurrentNode as Implements<Heap, LSub>>::LWord as NamesLangspecSort<
                            LSub,
                        >>::sort_idx() as usize,
                    )
                    .unwrap()
                    .clone(),
            ident: name,
        }))
    }
    pub fn vzom<Heap, CurrentNode>(&mut self, name: String)
    where
        CurrentNode: Implements<Heap, LSub>,
        <CurrentNode as Implements<Heap, LSub>>::LWord: NamesLangspecSort<LSub>,
    {
        self.stack.push(DynPattern::ZeroOrMore(crate::Variable {
            sid:
                self.int2sid
                    .get(
                        <<CurrentNode as Implements<Heap, LSub>>::LWord as NamesLangspecSort<
                            LSub,
                        >>::sort_idx() as usize,
                    )
                    .unwrap()
                    .clone(),
            ident: name,
        }))
    }
    pub fn ignored<Heap, CurrentNode>(&mut self)
    where
        CurrentNode: Implements<Heap, LSub>,
        <CurrentNode as Implements<Heap, LSub>>::LWord: NamesLangspecSort<LSub>,
    {
        self.stack.push(
            DynPattern::Ignored(
                self.int2sid
                    .get(
                        <<CurrentNode as Implements<Heap, LSub>>::LWord as NamesLangspecSort<
                            LSub,
                        >>::sort_idx() as usize,
                    )
                    .unwrap()
                    .clone(),
            ),
        )
    }
    pub fn named(&mut self, name: String) {
        let child = Box::new(self.stack.pop().unwrap());
        self.stack.push(DynPattern::Named(name, child))
    }
    pub(crate) fn result(mut self) -> Result<DynPattern<SortId>, PbResultError> {
        match self.stack.len() {
            0 => Err(PbResultError::Empty),
            1 => Ok(self.stack.pop().unwrap()),
            _ => Err(PbResultError::Ambiguous),
        }
    }
}
impl<L, LSub, CurrentNode, Heap, SortId> VisitEventSink<CurrentNode, Heap>
    for PatternBuilder<L, LSub, SortId>
where
// CurrentNode: Implements<Heap, LSub>,
// <CurrentNode as Implements<Heap, LSub>>::LWord: NamesLangspecSort<LSub>,
// SortId: Clone,
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
        // assert!(self.stack.len() >= total as usize);
        // let components = self.stack.split_off(self.stack.len() - (total as usize));
        // self.stack.push(DynPattern::Composite(CompositePattern {
        //     rs_ty:
        //         self.int2sid
        //             .get(
        //                 <<CurrentNode as Implements<Heap, LSub>>::LWord as NamesLangspecSort<
        //                     LSub,
        //                 >>::sort_idx() as usize,
        //             )
        //             .unwrap()
        //             .clone(),
        //     components,
        // }));
        todo!()
    }

    fn deconstruction_failure(&mut self) {
        unimplemented!()
    }
}
