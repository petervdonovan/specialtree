use ccf::CanonicallyConstructibleFrom;
use covisit::Covisit;
use parse_adt::{
    Lookahead, ParseCursor, Parser,
    cstfy::{Cstfy, cstfy_ok},
};
use pmsp::TmfMetadata;
use term::SuperHeap;

use crate::{OrVariable, OrVariableHeapBak, OrVariableZeroOrMore, OrVariableZeroOrMoreHeapBak};

impl<'a, Heap, L, MatchedTy, MatchedTyTmfMetadata, OvMapped>
    Covisit<
        TmfMetadata<OrVariable<Heap, MatchedTy>, (MatchedTyTmfMetadata, ())>,
        Cstfy<Heap, OvMapped>,
        Heap,
        L,
    > for Parser<'a, L>
where
    // Heap: SuperHeap<OrVariableHeapBak<Heap, MatchedTy>>,
    Parser<'a, L>: Covisit<MatchedTyTmfMetadata, MatchedTy, Heap, L>,
    OvMapped: CanonicallyConstructibleFrom<Heap, (OrVariable<Heap, MatchedTy>, ())>,
{
    fn covisit(&mut self, heap: &mut Heap) -> Cstfy<Heap, OvMapped> {
        // let previous_offset = self.pc.position;
        // let next_word = self.pc.pop_word().unwrap();
        // let ov = if next_word == "$" {
        //     let next = self.pc.pop_word().unwrap();
        //     let subheap = heap.subheap_mut::<OrVariableHeapBak<_, _>>();
        //     let intern = subheap.names.get_or_intern(next);
        //     OrVariable::Variable { name: intern }
        // } else if next_word == "_" {
        //     OrVariable::Ignored(std::marker::PhantomData)
        // } else {
        //     OrVariable::Ctor(self.covisit(heap))
        // };
        // cstfy_ok(
        //     OvMapped::construct(heap, (ov, ())),
        //     previous_offset,
        //     self.pc.position,
        // )
        todo!()
    }
}

impl<'a, Heap, L, MatchedTy, MatchedTyTmfMetadata, OvZomMapped>
    Covisit<
        TmfMetadata<OrVariableZeroOrMore<Heap, MatchedTy>, (MatchedTyTmfMetadata, ())>,
        Cstfy<Heap, OvZomMapped>,
        Heap,
        L,
    > for Parser<'a, L>
where
    Heap: SuperHeap<OrVariableZeroOrMoreHeapBak<Heap, MatchedTy>>,
    Parser<'a, L>: Covisit<MatchedTyTmfMetadata, MatchedTy, Heap, L>,
    OvZomMapped: CanonicallyConstructibleFrom<Heap, (OrVariableZeroOrMore<Heap, MatchedTy>, ())>,
{
    fn covisit(&mut self, heap: &mut Heap) -> Cstfy<Heap, OvZomMapped> {
        let previous_offset = self.pc.position;
        let next_word = self.pc.pop_word().unwrap();
        let ovzom = if next_word == "." {
            while self.pc.peek_words().next().unwrap().1 == "." {
                self.pc.pop_word();
            }
            let next = self.pc.pop_word().unwrap();
            let subheap = heap.subheap_mut::<OrVariableZeroOrMoreHeapBak<_, _>>();
            let intern = subheap.names.get_or_intern(next);
            OrVariableZeroOrMore::ZeroOrMore { name: intern }
        } else if next_word == "$" {
            let next = self.pc.pop_word().unwrap();
            let subheap = heap.subheap_mut::<OrVariableZeroOrMoreHeapBak<_, _>>();
            let intern = subheap.names.get_or_intern(next);
            OrVariableZeroOrMore::Variable { name: intern }
        } else if next_word == "_" {
            OrVariableZeroOrMore::Ignored(std::marker::PhantomData)
        } else {
            OrVariableZeroOrMore::Ctor(self.covisit(heap))
        };
        cstfy_ok(
            OvZomMapped::construct(heap, (ovzom, ())),
            previous_offset,
            self.pc.position,
        )
    }
}

impl<Heap, L, Elem> Lookahead<Heap, L> for OrVariable<Heap, Elem> {
    fn matches(_: &ParseCursor<'_>) -> bool {
        true // OK because we must be in a single-possibility context
    }
}
impl<Heap, L, Elem> Lookahead<Heap, L> for OrVariableZeroOrMore<Heap, Elem> {
    fn matches(_: &ParseCursor<'_>) -> bool {
        true // OK because we must be in a single-possibility context
    }
}
