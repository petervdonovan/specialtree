#![feature(never_type)]
use cstfy::Cstfy;
use parse::{KeywordSequence, UnexpectedTokenError};
use take_mut::Poisonable;
use term::{
    case_split::{AtLeastTwoConsList, ConsList, NonemptyConsList},
    co_visit::CoVisitor,
    fnlut::HasFn,
    select::{AcceptingCases, FromSelectCase, SelectCase},
};
use words::Implements;

pub mod cstfy;
mod tmfscore;

pub struct Parser<'a, L, AllCurrentCases> {
    source: &'a str,
    position: miette::SourceOffset,
    phantom: std::marker::PhantomData<(L, AllCurrentCases)>,
}
impl<'a, L, AllCurrentCases> Poisonable for Parser<'a, L, AllCurrentCases> {
    fn poisoned() -> Self {
        Self {
            source: "",
            position: miette::SourceOffset::from(usize::MAX),
            phantom: std::marker::PhantomData,
        }
    }
}
impl<'a, L, AllCurrentCases> Parser<'a, L, AllCurrentCases> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            position: miette::SourceOffset::from(0),
            phantom: std::marker::PhantomData,
        }
    }
    pub fn convert_to<To>(self) -> Parser<'a, L, To> {
        Parser {
            source: self.source,
            position: self.position,
            phantom: std::marker::PhantomData,
        }
    }
    pub fn peek_words(
        &self,
    ) -> impl Iterator<Item = (usize, &'a str)> + use<'a, AllCurrentCases, L> {
        unicode_segmentation::UnicodeSegmentation::split_word_bound_indices(
            &self.source[self.position.offset()..],
        )
        .filter(|it| !it.1.is_empty() && !it.1.chars().any(|c| c.is_whitespace()))
    }
    pub fn pop_word(&mut self) -> Option<&'a str> {
        let mut iter = self.peek_words();
        if let Some(word) = iter.next() {
            self.position = (self.position.offset() + word.0 + word.1.len()).into();
            Some(word.1)
        } else {
            None
        }
    }
    pub fn match_keywords(&self, keywords: &KeywordSequence) -> Option<miette::SourceOffset> {
        let mut keywords = keywords.0.iter();
        let mut last_offset = self.position;
        for (found, expected) in self.peek_words().zip(keywords.by_ref()) {
            if found.1 != expected.get() {
                return None;
            }
            last_offset = (self.position.offset() + found.0 + found.1.len()).into();
        }
        if keywords.next().is_some() {
            return None;
        }
        Some(last_offset)
    }
}

// pub trait ParseLL<Heap, L>: ParseLLImplementor {
//     const START: KeywordSequence;
//     const PROCEED: &'static [KeywordSequence];
//     const END: KeywordSequence;
// }

pub trait NamesParseLL {
    const START: KeywordSequence;
    const PROCEED: &'static [KeywordSequence];
    const END: KeywordSequence;
}

// impl<T, Heap, L> ParseLL<Heap, L> for T
// where
//     T: ParseLLImplementor,
//     Cstfy<Heap, T>: words::Implements<Heap, L>,
//     <Cstfy<Heap, T> as words::Implements<Heap, L>>::LWord: NamesParseLLFor<T>,
// {
//     const START: KeywordSequence =
//         <<Cstfy<Heap, T> as words::Implements<Heap, L>>::LWord as NamesParseLLFor<_>>::START;
//     const PROCEED: &'static [KeywordSequence] =
//         <<Cstfy<Heap, T> as words::Implements<Heap, L>>::LWord as NamesParseLLFor<_>>::PROCEED;
//     const END: KeywordSequence =
//         <<Cstfy<Heap, T> as words::Implements<Heap, L>>::LWord as NamesParseLLFor<_>>::END;
// }

// impl<T, Heap, L> ParseLL<Heap, L> for T
// where
//     T: ParseLLImplementor,
//     CstfyTransparent<Heap, T>: words::Implements<Heap, L>,
//     <CstfyTransparent<Heap, T> as words::Implements<Heap, L>>::LWord: NamesParseLLFor<T>,
// {
//     const START: KeywordSequence =
//         <<CstfyTransparent<Heap, T> as words::Implements<Heap, L>>::LWord as NamesParseLLFor<_>>::START;
//     const PROCEED: &'static [KeywordSequence] = <<CstfyTransparent<Heap, T> as words::Implements<
//         Heap,
//         L,
//     >>::LWord as NamesParseLLFor<_>>::PROCEED;
//     const END: KeywordSequence =
//         <<CstfyTransparent<Heap, T> as words::Implements<Heap, L>>::LWord as NamesParseLLFor<_>>::END;
// }

pub trait Lookahead<Heap, L> {
    fn matches<'a, T>(parser: &Parser<'a, L, T>) -> bool;
}
impl<T, Heap, L> Lookahead<Heap, L> for T
where
    // T: ParseLL<Heap, L> + LookaheadImplementor,
    T: term::case_split::Adt,
    Cstfy<Heap, T>: words::Implements<Heap, L>,
    <Cstfy<Heap, T> as words::Implements<Heap, L>>::LWord: NamesParseLL,
{
    fn matches<C>(parser: &Parser<'_, L, C>) -> bool {
        parser
            .match_keywords(
                &<<Cstfy<Heap, T> as words::Implements<Heap, L>>::LWord as NamesParseLL>::START,
            )
            .is_some()
    }
}
// impl<T, Heap, L> Lookahead<Heap, L> for T
// where
//     // T: ParseLL<Heap, L> + LookaheadImplementor,
//     T: term::case_split::Adt,
//     CstfyTransparent<Heap, T>: words::Implements<Heap, L>,
// {
//     fn matches<C>(parser: &Parser<'_, L, C>) -> bool {
//         parser.match_keywords(&Self::START).is_some()
//     }
// }

impl<'a, L> SelectCase for Parser<'a, L, ()> {
    type AC<CasesConsList: ConsList> = Parser<'a, L, CasesConsList>;

    fn start_cases<CasesConsList: ConsList>(self) -> Self::AC<CasesConsList> {
        Parser {
            source: self.source,
            position: self.position,
            phantom: std::marker::PhantomData,
        }
    }
}

impl<'a, L, Cases> FromSelectCase for Parser<'a, L, Cases> {
    type ShortCircuitsTo = Parser<'a, L, ()>;
}

impl<'a, L, AllCurrentCases> AcceptingCases<()> for Parser<'a, L, AllCurrentCases>
where
    AllCurrentCases: NonemptyConsList,
{
    type AcceptingRemainingCases = Self;

    fn try_case(self) -> Result<Self::ShortCircuitsTo, Self::AcceptingRemainingCases> {
        panic!()
    }
}

impl<'a, Heap, L, RemainingCasesUnwrappedCarCar, RemainingCasesCdr, AllCurrentCases>
    AcceptingCases<(
        (Cstfy<Heap, RemainingCasesUnwrappedCarCar>, ()),
        RemainingCasesCdr,
    )> for Parser<'a, L, AllCurrentCases>
where
    AllCurrentCases: AtLeastTwoConsList,
    RemainingCasesCdr: ConsList,
    Self: AcceptingCases<RemainingCasesCdr, ShortCircuitsTo = Parser<'a, L, ()>>,
    RemainingCasesUnwrappedCarCar: Lookahead<Heap, L>,
{
    type AcceptingRemainingCases = Self;

    fn try_case(self) -> Result<Self::ShortCircuitsTo, Self::AcceptingRemainingCases> {
        if RemainingCasesUnwrappedCarCar::matches(&self) {
            Ok(self.convert_to())
        } else {
            Err(self)
        }
    }
}

// impl<'a, Heap, L, RemainingCasesUnwrappedCarCar, RemainingCasesCdr, AllCurrentCases>
//     AcceptingCases<(
//         (CstfyTransparent<Heap, RemainingCasesUnwrappedCarCar>, ()),
//         RemainingCasesCdr,
//     )> for Parser<'a, L, AllCurrentCases>
// where
//     AllCurrentCases: AtLeastTwoConsList,
//     RemainingCasesCdr: ConsList,
//     Self: AcceptingCases<RemainingCasesCdr, ShortCircuitsTo = Parser<'a, L, ()>>,
//     RemainingCasesUnwrappedCarCar: Lookahead<Heap, L>,
// {
//     type AcceptingRemainingCases = Self;

//     fn try_case(self) -> Result<Self::ShortCircuitsTo, Self::AcceptingRemainingCases> {
//         todo!()
//     }
// }

impl<'a, L, Car> AcceptingCases<(Car, ())> for Parser<'a, L, (Car, ())> {
    type AcceptingRemainingCases = Self;

    fn try_case(self) -> Result<Self::ShortCircuitsTo, Self::AcceptingRemainingCases> {
        println!("assuming the only case at: {:?}", self.position);
        Ok(Parser {
            source: self.source,
            position: self.position,
            phantom: std::marker::PhantomData,
        })
    }
}

// impl<A, Heap, L, AllCurrentCases> CoVisitor<CstfyTransparent<Heap, A>>
//     for Parser<'_, L, AllCurrentCases>
// where
//     A: ParseLL<Heap, L>,
// {
//     fn co_push(&mut self) {
//         // do nothing (transparent)
//     }

//     fn co_proceed(&mut self) {
//         // do nothing (transparent)
//     }

//     fn co_pop(&mut self) {
//         // do nothing (transparent)
//     }
// }

impl<A, Heap, L, AllCurrentCases> CoVisitor<Cstfy<Heap, A>> for Parser<'_, L, AllCurrentCases>
where
    A: term::case_split::Adt,
    Cstfy<Heap, A>: Implements<Heap, L>,
    <Cstfy<Heap, A> as Implements<Heap, L>>::LWord: NamesParseLL,
{
    fn co_push(&mut self) {
        self.position = self
            .match_keywords(
                &<<Cstfy<Heap, A> as Implements<Heap, L>>::LWord as NamesParseLL>::START,
            )
            .unwrap_or_else(|| {
                panic!(
                    "Expected start keyword \"{}\" but got \"{}\"",
                    &<<Cstfy<Heap, A> as Implements<Heap, L>>::LWord as NamesParseLL>::START.0[0]
                        .get(),
                    &self.source[self.position.offset()..]
                );
            });
        println!("co_push: {:?}", self.position);
    }

    fn co_proceed(&mut self) {
        self.position = self
            .match_keywords(
                &<<Cstfy<Heap, A> as Implements<Heap, L>>::LWord as NamesParseLL>::PROCEED[0],
            )
            .unwrap_or_else(|| {
                panic!(
                    "Expected proceed keyword: {:?}",
                    &<<Cstfy<Heap, A> as Implements<Heap, L>>::LWord as NamesParseLL>::PROCEED[0].0
                        [0]
                );
            }); // todo: do not use 0
        println!("co_proceed: {:?}", self.position);
    }

    fn co_pop(&mut self) {
        self.position = self
            .match_keywords(&<<Cstfy<Heap, A> as Implements<Heap, L>>::LWord as NamesParseLL>::END)
            .unwrap_or_else(|| {
                panic!(
                    "Expected end keyword: {:?}",
                    &<<Cstfy<Heap, A> as Implements<Heap, L>>::LWord as NamesParseLL>::END.0[0]
                );
            });
        println!("co_pop: {:?}", self.position);
    }
}

impl<A, Heap, L, AllCurrentCases> term::co_case_split::AdmitNoMatchingCase<Heap, Cstfy<Heap, A>>
    for Parser<'_, L, AllCurrentCases>
// where
//     Cstfy<Heap, A>: Implements<Heap, L>,
//     <Cstfy<Heap, A> as Implements<Heap, L>>::LWord: NamesParseLL,
{
    fn no_matching_case(
        &self,
        _heap: &mut <Cstfy<Heap, A> as term::Heaped>::Heap,
    ) -> (Cstfy<Heap, A>, Self::ShortCircuitsTo) {
        (
            tymetafuncspec_core::Either::Right(
                std_parse_error::ParseError::new(parse::ParseError::UnexpectedToken(
                    UnexpectedTokenError {
                        at: self.position.into(),
                    },
                )),
                std::marker::PhantomData,
            ),
            Parser {
                source: self.source,
                position: self.position,
                phantom: std::marker::PhantomData,
            },
        )
    }
}

// impl<A, Heap, L, AllCurrentCases>
//     term::co_case_split::AdmitNoMatchingCase<Heap, CstfyTransparent<Heap, A>>
//     for Parser<'_, L, AllCurrentCases>
// // where
// //     A: ParseLL<Heap, L>,
// {
//     fn no_matching_case(
//         &self,
//         _heap: &mut <CstfyTransparent<Heap, A> as term::Heaped>::Heap,
//     ) -> (CstfyTransparent<Heap, A>, Self::ShortCircuitsTo) {
//         (
//             tymetafuncspec_core::Either::Right(
//                 std_parse_error::ParseError::new(parse::ParseError::UnexpectedToken(
//                     UnexpectedTokenError {
//                         at: self.position.into(),
//                     },
//                 )),
//                 std::marker::PhantomData,
//             ),
//             Parser {
//                 source: self.source,
//                 position: self.position,
//                 phantom: std::marker::PhantomData,
//             },
//         )
//     }
// }

impl<'a, Heap, L, Pmsp, Lookaheadable, Fnlut>
    term::co_visit::CoVisitable<Parser<'a, L, ()>, Pmsp, Heap, typenum::U0, Fnlut>
    for Cstfy<Heap, Lookaheadable>
where
    Lookaheadable: Lookahead<Heap, L>,
    Fnlut: HasFn<Self, FnType = fn(&mut Parser<'a, L, ()>, &mut Heap, Fnlut) -> Self>,
{
    fn co_visit(visitor: &mut Parser<'a, L, ()>, heap: &mut Heap, fnlut: Fnlut) -> Self {
        println!("dbg: recursion limit reached for cstfy; restarting");
        fnlut.get::<Self>()(visitor, heap, fnlut)
    }
}

// impl<'a, Heap, L, Pmsp, Lookaheadable, Fnlut>
//     term::co_visit::CoVisitable<Parser<'a, L, ()>, Pmsp, Heap, typenum::U0, Fnlut>
//     for CstfyTransparent<Heap, Lookaheadable>
// where
//     Lookaheadable: Lookahead<Heap, L>,
//     Fnlut: HasFn<Self, FnType = fn(&mut Parser<'a, L, ()>, &mut Heap, Fnlut) -> Self>,
// {
//     fn co_visit(visitor: &mut Parser<'a, L, ()>, heap: &mut Heap, fnlut: Fnlut) -> Self {
//         println!("dbg: recursion limit reached for cstfy transparent; restarting");
//         fnlut.get::<Self>()(visitor, heap, fnlut)
//     }
// }
