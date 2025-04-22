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

impl<Heap, Pmsp, Amc> term::co_visit::CoVisitable<Parser<'_, Amc>, Pmsp, Heap>
    for tymetafuncspec_core::Either<
        Heap,
        Pair<Heap, BoundedNat<Heap>, Maybe<Heap, std_parse_metadata::ParseMetadata<Heap>>>,
        std_parse_error::ParseError<Heap>,
    >
{
    fn co_visit(visitor: &mut Parser<'_, Amc>, _: &mut Heap) -> Self {
        let next_unicode_word = parse::unicode_segmentation::UnicodeSegmentation::unicode_words(
            &visitor.source[visitor.position.offset()..],
        )
        .next()
        .ok_or(parse::ParseError::UnexpectedEndOfInput(
            visitor.position.into(),
        ));
        return_if_err!(next_unicode_word);
        let n = next_unicode_word
            .parse::<usize>()
            .map_err(|_| parse::ParseError::TmfsParseFailure(visitor.position.into()));
        return_if_err!(n);
        let previous_offset = visitor.position;
        visitor.position =
            parse::miette::SourceOffset::from(visitor.position.offset() + next_unicode_word.len());
        cstfy_ok(BoundedNat::new(n), previous_offset, visitor.position)
    }
}
// fn parse(
//     source: &str,
//     initial_offset: parse::miette::SourceOffset,
//     heap: &mut Self::Heap,
//     errors: &mut Vec<parse::ParseError>,
// ) -> (
//     Either<
//         Heap,
//         Pair<Heap, Set<Heap, Elem>, Maybe<Heap, std_parse_metadata::ParseMetadata<Heap>>>,
//         std_parse_error::ParseError<Heap>,
//     >,
//     parse::miette::SourceOffset,
// ) {
//     let mut items = Vec::new();
//     let mut offset = initial_offset;
//     while !source[offset.offset()..].starts_with('}') {
//         let (item, new_offset) =
//             <Elem as parse::Parse<Heap>>::parse(source, offset, heap, errors);
//         items.push(item);
//         offset = new_offset;
//         if source[offset.offset()..].starts_with(',') {
//             offset = parse::miette::SourceOffset::from(offset.offset() + 1);
//         }
//     }
//     (
//         cstfy_ok(
//             Set {
//                 items: todo!(),
//                 heap: std::marker::PhantomData,
//             },
//             initial_offset,
//             offset,
//         ),
//         offset,
//     )
// }

impl<'a, Heap, Elem, Pmsp, Amc> term::co_visit::CoVisitable<Parser<'a, Amc>, Pmsp, Heap>
    for Cstfy<Heap, Set<Heap, Elem>>
where
    //     Elem: for<'b> term::co_visit::CoVisitable<Parser<'b>, Pmsp, Heap>,
    Heap: term::SuperHeap<SetHeapBak<Heap, Elem>>,
{
    fn co_visit(visitor: &mut Parser<'_, Amc>, heap: &mut Heap) -> Self {
        todo!()
        // let mut items = Vec::new();
        // let initial_offset = visitor.position;

        // while !visitor.source[visitor.position.offset()..].starts_with('}') {
        //     let item = Elem::co_visit(visitor, heap);
        //     items.push(item);

        //     if visitor.source[visitor.position.offset()..].starts_with(',') {
        //         visitor.position = parse::miette::SourceOffset::from(visitor.position.offset() + 1);
        //     }
        // }

        // let final_offset = visitor.position;
        // cstfy_ok(Set::new(heap, items), initial_offset, final_offset)
    }
}

impl<'a, Heap, Elem, Pmsp, Amc> term::co_visit::CoVisitable<Parser<'a, Amc>, Pmsp, Heap>
    for Cstfy<Heap, IdxBox<Heap, Elem>>
where
    // Elem: for<'b> term::co_visit::CoVisitable<Parser<'b, Amc>, Pmsp, Heap>,
    Heap: term::SuperHeap<IdxBoxHeapBak<Heap, Elem>>,
{
    fn co_visit(visitor: &mut Parser<'_, Amc>, heap: &mut Heap) -> Self {
        // let initial_offset = visitor.position;
        // let item = Elem::co_visit(visitor, heap);
        // let final_offset = visitor.position;
        // cstfy_ok(IdxBox::new(heap, item), initial_offset, final_offset)
        todo!()
    }
}

impl<Heap> Lookahead for BoundedNat<Heap> {}
impl<Heap, Elem> Lookahead for IdxBox<Heap, Elem> {}
impl<Heap, Elem> Lookahead for Set<Heap, Elem> {}

// impl<Heap, Elem> ParseLL for IdxBox<Heap, Elem> {
//     const START: KeywordSequence;

//     const PROCEED: &'static [KeywordSequence];

//     const END: KeywordSequence;
// }

// impl<Heap> IsSoleCaseHack for BoundedNat<Heap> {}
// impl<Heap, Elem> IsSoleCaseHack for IdxBox<Heap, Elem> {}
// impl<Heap, Elem> IsSoleCaseHack for Set<Heap, Elem> {}
