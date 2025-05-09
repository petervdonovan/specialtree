// see https://github.com/rust-lang/rust/pull/108033
#![allow(internal_features)] // this is research-quality software!
#![feature(rustc_attrs)]
#![feature(fundamental)]

// want: foreign crates can implement covisit for their concrete type and for a specific covisitor (no generics required), and
// foreign crates can implement a specific covisitor for all ADTs

pub mod covisiteventsink;
pub mod select;

#[rustc_coinductive]
pub trait Covisit<T, Heap, L> {
    fn covisit(&mut self, heap: &mut Heap) -> T;
}

pub(crate) mod helper_traits {
    #[rustc_coinductive]
    pub(crate) trait AllCovisit<T, Case, Heap, L> {
        fn all_covisit(&mut self, heap: &mut Heap) -> Case;
    }
    #[rustc_coinductive]
    pub(crate) trait EachCovisit<T, Heap, L, RemainingCases, SelfDone> {
        fn each_covisit(self, heap: &mut Heap) -> (SelfDone, T);
    }
}
mod impls {
    use ccf::CanonicallyConstructibleFrom;
    use conslist::{ConsList, NonemptyConsList};
    use pmsp::{NonemptyStrategy, StrategyOf, UsesStrategyForTraversal};
    use take_mut::Poisonable;

    use crate::Covisit;
    use crate::covisiteventsink::CovisitEventSink;
    use crate::helper_traits::{AllCovisit, EachCovisit};
    use crate::select::{AcceptingCases, FromSelectCase, SelectCase};

    impl<Covisitor, T, Heap, L> Covisit<T, Heap, L> for Covisitor
    where
        T: words::Implements<Heap, L> + UsesStrategyForTraversal<Covisitor>,
        <T as words::Implements<Heap, L>>::LWord: pmsp::NamesPatternMatchStrategyGivenContext<Heap>,
        // Covisitor: Covisit<<StrategyOf<T, Heap, L> as Strategy>::Cdr, Heap, L>,
        Covisitor: Poisonable,
        Covisitor: SelectCase,
        <Covisitor as SelectCase>::AC<StrategyOf<T, Heap, L>>: EachCovisit<
                T,
                Heap,
                L,
                StrategyOf<T, Heap, L>,
                <<Covisitor as SelectCase>::AC<StrategyOf<T, Heap, L>> as FromSelectCase>::Done,
            >,
    {
        fn covisit(&mut self, heap: &mut Heap) -> T {
            take_mut::take(self, |cv| {
                let ac = cv.start_cases();
                <<Covisitor as SelectCase>::AC<StrategyOf<T, Heap, L>> as EachCovisit<
                    T,
                    Heap,
                    L,
                    StrategyOf<T, Heap, L>,
                    <<Covisitor as SelectCase>::AC<StrategyOf<T, Heap, L>> as FromSelectCase>::Done,
                >>::each_covisit(ac, heap)
            })
        }
    }
    impl<AC, T, Heap, L, RemainingCases>
        EachCovisit<T, Heap, L, RemainingCases, <AC as FromSelectCase>::Done> for AC
    where
        AC: AcceptingCases<RemainingCases>,
        RemainingCases: NonemptyStrategy,
        <AC as FromSelectCase>::Done: AllCovisit<T, RemainingCases::Car, Heap, L>,
        T: CanonicallyConstructibleFrom<Heap, RemainingCases::Car>,
        AC::AcceptingRemainingCases:
            EachCovisit<T, Heap, L, RemainingCases::Cdr, <AC as FromSelectCase>::Done>,
        <AC as FromSelectCase>::Done: CovisitEventSink<T>,
    {
        fn each_covisit(self, heap: &mut Heap) -> (<AC as FromSelectCase>::Done, T) {
            self.try_case()
                .map(|mut x| {
                    <<AC as FromSelectCase>::Done as CovisitEventSink<T>>::push(&mut x);
                    let case = <<AC as FromSelectCase>::Done as AllCovisit<
                        T,
                        RemainingCases::Car,
                        Heap,
                        L,
                    >>::all_covisit(&mut x, heap);
                    <<AC as FromSelectCase>::Done as CovisitEventSink<T>>::pop(&mut x);
                    let ret =
                        <T as CanonicallyConstructibleFrom<Heap, RemainingCases::Car>>::construct(
                            heap, case,
                        );
                    (x, ret)
                })
                .unwrap_or_else(|remaining| {
                    <AC::AcceptingRemainingCases as EachCovisit<
                        T,
                        Heap,
                        L,
                        RemainingCases::Cdr,
                        <AC as FromSelectCase>::Done,
                    >>::each_covisit(remaining, heap)
                })
        }
    }
    impl<Covisitor, T, Case, Heap, L> AllCovisit<T, Case, Heap, L> for Covisitor
    where
        Case: NonemptyConsList,
        Covisitor: Covisit<Case::Car, Heap, L>,
        Covisitor: AllCovisit<T, Case::Cdr, Heap, L>,
        Covisitor: CovisitEventSink<T>,
    {
        fn all_covisit(&mut self, heap: &mut Heap) -> Case {
            ConsList::reconstruct(
                <Covisitor as Covisit<Case::Car, Heap, L>>::covisit(self, heap),
                {
                    self.proceed();
                    <Covisitor as AllCovisit<T, Case::Cdr, Heap, L>>::all_covisit(self, heap)
                },
            )
        }
    }
    mod base_cases {
        use crate::{
            helper_traits::{AllCovisit, EachCovisit},
            select::{AdmitNoMatchingCase, FromSelectCase},
        };

        impl<Covisitor, T, Heap, L> AllCovisit<T, (), Heap, L> for Covisitor {
            fn all_covisit(&mut self, _: &mut Heap) {}
        }
        impl<AC, T, Heap, L> EachCovisit<T, Heap, L, (), <AC as FromSelectCase>::Done> for AC
        where
            AC: AdmitNoMatchingCase<Heap, T>,
        {
            fn each_covisit(self, heap: &mut Heap) -> (<AC as FromSelectCase>::Done, T) {
                self.admit(heap)
            }
        }
    }
}
