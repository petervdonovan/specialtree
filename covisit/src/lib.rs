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
    pub(crate) trait AllCovisit<LWord, L, TVisitationImplementor, Heap, ConcreteCase, AbstractCase>
    {
        fn all_covisit(&mut self, heap: &mut Heap, idx: u32) -> ConcreteCase;
    }
    #[rustc_coinductive]
    pub(crate) trait AnyCovisit<LWord, L, T, TVisitationImplementor, Heap, RemainingCases, SelfDone>
    {
        fn any_covisit(self, heap: &mut Heap) -> (SelfDone, T);
    }
}
mod impls {
    use aspect::VisitationAspect;
    use aspect::{AdtLike, Adtishness};
    use ccf::transitivity::TransitivelyUnitCcf;
    use ccf::{CanonicallyConstructibleFrom, DirectlyCanonicallyConstructibleFrom};
    use conslist::{ConsList, NonemptyConsList};
    use pmsp::{NonemptyStrategy, StrategyOf};
    use take_mut::Poisonable;
    use words::{HasDeconstructionTargetForWordList, Implements};

    use crate::Covisit;
    use crate::covisiteventsink::CovisitEventSink;
    use crate::helper_traits::{AllCovisit, AnyCovisit};
    use crate::select::{AcceptingCases, FromSelectCase, SelectCase};

    impl<Covisitor, LWord, L, T, Heap, TVisitationImplementor> Covisit<LWord, L, T, Heap, AdtLike>
        for Covisitor
    where
        T: words::Implements<Heap, L, <Covisitor as SelectCase>::A, LWord = LWord>,
        T: CanonicallyConstructibleFrom<Heap, (TVisitationImplementor, ())>,
        Heap: words::InverseImplements<
                L,
                LWord,
                VisitationAspect,
                Implementor = TVisitationImplementor,
            >,
        TVisitationImplementor: words::Implements<Heap, L, VisitationAspect, LWord = LWord>,
        LWord: pmsp::NamesPatternMatchStrategy<L>,
        // Covisitor: Covisit<<StrategyOf<T, Heap, L> as Strategy>::Cdr, Heap, L>,
        Covisitor: Poisonable,
        Covisitor: SelectCase,
        <Covisitor as SelectCase>::AC<
            StrategyOf<TVisitationImplementor, Heap, L, VisitationAspect>,
        >: AnyCovisit<
                LWord,
                L,
                T,
                TVisitationImplementor,
                Heap,
                StrategyOf<TVisitationImplementor, Heap, L, aspect::VisitationAspect>,
                <<Covisitor as SelectCase>::AC<
                    StrategyOf<TVisitationImplementor, Heap, L, VisitationAspect>,
                > as FromSelectCase>::Done,
            >,
    {
        fn covisit(&mut self, heap: &mut Heap) -> T {
            take_mut::take(self, |cv| {
                let ac = cv.start_cases();
                <<Covisitor as SelectCase>::AC<
                    StrategyOf<TVisitationImplementor, Heap, L, VisitationAspect>,
                > as AnyCovisit<_, _, _, _, _, _, _>>::any_covisit(ac, heap)
            })
        }
    }
    impl<
        AC,
        LWord,
        L,
        T,
        TVisitationImplementor,
        Heap,
        RemainingCases,
        CarDeconstructionTargetImplementor,
    >
        AnyCovisit<
            LWord,
            L,
            T,
            TVisitationImplementor,
            Heap,
            RemainingCases,
            <AC as FromSelectCase>::Done,
        > for AC
    where
        AC: AcceptingCases<RemainingCases>,
        RemainingCases: NonemptyStrategy,
        Heap: HasDeconstructionTargetForWordList<
                L,
                RemainingCases::Car,
                TVisitationImplementor,
                Implementors = CarDeconstructionTargetImplementor,
            >,
        <AC as FromSelectCase>::Done: AllCovisit<
                LWord,
                L,
                TVisitationImplementor,
                Heap,
                CarDeconstructionTargetImplementor,
                RemainingCases::Car,
            >,
        AC::AcceptingRemainingCases: AnyCovisit<
                LWord,
                L,
                T,
                TVisitationImplementor,
                Heap,
                RemainingCases::Cdr,
                <AC as FromSelectCase>::Done,
            >,
        T: CanonicallyConstructibleFrom<Heap, (TVisitationImplementor, ())>,
        TVisitationImplementor:
            DirectlyCanonicallyConstructibleFrom<Heap, CarDeconstructionTargetImplementor>,
        <AC as FromSelectCase>::Done: CovisitEventSink<LWord>,
    {
        fn any_covisit(self, heap: &mut Heap) -> (<AC as FromSelectCase>::Done, T) {
            self.try_case()
                .map(|mut x| {
                    <<AC as FromSelectCase>::Done as CovisitEventSink<_>>::push(&mut x);
                    let case =
                        <<AC as FromSelectCase>::Done as AllCovisit<_, _, _, _, _, _>>::all_covisit(
                            &mut x, heap, 0,
                        );
                    <<AC as FromSelectCase>::Done as CovisitEventSink<_>>::pop(&mut x);
                    let visitation_implementor =
                        <TVisitationImplementor as DirectlyCanonicallyConstructibleFrom<_, _>>::construct(
                            heap, case,
                        );
                    let ret = <T as CanonicallyConstructibleFrom<Heap, _>>::construct(
                        heap,
                        (visitation_implementor, ()),
                    );
                    (x, ret)
                })
                .unwrap_or_else(|remaining| {
                    <AC::AcceptingRemainingCases as AnyCovisit<_, _, _, _, _, _, _>>::any_covisit(
                        remaining, heap,
                    )
                })
        }
    }
    impl<
        Covisitor,
        LWord,
        L,
        TVisitationImplementor,
        Heap,
        ConcreteCase,
        AbstractCase,
        CaseCarAspectImplementor,
    > AllCovisit<LWord, L, TVisitationImplementor, Heap, ConcreteCase, AbstractCase> for Covisitor
    where
        Heap: words::InverseImplements<
                L,
                AbstractCase::Car,
                <Covisitor as SelectCase>::A,
                Implementor = CaseCarAspectImplementor,
            >,
        Covisitor: SelectCase,
        AbstractCase: NonemptyConsList,
        ConcreteCase: NonemptyConsList,
        ConcreteCase::Car: CanonicallyConstructibleFrom<Heap, (CaseCarAspectImplementor, ())>,
        AbstractCase::Car: Adtishness<VisitationAspect>,
        Covisitor: Covisit<
                AbstractCase::Car,
                L,
                CaseCarAspectImplementor,
                Heap,
                <AbstractCase::Car as Adtishness<VisitationAspect>>::X,
            >,
        Covisitor: AllCovisit<LWord, L, TVisitationImplementor, Heap, ConcreteCase::Cdr, AbstractCase::Cdr>,
        Covisitor: CovisitEventSink<LWord>,
    {
        fn all_covisit(&mut self, heap: &mut Heap, idx: u32) -> ConcreteCase {
            let car_aspect_implementor = <Covisitor as Covisit<_, _, _, _, _>>::covisit(self, heap);
            ConsList::reconstruct(
                <ConcreteCase::Car as CanonicallyConstructibleFrom<
                    Heap,
                    (CaseCarAspectImplementor, ()),
                >>::construct(heap, (car_aspect_implementor, ())),
                {
                    self.proceed(idx, idx + ConcreteCase::LENGTH);
                    <Covisitor as AllCovisit<_, _, _, _, _, _>>::all_covisit(self, heap, idx + 1)
                },
            )
        }
    }
    mod base_cases {
        use crate::{
            helper_traits::{AllCovisit, AnyCovisit},
            select::{AdmitNoMatchingCase, FromSelectCase},
        };

        impl<Covisitor, LWord, L, T, Heap> AllCovisit<LWord, L, T, Heap, (), ()> for Covisitor {
            fn all_covisit(&mut self, _: &mut Heap, _idx: u32) {}
        }
        impl<AC, LWord, L, T, TVisitationImplementor, Heap>
            AnyCovisit<LWord, L, T, TVisitationImplementor, Heap, (), <AC as FromSelectCase>::Done>
            for AC
        where
            AC: AdmitNoMatchingCase<LWord, L, T, Heap>,
        {
            fn any_covisit(self, heap: &mut Heap) -> (<AC as FromSelectCase>::Done, T) {
                self.admit(heap)
            }
        }
    }
}
