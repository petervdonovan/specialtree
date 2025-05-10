// see https://github.com/rust-lang/rust/pull/108033
#![allow(internal_features)]
#![feature(rustc_attrs)]

pub mod visiteventsink;

#[rustc_coinductive]
pub trait Visit<T, Heap, L> {
    fn visit(&mut self, heap: &mut Heap, t: &T);
}

pub(crate) mod helper_traits {
    #[rustc_coinductive]
    pub(crate) trait AllVisit<T, Case, Heap, L> {
        fn all_visit(&mut self, heap: &mut Heap, case: &Case);
    }
    #[rustc_coinductive]
    pub(crate) trait AnyVisit<T, Heap, L, RemainingCases> {
        fn any_visit(&mut self, heap: &mut Heap, t: &T);
    }
}

mod impls {
    use ccf::CanonicallyConstructibleFrom;
    use conslist::NonemptyConsList;
    use pmsp::{NonemptyStrategy, StrategyOf, UsesStrategyForTraversal};

    use crate::{
        Visit,
        helper_traits::{AllVisit, AnyVisit},
        visiteventsink::VisitEventSink,
    };

    impl<V, T, Heap, L> Visit<T, Heap, L> for V
    where
        T: words::Implements<Heap, L> + UsesStrategyForTraversal<V>,
        <T as words::Implements<Heap, L>>::LWord: pmsp::NamesPatternMatchStrategyGivenContext<Heap>,
        V: AnyVisit<T, Heap, L, StrategyOf<T, Heap, L>>,
    {
        fn visit(&mut self, heap: &mut Heap, t: &T) {
            <V as AnyVisit<_, _, _, _>>::any_visit(self, heap, t)
        }
    }

    impl<V, T: Copy, Heap, L, RemainingCases> AnyVisit<T, Heap, L, RemainingCases> for V
    where
        T: CanonicallyConstructibleFrom<Heap, RemainingCases::Car>,
        RemainingCases: NonemptyStrategy,
        V: AnyVisit<T, Heap, L, RemainingCases::Cdr>,
        V: AllVisit<T, RemainingCases::Car, Heap, L>,
        V: VisitEventSink<T, Heap>,
    {
        fn any_visit(&mut self, heap: &mut Heap, t: &T) {
            if <T as CanonicallyConstructibleFrom<Heap, RemainingCases::Car>>::deconstruct_succeeds(
                t, heap,
            ) {
                self.push(heap, t);
                let car =
                    <T as CanonicallyConstructibleFrom<Heap, RemainingCases::Car>>::deconstruct(
                        *t, heap,
                    );
                self.all_visit(heap, &car);
                self.pop();
            } else {
                <V as AnyVisit<T, Heap, L, RemainingCases::Cdr>>::any_visit(self, heap, t);
            }
        }
    }

    impl<V, T, Case: Copy, Heap, L> AllVisit<T, Case, Heap, L> for V
    where
        Case: NonemptyConsList,
        V: AllVisit<T, Case::Cdr, Heap, L>,
        V: Visit<Case::Car, Heap, L>,
        V: VisitEventSink<T, Heap>,
    {
        fn all_visit(&mut self, heap: &mut Heap, case: &Case) {
            let (car, cdr) = case.deconstruct();
            self.visit(heap, &car);
            self.proceed();
            self.all_visit(heap, &cdr);
        }
    }

    mod base_cases {
        use crate::{
            helper_traits::{AllVisit, AnyVisit},
            visiteventsink::VisitEventSink,
        };

        impl<Visitor, T, Heap, L> AllVisit<T, (), Heap, L> for Visitor {
            fn all_visit(&mut self, _heap: &mut Heap, _case: &()) {
                // do nothing
            }
        }
        impl<V, T, Heap, L> AnyVisit<T, Heap, L, ()> for V
        where
            V: VisitEventSink<T, Heap>,
        {
            fn any_visit(&mut self, _heap: &mut Heap, _t: &T) {
                self.deconstruction_failure()
            }
        }
    }
}
