// see https://github.com/rust-lang/rust/pull/108033
#![allow(internal_features)]
#![feature(rustc_attrs)]

// want: foreign crates can implement covisit for their concrete type and for a specific covisitor (no generics required), and
// foreign crates can implement a specific covisitor for all ADTs

pub mod covisiteventsink;
pub mod select;

#[rustc_coinductive]
pub trait Covisit<LWord, L, T, Heap, AdtLikeOrNot> {
    fn covisit(&mut self, heap: &mut Heap) -> T;
}

pub(crate) mod helper_traits {
    #[rustc_coinductive]
    pub(crate) trait AllCovisit<LWord, L, T, Heap, ConcreteCase> {
        fn all_covisit(&mut self, heap: &mut Heap, idx: u32) -> ConcreteCase;
    }
    #[rustc_coinductive]
    pub(crate) trait AnyCovisit<LWord, L, T, Heap, RemainingCases, SelfDone> {
        fn any_covisit(self, heap: &mut Heap) -> (SelfDone, T);
    }
}
mod impls {
    use ccf::{CanonicallyConstructibleFrom, VisitationInfo};
    use conslist::{ConsList, NonemptyConsList};
    use pmsp::{NonemptyStrategy, StrategyOf};
    use take_mut::Poisonable;
    use words::{AdtLike, Implements, InverseImplementsAll};

    use crate::Covisit;
    use crate::covisiteventsink::CovisitEventSink;
    use crate::helper_traits::{AllCovisit, AnyCovisit};
    use crate::select::{AcceptingCases, FromSelectCase, SelectCase};

    impl<Covisitor, LWord, L, T, Heap> Covisit<LWord, L, T, Heap, AdtLike> for Covisitor
    where
        T: words::Implements<Heap, L, LWord = LWord>,
        LWord: pmsp::NamesPatternMatchStrategy<L>,
        // Covisitor: Covisit<<StrategyOf<T, Heap, L> as Strategy>::Cdr, Heap, L>,
        Covisitor: Poisonable,
        Covisitor: SelectCase,
        <Covisitor as SelectCase>::AC<StrategyOf<T, Heap, L>>: AnyCovisit<
                LWord,
                L,
                T,
                Heap,
                StrategyOf<T, Heap, L>,
                <<Covisitor as SelectCase>::AC<StrategyOf<T, Heap, L>> as FromSelectCase>::Done,
            >,
    {
        fn covisit(&mut self, heap: &mut Heap) -> T {
            take_mut::take(self, |cv| {
                let ac = cv.start_cases();
                <<Covisitor as SelectCase>::AC<StrategyOf<T, Heap, L>> as AnyCovisit<
                    _,
                    _,
                    _,
                    _,
                    _,
                    _,
                >>::any_covisit(ac, heap)
            })
        }
    }
    impl<AC, LWord, L, T, Heap, RemainingCases>
        AnyCovisit<LWord, L, T, Heap, RemainingCases, <AC as FromSelectCase>::Done> for AC
    where
        AC: AcceptingCases<RemainingCases>,
        RemainingCases: NonemptyStrategy,
        Heap: InverseImplementsAll<L, RemainingCases::Car>,
        <AC as FromSelectCase>::Done: AllCovisit<
                LWord,
                L,
                T,
                Heap,
                <Heap as InverseImplementsAll<L, RemainingCases::Car>>::StructuralImplementors,
            >,
        T: CanonicallyConstructibleFrom<
                Heap,
                <Heap as InverseImplementsAll<L, RemainingCases::Car>>::StructuralImplementors,
            >,
        AC::AcceptingRemainingCases:
            AnyCovisit<LWord, L, T, Heap, RemainingCases::Cdr, <AC as FromSelectCase>::Done>,
        <AC as FromSelectCase>::Done: CovisitEventSink<LWord>,
    {
        fn any_covisit(self, heap: &mut Heap) -> (<AC as FromSelectCase>::Done, T) {
            self.try_case()
                .map(|mut x| {
                    <<AC as FromSelectCase>::Done as CovisitEventSink<_>>::push(&mut x);
                    let case =
                        <<AC as FromSelectCase>::Done as AllCovisit<_, _, _, _, _>>::all_covisit(
                            &mut x, heap, 0,
                        );
                    <<AC as FromSelectCase>::Done as CovisitEventSink<_>>::pop(&mut x);
                    let ret = <T as CanonicallyConstructibleFrom<Heap, _>>::construct(heap, case);
                    (x, ret)
                })
                .unwrap_or_else(|remaining| {
                    <AC::AcceptingRemainingCases as AnyCovisit<_, _, _, _, _, _>>::any_covisit(
                        remaining, heap,
                    )
                })
        }
    }
    impl<Covisitor, LWord, L, T, Heap, ConcreteCase> AllCovisit<LWord, L, T, Heap, ConcreteCase>
        for Covisitor
    where
        ConcreteCase: NonemptyConsList,
        ConcreteCase::Car: Implements<Heap, L>,
        <ConcreteCase::Car as Implements<Heap, L>>::LWord: VisitationInfo,
        Covisitor: Covisit<
                <ConcreteCase::Car as Implements<Heap, L>>::LWord,
                L,
                ConcreteCase::Car,
                Heap,
                <<ConcreteCase::Car as Implements<Heap, L>>::LWord as VisitationInfo>::AdtLikeOrNot,
            >,
        Covisitor: AllCovisit<LWord, L, T, Heap, ConcreteCase::Cdr>,
        Covisitor: CovisitEventSink<LWord>,
    {
        fn all_covisit(&mut self, heap: &mut Heap, idx: u32) -> ConcreteCase {
            ConsList::reconstruct(
                <Covisitor as Covisit<_, _, _, _, _>>::covisit(self, heap),
                {
                    self.proceed(idx, idx + ConcreteCase::LENGTH);
                    <Covisitor as AllCovisit<_, _, _, _, _>>::all_covisit(self, heap, idx + 1)
                },
            )
        }
    }
    mod base_cases {
        use crate::{
            helper_traits::{AllCovisit, AnyCovisit},
            select::{AdmitNoMatchingCase, FromSelectCase},
        };

        impl<Covisitor, LWord, L, T, Heap> AllCovisit<LWord, L, T, Heap, ()> for Covisitor {
            fn all_covisit(&mut self, _: &mut Heap, _idx: u32) {}
        }
        impl<AC, LWord, L, T, Heap> AnyCovisit<LWord, L, T, Heap, (), <AC as FromSelectCase>::Done> for AC
        where
            AC: AdmitNoMatchingCase<LWord, L, T, Heap>,
        {
            fn any_covisit(self, heap: &mut Heap) -> (<AC as FromSelectCase>::Done, T) {
                self.admit(heap)
            }
        }
    }
}
