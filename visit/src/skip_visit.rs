use aspect::VisitationAspect;
use ccf::CanonicallyConstructibleFrom;
use pmsp::StrategyOf;

use crate::{
    helper_traits::AnyVisit,
    visiteventsink::{AspectVisitor, PopOrProceed, VisitEventSink},
};

pub trait SkipVisit<LWord, L, T, Heap, AdtLikeOrNot> {
    fn skip_visit(&mut self, heap: &Heap, t: &T);
}

pub struct SkipVisitor<V>(pub V);

impl<V> AspectVisitor for SkipVisitor<V> {
    type A = VisitationAspect;
}

impl<V, T, Heap> VisitEventSink<T, Heap> for SkipVisitor<V> {
    fn push(&mut self, heap: &Heap, t: &T, total: u32) -> PopOrProceed {
        todo!()
    }

    fn proceed(&mut self, idx: u32, total: u32) -> PopOrProceed {
        todo!()
    }

    fn pop(&mut self, total: u32) {
        todo!()
    }

    fn deconstruction_failure(&mut self) {
        todo!()
    }
}

// // struct SkipVisitable<T>(T);

// // impl<Heap, Case, T> CanonicallyConstructibleFrom<Heap, Case> for SkipVisitable<T>
// // where
// //     T: CanonicallyConstructibleFrom<Heap, Case>,
// // {
// //     fn construct(heap: &mut Heap, t: Case) -> Self {
// //         SkipVisitable(T::construct(heap, t))
// //     }

// //     fn deconstruct_succeeds(&self, heap: &Heap) -> bool {
// //         T::deconstruct_succeeds(&self.0, heap)
// //     }

// //     fn deconstruct(self, heap: &Heap) -> Case {
// //         T::deconstruct(self.0, heap)
// //     }
// // }

// // impl<T, U, Heap> VisitEventSink<SkipVisitable<U>, Heap> for T {
// //     fn push(&mut self, _heap: &Heap, _t: &SkipVisitable<U>, _total: u32) -> PopOrProceed {
// //         PopOrProceed::Proceed
// //     }

// //     fn proceed(&mut self, _idx: u32, _total: u32) -> PopOrProceed {
// //         PopOrProceed::Proceed
// //     }

// //     fn pop(&mut self, _total: u32) {}

// //     fn deconstruction_failure(&mut self) {}
// // }

// // impl<V, T, Heap, L> SkipVisit<AdtMetadata, T, Heap, L> for V
// // where
// //     T: words::Implements<Heap, L> + Copy,
// //     <T as words::Implements<Heap, L>>::LWord: pmsp::NamesPatternMatchStrategyGivenContext<Heap>,
// //     V: AnyVisit<SkipVisitable<T>, Heap, L, StrategyOf<T, Heap, L>, TyMetadataOf<T, Heap, L>>,
// // {
// //     fn skip_visit(&mut self, heap: &Heap, t: &T) {
// //         <V as AnyVisit<_, _, _, _, _>>::any_visit(self, heap, &SkipVisitable(*t))
// //     }
// // }

// /// unfortunately this essentially duplicates the visit impls, except without
// /// triggering any [VisitEventSink] events
// pub(crate) mod helper_traits {
//     #[rustc_coinductive]
//     pub(crate) trait AllSkipVisit<LWord, L, T, Heap, ConcreteCase> {
//         fn all_visit(&mut self, heap: &Heap, case: &ConcreteCase, idx: u32);
//     }
//     #[rustc_coinductive]
//     pub(crate) trait AnySkipVisit<LWord, L, T, Heap, RemainingCases> {
//         fn any_visit(&mut self, heap: &Heap, t: &T);
//     }
// }

// mod impls {
//     use ccf::CanonicallyConstructibleFrom;
//     use conslist::NonemptyConsList;
//     use pmsp::{NonemptyStrategy, StrategyOf, Visitation};
//     use words::{AdtLike, Adtishness, Implements, InverseImplementsAll};

//     use crate::{
//         Visit,
//         skip_visit::{
//             SkipVisit,
//             helper_traits::{AllSkipVisit, AnySkipVisit},
//         },
//     };

//     impl<V, LWord, L, T, Heap> SkipVisit<LWord, L, T, Heap, AdtLike> for V
//     where
//         T: words::Implements<Heap, L>,
//         <T as words::Implements<Heap, L>>::LWord: pmsp::NamesPatternMatchStrategy<L>,
//         V: AnySkipVisit<LWord, L, T, Heap, StrategyOf<T, Heap, L>>,
//     {
//         fn skip_visit(&mut self, heap: &Heap, t: &T) {
//             <V as AnySkipVisit<_, _, _, _, _>>::any_visit(self, heap, t)
//         }
//     }

//     impl<V, LWord, L, T, Heap, RemainingCases> AnySkipVisit<LWord, L, T, Heap, RemainingCases> for V
//     where
//         Heap: InverseImplementsAll<L, RemainingCases::Car, aspect::VisitationAspect>,
//         T: Copy
//             + CanonicallyConstructibleFrom<
//                 Heap,
//                 <Heap as InverseImplementsAll<L, RemainingCases::Car, aspect::VisitationAspect>>::Implementors,
//             >,
//         RemainingCases: NonemptyStrategy,
//         V: AnySkipVisit<LWord, L, T, Heap, RemainingCases::Cdr>,
//         V: AllSkipVisit<
//                 LWord,
//                 L,
//                 T,
//                 Heap,
//                 <Heap as InverseImplementsAll<L, RemainingCases::Car, aspect::VisitationAspect>>::Implementors,
//             >,
//     {
//         fn any_visit(&mut self, heap: &Heap, t: &T) {
//             if <T as CanonicallyConstructibleFrom<Heap, _>>::deconstruct_succeeds(t, heap) {
//                 let car = <T as CanonicallyConstructibleFrom<Heap, _>>::deconstruct(*t, heap);
//                 self.all_visit(heap, &car, 0);
//             } else {
//                 <V as AnySkipVisit<_, _, _, _, RemainingCases::Cdr>>::any_visit(self, heap, t);
//             }
//         }
//     }

//     impl<V, ConcreteCase: Copy, LWord, L, T, Heap> AllSkipVisit<LWord, L, T, Heap, ConcreteCase> for V
//     where
//         ConcreteCase: NonemptyConsList,
//         V: AllSkipVisit<LWord, L, T, Heap, ConcreteCase::Cdr>,
//         // Heap: InverseImplements<L, Case::Car>,
//         ConcreteCase::Car: Implements<Heap, L>,
//         <ConcreteCase::Car as Implements<Heap, L>>::LWord: Adtishness<Visitation>,
//         V: Visit<
//                 <ConcreteCase::Car as Implements<Heap, L>>::LWord,
//                 L,
//                 ConcreteCase::Car,
//                 Heap,
//                 <<ConcreteCase::Car as Implements<Heap, L>>::LWord as Adtishness<Visitation>>::X,
//             >,
//     {
//         fn all_visit(&mut self, heap: &Heap, case: &ConcreteCase, idx: u32) {
//             let (car, cdr) = case.deconstruct();
//             self.visit(heap, &car);
//             self.all_visit(heap, &cdr, idx + 1);
//         }
//     }

//     mod base_cases {

//         use crate::skip_visit::helper_traits::{AllSkipVisit, AnySkipVisit};

//         impl<Visitor, LWord, L, T, Heap> AllSkipVisit<LWord, L, T, Heap, ()> for Visitor {
//             fn all_visit(&mut self, _heap: &Heap, _case: &(), _idx: u32) {
//                 // do nothing
//             }
//         }
//         impl<V, LWord, L, T, Heap> AnySkipVisit<LWord, L, T, Heap, ()> for V {
//             fn any_visit(&mut self, _heap: &Heap, _t: &T) {
//                 panic!("no skip visit case matched")
//             }
//         }
//     }
// }
