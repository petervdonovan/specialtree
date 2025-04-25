#![feature(negative_impls)]
#![feature(fundamental)]

use cstfy::{Cstfy, CstfyTransparent};
use parse::{Keyword, KeywordSequence, UnexpectedTokenError};
use term::{
    case_split::{Adt, AtLeastTwoConsList, ConsList, HasPatternMatchStrategyFor, NonemptyConsList},
    co_visit::{CoVisitable, CoVisitor},
    select::{AcceptingCases, FromSelectCase, SelectCase},
    CanonicallyConstructibleFrom,
};
use tymetafuncspec_core::{BoundedNat, BoundedNatHeapBak};

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
            last_offset = (found.0 + found.1.len()).into();
        }
        if keywords.next().is_some() {
            return None;
        }
        return Some(last_offset);
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

    // fn stop_with_external_success(
    //     self,
    //     proof_of_success: <() as term::case_split::ConsList>::Car,
    // ) -> Self::ShortCircuitsTo {
    //     todo!()
    // }
}

impl<'a, Heap, RemainingCasesUnwrappedCarCar, RemainingCasesCdr, AllCurrentCases>
    AcceptingCases<(
        (Cstfy<Heap, RemainingCasesUnwrappedCarCar>, ()),
        RemainingCasesCdr,
    )> for Parser<'a, AllCurrentCases>
where
    AllCurrentCases: AtLeastTwoConsList,
    RemainingCasesCdr: ConsList,
    // CdrCdr: ConsList,
    // Self: AcceptingCases<(CdrCar, CdrCdr), ShortCircuitsTo = Self>,
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
    // CdrCdr: ConsList,
    // Self: AcceptingCases<(CdrCar, CdrCdr), ShortCircuitsTo = Self>,
    Self: AcceptingCases<RemainingCasesCdr, ShortCircuitsTo = Parser<'a, ()>>,
    RemainingCasesUnwrappedCarCar: Lookahead,
{
    type AcceptingRemainingCases = Self;

    fn try_case(self) -> Result<Self::ShortCircuitsTo, Self::AcceptingRemainingCases> {
        todo!()
    }
}

impl<'a, Car> AcceptingCases<((Car, ()), ())> for Parser<'a, ((Car, ()), ())>
where
// Cdr: ConsList,
// Self: AcceptingCases<(), ShortCircuitsTo = Parser<'a, ()>>,
// Car: ParseLL,
// Car: IsSoleCaseHack,
{
    type AcceptingRemainingCases = Self;

    fn try_case(self) -> Result<Self::ShortCircuitsTo, Self::AcceptingRemainingCases> {
        Ok(Parser {
            source: self.source,
            position: self.position,
            phantom: std::marker::PhantomData,
        })
    }
}

// impl<Heap, Car, CarCdrCar, CarCdrCdr, AllCurrentCases>
//     AcceptingCases<((Cstfy<Heap, Car>, (CarCdrCar, CarCdrCdr)), ())> for Parser<'_, AllCurrentCases>
// where
//     AllCurrentCases: AtLeastTwoConsList,
//     Self: AcceptingCases<(), ShortCircuitsTo = Self>,
//     Car: ParseLL,
// {
//     type AcceptingRemainingCases = Self;

//     fn try_case(self) -> Result<Self::ShortCircuitsTo, Self::AcceptingRemainingCases> {
//         Ok(todo!())
//     }
// }

// impl<Heap, Car, CarCdrCar, CarCdrCdr, AllCurrentCases>
//     AcceptingCases<((CstfyTransparent<Heap, Car>, (CarCdrCar, CarCdrCdr)), ())>
//     for Parser<'_, AllCurrentCases>
// where
//     AllCurrentCases: AtLeastTwoConsList,
//     Self: AcceptingCases<(), ShortCircuitsTo = Self>,
//     Car: ParseLL,
// {
//     type AcceptingRemainingCases = Self;

//     fn try_case(self) -> Result<Self::ShortCircuitsTo, Self::AcceptingRemainingCases> {
//         Ok(todo!())
//     }
// }

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
            panic!("Expected start keyword: {:?}", &A::START.0[0]);
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

// fn test() {
//     struct Pmsp;
//     impl HasPatternMatchStrategyFor<BoundedNat<BoundedNatHeapBak<()>>> for Pmsp {
//         type Strategy = ((BoundedNat<BoundedNatHeapBak<()>>, ()), ());
//     }
//     // impl term::co_visit::CoVisitable<Parser<'_>, Pmsp, tymetafuncspec_core::BoundedNatHeapBak<()>>
//     //     for BoundedNat<BoundedNatHeapBak<()>>
//     // {
//     //     fn co_visit(
//     //         visitor: &mut Parser<'_>,
//     //         heap: &mut tymetafuncspec_core::BoundedNatHeapBak<()>,
//     //     ) -> Self {
//     //         todo!()
//     //     }
//     // }

//     // impl<'a>
//     //     term::co_visit::CoVisitor<
//     //         tymetafuncspec_core::BoundedNat<tymetafuncspec_core::BoundedNatHeapBak<()>>,
//     //     > for Parser<'a>
//     // {
//     //     fn co_push(&mut self) {
//     //         todo!()
//     //     }

//     //     fn co_proceed(&mut self) {
//     //         todo!()
//     //     }

//     //     fn co_pop(&mut self) {
//     //         todo!()
//     //     }
//     // }
//     // impl<'a>
//     //     term::select::AcceptingCases<(
//     //         tymetafuncspec_core::BoundedNat<tymetafuncspec_core::BoundedNatHeapBak<()>>,
//     //         (),
//     //     )> for Parser<'a>
//     // {
//     //     type ShortCircuitsTo = Self;

//     //     type AcceptingRemainingCases = Self;

//     //     fn try_case(&mut self) -> Result<Self::ShortCircuitsTo, Self::AcceptingRemainingCases> {
//     //         todo!()
//     //     }
//     // }
//     // impl<'a>
//     //     term::co_case_split::CoCallable<
//     //         tymetafuncspec_core::BoundedNatHeapBak<()>,
//     //         tymetafuncspec_core::BoundedNat<tymetafuncspec_core::BoundedNatHeapBak<()>>,
//     //     > for Parser<'a>
//     // {
//     //     fn call(
//     //         &mut self,
//     //         heap: &mut tymetafuncspec_core::BoundedNatHeapBak<()>,
//     //     ) -> tymetafuncspec_core::BoundedNat<tymetafuncspec_core::BoundedNatHeapBak<()>> {
//     //         todo!()
//     //     }
//     // }
//     // impl
//     //     term::co_case_split::AdmitNoMatchingCase<
//     //         tymetafuncspec_core::BoundedNat<tymetafuncspec_core::BoundedNatHeapBak<()>>,
//     //     > for Parser<'_>
//     // {
//     //     fn no_matching_case(
//     //         &self,
//     //         heap: &mut <tymetafuncspec_core::BoundedNat<tymetafuncspec_core::BoundedNatHeapBak<()>> as term::Heaped>::Heap,
//     //     ) -> tymetafuncspec_core::BoundedNat<tymetafuncspec_core::BoundedNatHeapBak<()>> {
//     //         panic!("No matching case")
//     //     }
//     // }

//     let mut parser = Parser {
//         source: "test",
//         position: 0.into(),
//     };
//     let mut heap = BoundedNatHeapBak::<()>::default();
//     <Cstfy<BoundedNatHeapBak<()>, BoundedNat<BoundedNatHeapBak<()>>> as CoVisitable<
//         Parser<'_>,
//         Pmsp,
//         BoundedNatHeapBak<()>,
//     >>::co_visit(&mut parser, &mut heap);
// }

// fn test2() {
//     // pub struct Directly;
//     // pub struct Transitively;
//     // pub trait CorrespondsTo {
//     //     type DirectlyOrTransitively;
//     // }
//     // pub trait Goal {}

//     // impl<T> Goal for T where T: CorrespondsTo<DirectlyOrTransitively = Directly> {}
//     // impl<T> Goal for T where T: CorrespondsTo<DirectlyOrTransitively = Transitively> {}

//     pub trait HasEmbeddingFrom
//     where
//         Self: CanonicallyConstructibleFrom<Self::Intermediary>,
//     {
//         type Intermediary;
//         // fn from_intermediary(intermediary: Self::Intermediary) -> Self;
//     }

//     pub trait TransitivelyConstructible<T> {
//         fn construct(t: T) -> Self;
//     }
//     impl<T, U> TransitivelyConstructible<T> for U
//     where
//         U: HasEmbeddingFrom,
//         U::Intermediary: TransitivelyConstructible<T>,
//     {
//         fn construct(t: T) -> Self {
//             let intermediary = U::Intermediary::construct(t);
//             Self::from_intermediary(intermediary)
//         }
//     }

//     // impl<U> TransitivelyConstructible<U> for U {
//     //     fn construct(t: U) -> Self {
//     //         t
//     //     }
//     // }
// }
