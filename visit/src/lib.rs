// see https://github.com/rust-lang/rust/pull/108033
#![allow(internal_features)]
#![feature(rustc_attrs)]

pub mod skip_visit;
pub mod visiteventsink;

#[rustc_coinductive]
pub trait Visit<TyMetadata, T, Heap, L> {
    fn visit(&mut self, heap: &Heap, t: &T);
}

pub(crate) mod helper_traits {
    #[rustc_coinductive]
    pub(crate) trait AllVisit<T, Heap, L, Case, TyMetadata> {
        fn all_visit(&mut self, heap: &Heap, case: &Case, idx: u32);
    }
    #[rustc_coinductive]
    pub(crate) trait AnyVisit<T, Heap, L, RemainingCases, TyMetadata> {
        fn any_visit(&mut self, heap: &Heap, t: &T);
    }
}

mod impls {
    use ccf::CanonicallyConstructibleFrom;
    use conslist::{ConsList, NonemptyConsList};
    use pmsp::{AdtMetadata, NonemptyStrategy, StrategyOf, TyMetadataOf};

    use crate::{
        Visit,
        helper_traits::{AllVisit, AnyVisit},
        visiteventsink::VisitEventSink,
    };

    impl<V, T, Heap, L> Visit<AdtMetadata, T, Heap, L> for V
    where
        T: words::Implements<Heap, L>,
        <T as words::Implements<Heap, L>>::LWord: pmsp::NamesPatternMatchStrategyGivenContext<Heap>,
        V: AnyVisit<T, Heap, L, StrategyOf<T, Heap, L>, TyMetadataOf<T, Heap, L>>,
    {
        fn visit(&mut self, heap: &Heap, t: &T) {
            <V as AnyVisit<_, _, _, _, _>>::any_visit(self, heap, t)
        }
    }

    impl<V, T: Copy, Heap, L, RemainingCases, RemainingTyMetadatas>
        AnyVisit<T, Heap, L, RemainingCases, RemainingTyMetadatas> for V
    where
        T: CanonicallyConstructibleFrom<Heap, RemainingCases::Car>,
        RemainingCases: NonemptyStrategy,
        RemainingTyMetadatas: NonemptyStrategy,
        V: AnyVisit<T, Heap, L, RemainingCases::Cdr, RemainingTyMetadatas::Cdr>,
        V: AllVisit<T, Heap, L, RemainingCases::Car, RemainingTyMetadatas::Car>,
        V: VisitEventSink<T, Heap>,
    {
        fn any_visit(&mut self, heap: &Heap, t: &T) {
            if <T as CanonicallyConstructibleFrom<Heap, RemainingCases::Car>>::deconstruct_succeeds(
                t, heap,
            ) {
                self.push(heap, t, <RemainingCases::Car as ConsList>::LENGTH);
                let car =
                    <T as CanonicallyConstructibleFrom<Heap, RemainingCases::Car>>::deconstruct(
                        *t, heap,
                    );
                self.all_visit(heap, &car, 0);
                self.pop(<RemainingCases::Car as ConsList>::LENGTH);
            } else {
                <V as AnyVisit<_, _, _, RemainingCases::Cdr, RemainingTyMetadatas::Cdr>>::any_visit(
                    self, heap, t,
                );
            }
        }
    }

    impl<V, T, Case: Copy, Heap, L, TyMetadatas> AllVisit<T, Heap, L, Case, TyMetadatas> for V
    where
        Case: NonemptyConsList,
        TyMetadatas: NonemptyConsList,
        V: AllVisit<T, Heap, L, Case::Cdr, TyMetadatas::Cdr>,
        V: Visit<TyMetadatas::Car, Case::Car, Heap, L>,
        V: VisitEventSink<T, Heap>,
    {
        fn all_visit(&mut self, heap: &Heap, case: &Case, idx: u32) {
            let (car, cdr) = case.deconstruct();
            self.visit(heap, &car);
            self.proceed(idx, idx + Case::LENGTH);
            self.all_visit(heap, &cdr, idx + 1);
        }
    }

    mod base_cases {

        use crate::{
            helper_traits::{AllVisit, AnyVisit},
            visiteventsink::VisitEventSink,
        };

        impl<Visitor, T, Heap, L> AllVisit<T, Heap, L, (), ()> for Visitor {
            fn all_visit(&mut self, _heap: &Heap, _case: &(), _idx: u32) {
                // do nothing
            }
        }
        impl<V, T, Heap, L> AnyVisit<T, Heap, L, (), ()> for V
        where
            V: VisitEventSink<T, Heap>,
        {
            fn any_visit(&mut self, _heap: &Heap, _t: &T) {
                self.deconstruction_failure()
            }
        }
    }
}
