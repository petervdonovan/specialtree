// use crate::{
//     CanonicallyConstructibleFrom, Heaped,
//     case_split::ConsList,
//     select::{AcceptingCases, InSelectCase},
// };

// pub trait CoCaseSplittable<F, Heap, CcfConsList>: Sized
// where
//     F: InSelectCase,
// {
//     fn co_case_split(callable: F, heap: &mut Heap) -> (Self, F::EndSelectCase);
// }

// pub trait CoCallable<Heap, Case> {
//     fn call(&mut self, heap: &mut Heap) -> Case;
// }
// pub trait AdmitNoMatchingCase<Heap, T>
// where
//     Self: InSelectCase,
// {
//     fn no_matching_case(&self, heap: &mut Heap) -> (T, Self::EndSelectCase);
// }
// impl<F, Heap, T> CoCaseSplittable<F, Heap, ()> for T
// where
//     F: AdmitNoMatchingCase<Heap, T> + InSelectCase,
// {
//     fn co_case_split(callable: F, heap: &mut Heap) -> (Self, F::EndSelectCase) {
//         callable.no_matching_case(heap)
//     }
// }

// impl<F, Heap, T, CasesCar, CasesCdr> CoCaseSplittable<F, Heap, (CasesCar, CasesCdr)> for T
// where
//     CasesCdr: ConsList,
//     T: Copy,
//     T: CanonicallyConstructibleFrom<Heap, CasesCar>,
//     F: AdmitNoMatchingCase<Heap, T>,
//     F: AcceptingCases<(CasesCar, CasesCdr)>,
//     <F as InSelectCase>::EndSelectCase: CoCallable<Heap, CasesCar>, //  + AcceptingCases<(CasesCar, CasesCdr)>
//     T: CoCaseSplittable<
//             <F as AcceptingCases<(CasesCar, CasesCdr)>>::AcceptingRemainingCases,
//             Heap,
//             CasesCdr,
//         >,
// {
//     fn co_case_split(callable: F, heap: &mut Heap) -> (Self, F::EndSelectCase) {
//         match <F as AcceptingCases<(CasesCar, CasesCdr)>>::try_case(callable) {
//             Ok(mut short_circuited) => {
//                 let ok = short_circuited.call(heap);
//                 (T::construct(heap, ok), short_circuited)
//             }
//             Err(arc) => <T as CoCaseSplittable<
//                 <F as AcceptingCases<(CasesCar, CasesCdr)>>::AcceptingRemainingCases,
//                 Heap,
//                 CasesCdr,
//             >>::co_case_split(arc, heap),
//         }
//     }
// }
// // impl<F, Heap, T, CasesCar, CasesCdr> CoCaseSplittable<F, Heap, (CasesCar, CasesCdr)> for T
// // where
// //     CasesCdr: ConsList,
// //     T: Copy,
// //     T: CanonicallyConstructibleFrom<Heap, CasesCar>,
// //     F: AdmitNoMatchingCase<Heap, T>,
// //     F: AcceptingCases<(CasesCar, CasesCdr)>,
// //     // <F as FromSelectCase>::ShortCircuitsTo: CoCallable<Heap, CasesCar>, //  + AcceptingCases<(CasesCar, CasesCdr)>
// //     // T: CoCaseSplittable<
// //     //         <F as AcceptingCases<(CasesCar, CasesCdr)>>::AcceptingRemainingCases,
// //     //         Heap,
// //     //         CasesCdr,
// //     //     >,
// // {
// //     fn co_case_split(callable: F, heap: &mut Heap) -> (Self, F::ShortCircuitsTo) {
// //         match <F as AcceptingCases<(CasesCar, CasesCdr)>>::try_case(callable) {
// //             Ok(mut short_circuited) => {
// //                 // let ok = short_circuited.call(heap);
// //                 // (T::construct(heap, ok), short_circuited)
// //                 todo!()
// //             }
// //             Err(arc) => todo!(),
// //         }
// //     }
// // }
