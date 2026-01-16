use core::panic;

use aspect::Adtishness;
use aspect::VisitationAspect;
use aspect::{AdtLike, NotAdtLike};
use ccf::CanonicallyConstructibleFrom;
use covisit::Covisit;
use tymetafuncspec_core::{BoundedNat, IdxBox, IdxBoxHeapBak, Set, SetHeapBak};
use words::InverseImplements;

use crate::CstFallibilityAspect;
use crate::LookaheadAspect;
use crate::{
    Lookahead, ParseCursor, Parser,
    cstfy::{Cstfy, cstfy_ok},
    return_if_err,
};

impl<Heap, L, MappedBNat> Covisit<BoundedNat<()>, L, Cstfy<Heap, MappedBNat>, Heap, NotAdtLike>
    for Parser<'_, L>
where
    MappedBNat: CanonicallyConstructibleFrom<Heap, (BoundedNat<Heap>, ())>,
{
    fn covisit(&mut self, heap: &mut Heap) -> Cstfy<Heap, MappedBNat> {
        let previous_offset = self.pc.position;
        let next_word = self.pc.pop_word();
        let n = next_word
            .unwrap()
            .parse::<usize>()
            .map_err(|_| parse::ParseError::TmfsParseFailure(self.pc.position.into()));
        return_if_err!(n);
        let mb = BoundedNat::new(n);
        cstfy_ok(
            MappedBNat::construct(heap, (mb, ())),
            previous_offset,
            self.pc.position,
        )
    }
}

// ((tymetafuncspec_core::Either<term_specialized_cst_autoboxed_pattern_fib::Heap, tymetafuncspec_core::Pair<term_specialized_cst_autoboxed_pattern_fib::Heap, pattern_tmf::OrVariable<term_specialized_cst_autoboxed_pattern_fib::Heap, tymetafuncspec_core::Either<term_specialized_cst_autoboxed_pattern_fib::Heap, tymetafuncspec_core::Pair<term_specialized_cst_autoboxed_pattern_fib::Heap, term_specialized_cst_autoboxed_pattern_fib::Nat, tymetafuncspec_core::Maybe<term_specialized_cst_autoboxed_pattern_fib::Heap, std_parse_metadata::ParseMetadata<term_specialized_cst_autoboxed_pattern_fib::Heap>>>, std_parse_error::ParseError<term_specialized_cst_autoboxed_pattern_fib::Heap>>>, tymetafuncspec_core::Maybe<term_specialized_cst_autoboxed_pattern_fib::Heap, std_parse_metadata::ParseMetadata<term_specialized_cst_autoboxed_pattern_fib::Heap>>>, std_parse_error::ParseError<term_specialized_cst_autoboxed_pattern_fib::Heap>>, ()), ())

impl<'a, Heap, L, ElemLWord, MappedSet, MappedElem>
    Covisit<Set<(), ElemLWord>, L, Cstfy<Heap, MappedSet>, Heap, NotAdtLike> for Parser<'a, L>
where
    // Heap: InverseImplements<L, ElemLWord, LookaheadAspect>,
    Heap: InverseImplements<L, ElemLWord, VisitationAspect>,
    Heap: InverseImplements<L, ElemLWord, CstFallibilityAspect>,
    MappedElem: CanonicallyConstructibleFrom<
            Heap,
            (
                <Heap as InverseImplements<L, ElemLWord, CstFallibilityAspect>>::Implementor,
                (),
            ),
        >,
    Heap: InverseImplements<
            L,
            Set<(), ElemLWord>,
            VisitationAspect,
            Implementor = Set<Heap, MappedElem>,
        >,
    Heap: term::SuperHeap<SetHeapBak<Heap, MappedElem>>,
    ElemLWord: Adtishness<VisitationAspect>,
    Parser<'a, L>: Covisit<
            ElemLWord,
            L,
            <Heap as InverseImplements<L, ElemLWord, CstFallibilityAspect>>::Implementor,
            Heap,
            <ElemLWord as Adtishness<VisitationAspect>>::X,
        >,
    MappedSet: CanonicallyConstructibleFrom<Heap, (Set<Heap, MappedElem>, ())>,
    MappedElem: CanonicallyConstructibleFrom<
            Heap,
            (
                <Heap as InverseImplements<L, ElemLWord, VisitationAspect>>::Implementor,
                (),
            ),
        >,
{
    fn covisit(&mut self, heap: &mut Heap) -> Cstfy<Heap, MappedSet> {
        let mut items = Vec::new();
        let initial_offset = self.pc.position;
        match self.pc.pop_word() {
            Some("{") => {}
            Some(word) => {
                panic!("Unexpected word: got {word} when expecting {{");
            }
            None => {
                todo!();
            }
        }
        loop {
            println!("dbg: loop");
            let item = Self::covisit(self, heap);
            items.push(
                <MappedElem as CanonicallyConstructibleFrom<_, _>>::construct(heap, (item, ())),
            );
            match self.pc.pop_word() {
                Some("}") => break,
                Some(",") => {}
                Some(word) => {
                    panic!("Unexpected word: {word}");
                }
                None => {
                    todo!();
                }
            }
        }

        let final_offset = self.pc.position;
        let set = Set::new(heap, items);
        cstfy_ok(
            MappedSet::construct(heap, (set, ())),
            initial_offset,
            final_offset,
        )
        // todo!()
    }
}

// impl<'a, Heap, L, Elem, TyMetadata, MappedIdxBox>
//     Covisit<TmfMetadata<IdxBox<Heap, Elem>, (TyMetadata, ())>, Cstfy<Heap, MappedIdxBox>, Heap, L>
//     for Parser<'a, L>
// where
//     Elem: Lookahead<Heap, L>,
//     Self: Covisit<TyMetadata, Elem, Heap, L>,
//     Heap: term::SuperHeap<IdxBoxHeapBak<Heap, Elem>>,
//     MappedIdxBox: CanonicallyConstructibleFrom<Heap, (IdxBox<Heap, Elem>, ())>,
// {
//     fn covisit(&mut self, heap: &mut Heap) -> Cstfy<Heap, MappedIdxBox> {
//         let initial_offset = self.pc.position;
//         let item = Self::covisit(self, heap);
//         let final_offset = self.pc.position;
//         let ib = IdxBox::new(heap, item);
//         cstfy_ok(
//             MappedIdxBox::construct(heap, (ib, ())),
//             initial_offset,
//             final_offset,
//         )
//     }
// }

impl Adtishness<LookaheadAspect> for BoundedNat<()> {
    type X = NotAdtLike;
}
impl<ElemLWord> Adtishness<LookaheadAspect> for IdxBox<(), ElemLWord> {
    type X = NotAdtLike;
}
impl<ElemLWord> Adtishness<LookaheadAspect> for Set<(), ElemLWord> {
    type X = NotAdtLike;
}

impl Lookahead<NotAdtLike> for BoundedNat<()> {
    fn matches(parser: &ParseCursor<'_>) -> bool {
        parser
            .peek_words()
            .next()
            .is_some_and(|word| word.1.parse::<usize>().is_ok())
    }
}
impl<ElemLWord> Lookahead<NotAdtLike> for IdxBox<(), ElemLWord>
where
    ElemLWord: Lookahead<AdtLike>, // todo
{
    fn matches(parser: &ParseCursor<'_>) -> bool {
        println!("dbg: checking if matches Idxbox Cstfy");
        ElemLWord::matches(parser)
    }
}
impl<ElemLWord> Lookahead<NotAdtLike> for Set<(), ElemLWord> {
    fn matches(parser: &ParseCursor<'_>) -> bool {
        println!("dbg: checking if matches Set");
        parser.peek_words().next().is_some_and(|word| word.1 == "{")
    }
}
