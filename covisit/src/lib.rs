// see https://github.com/rust-lang/rust/pull/108033
#![allow(internal_features)]
#![feature(rustc_attrs)]

// want: foreign crates can implement covisit for their concrete type and for a specific covisitor (no generics required), and
// foreign crates can implement a specific covisitor for all ADTs

pub mod covisiteventsink;
pub mod select;

#[rustc_coinductive]
pub trait Covisit<TyMetadata, T, Heap, L> {
    fn covisit(&mut self, heap: &mut Heap) -> T;
}

pub(crate) mod helper_traits {
    #[rustc_coinductive]
    pub(crate) trait AllCovisit<T, Heap, L, Case, TyMetadata> {
        fn all_covisit(&mut self, heap: &mut Heap, idx: u32) -> Case;
    }
    #[rustc_coinductive]
    pub(crate) trait AnyCovisit<T, Heap, L, RemainingCases, RemainingTyMetadata, SelfDone> {
        fn any_covisit(self, heap: &mut Heap) -> (SelfDone, T);
    }
}
mod impls {
    use ccf::CanonicallyConstructibleFrom;
    use conslist::{ConsList, NonemptyConsList};
    use pmsp::{AdtMetadata, NonemptyStrategy, StrategyOf, TyMetadataOf};
    use take_mut::Poisonable;

    use crate::Covisit;
    use crate::covisiteventsink::CovisitEventSink;
    use crate::helper_traits::{AllCovisit, AnyCovisit};
    use crate::select::{AcceptingCases, FromSelectCase, SelectCase};

    impl<Covisitor, T, Heap, L> Covisit<AdtMetadata, T, Heap, L> for Covisitor
    where
        T: words::Implements<Heap, L>,
        <T as words::Implements<Heap, L>>::LWord: pmsp::NamesPatternMatchStrategyGivenContext<Heap>,
        // Covisitor: Covisit<<StrategyOf<T, Heap, L> as Strategy>::Cdr, Heap, L>,
        Covisitor: Poisonable,
        Covisitor: SelectCase,
        <Covisitor as SelectCase>::AC<StrategyOf<T, Heap, L>>: AnyCovisit<
                T,
                Heap,
                L,
                StrategyOf<T, Heap, L>,
                TyMetadataOf<T, Heap, L>,
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
    impl<AC, T, Heap, L, RemainingCases, RemainingTyMetadata>
        AnyCovisit<T, Heap, L, RemainingCases, RemainingTyMetadata, <AC as FromSelectCase>::Done>
        for AC
    where
        AC: AcceptingCases<RemainingCases>,
        RemainingCases: NonemptyStrategy,
        RemainingTyMetadata: NonemptyStrategy,
        <AC as FromSelectCase>::Done:
            AllCovisit<T, Heap, L, RemainingCases::Car, RemainingTyMetadata::Car>,
        T: CanonicallyConstructibleFrom<Heap, RemainingCases::Car>,
        AC::AcceptingRemainingCases: AnyCovisit<
                T,
                Heap,
                L,
                RemainingCases::Cdr,
                RemainingTyMetadata::Cdr,
                <AC as FromSelectCase>::Done,
            >,
        <AC as FromSelectCase>::Done: CovisitEventSink<T>,
    {
        fn any_covisit(self, heap: &mut Heap) -> (<AC as FromSelectCase>::Done, T) {
            self.try_case()
                .map(|mut x| {
                    <<AC as FromSelectCase>::Done as CovisitEventSink<T>>::push(&mut x);
                    let case =
                        <<AC as FromSelectCase>::Done as AllCovisit<_, _, _, _, _>>::all_covisit(
                            &mut x, heap, 0,
                        );
                    <<AC as FromSelectCase>::Done as CovisitEventSink<T>>::pop(&mut x);
                    let ret =
                        <T as CanonicallyConstructibleFrom<Heap, RemainingCases::Car>>::construct(
                            heap, case,
                        );
                    (x, ret)
                })
                .unwrap_or_else(|remaining| {
                    <AC::AcceptingRemainingCases as AnyCovisit<_, _, _, _, _, _>>::any_covisit(
                        remaining, heap,
                    )
                })
        }
    }
    impl<Covisitor, T, Case, Heap, L, TyMetadata> AllCovisit<T, Heap, L, Case, TyMetadata> for Covisitor
    where
        Case: NonemptyConsList,
        TyMetadata: NonemptyConsList,
        Covisitor: Covisit<TyMetadata::Car, Case::Car, Heap, L>,
        Covisitor: AllCovisit<T, Heap, L, Case::Cdr, TyMetadata::Cdr>,
        Covisitor: CovisitEventSink<T>,
    {
        fn all_covisit(&mut self, heap: &mut Heap, idx: u32) -> Case {
            ConsList::reconstruct(<Covisitor as Covisit<_, _, _, _>>::covisit(self, heap), {
                self.proceed(idx, idx + Case::LENGTH);
                <Covisitor as AllCovisit<_, _, _, _, _>>::all_covisit(self, heap, idx + 1)
            })
        }
    }
    mod base_cases {
        use crate::{
            helper_traits::{AllCovisit, AnyCovisit},
            select::{AdmitNoMatchingCase, FromSelectCase},
        };

        impl<Covisitor, T, Heap, L> AllCovisit<T, Heap, L, (), ()> for Covisitor {
            fn all_covisit(&mut self, _: &mut Heap, _idx: u32) {}
        }
        impl<AC, T, Heap, L> AnyCovisit<T, Heap, L, (), (), <AC as FromSelectCase>::Done> for AC
        where
            AC: AdmitNoMatchingCase<Heap, T>,
        {
            fn any_covisit(self, heap: &mut Heap) -> (<AC as FromSelectCase>::Done, T) {
                self.admit(heap)
            }
        }
    }
}
