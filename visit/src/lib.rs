// see https://github.com/rust-lang/rust/pull/108033
#![allow(internal_features)]
#![feature(rustc_attrs)]

pub mod skip_visit;
pub mod visiteventsink;

#[rustc_coinductive]
pub trait Visit<LWord, L, T, Heap, AdtLikeOrNot> {
    fn visit(&mut self, heap: &Heap, t: &T);
}

pub(crate) mod helper_traits {
    #[rustc_coinductive]
    pub(crate) trait AllVisit<LWord, L, T, Heap, ConcreteCase> {
        fn all_visit(&mut self, heap: &Heap, case: &ConcreteCase, idx: u32);
    }
    #[rustc_coinductive]
    pub(crate) trait AnyVisit<LWord, L, T, Heap, RemainingCases> {
        fn any_visit(&mut self, heap: &Heap, t: &T);
    }
}

mod impls {
    use ccf::{CanonicallyConstructibleFrom, VisitationInfo};
    use conslist::{ConsList, NonemptyConsList};
    use pmsp::{NonemptyStrategy, StrategyOf};
    use words::{AdtLike, Implements, InverseImplementsAll};

    use crate::{
        Visit,
        helper_traits::{AllVisit, AnyVisit},
        visiteventsink::VisitEventSink,
    };

    impl<V, LWord, L, T, Heap> Visit<LWord, L, T, Heap, AdtLike> for V
    where
        T: words::Implements<Heap, L>,
        <T as words::Implements<Heap, L>>::LWord: pmsp::NamesPatternMatchStrategy<L>,
        V: AnyVisit<LWord, L, T, Heap, StrategyOf<T, Heap, L>>,
    {
        fn visit(&mut self, heap: &Heap, t: &T) {
            <V as AnyVisit<_, _, _, _, _>>::any_visit(self, heap, t)
        }
    }

    impl<V, LWord, L, T, Heap, RemainingCases> AnyVisit<LWord, L, T, Heap, RemainingCases> for V
    where
        Heap: InverseImplementsAll<L, RemainingCases::Car>,
        T: Copy
            + CanonicallyConstructibleFrom<
                Heap,
                <Heap as InverseImplementsAll<L, RemainingCases::Car>>::StructuralImplementors,
            >,
        RemainingCases: NonemptyStrategy,
        V: AnyVisit<LWord, L, T, Heap, RemainingCases::Cdr>,
        V: AllVisit<
                LWord,
                L,
                T,
                Heap,
                <Heap as InverseImplementsAll<L, RemainingCases::Car>>::StructuralImplementors,
            >,
        V: VisitEventSink<T, Heap>,
    {
        fn any_visit(&mut self, heap: &Heap, t: &T) {
            if <T as CanonicallyConstructibleFrom<Heap, _>>::deconstruct_succeeds(t, heap) {
                self.push(heap, t, <RemainingCases::Car as ConsList>::LENGTH);
                let car = <T as CanonicallyConstructibleFrom<Heap, _>>::deconstruct(*t, heap);
                self.all_visit(heap, &car, 0);
                self.pop(<RemainingCases::Car as ConsList>::LENGTH);
            } else {
                <V as AnyVisit<_, _, _, _, RemainingCases::Cdr>>::any_visit(self, heap, t);
            }
        }
    }

    impl<V, ConcreteCase: Copy, LWord, L, T, Heap> AllVisit<LWord, L, T, Heap, ConcreteCase> for V
    where
        ConcreteCase: NonemptyConsList,
        V: AllVisit<LWord, L, T, Heap, ConcreteCase::Cdr>,
        // Heap: InverseImplements<L, Case::Car>,
        ConcreteCase::Car: Implements<Heap, L>,
        <ConcreteCase::Car as Implements<Heap, L>>::LWord: VisitationInfo,
        V: Visit<
                <ConcreteCase::Car as Implements<Heap, L>>::LWord,
                L,
                ConcreteCase::Car,
                Heap,
                <<ConcreteCase::Car as Implements<Heap, L>>::LWord as VisitationInfo>::AdtLikeOrNot,
            >,
        V: VisitEventSink<T, Heap>,
    {
        fn all_visit(&mut self, heap: &Heap, case: &ConcreteCase, idx: u32) {
            let (car, cdr) = case.deconstruct();
            self.visit(heap, &car);
            self.proceed(idx, idx + ConcreteCase::LENGTH);
            self.all_visit(heap, &cdr, idx + 1);
        }
    }

    mod base_cases {

        use crate::{
            helper_traits::{AllVisit, AnyVisit},
            visiteventsink::VisitEventSink,
        };

        impl<Visitor, LWord, L, T, Heap> AllVisit<LWord, L, T, Heap, ()> for Visitor {
            fn all_visit(&mut self, _heap: &Heap, _case: &(), _idx: u32) {
                // do nothing
            }
        }
        impl<V, LWord, L, T, Heap> AnyVisit<LWord, L, T, Heap, ()> for V
        where
            V: VisitEventSink<T, Heap>,
        {
            fn any_visit(&mut self, _heap: &Heap, _t: &T) {
                self.deconstruction_failure()
            }
        }
    }
}
