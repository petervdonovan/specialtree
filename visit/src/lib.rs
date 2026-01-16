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
    pub(crate) trait AllVisit<LWord, L, TVisitationImplementor, Heap, ConcreteCase, AbstractCase> {
        fn all_visit(&mut self, heap: &Heap, case: &ConcreteCase, idx: u32);
    }
    #[rustc_coinductive]
    pub(crate) trait AnyVisit<LWord, L, T, TVisitationImplementor, Heap, RemainingCases> {
        fn any_visit(&mut self, heap: &Heap, t: &T);
    }
}

mod impls {
    use aspect::VisitationAspect;
    use aspect::{AdtLike, Adtishness};
    use ccf::{CanonicallyConstructibleFrom, DirectlyCanonicallyConstructibleFrom};
    use conslist::{ConsList, NonemptyConsList};
    use pmsp::{NonemptyStrategy, StrategyOf};
    use words::{HasDeconstructionTargetForWordList, Implements, InverseImplements};

    use crate::visiteventsink::AspectVisitor;
    use crate::{
        Visit,
        helper_traits::{AllVisit, AnyVisit},
        visiteventsink::VisitEventSink,
    };

    impl<V, LWord, L, T, Heap, TVisitationImplementor> Visit<LWord, L, T, Heap, AdtLike> for V
    where
        // T: words::Implements<Heap, L, aspect::VisitationAspect>,
        // <T as words::Implements<Heap, L, aspect::VisitationAspect>>::LWord:
        //     pmsp::NamesPatternMatchStrategy<L>,
        Heap: words::InverseImplements<
                L,
                LWord,
                VisitationAspect,
                Implementor = TVisitationImplementor,
            >,
        TVisitationImplementor: words::Implements<Heap, L, aspect::VisitationAspect, LWord = LWord>,
        LWord: pmsp::NamesPatternMatchStrategy<L>,
        V: AnyVisit<
                LWord,
                L,
                T,
                TVisitationImplementor,
                Heap,
                StrategyOf<TVisitationImplementor, Heap, L, aspect::VisitationAspect>,
            >,
    {
        fn visit(&mut self, heap: &Heap, t: &T) {
            <V as AnyVisit<_, _, _, _, _, _>>::any_visit(self, heap, t)
        }
    }

    impl<
        V,
        LWord,
        L,
        T,
        TVisitationImplementor,
        TAspectImplementor,
        Heap,
        RemainingCases,
        CarDeconstructionTargetImplementor,
    > AnyVisit<LWord, L, T, TVisitationImplementor, Heap, RemainingCases> for V
    where
        Heap: HasDeconstructionTargetForWordList<
                L,
                RemainingCases::Car,
                TVisitationImplementor,
                Implementors = CarDeconstructionTargetImplementor,
            >,
        T: Copy + CanonicallyConstructibleFrom<Heap, (TAspectImplementor, ())>,
        TAspectImplementor: Copy + CanonicallyConstructibleFrom<Heap, (TVisitationImplementor, ())>,
        TVisitationImplementor:
            Copy + DirectlyCanonicallyConstructibleFrom<Heap, CarDeconstructionTargetImplementor>,
        RemainingCases: NonemptyStrategy,
        V: AnyVisit<LWord, L, T, TVisitationImplementor, Heap, RemainingCases::Cdr>,
        V: AllVisit<
                LWord,
                L,
                TVisitationImplementor,
                Heap,
                CarDeconstructionTargetImplementor,
                RemainingCases::Car,
            >,
        Heap:
            InverseImplements<L, LWord, <V as AspectVisitor>::A, Implementor = TAspectImplementor>,
        V: VisitEventSink<
                <Heap as InverseImplements<L, LWord, <V as AspectVisitor>::A>>::Implementor,
                Heap,
            >,
    {
        fn any_visit(&mut self, heap: &Heap, t: &T) {
            if t.deconstruct_succeeds(heap) {
                let tai = t.deconstruct(heap).0;
                if tai.deconstruct_succeeds(heap) {
                    let tvi = tai.deconstruct(heap).0;
                    if tvi.deconstruct_succeeds(heap) {
                        let car = tvi.deconstruct(heap);
                        self.push(heap, &tai, <RemainingCases::Car as ConsList>::LENGTH);
                        self.all_visit(heap, &car, 0);
                        self.pop(<RemainingCases::Car as ConsList>::LENGTH);
                        return;
                    }
                }
            }
            <V as AnyVisit<_, _, _, _, _, RemainingCases::Cdr>>::any_visit(self, heap, t);
        }
    }

    impl<V, ConcreteCase: Copy, AbstractCase, LWord, L, T, Heap, TAspectImplementor>
        AllVisit<LWord, L, T, Heap, ConcreteCase, AbstractCase> for V
    where
        ConcreteCase: NonemptyConsList,
        AbstractCase: NonemptyConsList,
        V: AllVisit<LWord, L, T, Heap, ConcreteCase::Cdr, AbstractCase::Cdr>,
        // Heap: InverseImplements<L, Case::Car>,
        // ConcreteCase::Car: Implements<Heap, L, VisitationAspect>,
        // <ConcreteCase::Car as Implements<Heap, L, VisitationAspect>>::LWord: Adtishness<VisitationAspect>,
        AbstractCase::Car: Adtishness<VisitationAspect>,
        V: Visit<
                AbstractCase::Car,
                L,
                ConcreteCase::Car,
                Heap,
                <AbstractCase::Car as Adtishness<VisitationAspect>>::X,
            >,
        Heap:
            InverseImplements<L, LWord, <V as AspectVisitor>::A, Implementor = TAspectImplementor>,
        V: VisitEventSink<
                <Heap as InverseImplements<L, LWord, <V as AspectVisitor>::A>>::Implementor,
                Heap,
            >,
    {
        fn all_visit(&mut self, heap: &Heap, case: &ConcreteCase, idx: u32) {
            let (car, cdr) = case.deconstruct();
            self.visit(heap, &car);
            self.proceed(idx, idx + ConcreteCase::LENGTH);
            self.all_visit(heap, &cdr, idx + 1);
        }
    }

    mod base_cases {

        use words::InverseImplements;

        use crate::{
            helper_traits::{AllVisit, AnyVisit},
            visiteventsink::{AspectVisitor, VisitEventSink},
        };

        impl<Visitor, LWord, L, T, Heap> AllVisit<LWord, L, T, Heap, (), ()> for Visitor {
            fn all_visit(&mut self, _heap: &Heap, _case: &(), _idx: u32) {
                // do nothing
            }
        }
        impl<V, LWord, L, T, TVisitationImplementor, TAspectImplementor, Heap>
            AnyVisit<LWord, L, T, TVisitationImplementor, Heap, ()> for V
        where
            Heap: InverseImplements<
                    L,
                    LWord,
                    <V as AspectVisitor>::A,
                    Implementor = TAspectImplementor,
                >,
            V: VisitEventSink<
                    <Heap as InverseImplements<L, LWord, <V as AspectVisitor>::A>>::Implementor,
                    Heap,
                >,
        {
            fn any_visit(&mut self, _heap: &Heap, _t: &T) {
                self.deconstruction_failure()
            }
        }
    }
}
