use ccf::{CanonicallyConstructibleFrom, VisitationInfo};
use covisit::Covisit;
use parse_adt::{
    Lookahead, ParseCursor, Parser,
    cstfy::{Cstfy, cstfy_ok},
};
use term::SuperHeap;
use words::{AdtLike, InverseImplements, NotAdtLike};

use crate::{
    NamedPattern, NamedPatternHeapBak, OrVariable, OrVariableHeapBak, OrVariableZeroOrMore,
    OrVariableZeroOrMoreHeapBak,
};

impl<'a, Heap, L, MatchedTy, MatchedTyLWord, OvMapped>
    Covisit<OrVariable<(), MatchedTyLWord>, L, Cstfy<Heap, OvMapped>, Heap, NotAdtLike>
    for Parser<'a, L>
where
    Heap: SuperHeap<OrVariableHeapBak<Heap, MatchedTy>>,
    Heap: InverseImplements<
            L,
            OrVariable<(), MatchedTyLWord>,
            ExternBehavioralImplementor = OrVariable<Heap, MatchedTy>,
        >,
    MatchedTyLWord: VisitationInfo,
    Parser<'a, L>: Covisit<
            MatchedTyLWord,
            L,
            MatchedTy,
            Heap,
            <MatchedTyLWord as VisitationInfo>::AdtLikeOrNot,
        >,
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

impl<'a, Heap, L, MatchedTy, MatchedTyLWord, OvZomMapped>
    Covisit<OrVariableZeroOrMore<(), MatchedTyLWord>, L, Cstfy<Heap, OvZomMapped>, Heap, NotAdtLike>
    for Parser<'a, L>
where
    Heap: SuperHeap<OrVariableZeroOrMoreHeapBak<Heap, MatchedTy>>,
    Heap: InverseImplements<
            L,
            OrVariable<(), MatchedTyLWord>,
            ExternBehavioralImplementor = OrVariableZeroOrMore<Heap, MatchedTy>,
        >,
    MatchedTyLWord: VisitationInfo,
    Parser<'a, L>: Covisit<
            MatchedTyLWord,
            L,
            MatchedTy,
            Heap,
            <MatchedTyLWord as VisitationInfo>::AdtLikeOrNot,
        >,
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

impl<'a, Heap, L, Pattern, PatternLWord, NamedPatternMapped>
    Covisit<NamedPattern<(), PatternLWord>, L, Cstfy<Heap, NamedPatternMapped>, Heap, NotAdtLike>
    for Parser<'a, L>
where
    Heap: SuperHeap<NamedPatternHeapBak<Heap, Pattern>>,
    Heap: InverseImplements<
            L,
            NamedPattern<(), PatternLWord>,
            ExternBehavioralImplementor = NamedPattern<Heap, Pattern>,
        >,
    PatternLWord: VisitationInfo,
    Parser<'a, L>:
        Covisit<PatternLWord, L, Pattern, Heap, <PatternLWord as VisitationInfo>::AdtLikeOrNot>,
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

impl<Elem> Lookahead<NotAdtLike> for OrVariable<(), Elem> {
    fn matches(_: &ParseCursor<'_>) -> bool {
        true // OK because we must be in a single-possibility context
    }
}
impl<Elem> Lookahead<NotAdtLike> for OrVariableZeroOrMore<(), Elem> {
    fn matches(_: &ParseCursor<'_>) -> bool {
        true // OK because we must be in a single-possibility context
    }
}
impl<ElemLWord> Lookahead<NotAdtLike> for NamedPattern<(), Cstfy<(), ElemLWord>>
where
    ElemLWord: Lookahead<AdtLike>,
{
    fn matches(pc: &ParseCursor<'_>) -> bool {
        let mut pc = *pc;
        while let Some(word) = pc.pop_word()
            && word != "="
        {}
        ElemLWord::matches(&pc)
    }
}
