use ccf::CanonicallyConstructibleFrom;
use pmsp::{AdtMetadata, StrategyOf, TyMetadataOf};

use crate::{
    helper_traits::AnyVisit,
    visiteventsink::{PopOrProceed, VisitEventSink},
};

pub trait SkipVisit<TyMetadata, T, Heap, L> {
    fn skip_visit(&mut self, heap: &Heap, t: &T);
}

// struct SkipVisitable<T>(T);

// impl<Heap, Case, T> CanonicallyConstructibleFrom<Heap, Case> for SkipVisitable<T>
// where
//     T: CanonicallyConstructibleFrom<Heap, Case>,
// {
//     fn construct(heap: &mut Heap, t: Case) -> Self {
//         SkipVisitable(T::construct(heap, t))
//     }

//     fn deconstruct_succeeds(&self, heap: &Heap) -> bool {
//         T::deconstruct_succeeds(&self.0, heap)
//     }

//     fn deconstruct(self, heap: &Heap) -> Case {
//         T::deconstruct(self.0, heap)
//     }
// }

// impl<T, U, Heap> VisitEventSink<SkipVisitable<U>, Heap> for T {
//     fn push(&mut self, _heap: &Heap, _t: &SkipVisitable<U>, _total: u32) -> PopOrProceed {
//         PopOrProceed::Proceed
//     }

//     fn proceed(&mut self, _idx: u32, _total: u32) -> PopOrProceed {
//         PopOrProceed::Proceed
//     }

//     fn pop(&mut self, _total: u32) {}

//     fn deconstruction_failure(&mut self) {}
// }

// impl<V, T, Heap, L> SkipVisit<AdtMetadata, T, Heap, L> for V
// where
//     T: words::Implements<Heap, L> + Copy,
//     <T as words::Implements<Heap, L>>::LWord: pmsp::NamesPatternMatchStrategyGivenContext<Heap>,
//     V: AnyVisit<SkipVisitable<T>, Heap, L, StrategyOf<T, Heap, L>, TyMetadataOf<T, Heap, L>>,
// {
//     fn skip_visit(&mut self, heap: &Heap, t: &T) {
//         <V as AnyVisit<_, _, _, _, _>>::any_visit(self, heap, &SkipVisitable(*t))
//     }
// }

/// unfortunately this essentially duplicates the visit impls, except without
/// triggering any [VisitEventSink] events
pub(crate) mod helper_traits {
    #[rustc_coinductive]
    pub(crate) trait AllSkipVisit<T, Heap, L, Case, TyMetadata> {
        fn all_visit(&mut self, heap: &Heap, case: &Case, idx: u32);
    }
    #[rustc_coinductive]
    pub(crate) trait AnySkipVisit<T, Heap, L, RemainingCases, TyMetadata> {
        fn any_visit(&mut self, heap: &Heap, t: &T);
    }
}

mod impls {
    use ccf::CanonicallyConstructibleFrom;
    use conslist::NonemptyConsList;
    use pmsp::{AdtMetadata, NonemptyStrategy, StrategyOf, TyMetadataOf};

    use crate::{
        Visit,
        skip_visit::{
            SkipVisit,
            helper_traits::{AllSkipVisit, AnySkipVisit},
        },
    };

    impl<V, T, Heap, L> SkipVisit<AdtMetadata, T, Heap, L> for V
    where
        T: words::Implements<Heap, L>,
        <T as words::Implements<Heap, L>>::LWord: pmsp::NamesPatternMatchStrategyGivenContext<Heap>,
        V: AnySkipVisit<T, Heap, L, StrategyOf<T, Heap, L>, TyMetadataOf<T, Heap, L>>,
    {
        fn skip_visit(&mut self, heap: &Heap, t: &T) {
            <V as AnySkipVisit<_, _, _, _, _>>::any_visit(self, heap, t)
        }
    }

    impl<V, T: Copy, Heap, L, RemainingCases, RemainingTyMetadatas>
        AnySkipVisit<T, Heap, L, RemainingCases, RemainingTyMetadatas> for V
    where
        T: CanonicallyConstructibleFrom<Heap, RemainingCases::Car>,
        RemainingCases: NonemptyStrategy,
        RemainingTyMetadatas: NonemptyStrategy,
        V: AnySkipVisit<T, Heap, L, RemainingCases::Cdr, RemainingTyMetadatas::Cdr>,
        V: AllSkipVisit<T, Heap, L, RemainingCases::Car, RemainingTyMetadatas::Car>,
    {
        fn any_visit(&mut self, heap: &Heap, t: &T) {
            if <T as CanonicallyConstructibleFrom<Heap, RemainingCases::Car>>::deconstruct_succeeds(
                t, heap,
            ) {
                let car =
                    <T as CanonicallyConstructibleFrom<Heap, RemainingCases::Car>>::deconstruct(
                        *t, heap,
                    );
                self.all_visit(heap, &car, 0);
            } else {
                <V as AnySkipVisit<_, _, _, RemainingCases::Cdr, RemainingTyMetadatas::Cdr>>::any_visit(
                    self, heap, t,
                );
            }
        }
    }

    impl<V, T, Case: Copy, Heap, L, TyMetadatas> AllSkipVisit<T, Heap, L, Case, TyMetadatas> for V
    where
        Case: NonemptyConsList,
        TyMetadatas: NonemptyConsList,
        V: AllSkipVisit<T, Heap, L, Case::Cdr, TyMetadatas::Cdr>,
        V: Visit<TyMetadatas::Car, Case::Car, Heap, L>,
    {
        fn all_visit(&mut self, heap: &Heap, case: &Case, idx: u32) {
            let (car, cdr) = case.deconstruct();
            self.visit(heap, &car);
            self.all_visit(heap, &cdr, idx + 1);
        }
    }

    mod base_cases {

        use crate::skip_visit::helper_traits::{AllSkipVisit, AnySkipVisit};

        impl<Visitor, T, Heap, L> AllSkipVisit<T, Heap, L, (), ()> for Visitor {
            fn all_visit(&mut self, _heap: &Heap, _case: &(), _idx: u32) {
                // do nothing
            }
        }
        impl<V, T, Heap, L> AnySkipVisit<T, Heap, L, (), ()> for V {
            fn any_visit(&mut self, _heap: &Heap, _t: &T) {
                panic!("no skip visit case matched")
            }
        }
    }
}
