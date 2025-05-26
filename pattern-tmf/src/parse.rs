use ccf::CanonicallyConstructibleFrom;
use covisit::Covisit;
use parse_adt::{
    Lookahead, ParseCursor, Parser,
    cstfy::{Cstfy, cstfy_ok},
};
use pmsp::TmfMetadata;
use term::SuperHeap;

use crate::{
    NamedPattern, NamedPatternHeapBak, OrVariable, OrVariableHeapBak, OrVariableZeroOrMore,
    OrVariableZeroOrMoreHeapBak,
};

impl<'a, Heap, L, MatchedTy, MatchedTyTmfMetadata, OvMapped>
    Covisit<
        TmfMetadata<OrVariable<Heap, MatchedTy>, (MatchedTyTmfMetadata, ())>,
        Cstfy<Heap, OvMapped>,
        Heap,
        L,
    > for Parser<'a, L>
where
    Heap: SuperHeap<OrVariableHeapBak<Heap, MatchedTy>>,
    Parser<'a, L>: Covisit<MatchedTyTmfMetadata, MatchedTy, Heap, L>,
    OvMapped: CanonicallyConstructibleFrom<Heap, (OrVariable<Heap, MatchedTy>, ())>,
{
    fn covisit(&mut self, heap: &mut Heap) -> Cstfy<Heap, OvMapped> {
        let previous_offset = self.pc.position;
        let next_word = self.pc.peek_words().next().unwrap();
        let ov = if next_word.1 == "$" {
            self.pc.pop_word();
            let next = self.pc.pop_word().unwrap();
            let subheap = heap.subheap_mut::<OrVariableHeapBak<_, _>>();
            let intern = subheap.names.get_or_intern(next);
            OrVariable::Variable { name: intern }
        } else if next_word.1 == "_" {
            self.pc.pop_word();
            OrVariable::Ignored(std::marker::PhantomData)
        } else {
            println!("dbg: falling through in ov");
            OrVariable::Ctor(self.covisit(heap))
        };
        cstfy_ok(
            OvMapped::construct(heap, (ov, ())),
            previous_offset,
            self.pc.position,
        )
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
        let next_word = self.pc.peek_words().next().unwrap();
        let ovzom = if next_word.1 == "." {
            while self.pc.peek_words().next().unwrap().1 == "." {
                self.pc.pop_word();
            }
            let next = self.pc.pop_word().unwrap();
            let subheap = heap.subheap_mut::<OrVariableZeroOrMoreHeapBak<_, _>>();
            let intern = subheap.names.get_or_intern(next);
            OrVariableZeroOrMore::ZeroOrMore { name: intern }
        } else if next_word.1 == "$" {
            self.pc.pop_word();
            let next = self.pc.pop_word().unwrap();
            let subheap = heap.subheap_mut::<OrVariableZeroOrMoreHeapBak<_, _>>();
            let intern = subheap.names.get_or_intern(next);
            OrVariableZeroOrMore::Variable { name: intern }
        } else if next_word.1 == "_" {
            self.pc.pop_word();
            OrVariableZeroOrMore::Ignored(std::marker::PhantomData)
        } else {
            println!("dbg: falling through in ovzom");
            OrVariableZeroOrMore::Ctor(self.covisit(heap))
        };
        cstfy_ok(
            OvZomMapped::construct(heap, (ovzom, ())),
            previous_offset,
            self.pc.position,
        )
    }
}

impl<'a, Heap, L, Pattern, PatternTmfMetadata, NamedPatternMapped>
    Covisit<
        TmfMetadata<NamedPattern<Heap, Pattern>, (PatternTmfMetadata, ())>,
        Cstfy<Heap, NamedPatternMapped>,
        Heap,
        L,
    > for Parser<'a, L>
where
    Heap: SuperHeap<NamedPatternHeapBak<Heap, Pattern>>,
    Parser<'a, L>: Covisit<PatternTmfMetadata, Pattern, Heap, L>,
    NamedPatternMapped: CanonicallyConstructibleFrom<Heap, (NamedPattern<Heap, Pattern>, ())>,
{
    fn covisit(&mut self, heap: &mut Heap) -> Cstfy<Heap, NamedPatternMapped> {
        let previous_offset = self.pc.position;
        let next_word = self.pc.pop_word().unwrap();
        if next_word != "@" {
            panic!("expected @");
        }
        let next_word = self.pc.pop_word().unwrap();
        let subheap = heap.subheap_mut::<NamedPatternHeapBak<_, _>>();
        let intern = subheap.names.get_or_intern(next_word);
        let next_word = self.pc.pop_word().unwrap();
        if next_word != "=" {
            panic!("expected =");
        }
        let t = NamedPattern {
            pattern: self.covisit(heap),
            name: intern,
            phantom: Default::default(),
        };
        cstfy_ok(
            NamedPatternMapped::construct(heap, (t, ())),
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
impl<Heap, L, Elem> Lookahead<Heap, L> for NamedPattern<Heap, Cstfy<Heap, Elem>>
where
    Elem: Lookahead<Heap, L>,
{
    fn matches(pc: &ParseCursor<'_>) -> bool {
        let mut pc = *pc;
        while let Some(word) = pc.pop_word()
            && word != "="
        {}
        Elem::matches(&pc)
    }
}
