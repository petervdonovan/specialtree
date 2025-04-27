use core::panic;

use tymetafuncspec_core::{BoundedNat, IdxBox, IdxBoxHeapBak, Maybe, Pair, Set, SetHeapBak};

use crate::{
    Lookahead, Parser,
    cstfy::{Cstfy, CstfyTransparent, cstfy_ok},
    return_if_err,
};

impl<Heap, Pmsp, Amc, DepthFuelUpperBits, DepthFuelLastBit, Fnlut>
    term::co_visit::CoVisitable<
        Parser<'_, Amc>,
        Pmsp,
        Heap,
        typenum::UInt<DepthFuelUpperBits, DepthFuelLastBit>,
        Fnlut,
    >
    for tymetafuncspec_core::Either<
        Heap,
        Pair<Heap, BoundedNat<Heap>, Maybe<Heap, std_parse_metadata::ParseMetadata<Heap>>>,
        std_parse_error::ParseError<Heap>,
    >
{
    fn co_visit(visitor: &mut Parser<'_, Amc>, _: &mut Heap, _: Fnlut) -> Self {
        let previous_offset = visitor.position;
        let next_word = visitor.pop_word();
        let n = next_word
            .unwrap()
            .parse::<usize>()
            .map_err(|_| parse::ParseError::TmfsParseFailure(visitor.position.into()));
        return_if_err!(n);
        cstfy_ok(BoundedNat::new(n), previous_offset, visitor.position)
    }
}

impl<'a, Heap, Elem, Pmsp, DepthFuelUpperBits, DepthFuelLastBit, Fnlut: Copy>
    term::co_visit::CoVisitable<
        Parser<'a, ()>,
        Pmsp,
        Heap,
        typenum::UInt<DepthFuelUpperBits, DepthFuelLastBit>,
        Fnlut,
    > for Cstfy<Heap, Set<Heap, Elem>>
where
    typenum::UInt<DepthFuelUpperBits, DepthFuelLastBit>: std::ops::Sub<typenum::B1>,
    Elem: term::co_visit::CoVisitable<
            Parser<'a, ()>,
            Pmsp,
            Heap,
            typenum::Sub1<typenum::UInt<DepthFuelUpperBits, DepthFuelLastBit>>,
            Fnlut,
        >,
    Heap: term::SuperHeap<SetHeapBak<Heap, Elem>>,
{
    fn co_visit(visitor: &mut Parser<'a, ()>, heap: &mut Heap, fnlut: Fnlut) -> Self {
        let mut items = Vec::new();
        let initial_offset = visitor.position;
        dbg!(initial_offset);
        match visitor.pop_word() {
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
            let item = Elem::co_visit(visitor, heap, fnlut);
            items.push(item);
            match visitor.pop_word() {
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

        let final_offset = visitor.position;
        cstfy_ok(Set::new(heap, items), initial_offset, final_offset)
    }
}

impl<'a, Heap, Elem, Pmsp, DepthFuelUpperBits, DepthFuelLastBit, Fnlut>
    term::co_visit::CoVisitable<
        Parser<'a, ()>,
        Pmsp,
        Heap,
        typenum::UInt<DepthFuelUpperBits, DepthFuelLastBit>,
        Fnlut,
    > for Cstfy<Heap, IdxBox<Heap, Cstfy<Heap, Elem>>>
where
    typenum::UInt<DepthFuelUpperBits, DepthFuelLastBit>: std::ops::Sub<typenum::B1>,
    Elem: Lookahead,
    Cstfy<Heap, Elem>: term::co_visit::CoVisitable<
            Parser<'a, ()>,
            Pmsp,
            Heap,
            typenum::Sub1<typenum::UInt<DepthFuelUpperBits, DepthFuelLastBit>>,
            Fnlut,
        >,
    Heap: term::SuperHeap<IdxBoxHeapBak<Heap, Cstfy<Heap, Elem>>>,
{
    fn co_visit(visitor: &mut Parser<'a, ()>, heap: &mut Heap, fnlut: Fnlut) -> Self {
        let initial_offset = visitor.position;
        let item = Cstfy::<Heap, Elem>::co_visit(visitor, heap, fnlut);
        let final_offset = visitor.position;
        cstfy_ok(IdxBox::new(heap, item), initial_offset, final_offset)
    }
}

impl<'a, Heap, Elem, Pmsp, DepthFuelUpperBits, DepthFuelLastBit, Fnlut>
    term::co_visit::CoVisitable<
        Parser<'a, ()>,
        Pmsp,
        Heap,
        typenum::UInt<DepthFuelUpperBits, DepthFuelLastBit>,
        Fnlut,
    > for Cstfy<Heap, IdxBox<Heap, Elem>>
where
    typenum::UInt<DepthFuelUpperBits, DepthFuelLastBit>: std::ops::Sub<typenum::B1>,
    Elem: Lookahead,
    Cstfy<Heap, Elem>: term::co_visit::CoVisitable<
            Parser<'a, ()>,
            Pmsp,
            Heap,
            typenum::Sub1<typenum::UInt<DepthFuelUpperBits, DepthFuelLastBit>>,
            Fnlut,
        >,
    Heap: term::SuperHeap<IdxBoxHeapBak<Heap, Elem>>,
{
    fn co_visit(visitor: &mut Parser<'a, ()>, heap: &mut Heap, fnlut: Fnlut) -> Self {
        let item = Cstfy::<Heap, Elem>::co_visit(visitor, heap, fnlut);
        item.fmap_l_fnmut(|p| p.fmap_l_fnmut(|elem| IdxBox::new(heap, elem)))
    }
}

impl<Heap> Lookahead for BoundedNat<Heap> {
    fn matches<T>(parser: &Parser<'_, T>) -> bool {
        parser
            .peek_words()
            .next()
            .is_some_and(|word| word.1.parse::<usize>().is_ok())
    }
}
impl<Heap, Elem> Lookahead for IdxBox<Heap, Cstfy<Heap, Elem>>
where
    Elem: Lookahead,
{
    fn matches<T>(parser: &Parser<'_, T>) -> bool {
        println!("dbg: checking if matches Idxbox Cstfy");
        Elem::matches(parser)
    }
}
impl<Heap, Elem> Lookahead for IdxBox<Heap, CstfyTransparent<Heap, Elem>>
where
    Elem: Lookahead,
{
    fn matches<T>(parser: &Parser<'_, T>) -> bool {
        println!("dbg: checking if matches Idxbox CstfyTransparent");
        Elem::matches(parser)
    }
}
impl<Heap, Elem> Lookahead for Set<Heap, Elem> {
    fn matches<T>(parser: &Parser<'_, T>) -> bool {
        println!("dbg: checking if matches Set");
        parser.peek_words().next().is_some_and(|word| word.1 == "{")
    }
}
