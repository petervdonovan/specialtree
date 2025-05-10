use parse_adt::NamesParseLL;
use pmsp::{AdtMetadata, UsesStrategyForTraversal};
// use parse_adt::NamesParseLL;
// use term::{
//     case_split::HasBorrowedHeapRef,
//     visit::{Visitable, Visitor},
// };
use unparse::Unparse;
use visit::{
    Visit,
    visiteventsink::{PopOrProceed, VisitEventSink},
};

pub mod tmfscore;
pub mod unparse;

pub struct Unparser<'arena, L> {
    pub unparse: Unparse<'arena>,
    phantom: std::marker::PhantomData<L>,
}

impl<'arena, L, Heap, EitherLeft, EitherRight> UsesStrategyForTraversal<Unparser<'arena, L>>
    for tymetafuncspec_core::Either<Heap, EitherLeft, EitherRight>
{
}

pub fn unparse<Heap, L, T>(heap: &mut Heap, t: &T) -> String
where
    for<'a> Unparser<'a, L>: Visit<AdtMetadata, T, Heap, L>,
{
    let arena = bumpalo::Bump::new();
    let mut unparser = Unparser::new(&arena);
    <Unparser<'_, L> as Visit<_, _, _, _>>::visit(&mut unparser, heap, &t);
    format!("{:?}", &unparser.unparse)
}

impl<'arena, L> Unparser<'arena, L> {
    pub fn new(arena: &'arena bumpalo::Bump) -> Self {
        Unparser {
            unparse: Unparse::new(arena),
            phantom: std::marker::PhantomData,
        }
    }
}

impl<'arena, L, CurrentNode, Heap> VisitEventSink<CurrentNode, Heap> for Unparser<'arena, L>
where
    CurrentNode: words::Implements<Heap, L>,
    <CurrentNode as words::Implements<Heap, L>>::LWord: NamesParseLL,
{
    fn push(&mut self, _heap: &Heap, _t: &CurrentNode) -> visit::visiteventsink::PopOrProceed {
        self.unparse.consistent_group_start();
        let start = <<CurrentNode as words::Implements<Heap, L>>::LWord as NamesParseLL>::START;
        for kw in start.0 {
            self.unparse.static_text(kw.get());
        }
        PopOrProceed::Proceed
    }

    fn proceed(&mut self) -> visit::visiteventsink::PopOrProceed {
        // todo
        PopOrProceed::Proceed
    }

    fn pop(&mut self) {
        // todo
    }

    fn deconstruction_failure(&mut self) {
        self.unparse.static_text("???");
    }
}
