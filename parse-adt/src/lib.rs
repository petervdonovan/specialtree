use conslist::ConsList;
use covisit::{
    covisiteventsink::CovisitEventSink,
    select::{AcceptingCases, AdmitNoMatchingCase, FromSelectCase, SelectCase},
};
use cstfy::Cstfy;
use parse::{KeywordSequence, UnexpectedTokenError};
use pmsp::{AtLeastTwoStrategy, Strategy};
use take_mut::Poisonable;
use words::Implements;

pub mod cstfy;
mod tmfscore;

pub struct Parser<'a, L> {
    pc: ParseCursor<'a>,
    phantom: std::marker::PhantomData<L>,
}
pub struct ParserSelecting<'a, L, AllCurrentCases> {
    pc: ParseCursor<'a>,
    phantom: std::marker::PhantomData<(L, AllCurrentCases)>,
}
pub struct ParseCursor<'a> {
    source: &'a str,
    position: miette::SourceOffset,
}
impl<'a, L> Poisonable for Parser<'a, L> {
    fn poisoned() -> Self {
        Self {
            pc: ParseCursor {
                source: "",
                position: miette::SourceOffset::from(usize::MAX),
            },
            phantom: std::marker::PhantomData,
        }
    }
}
impl<'a, L> Parser<'a, L> {
    pub fn new(source: &'a str) -> Self {
        Self {
            pc: ParseCursor {
                source,
                position: miette::SourceOffset::from(0),
            },
            phantom: std::marker::PhantomData,
        }
    }
}
impl<'a> ParseCursor<'a> {
    pub fn peek_words(&self) -> impl Iterator<Item = (usize, &'a str)> + use<'a> {
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
impl<'a, L, AllCurrentCases> ParserSelecting<'a, L, AllCurrentCases> {
    fn done(self) -> Parser<'a, L> {
        Parser {
            pc: self.pc,
            phantom: std::marker::PhantomData,
        }
    }
}
pub trait NamesParseLL {
    const START: KeywordSequence;
    const PROCEED: &'static [KeywordSequence];
    const END: KeywordSequence;
}

pub trait Lookahead<Heap, L> {
    fn matches<'a>(parser: &ParseCursor<'a>) -> bool;
}
impl<T, Heap, L> Lookahead<Heap, L> for T
where
    T: words::Adt,
    Cstfy<Heap, T>: words::Implements<Heap, L>,
    <Cstfy<Heap, T> as words::Implements<Heap, L>>::LWord: NamesParseLL,
{
    fn matches(parser: &ParseCursor<'_>) -> bool {
        parser
            .match_keywords(
                &<<Cstfy<Heap, T> as words::Implements<Heap, L>>::LWord as NamesParseLL>::START,
            )
            .is_some()
    }
}

impl<'a, L> SelectCase for Parser<'a, L> {
    type AC<CasesConsList: Strategy> = ParserSelecting<'a, L, CasesConsList>;

    fn start_cases<CasesConsList: Strategy>(self) -> Self::AC<CasesConsList> {
        ParserSelecting {
            pc: self.pc,
            phantom: std::marker::PhantomData,
        }
    }
}

impl<'a, L, Cases> FromSelectCase for ParserSelecting<'a, L, Cases> {
    type Done = Parser<'a, L>;
}

impl<'a, Heap, A, L, Cases, AllCurrentCases> AcceptingCases<Cases>
    for ParserSelecting<'a, L, AllCurrentCases>
where
    Cases: AtLeastTwoStrategy<Car = (Cstfy<Heap, A>, ())>,
    Self: AcceptingCases<Cases::Cdr, Done = Parser<'a, L>>,
    A: Lookahead<Heap, L>,
{
    type AcceptingRemainingCases = Self;

    fn try_case(self) -> Result<Self::Done, Self::AcceptingRemainingCases> {
        if A::matches(&self.pc) {
            Ok(self.done())
        } else {
            Err(self)
        }
    }
}

impl<'a, L, Case, AllCurrentCases> AcceptingCases<(Case, ())>
    for ParserSelecting<'a, L, AllCurrentCases>
where
    Case: ConsList,
{
    type AcceptingRemainingCases = Self;

    fn try_case(self) -> Result<Self::Done, Self::AcceptingRemainingCases> {
        Ok(self.done())
    }
}

impl<'a, L, AllCurrentCases> AcceptingCases<()> for ParserSelecting<'a, L, AllCurrentCases> {
    type AcceptingRemainingCases = Self;

    fn try_case(self) -> Result<Self::Done, Self::AcceptingRemainingCases> {
        panic!()
    }
}

impl<Heap, A, L> CovisitEventSink<Cstfy<Heap, A>> for Parser<'_, L>
where
    A: words::Adt,
    Cstfy<Heap, A>: Implements<Heap, L>,
    <Cstfy<Heap, A> as Implements<Heap, L>>::LWord: NamesParseLL,
{
    fn push(&mut self) {
        self.pc.position = self
            .pc
            .match_keywords(
                &<<Cstfy<Heap, A> as Implements<Heap, L>>::LWord as NamesParseLL>::START,
            )
            .unwrap_or_else(|| {
                panic!(
                    "Expected start keyword \"{}\" but got \"{}\"",
                    &<<Cstfy<Heap, A> as Implements<Heap, L>>::LWord as NamesParseLL>::START.0[0]
                        .get(),
                    &self.pc.source[self.pc.position.offset()..]
                );
            });
    }

    fn proceed(&mut self, idx: u32, _total: u32) {
        if let Some(kw) = <<Cstfy<Heap, A> as Implements<Heap, L>>::LWord as NamesParseLL>::PROCEED
            .get(idx as usize)
        {
            self.pc.position = self.pc.match_keywords(kw).unwrap_or_else(|| {
                panic!("Expected proceed keyword: {:?}", kw.0[0]);
            }); // todo: do not use 0
        } else if let Some(kw) =
            <<Cstfy<Heap, A> as Implements<Heap, L>>::LWord as NamesParseLL>::PROCEED.last()
        {
            self.pc.position = self.pc.match_keywords(kw).unwrap_or_else(|| {
                panic!("Expected proceed keyword: {:?}", kw.0[0]);
            }); // todo: do not use 0
        } else {
            // do nothing
        }
    }

    fn pop(&mut self) {
        self.pc.position = self
            .pc
            .match_keywords(&<<Cstfy<Heap, A> as Implements<Heap, L>>::LWord as NamesParseLL>::END)
            .unwrap_or_else(|| {
                panic!(
                    "Expected end keyword: {:?}",
                    &<<Cstfy<Heap, A> as Implements<Heap, L>>::LWord as NamesParseLL>::END.0[0]
                );
            });
    }
}

impl<Heap, A, L, AllCases> AdmitNoMatchingCase<Heap, Cstfy<Heap, A>>
    for ParserSelecting<'_, L, AllCases>
where
    A: Lookahead<Heap, L>,
{
    fn admit(self, _: &mut Heap) -> (Self::Done, Cstfy<Heap, A>) {
        let err = tymetafuncspec_core::Either::Right(
            std_parse_error::ParseError::new(parse::ParseError::UnexpectedToken(
                UnexpectedTokenError {
                    at: self.pc.position.into(),
                },
            )),
            std::marker::PhantomData,
        );
        (self.done(), err)
    }
}
