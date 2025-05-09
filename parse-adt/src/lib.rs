use conslist::ConsList;
use covisit::{
    covisiteventsink::CovisitEventSink,
    select::{AcceptingCases, AdmitNoMatchingCase, FromSelectCase, SelectCase},
};
use cstfy::Cstfy;
use parse::{KeywordSequence, UnexpectedTokenError};
use pmsp::{AtLeastTwoStrategy, NonemptyStrategy, Strategy, UsesStrategyForTraversal};
use take_mut::Poisonable;
use words::Implements;

pub mod cstfy;
mod tmfscore;

pub struct Parser<'a, L, AllCurrentCases> {
    source: &'a str,
    position: miette::SourceOffset,
    current_covisit_idx: usize,
    phantom: std::marker::PhantomData<(L, AllCurrentCases)>,
}
impl<'a, L, AllCurrentCases> Poisonable for Parser<'a, L, AllCurrentCases> {
    fn poisoned() -> Self {
        Self {
            source: "",
            position: miette::SourceOffset::from(usize::MAX),
            current_covisit_idx: usize::MAX,
            phantom: std::marker::PhantomData,
        }
    }
}
impl<'a, L, AllCurrentCases> Parser<'a, L, AllCurrentCases> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            position: miette::SourceOffset::from(0),
            current_covisit_idx: 0,
            phantom: std::marker::PhantomData,
        }
    }
    pub fn convert_to<To>(self) -> Parser<'a, L, To> {
        Parser {
            source: self.source,
            position: self.position,
            current_covisit_idx: self.current_covisit_idx,
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

pub trait NamesParseLL {
    const START: KeywordSequence;
    const PROCEED: &'static [KeywordSequence];
    const END: KeywordSequence;
}

pub trait Lookahead<Heap, L> {
    fn matches<'a, T>(parser: &Parser<'a, L, T>) -> bool;
}
impl<T, Heap, L> Lookahead<Heap, L> for T
where
    // T: ParseLL<Heap, L> + LookaheadImplementor,
    T: words::Adt,
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

impl<'a, L> SelectCase for Parser<'a, L, ()> {
    type AC<CasesConsList: Strategy> = Parser<'a, L, CasesConsList>;

    fn start_cases<CasesConsList: Strategy>(self) -> Self::AC<CasesConsList> {
        Parser {
            source: self.source,
            position: self.position,
            current_covisit_idx: 0, // fixme: meaningless
            phantom: std::marker::PhantomData,
        }
    }
}

// impl<'a, L, Cases> InSelectCase for Parser<'a, L, Cases> {
//     type EndSelectCase = Parser<'a, L, ()>;
// }

impl<'a, L, Cases> FromSelectCase for Parser<'a, L, Cases> {
    type Done = Parser<'a, L, ()>;
}

// impl<'a, L, AllCurrentCases> AcceptingCases<()> for Parser<'a, L, AllCurrentCases>
// where
//     AllCurrentCases: NonemptyConsList,
// {
//     type AcceptingRemainingCases = Self;

//     fn try_case(self) -> Result<Self::EndSelectCase, Self::AcceptingRemainingCases> {
//         panic!()
//     }
// }

// impl<'a, Heap, L, RemainingCasesUnwrappedCarCar, RemainingCasesCdr, AllCurrentCases>
//     AcceptingCases<(
//         (Cstfy<Heap, RemainingCasesUnwrappedCarCar>, ()),
//         RemainingCasesCdr,
//     )> for Parser<'a, L, AllCurrentCases>
// where
//     AllCurrentCases: AtLeastTwoConsList,
//     RemainingCasesCdr: ConsList,
//     Self: AcceptingCases<RemainingCasesCdr, EndSelectCase = Parser<'a, L, ()>>,
//     RemainingCasesUnwrappedCarCar: Lookahead<Heap, L>,
// {
//     type AcceptingRemainingCases = Self;

//     fn try_case(self) -> Result<Self::EndSelectCase, Self::AcceptingRemainingCases> {
//         if RemainingCasesUnwrappedCarCar::matches(&self) {
//             Ok(self.convert_to())
//         } else {
//             Err(self)
//         }
//     }
// }

impl<'a, Heap, A, L, Cases, AllCurrentCases> AcceptingCases<Cases>
    for Parser<'a, L, AllCurrentCases>
where
    Cases: AtLeastTwoStrategy<Car = (Cstfy<Heap, A>, ())>,
    Self: AcceptingCases<Cases::Cdr, Done = Parser<'a, L, ()>>,
    A: Lookahead<Heap, L>,
{
    type AcceptingRemainingCases = Self;

    fn try_case(self) -> Result<Self::Done, Self::AcceptingRemainingCases> {
        if A::matches(&self) {
            Ok(self.convert_to())
        } else {
            Err(self)
        }
    }
}

impl<'a, L, Case, AllCurrentCases> AcceptingCases<(Case, ())> for Parser<'a, L, AllCurrentCases>
where
    Case: ConsList,
    // Self: AcceptingCases<Cases::Cdr, Done = Parser<'a, L, ()>>,
    // A: Lookahead<Heap, L>,
{
    type AcceptingRemainingCases = Self;

    fn try_case(self) -> Result<Self::Done, Self::AcceptingRemainingCases> {
        Ok(Parser {
            source: self.source,
            position: self.position,
            current_covisit_idx: 0, // fixme: meaningless
            phantom: std::marker::PhantomData,
        })
    }
}

impl<'a, L, AllCurrentCases> AcceptingCases<()> for Parser<'a, L, AllCurrentCases>
where
// Cases: NonemptyStrategy<Car = Cstfy<Heap, A>>,
// Self: AcceptingCases<(), Done = Parser<'a, L, ()>>,
// A: Lookahead<Heap, L>,
{
    type AcceptingRemainingCases = Self;

    fn try_case(self) -> Result<Self::Done, Self::AcceptingRemainingCases> {
        panic!()
    }
}

// impl<A, Heap, L, AllCurrentCases> CoVisitor<Cstfy<Heap, A>> for Parser<'_, L, AllCurrentCases>
// where
//     A: term::case_split::Adt,
//     Cstfy<Heap, A>: Implements<Heap, L>,
//     <Cstfy<Heap, A> as Implements<Heap, L>>::LWord: NamesParseLL,
// {
//     fn co_push(&mut self) {
//         self.position = self
//             .match_keywords(
//                 &<<Cstfy<Heap, A> as Implements<Heap, L>>::LWord as NamesParseLL>::START,
//             )
//             .unwrap_or_else(|| {
//                 panic!(
//                     "Expected start keyword \"{}\" but got \"{}\"",
//                     &<<Cstfy<Heap, A> as Implements<Heap, L>>::LWord as NamesParseLL>::START.0[0]
//                         .get(),
//                     &self.source[self.position.offset()..]
//                 );
//             });
//         println!("co_push: {:?}", self.position);
//     }

//     fn co_proceed(&mut self) {
//         self.position = self
//             .match_keywords(
//                 &<<Cstfy<Heap, A> as Implements<Heap, L>>::LWord as NamesParseLL>::PROCEED[0],
//             )
//             .unwrap_or_else(|| {
//                 panic!(
//                     "Expected proceed keyword: {:?}",
//                     &<<Cstfy<Heap, A> as Implements<Heap, L>>::LWord as NamesParseLL>::PROCEED[0].0
//                         [0]
//                 );
//             }); // todo: do not use 0
//         println!("co_proceed: {:?}", self.position);
//     }

//     fn co_pop(&mut self) {
//         self.position = self
//             .match_keywords(&<<Cstfy<Heap, A> as Implements<Heap, L>>::LWord as NamesParseLL>::END)
//             .unwrap_or_else(|| {
//                 panic!(
//                     "Expected end keyword: {:?}",
//                     &<<Cstfy<Heap, A> as Implements<Heap, L>>::LWord as NamesParseLL>::END.0[0]
//                 );
//             });
//         println!("co_pop: {:?}", self.position);
//     }
// }

impl<Heap, A, L> CovisitEventSink<Cstfy<Heap, A>> for Parser<'_, L, ()>
where
    A: words::Adt,
    Cstfy<Heap, A>: Implements<Heap, L>,
    <Cstfy<Heap, A> as Implements<Heap, L>>::LWord: NamesParseLL,
{
    fn push(&mut self) {
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
        self.current_covisit_idx = 0;
        println!("push: {:?}", self.position);
    }

    fn proceed(&mut self) {
        if let Some(kw) = <<Cstfy<Heap, A> as Implements<Heap, L>>::LWord as NamesParseLL>::PROCEED
            .get(self.current_covisit_idx)
        {
            self.position = self.match_keywords(kw).unwrap_or_else(|| {
                panic!("Expected proceed keyword: {:?}", kw.0[0]);
            }); // todo: do not use 0
            println!("proceed: {:?}", self.position);
        } else if let Some(kw) =
            <<Cstfy<Heap, A> as Implements<Heap, L>>::LWord as NamesParseLL>::PROCEED.last()
        {
            self.position = self.match_keywords(kw).unwrap_or_else(|| {
                panic!("Expected proceed keyword: {:?}", kw.0[0]);
            }); // todo: do not use 0
            println!("proceed: {:?}", self.position);
        } else {
            // do nothing
        }
        self.current_covisit_idx += 1;
    }

    fn pop(&mut self) {
        self.position = self
            .match_keywords(&<<Cstfy<Heap, A> as Implements<Heap, L>>::LWord as NamesParseLL>::END)
            .unwrap_or_else(|| {
                panic!(
                    "Expected end keyword: {:?}",
                    &<<Cstfy<Heap, A> as Implements<Heap, L>>::LWord as NamesParseLL>::END.0[0]
                );
            });
        self.current_covisit_idx = 0;
        println!("pop: {:?}", self.position);
    }
}

// impl<A, Heap, L, AllCurrentCases> term::co_case_split::AdmitNoMatchingCase<Heap, Cstfy<Heap, A>>
//     for Parser<'_, L, AllCurrentCases>
// {
//     fn no_matching_case(
//         &self,
//         _heap: &mut <Cstfy<Heap, A> as term::Heaped>::Heap,
//     ) -> (Cstfy<Heap, A>, Self::EndSelectCase) {
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

impl<Heap, A, L, AllCases> AdmitNoMatchingCase<Heap, Cstfy<Heap, A>> for Parser<'_, L, AllCases>
where
    // Cases: Strategy<Car = Cstfy<Heap, A>>,
    // Self: AcceptingCases<Cases::Cdr, Done = Parser<'a, L, ()>>,
    A: Lookahead<Heap, L>,
{
    fn admit(self, _: &mut Heap) -> (Self::Done, Cstfy<Heap, A>) {
        let err = tymetafuncspec_core::Either::Right(
            std_parse_error::ParseError::new(parse::ParseError::UnexpectedToken(
                UnexpectedTokenError {
                    at: self.position.into(),
                },
            )),
            std::marker::PhantomData,
        );
        (self.convert_to(), err)
    }
}

// impl<'a, Heap, L, Pmsp, Lookaheadable, Fnlut>
//     term::co_visit::CoVisitable<Parser<'a, L, ()>, Pmsp, Heap, typenum::U0, Fnlut>
//     for Cstfy<Heap, Lookaheadable>
// where
//     Lookaheadable: Lookahead<Heap, L>,
//     Fnlut: HasFn<Self, FnType = fn(&mut Parser<'a, L, ()>, &mut Heap, Fnlut) -> Self>,
// {
//     fn co_visit(visitor: &mut Parser<'a, L, ()>, heap: &mut Heap, fnlut: Fnlut) -> Self {
//         println!("dbg: recursion limit reached for cstfy; restarting");
//         fnlut.get::<Self>()(visitor, heap, fnlut)
//     }
// }
impl<'a, L, Cases, Heap, T> UsesStrategyForTraversal<Parser<'a, L, Cases>> for Cstfy<Heap, T> where
    T: UsesStrategyForTraversal<Parser<'a, L, Cases>>
{
}
