use core::panic;

use covisit::Covisit;
use pmsp::TmfMetadata;
use tymetafuncspec_core::{BoundedNat, IdxBox, IdxBoxHeapBak, Set, SetHeapBak};

use crate::{
    Lookahead, ParseCursor, Parser,
    cstfy::{Cstfy, cstfy_ok},
    return_if_err,
};

impl<Heap, L> Covisit<(TmfMetadata<BoundedNat<Heap>>, ()), Cstfy<Heap, BoundedNat<Heap>>, Heap, L>
    for Parser<'_, L>
{
    fn covisit(&mut self, _: &mut Heap) -> Cstfy<Heap, BoundedNat<Heap>> {
        let previous_offset = self.pc.position;
        let next_word = self.pc.pop_word();
        let n = next_word
            .unwrap()
            .parse::<usize>()
            .map_err(|_| parse::ParseError::TmfsParseFailure(self.pc.position.into()));
        return_if_err!(n);
        cstfy_ok(BoundedNat::new(n), previous_offset, self.pc.position)
    }
}

impl<'a, Heap, L, Elem, TyMetadata>
    Covisit<(TmfMetadata<Set<Heap, Elem>>, (TyMetadata, ())), Cstfy<Heap, Set<Heap, Elem>>, Heap, L>
    for Parser<'a, L>
where
    Heap: term::SuperHeap<SetHeapBak<Heap, Elem>>,
    Parser<'a, L>: Covisit<TyMetadata, Elem, Heap, L>,
{
    fn covisit(&mut self, heap: &mut Heap) -> Cstfy<Heap, Set<Heap, Elem>> {
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
            items.push(item);
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
        cstfy_ok(Set::new(heap, items), initial_offset, final_offset)
    }
}

impl<'a, Heap, L, Elem, TyMetadata>
    Covisit<
        (TmfMetadata<IdxBox<Heap, Elem>>, (TyMetadata, ())),
        Cstfy<Heap, IdxBox<Heap, Elem>>,
        Heap,
        L,
    > for Parser<'a, L>
where
    Elem: Lookahead<Heap, L>,
    Self: Covisit<TyMetadata, Elem, Heap, L>,
    Heap: term::SuperHeap<IdxBoxHeapBak<Heap, Elem>>,
{
    fn covisit(&mut self, heap: &mut Heap) -> Cstfy<Heap, IdxBox<Heap, Elem>> {
        let initial_offset = self.pc.position;
        let item = Self::covisit(self, heap);
        let final_offset = self.pc.position;
        cstfy_ok(IdxBox::new(heap, item), initial_offset, final_offset)
    }
}

impl<Heap, L> Lookahead<Heap, L> for BoundedNat<Heap> {
    fn matches(parser: &ParseCursor<'_>) -> bool {
        parser
            .peek_words()
            .next()
            .is_some_and(|word| word.1.parse::<usize>().is_ok())
    }
}
impl<Heap, L, Elem> Lookahead<Heap, L> for IdxBox<Heap, Cstfy<Heap, Elem>>
where
    Elem: Lookahead<Heap, L>,
{
    fn matches(parser: &ParseCursor<'_>) -> bool {
        println!("dbg: checking if matches Idxbox Cstfy");
        Elem::matches(parser)
    }
}
impl<Heap, L, Elem> Lookahead<Heap, L> for Set<Heap, Elem> {
    fn matches(parser: &ParseCursor<'_>) -> bool {
        println!("dbg: checking if matches Set");
        parser.peek_words().next().is_some_and(|word| word.1 == "{")
    }
}
