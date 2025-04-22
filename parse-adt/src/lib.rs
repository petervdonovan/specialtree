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
impl<'a, AllCurrentCases> Parser<'a, AllCurrentCases> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            position: miette::SourceOffset::from(0),
            phantom: std::marker::PhantomData,
        }
    }
}

pub trait ParseLL {
    const START: KeywordSequence;
    const PROCEED: &'static [KeywordSequence];
    const END: KeywordSequence;
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

impl<Heap, Car, CdrCar, CdrCdr, AllCurrentCases>
    AcceptingCases<((Cstfy<Heap, Car>, ()), (CdrCar, CdrCdr))> for Parser<'_, AllCurrentCases>
where
    AllCurrentCases: AtLeastTwoConsList,
    CdrCdr: ConsList,
    Self: AcceptingCases<(CdrCar, CdrCdr), ShortCircuitsTo = Self>,
    Car: ParseLL,
{
    type AcceptingRemainingCases = Self;

    fn try_case(self) -> Result<Self::ShortCircuitsTo, Self::AcceptingRemainingCases> {
        todo!()
    }
}

impl<Heap, Car, CdrCar, CdrCdr, AllCurrentCases>
    AcceptingCases<((CstfyTransparent<Heap, Car>, ()), (CdrCar, CdrCdr))>
    for Parser<'_, AllCurrentCases>
where
    AllCurrentCases: AtLeastTwoConsList,
    CdrCdr: ConsList,
    Self: AcceptingCases<(CdrCar, CdrCdr), ShortCircuitsTo = Self>,
    Car: ParseLL,
{
    type AcceptingRemainingCases = Self;

    fn try_case(self) -> Result<Self::ShortCircuitsTo, Self::AcceptingRemainingCases> {
        todo!()
    }
}

impl<'a, Car> AcceptingCases<((Car, ()), ())> for Parser<'a, ((Car, ()), ())>
where
    // Cdr: ConsList,
    Self: AcceptingCases<(), ShortCircuitsTo = Parser<'a, ()>>,
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
        todo!()
    }

    fn co_proceed(&mut self) {
        todo!()
    }

    fn co_pop(&mut self) {
        todo!()
    }
}

impl<A, Heap, AllCurrentCases> CoVisitor<Cstfy<Heap, A>> for Parser<'_, AllCurrentCases>
where
    A: ParseLL,
{
    fn co_push(&mut self) {
        todo!()
    }

    fn co_proceed(&mut self) {
        todo!()
    }

    fn co_pop(&mut self) {
        todo!()
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
