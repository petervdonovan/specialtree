// use crate::{CanonicallyConstructibleFrom, Heaped};

// #[fundamental]
// pub trait Adt: Sized {
//     type PatternMatchStrategyProvider: HasPatternMatchStrategyFor<Self>;
// }

// pub trait ConsList {
//     type Car;
//     type Cdr: ConsList;
//     const LENGTH: u32;
//     fn deconstruct(self) -> (Self::Car, Self::Cdr);
// }
// impl ConsList for () {
//     type Car = ();
//     type Cdr = ();
//     const LENGTH: u32 = 0;
//     fn deconstruct(self) -> (Self::Car, Self::Cdr) {
//         ((), ())
//     }
// }
// #[fundamental]
// pub trait NonemptyConsList: ConsList {}
// impl<Car, Cdr> NonemptyConsList for (Car, Cdr) where Cdr: ConsList {}
// #[fundamental]
// pub trait AtLeastTwoConsList: ConsList {}
// impl<Car, Cdr> AtLeastTwoConsList for (Car, Cdr) where Cdr: NonemptyConsList {}
// impl<T, Cdr> ConsList for (T, Cdr)
// where
//     Cdr: ConsList,
// {
//     type Car = T;
//     type Cdr = Cdr;
//     const LENGTH: u32 = Cdr::LENGTH + 1;
//     fn deconstruct(self) -> (Self::Car, Self::Cdr) {
//         self
//     }
// }
// pub trait CaseSplittable<F: HasBorrowedHeapRef, Heap, CcfConsList: ConsList>: Sized {
//     fn case_split(&self, callable: &mut F, heap: &mut F::Borrowed<'_, Heap>);
// }
// pub trait HasBorrowedHeapRef {
//     type Borrowed<'a, Heap: 'a>: 'a;
//     fn with_decayed_heapref<Heap, T, F: Fn(&Heap) -> T>(
//         heap: &mut Self::Borrowed<'_, Heap>,
//         f: F,
//     ) -> T;
// }
// pub trait AdmitNoMatchingCase<Heap, T>: HasBorrowedHeapRef {
//     fn no_matching_case(&self, t: &T, heap: &mut Self::Borrowed<'_, Heap>);
// }
// pub trait Callable<Heap, Case>: HasBorrowedHeapRef {
//     fn call(&mut self, t: Case, heap: &mut Self::Borrowed<'_, Heap>);
// }
// impl<Heap, T, F: AdmitNoMatchingCase<Heap, T> + Heaped<Heap = Heap>> CaseSplittable<F, Heap, ()>
//     for T
// {
//     fn case_split(&self, callable: &mut F, heap: &mut F::Borrowed<'_, Heap>) {
//         callable.no_matching_case(self, heap);
//     }
// }
// impl<F, Heap, T, CasesCar, CasesCdr: ConsList> CaseSplittable<F, Heap, (CasesCar, CasesCdr)> for T
// where
//     F: Callable<Heap, CasesCar> + Heaped<Heap = Heap>,
//     T: CanonicallyConstructibleFrom<Heap, CasesCar>,
//     T: CaseSplittable<F, Heap, CasesCdr>,
//     T: Copy,
// {
//     fn case_split(
//         &self,
//         callable: &mut F,
//         heap: &mut <F as HasBorrowedHeapRef>::Borrowed<'_, Heap>,
//     ) {
//         if <F as HasBorrowedHeapRef>::with_decayed_heapref(heap, |hr| {
//             <Self as CanonicallyConstructibleFrom<Heap, CasesCar>>::deconstruct_succeeds(self, hr)
//         }) {
//             let t = <F as HasBorrowedHeapRef>::with_decayed_heapref(heap, |hr| {
//                 <Self as CanonicallyConstructibleFrom<Heap, CasesCar>>::deconstruct(*self, hr)
//             });
//             callable.call(t, heap);
//         } else {
//             <Self as CaseSplittable<F, Heap, CasesCdr>>::case_split(self, callable, heap);
//         }
//     }
// }
// pub trait HasPatternMatchStrategyFor<T> {
//     type Strategy: ConsList;
// }
// // pub trait HasPatternMatchStrategyForWord<T> {
// //     type Strategy: ConsList;
// // }
// pub trait NamesPatternMatchStrategyGivenContext<Heap> {
//     type Strategy: ConsList;
// }
// // impl<Heap, T, U> HasPatternMatchStrategyFor<T> for U
// // where
// //     T: Implements<Heap, wmf::L>,
// //     U: HasPatternMatchStrategyForWord<T::LWord>,
// // {
// //     type Strategy = <U as HasPatternMatchStrategyForWord<T::LWord>>::Strategy;
// // }
// pub trait PatternMatchable<F: HasBorrowedHeapRef, Heap, Strategies> {
//     fn do_match(&self, callable: &mut F, heap: &mut F::Borrowed<'_, Heap>);
// }
// impl<F, Heap, T, StrategyProvider> PatternMatchable<F, Heap, StrategyProvider> for T
// where
//     StrategyProvider: HasPatternMatchStrategyFor<T>,
//     F: HasBorrowedHeapRef + Heaped<Heap = Heap>,
//     T: CaseSplittable<F, Heap, StrategyProvider::Strategy>,
// {
//     fn do_match(&self, callable: &mut F, heap: &mut F::Borrowed<'_, Heap>) {
//         <Self as CaseSplittable<F, Heap, StrategyProvider::Strategy>>::case_split(
//             self, callable, heap,
//         );
//     }
// }
