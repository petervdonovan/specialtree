#![feature(negative_impls)]
#![feature(fundamental)]

use cstfy::{Cstfy, CstfyTransparent};
use parse::{KeywordSequence, UnexpectedTokenError};
use term::{
    case_split::{AtLeastTwoConsList, ConsList, NonemptyConsList},
    co_visit::CoVisitor,
    select::{AcceptingCases, FromSelectCase, SelectCase},
};

pub mod cstfy;
mod tmfscore;

pub struct Parser<'a, AllCurrentCases> {
    source: &'a str,
    position: miette::SourceOffset,
    phantom: std::marker::PhantomData<AllCurrentCases>,
}
impl<'a, AllCurrentCases> Default for Parser<'a, AllCurrentCases> {
    fn default() -> Self {
        // todo: eliminate the need for this
        Self {
            source: "",
            position: miette::SourceOffset::from(0),
            phantom: std::marker::PhantomData,
        }
    }
}
impl<'a, AllCurrentCases> Parser<'a, AllCurrentCases> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            position: miette::SourceOffset::from(0),
            phantom: std::marker::PhantomData,
        }
    }
    pub fn convert_to<To>(self) -> Parser<'a, To> {
        Parser {
            source: self.source,
            position: self.position,
            phantom: std::marker::PhantomData,
        }
    }
    pub fn peek_words(&self) -> impl Iterator<Item = (usize, &'a str)> {
        unicode_segmentation::UnicodeSegmentation::split_word_bound_indices(
            &self.source[self.position.offset()..],
        )
        .filter(|it| !it.1.is_empty() && !it.1.chars().any(|c| c.is_whitespace()))
    }
    pub fn pop_word(&mut self) -> Option<&'a str> {
        let mut iter = self.peek_words();
        if let Some(word) = iter.next() {
            self.position = (self.position.offset() + word.0 + word.1.len()).into();
            dbg!(word.1);
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

pub trait ParseLL {
    const START: KeywordSequence;
    const PROCEED: &'static [KeywordSequence];
    const END: KeywordSequence;
}

pub trait Lookahead {
    fn matches<T>(parser: &Parser<'_, T>) -> bool;
}

impl<T> Lookahead for T
where
    T: ParseLL,
{
    fn matches<C>(parser: &Parser<'_, C>) -> bool {
        parser.match_keywords(&Self::START).is_some()
    }
}

impl<'a> SelectCase for Parser<'a, ()> {
    type AC<CasesConsList: ConsList> = Parser<'a, CasesConsList>;

    fn start_cases<CasesConsList: ConsList>(self) -> Self::AC<CasesConsList> {
        Parser {
            source: self.source,
            position: self.position,
            phantom: std::marker::PhantomData,
        }
    }
}

impl<'a, Cases> FromSelectCase for Parser<'a, Cases> {
    type ShortCircuitsTo = Parser<'a, ()>;
}

impl<AllCurrentCases> AcceptingCases<()> for Parser<'_, AllCurrentCases>
where
    AllCurrentCases: NonemptyConsList,
{
    type AcceptingRemainingCases = Self;

    fn try_case(self) -> Result<Self::ShortCircuitsTo, Self::AcceptingRemainingCases> {
        todo!()
    }
}

impl<'a, Heap, RemainingCasesUnwrappedCarCar, RemainingCasesCdr, AllCurrentCases>
    AcceptingCases<(
        (Cstfy<Heap, RemainingCasesUnwrappedCarCar>, ()),
        RemainingCasesCdr,
    )> for Parser<'a, AllCurrentCases>
where
    AllCurrentCases: AtLeastTwoConsList,
    RemainingCasesCdr: ConsList,
    Self: AcceptingCases<RemainingCasesCdr, ShortCircuitsTo = Parser<'a, ()>>,
    RemainingCasesUnwrappedCarCar: Lookahead,
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

impl<'a, Heap, RemainingCasesUnwrappedCarCar, RemainingCasesCdr, AllCurrentCases>
    AcceptingCases<(
        (CstfyTransparent<Heap, RemainingCasesUnwrappedCarCar>, ()),
        RemainingCasesCdr,
    )> for Parser<'a, AllCurrentCases>
where
    AllCurrentCases: AtLeastTwoConsList,
    RemainingCasesCdr: ConsList,
    Self: AcceptingCases<RemainingCasesCdr, ShortCircuitsTo = Parser<'a, ()>>,
    RemainingCasesUnwrappedCarCar: Lookahead,
{
    type AcceptingRemainingCases = Self;

    fn try_case(self) -> Result<Self::ShortCircuitsTo, Self::AcceptingRemainingCases> {
        todo!()
    }
}

impl<'a, Car> AcceptingCases<(Car, ())> for Parser<'a, (Car, ())> {
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

impl<A, Heap, AllCurrentCases> CoVisitor<CstfyTransparent<Heap, A>> for Parser<'_, AllCurrentCases>
where
    A: ParseLL,
{
    fn co_push(&mut self) {
        // do nothing (transparent)
    }

    fn co_proceed(&mut self) {
        // do nothing (transparent)
    }

    fn co_pop(&mut self) {
        // do nothing (transparent)
    }
}

impl<A, Heap, AllCurrentCases> CoVisitor<Cstfy<Heap, A>> for Parser<'_, AllCurrentCases>
where
    A: ParseLL,
{
    fn co_push(&mut self) {
        self.position = self.match_keywords(&A::START).unwrap_or_else(|| {
            panic!(
                "Expected start keyword \"{}\" but got \"{}\"",
                &A::START.0[0].get(),
                &self.source[self.position.offset()..]
            );
        });
        println!("co_push: {:?}", self.position);
    }

    fn co_proceed(&mut self) {
        self.position = self.match_keywords(&A::PROCEED[0]).unwrap_or_else(|| {
            panic!("Expected proceed keyword: {:?}", &A::PROCEED[0].0[0]);
        }); // todo: do not use 0
        println!("co_proceed: {:?}", self.position);
    }

    fn co_pop(&mut self) {
        self.position = self.match_keywords(&A::END).unwrap_or_else(|| {
            panic!("Expected end keyword: {:?}", &A::END.0[0]);
        });
        println!("co_pop: {:?}", self.position);
    }
}

impl<A, Heap, AllCurrentCases> term::co_case_split::AdmitNoMatchingCase<Cstfy<Heap, A>>
    for Parser<'_, AllCurrentCases>
where
    A: ParseLL,
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

impl<A, Heap, AllCurrentCases> term::co_case_split::AdmitNoMatchingCase<CstfyTransparent<Heap, A>>
    for Parser<'_, AllCurrentCases>
where
    A: ParseLL,
{
    fn no_matching_case(
        &self,
        _heap: &mut <CstfyTransparent<Heap, A> as term::Heaped>::Heap,
    ) -> (CstfyTransparent<Heap, A>, Self::ShortCircuitsTo) {
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

impl<'a, Heap, Pmsp, Lookaheadable>
    term::co_visit::CoVisitable<Parser<'a, ()>, Pmsp, Heap, typenum::U0>
    for Cstfy<Heap, Lookaheadable>
where
    Lookaheadable: Lookahead,
{
    fn co_visit(visitor: &mut Parser<'_, ()>, _heap: &mut Heap) -> Self {
        println!("dbg: recursion limit exceeded");
        let current_position = visitor.position;
        visitor.pop_word();
        tymetafuncspec_core::Either::Right(
            std_parse_error::ParseError::new(parse::ParseError::RecursionLimitExceeded(
                current_position.into(),
            )),
            std::marker::PhantomData,
        )
    }
}

impl<'a, Heap, Pmsp, Lookaheadable>
    term::co_visit::CoVisitable<Parser<'a, ()>, Pmsp, Heap, typenum::U0>
    for CstfyTransparent<Heap, Lookaheadable>
where
    Lookaheadable: Lookahead,
{
    fn co_visit(visitor: &mut Parser<'_, ()>, _heap: &mut Heap) -> Self {
        println!("dbg: recursion limit exceeded");
        let current_position = visitor.position;
        visitor.pop_word();
        tymetafuncspec_core::Either::Right(
            std_parse_error::ParseError::new(parse::ParseError::RecursionLimitExceeded(
                current_position.into(),
            )),
            std::marker::PhantomData,
        )
    }
}
