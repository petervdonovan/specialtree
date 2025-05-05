// use take_mut::Poisonable;

// use crate::{
//     case_split::ConsList,
//     co_case_split::{AdmitNoMatchingCase, CoCallable, CoCaseSplittable},
//     select::{AcceptingCases, InSelectCase, SelectCase},
// };

// pub trait CoVisitor<T> {
//     fn co_push(&mut self);
//     fn co_proceed(&mut self);
//     fn co_pop(&mut self);
// }

// pub trait CoVisitable<CV, PatternMatchStrategyProvider, Heap, DepthFuel, Fnlut>: Sized {
//     fn co_visit(visitor: &mut CV, heap: &mut Heap, fnlut: Fnlut) -> Self;
// }

// pub trait AllCoVisitable<Heap, CV, Ctx, PatternMatchStrategyProvider, DepthFuel, Fnlut>:
//     Sized
// {
//     fn all_co_visit(visitor: &mut CV, heap: &mut Heap, fnlut: Fnlut) -> Self;
// }

// impl<CV, Ctx, Heap, StrategyProvider, DepthFuel, Fnlut: Copy, FieldsCar, FieldsCdrCar, FieldsCdrCdr>
//     AllCoVisitable<Heap, CV, Ctx, StrategyProvider, DepthFuel, Fnlut>
//     for (FieldsCar, (FieldsCdrCar, FieldsCdrCdr))
// where
//     FieldsCar: CoVisitable<CV, StrategyProvider, Heap, DepthFuel, Fnlut>,
//     // Fnlut: HasFn<FieldsCar, for<'a, 'b> fn(&'a mut CV, &'b mut Heap, Fnlut) -> FieldsCar>,
//     (FieldsCdrCar, FieldsCdrCdr): AllCoVisitable<Heap, CV, Ctx, StrategyProvider, DepthFuel, Fnlut>,
// {
//     fn all_co_visit(visitor: &mut CV, heap: &mut Heap, fnlut: Fnlut) -> Self {
//         // let car = (fnlut.get::<FieldsCar>())(visitor, heap, fnlut);
//         let car = FieldsCar::co_visit(visitor, heap, fnlut);
//         // visitor.co_proceed();
//         let cdr = <(FieldsCdrCar, FieldsCdrCdr)>::all_co_visit(visitor, heap, fnlut);
//         (car, cdr)
//     }
// }

// impl<CV, Ctx, Heap, StrategyProvider, FieldsCar, DepthFuel, Fnlut>
//     AllCoVisitable<Heap, CV, Ctx, StrategyProvider, DepthFuel, Fnlut> for (FieldsCar, ())
// where
//     FieldsCar: CoVisitable<CV, StrategyProvider, Heap, DepthFuel, Fnlut>,
// {
//     fn all_co_visit(visitor: &mut CV, heap: &mut Heap, fnlut: Fnlut) -> Self {
//         let car = FieldsCar::co_visit(visitor, heap, fnlut);
//         (car, ())
//     }
// }

// struct CoCallablefyCoVisitor<CV, MatchedType, StrategyProvider, DepthFuel, Fnlut> {
//     cv: CV,
//     fnlut: Fnlut,
//     phantom: std::marker::PhantomData<(MatchedType, StrategyProvider, DepthFuel)>,
// }

// impl<CV, MatchedType, StrategyProvider, DepthFuel, Fnlut>
//     CoCallablefyCoVisitor<CV, MatchedType, StrategyProvider, DepthFuel, Fnlut>
// {
//     fn new(cv: CV, fnlut: Fnlut) -> Self {
//         Self {
//             cv,
//             fnlut,
//             phantom: std::marker::PhantomData,
//         }
//     }
// }

// // impl<V: Heaped, MatchedType, StrategyProvider, DepthFuel, Fnlut> Heaped
// //     for CoCallablefyCoVisitor<V, MatchedType, StrategyProvider, DepthFuel, Fnlut>
// // {
// //     type Heap = V::Heap;
// // }

// impl<V, MatchedType, Heap, CaseCar, CaseCdr, StrategyProvider, DepthFuel, Fnlut: Copy>
//     CoCallable<Heap, (CaseCar, CaseCdr)>
//     for CoCallablefyCoVisitor<V, MatchedType, StrategyProvider, DepthFuel, Fnlut>
// where
//     MatchedType: Copy,
//     (CaseCar, CaseCdr):
//         Copy + AllCoVisitable<Heap, V, MatchedType, StrategyProvider, DepthFuel, Fnlut>,
// {
//     fn call(&mut self, heap: &mut Heap) -> (CaseCar, CaseCdr) {
//         <(CaseCar, CaseCdr)>::all_co_visit(&mut self.cv, heap, self.fnlut)
//     }
// }

// impl<Heap, T, V, M, S, D, F> AdmitNoMatchingCase<Heap, T> for CoCallablefyCoVisitor<V, M, S, D, F>
// where
//     V: AdmitNoMatchingCase<Heap, T>,
//     F: Copy,
// {
//     fn no_matching_case(&self, heap: &mut Heap) -> (T, Self::EndSelectCase) {
//         let (t, short) = self.cv.no_matching_case(heap);
//         (t, CoCallablefyCoVisitor::new(short, self.fnlut))
//     }
// }

// impl<CV, T, PatternMatchStrategyProvider, DepthFuel, Fnlut> InSelectCase
//     for CoCallablefyCoVisitor<CV, T, PatternMatchStrategyProvider, DepthFuel, Fnlut>
// where
//     CV: InSelectCase,
// {
//     type EndSelectCase =
//         CoCallablefyCoVisitor<CV::EndSelectCase, T, PatternMatchStrategyProvider, DepthFuel, Fnlut>;
// }

// impl<CV, T, PatternMatchStrategyProvider, Cases, DepthFuel, Fnlut> AcceptingCases<Cases>
//     for CoCallablefyCoVisitor<CV, T, PatternMatchStrategyProvider, DepthFuel, Fnlut>
// where
//     Cases: ConsList,
//     CV: AcceptingCases<Cases> + InSelectCase,
// {
//     type AcceptingRemainingCases = CoCallablefyCoVisitor<
//         CV::AcceptingRemainingCases,
//         T,
//         PatternMatchStrategyProvider,
//         DepthFuel,
//         Fnlut,
//     >;

//     fn try_case(self) -> Result<Self::EndSelectCase, Self::AcceptingRemainingCases> {
//         match self.cv.try_case() {
//             Ok(short_circuited) => Ok(CoCallablefyCoVisitor::new(short_circuited, self.fnlut)),
//             Err(remaining_cases) => {
//                 let remaining_cases = CoCallablefyCoVisitor {
//                     cv: remaining_cases,
//                     fnlut: self.fnlut,
//                     phantom: std::marker::PhantomData,
//                 };
//                 Err(remaining_cases)
//             }
//         }
//     }
// }

// impl<Heap, CV, PatternMatchStrategyProvider, T, DepthFuelUpperBits, DepthFuelLastBit, Fnlut>
//     CoVisitable<
//         CV,
//         PatternMatchStrategyProvider,
//         Heap,
//         typenum::UInt<DepthFuelUpperBits, DepthFuelLastBit>,
//         Fnlut,
//     > for T
// where
//     typenum::UInt<DepthFuelUpperBits, DepthFuelLastBit>: std::ops::Sub<typenum::B1>,
//     T: CoCaseSplittable<
//             CoCallablefyCoVisitor<
//                 CV::AC<PatternMatchStrategyProvider::Strategy>,
//                 T,
//                 PatternMatchStrategyProvider,
//                 typenum::Sub1<typenum::UInt<DepthFuelUpperBits, DepthFuelLastBit>>,
//                 Fnlut,
//             >,
//             Heap,
//             PatternMatchStrategyProvider::Strategy,
//         > + Copy,
//     CV: CoVisitor<T> + SelectCase + Poisonable,
//     CV::AC<PatternMatchStrategyProvider::Strategy>: AdmitNoMatchingCase<Heap, T>,
//     PatternMatchStrategyProvider: crate::case_split::HasPatternMatchStrategyFor<T>,
// {
//     // todo: consider optimizing by adding a type parameter that provides the fnlut.
//     // you would need to verify experimentally that this enables the compiler to
//     // optimize out and inline the function pointer.
//     fn co_visit(visitor: &mut CV, heap: &mut Heap, fnlut: Fnlut) -> Self {
//         visitor.co_push();
//         let new_ret = take_mut::take(visitor, move |visitor| {
//             let visitor_co = visitor.start_cases();
//             let callable = CoCallablefyCoVisitor {
//                 cv: visitor_co,
//                 fnlut,
//                 phantom: std::marker::PhantomData,
//             };
//             let (new_ret, short) = <T as CoCaseSplittable<_, _, _>>::co_case_split(callable, heap);
//             (short.cv, new_ret)
//         });
//         visitor.co_pop();
//         new_ret
//     }
// }

// pub type CoVisitFn<This, CV, Heap, Fnlut> = fn(&mut CV, &mut Heap, Fnlut) -> This;
