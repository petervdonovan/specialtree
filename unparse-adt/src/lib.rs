use parse_adt::NamesParseLL;
use pmsp::UsesStrategyForTraversal;
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

pub struct Unparser<'arena, L, CurrentNode> {
    pub unparse: Unparse<'arena>,
    phantom: std::marker::PhantomData<(CurrentNode, L)>,
}

impl<'arena, L, Heap, EitherLeft, EitherRight>
    UsesStrategyForTraversal<
        Unparser<'arena, L, tymetafuncspec_core::Either<Heap, EitherLeft, EitherRight>>,
    > for tymetafuncspec_core::Either<Heap, EitherLeft, EitherRight>
{
}

pub fn unparse<Heap, L, T>(heap: &mut Heap, t: &T) -> String
where
    for<'a> Unparser<'a, L, T>: Visit<T, Heap, L>,
{
    let arena = bumpalo::Bump::new();
    let mut unparser = Unparser::new(&arena);
    <Unparser<'_, L, T> as Visit<T, Heap, L>>::visit(&mut unparser, heap, &t);
    format!("{:?}", &unparser.unparse)
}

impl<'arena, L, CurrentNode> Unparser<'arena, L, CurrentNode> {
    pub fn new(arena: &'arena bumpalo::Bump) -> Self {
        Unparser {
            unparse: Unparse::new(arena),
            phantom: std::marker::PhantomData,
        }
    }
}

impl<'arena, L, CurrentNode, Heap> VisitEventSink<CurrentNode, Heap>
    for Unparser<'arena, L, CurrentNode>
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

// impl<'arena, L, C> HasBorrowedHeapRef for Unparser<'arena, L, C> {
//     type Borrowed<'a, Heap: 'a> = &'a Heap;
//     fn with_decayed_heapref<Heap, T, F: Fn(&Heap) -> T>(
//         heap: &mut Self::Borrowed<'_, Heap>,
//         f: F,
//     ) -> T {
//         f(heap)
//     }
// }

// impl<'a, Heap, L, CurrentNode> Visitor<Heap, CurrentNode> for Unparser<'a, L, CurrentNode>
// where
//     CurrentNode: words::Implements<Heap, L>,
//     <CurrentNode as words::Implements<Heap, L>>::LWord: NamesParseLL,
// {
//     fn push(
//         &mut self,
//         _t: CurrentNode,
//         _heap: &mut Self::Borrowed<'_, Heap>,
//     ) -> term::visit::MaybeAbortThisSubtree {
//         for word in <<CurrentNode as words::Implements<Heap, L>>::LWord as NamesParseLL>::START.0 {
//             self.unparse.static_text(word.get());
//         }
//         term::visit::MaybeAbortThisSubtree::Proceed
//     }

//     fn proceed(&mut self, _t: CurrentNode, _heap: &mut Self::Borrowed<'_, Heap>) {
//         todo!()
//     }

//     fn pop(&mut self, _t: CurrentNode, _heap: &mut Self::Borrowed<'_, Heap>) {
//         todo!()
//     }
// }

// // impl<'a, Heap, L, Pmsp, CurrentNode, Fnlut>
// //     Visitable<Unparser<'a, L, CurrentNode>, Pmsp, Heap, typenum::U0, Fnlut> for CurrentNode
// // where
// //     CurrentNode: words::Implements<Heap, L>,
// //     <CurrentNode as words::Implements<Heap, L>>::LWord: NamesParseLL,
// //     Fnlut: HasFn<Self, FnType = fn(&mut Unparser<'a, L, CurrentNode>, &mut Heap, Fnlut) -> Self>,
// // {
// //     fn visit(&self, visitor: &mut Unparser<'_, L, CurrentNode>, heap: &mut Heap, fnlut: Fnlut) {
// //         println!("dbg: recursion limit reached for unparse; restarting");
// //         fnlut.get::<Self>()(visitor, heap, fnlut)
// //     }
// // }
