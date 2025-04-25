use core::panic;

use parse::{Keyword, KeywordSequence};
use tymetafuncspec_core::{BoundedNat, IdxBox, IdxBoxHeapBak, Maybe, Pair, Set, SetHeapBak};

use crate::{
    cstfy::{cstfy_ok, Cstfy, CstfyTransparent},
    return_if_err, Lookahead, ParseLL, Parser,
};

// impl<Heap, L, R> !term::case_split::Adt for tymetafuncspec_core::Either<Heap, L, R> {}

// impl<Heap> crate::ParseLL for BoundedNat<Heap> {
//     const START: KeywordSequence = KeywordSequence(&[Keyword::new("BoundedNat")]);
//     const PROCEED: &'static [KeywordSequence] = &[];
//     const END: KeywordSequence = KeywordSequence(&[Keyword::new("}")]);
// }

// impl<Heap, Elem> crate::ParseLL for IdxBox<Heap, Elem> {
//     const START: KeywordSequence = KeywordSequence(&[Keyword::new("BoundedNat")]);
//     const PROCEED: &'static [KeywordSequence] = &[];
//     const END: KeywordSequence = KeywordSequence(&[Keyword::new("}")]);
// }

// impl<Heap, Elem> crate::ParseLL for Set<Heap, Elem> {
//     const START: KeywordSequence = KeywordSequence(&[Keyword::new("BoundedNat")]);
//     const PROCEED: &'static [KeywordSequence] = &[];
//     const END: KeywordSequence = KeywordSequence(&[Keyword::new("}")]);
// }

impl<Heap, Pmsp, Amc, DepthFuel> term::co_visit::CoVisitable<Parser<'_, Amc>, Pmsp, Heap, DepthFuel>
    for tymetafuncspec_core::Either<
        Heap,
        Pair<Heap, BoundedNat<Heap>, Maybe<Heap, std_parse_metadata::ParseMetadata<Heap>>>,
        std_parse_error::ParseError<Heap>,
    >
{
    fn co_visit(visitor: &mut Parser<'_, Amc>, _: &mut Heap) -> Self {
        // let next_unicode_word = parse::unicode_segmentation::UnicodeSegmentation::unicode_words(
        //     &visitor.source[visitor.position.offset()..],
        // )
        // .next()
        // .ok_or(parse::ParseError::UnexpectedEndOfInput(
        //     visitor.position.into(),
        // ));
        // return_if_err!(next_unicode_word);
        // let n = next_unicode_word
        //     .parse::<usize>()
        //     .map_err(|_| parse::ParseError::TmfsParseFailure(visitor.position.into()));
        // return_if_err!(n);
        let previous_offset = visitor.position;
        // visitor.position =
        //     parse::miette::SourceOffset::from(visitor.position.offset() + next_unicode_word.len());
        let next_word = visitor.pop_word();
        let n = next_word
            .unwrap()
            .parse::<usize>()
            .map_err(|_| parse::ParseError::TmfsParseFailure(visitor.position.into()));
        return_if_err!(n);
        cstfy_ok(BoundedNat::new(n), previous_offset, visitor.position)
    }
}

impl<'a, Heap, Elem, Pmsp, DepthFuelUpperBits, DepthFuelLastBit>
    term::co_visit::CoVisitable<
        Parser<'a, ()>,
        Pmsp,
        Heap,
        typenum::UInt<DepthFuelUpperBits, DepthFuelLastBit>,
    > for Cstfy<Heap, Set<Heap, Elem>>
where
    typenum::UInt<DepthFuelUpperBits, DepthFuelLastBit>: std::ops::Sub<typenum::B1>,
    Elem: for<'b> term::co_visit::CoVisitable<
        Parser<'b, ()>,
        Pmsp,
        Heap,
        typenum::Sub1<typenum::UInt<DepthFuelUpperBits, DepthFuelLastBit>>,
    >,
    Heap: term::SuperHeap<SetHeapBak<Heap, Elem>>,
{
    fn co_visit(visitor: &mut Parser<'_, ()>, heap: &mut Heap) -> Self {
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
            let item = Elem::co_visit(visitor, heap);
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

impl<'a, Heap, Elem, Pmsp, DepthFuelUpperBits, DepthFuelLastBit>
    term::co_visit::CoVisitable<
        Parser<'a, ()>,
        Pmsp,
        Heap,
        typenum::UInt<DepthFuelUpperBits, DepthFuelLastBit>,
    > for Cstfy<Heap, IdxBox<Heap, Elem>>
where
    typenum::UInt<DepthFuelUpperBits, DepthFuelLastBit>: std::ops::Sub<typenum::B1>,
    Elem: for<'b> term::co_visit::CoVisitable<
        Parser<'b, ()>,
        Pmsp,
        Heap,
        typenum::Sub1<typenum::UInt<DepthFuelUpperBits, DepthFuelLastBit>>,
    >,
    Heap: term::SuperHeap<IdxBoxHeapBak<Heap, Elem>>,
{
    fn co_visit(visitor: &mut Parser<'_, ()>, heap: &mut Heap) -> Self {
        let initial_offset = visitor.position;
        let item = Elem::co_visit(visitor, heap);
        let final_offset = visitor.position;
        cstfy_ok(IdxBox::new(heap, item), initial_offset, final_offset)
        // todo!()
    }
}

/////////////////////////////////////////////////////

impl<'a, Heap, Elem, Pmsp> term::co_visit::CoVisitable<Parser<'a, ()>, Pmsp, Heap, typenum::U0>
    for Cstfy<Heap, Set<Heap, Elem>>
where
    Heap: term::SuperHeap<SetHeapBak<Heap, Elem>>,
{
    fn co_visit(visitor: &mut Parser<'_, ()>, _heap: &mut Heap) -> Self {
        tymetafuncspec_core::Either::Right(
            std_parse_error::ParseError::new(parse::ParseError::RecursionLimitExceeded(
                visitor.position.into(),
            )),
            std::marker::PhantomData,
        )
    }
}

impl<'a, Heap, Elem, Pmsp> term::co_visit::CoVisitable<Parser<'a, ()>, Pmsp, Heap, typenum::U0>
    for Cstfy<Heap, IdxBox<Heap, Elem>>
where
    Heap: term::SuperHeap<IdxBoxHeapBak<Heap, Elem>>,
{
    fn co_visit(visitor: &mut Parser<'_, ()>, _heap: &mut Heap) -> Self {
        tymetafuncspec_core::Either::Right(
            std_parse_error::ParseError::new(parse::ParseError::RecursionLimitExceeded(
                visitor.position.into(),
            )),
            std::marker::PhantomData,
        )
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
        Elem::matches(parser)
    }
}
impl<Heap, Elem> Lookahead for Set<Heap, Elem> {
    fn matches<T>(parser: &Parser<'_, T>) -> bool {
        parser.peek_words().next().is_some_and(|word| word.1 == "{")
    }
}
