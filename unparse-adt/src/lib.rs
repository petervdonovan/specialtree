use aspect::VisitationAspect;
use parse_adt::NamesParseLL;
// use parse_adt::NamesParseLL;
// use term::{
//     case_split::HasBorrowedHeapRef,
//     visit::{Visitable, Visitor},
// };
use aspect::Adtishness;
use unparse::Unparse;
use visit::{
    Visit,
    visiteventsink::{AspectVisitor, PopOrProceed, VisitEventSink},
};

pub mod tmfscore;

pub struct Unparser<'arena, L> {
    pub unparse: Unparse<'arena>,
    phantom: std::marker::PhantomData<L>,
}

pub fn unparse<LWord, L, T, Heap>(heap: &Heap, t: &T) -> String
where
    LWord: Adtishness<VisitationAspect>,
    for<'a> Unparser<'a, L>: Visit<LWord, L, T, Heap, <LWord as Adtishness<VisitationAspect>>::X>,
{
    let arena = bumpalo::Bump::new();
    let mut unparser = Unparser::new(&arena);
    <Unparser<'_, L> as Visit<_, _, _, _, _>>::visit(&mut unparser, heap, t);
    format!("{}", &unparser.unparse)
}

impl<'arena, L> Unparser<'arena, L> {
    pub fn new(arena: &'arena bumpalo::Bump) -> Self {
        Unparser {
            unparse: Unparse::new(arena),
            phantom: std::marker::PhantomData,
        }
    }
}

impl<'arena, L> AspectVisitor for Unparser<'arena, L> {
    type A = VisitationAspect;
}

impl<'arena, L, CurrentNode, Heap> VisitEventSink<CurrentNode, Heap> for Unparser<'arena, L>
where
    CurrentNode: words::Implements<Heap, L, VisitationAspect>,
    <CurrentNode as words::Implements<Heap, L, VisitationAspect>>::LWord: NamesParseLL,
{
    fn push(
        &mut self,
        _heap: &Heap,
        _t: &CurrentNode,
        _total: u32,
    ) -> visit::visiteventsink::PopOrProceed {
        self.unparse.consistent_group_start();
        let start = <<CurrentNode as words::Implements<Heap, L, VisitationAspect>>::LWord as NamesParseLL>::START;
        for kw in start.0 {
            self.unparse.static_text(kw.get());
        }
        PopOrProceed::Proceed
    }

    fn proceed(&mut self, _idx: u32, _total: u32) -> visit::visiteventsink::PopOrProceed {
        // todo
        PopOrProceed::Proceed
    }

    fn pop(&mut self, _total: u32) {
        self.unparse.group_end();
    }

    fn deconstruction_failure(&mut self) {
        self.unparse.static_text("???");
    }
}
