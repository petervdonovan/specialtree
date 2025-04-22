#![feature(negative_impls)]
#![feature(fundamental)]

use cstfy::{Cstfy, CstfyTransparent};
use parse::{Keyword, KeywordSequence, UnexpectedTokenError};
use term::{
    case_split::{Adt, ConsList, HasPatternMatchStrategyFor},
    co_visit::{CoVisitable, CoVisitor},
    select::AcceptingCases,
    CanonicallyConstructibleFrom,
};
use tymetafuncspec_core::{BoundedNat, BoundedNatHeapBak};

pub mod cstfy;
mod tmfscore;

pub struct Parser<'a> {
    source: &'a str,
    position: miette::SourceOffset,
}
impl<'a> Parser<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            position: miette::SourceOffset::from(0),
        }
    }
}

pub trait ParseLL {
    const START: KeywordSequence;
    const PROCEED: &'static [KeywordSequence];
    const END: KeywordSequence;
}

trait IsSoleCaseHack {}

impl AcceptingCases<()> for Parser<'_> {
    type ShortCircuitsTo = Self;
    type AcceptingRemainingCases = Self;

    fn try_case(&mut self) -> Result<Self::ShortCircuitsTo, Self::AcceptingRemainingCases> {
        todo!()
    }

    // fn stop_with_external_success(
    //     self,
    //     proof_of_success: <() as term::case_split::ConsList>::Car,
    // ) -> Self::ShortCircuitsTo {
    //     todo!()
    // }
}

impl<Heap, Car, CdrCar, CdrCdr> AcceptingCases<((Cstfy<Heap, Car>, ()), (CdrCar, CdrCdr))>
    for Parser<'_>
where
    CdrCdr: ConsList,
    Self: AcceptingCases<(CdrCar, CdrCdr), ShortCircuitsTo = Self>,
    Car: ParseLL,
{
    type ShortCircuitsTo = Self;
    type AcceptingRemainingCases = Self;

    fn try_case(&mut self) -> Result<Self::ShortCircuitsTo, Self::AcceptingRemainingCases> {
        todo!()
    }
}

impl<Heap, Car, CdrCar, CdrCdr>
    AcceptingCases<((CstfyTransparent<Heap, Car>, ()), (CdrCar, CdrCdr))> for Parser<'_>
where
    CdrCdr: ConsList,
    Self: AcceptingCases<(CdrCar, CdrCdr), ShortCircuitsTo = Self>,
    Car: ParseLL,
{
    type ShortCircuitsTo = Self;
    type AcceptingRemainingCases = Self;

    fn try_case(&mut self) -> Result<Self::ShortCircuitsTo, Self::AcceptingRemainingCases> {
        todo!()
    }
}

impl<Heap, Car> AcceptingCases<((Cstfy<Heap, Car>, ()), ())> for Parser<'_>
where
    // Cdr: ConsList,
    Self: AcceptingCases<(), ShortCircuitsTo = Self>,
    // Car: ParseLL,
    Car: IsSoleCaseHack,
{
    type ShortCircuitsTo = Self;
    type AcceptingRemainingCases = Self;

    fn try_case(&mut self) -> Result<Self::ShortCircuitsTo, Self::AcceptingRemainingCases> {
        todo!()
    }
}

impl<Heap, Car> AcceptingCases<((CstfyTransparent<Heap, Car>, ()), ())> for Parser<'_>
where
    // Cdr: ConsList,
    Self: AcceptingCases<(), ShortCircuitsTo = Self>,
    // Car: ParseLL,
    Car: IsSoleCaseHack,
{
    type ShortCircuitsTo = Self;
    type AcceptingRemainingCases = Self;

    fn try_case(&mut self) -> Result<Self::ShortCircuitsTo, Self::AcceptingRemainingCases> {
        todo!()
    }
}

impl<Heap, Car, CarCdrCar, CarCdrCdr>
    AcceptingCases<((Cstfy<Heap, Car>, (CarCdrCar, CarCdrCdr)), ())> for Parser<'_>
where
    Self: AcceptingCases<(), ShortCircuitsTo = Self>,
    Car: ParseLL,
{
    type ShortCircuitsTo = Self;
    type AcceptingRemainingCases = Self;

    fn try_case(&mut self) -> Result<Self::ShortCircuitsTo, Self::AcceptingRemainingCases> {
        Ok(todo!())
    }
}

impl<Heap, Car, CarCdrCar, CarCdrCdr>
    AcceptingCases<((CstfyTransparent<Heap, Car>, (CarCdrCar, CarCdrCdr)), ())> for Parser<'_>
where
    Self: AcceptingCases<(), ShortCircuitsTo = Self>,
    Car: ParseLL,
{
    type ShortCircuitsTo = Self;
    type AcceptingRemainingCases = Self;

    fn try_case(&mut self) -> Result<Self::ShortCircuitsTo, Self::AcceptingRemainingCases> {
        Ok(todo!())
    }
}

impl<A, Heap> CoVisitor<CstfyTransparent<Heap, A>> for Parser<'_>
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

impl<A, Heap> CoVisitor<Cstfy<Heap, A>> for Parser<'_>
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

impl<A, Heap> term::co_case_split::AdmitNoMatchingCase<Cstfy<Heap, A>> for Parser<'_>
where
    A: ParseLL,
{
    fn no_matching_case(
        &self,
        _heap: &mut <Cstfy<Heap, A> as term::Heaped>::Heap,
    ) -> Cstfy<Heap, A> {
        tymetafuncspec_core::Either::Right(
            std_parse_error::ParseError::new(parse::ParseError::UnexpectedToken(
                UnexpectedTokenError {
                    at: self.position.into(),
                },
            )),
            std::marker::PhantomData,
        )
    }
}

impl<A, Heap> term::co_case_split::AdmitNoMatchingCase<CstfyTransparent<Heap, A>> for Parser<'_>
where
    A: ParseLL,
{
    fn no_matching_case(
        &self,
        _heap: &mut <CstfyTransparent<Heap, A> as term::Heaped>::Heap,
    ) -> CstfyTransparent<Heap, A> {
        tymetafuncspec_core::Either::Right(
            std_parse_error::ParseError::new(parse::ParseError::UnexpectedToken(
                UnexpectedTokenError {
                    at: self.position.into(),
                },
            )),
            std::marker::PhantomData,
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
