use crate::{
    Heaped,
    case_split::{Adt, ConsList, HasPatternMatchStrategyFor},
    co_case_split::{AdmitNoMatchingCase, CoCallable, CoCaseSplittable},
    select::AcceptingCases,
};

pub trait CoVisitor<T> {
    fn co_push(&mut self);
    fn co_proceed(&mut self);
    fn co_pop(&mut self);
}

pub trait CoVisitable<CV, PatternMatchStrategyProvider, Heap>: Sized {
    fn co_visit(visitor: &mut CV, heap: &mut Heap) -> Self;
}

pub trait AllCoVisitable<Heap, CV, Ctx, PatternMatchStrategyProvider>: Sized {
    fn all_co_visit(visitor: &mut CV, heap: &mut Heap) -> Self;
}

impl<CV, Ctx, Heap, StrategyProvider, FieldsCar, FieldsCdrCar, FieldsCdrCdr>
    AllCoVisitable<Heap, CV, Ctx, StrategyProvider> for (FieldsCar, (FieldsCdrCar, FieldsCdrCdr))
where
    // Self: Copy,
    // Ctx: Copy,
    // CV: CoVisitor<Ctx>,
    FieldsCar: CoVisitable<CV, StrategyProvider, Heap>,
    // CV: Heaped<Heap = FieldsCar::Heap>,
    // StrategyProvider: crate::case_split::HasPatternMatchStrategyFor<FieldsCar>,
    (FieldsCdrCar, FieldsCdrCdr): AllCoVisitable<Heap, CV, Ctx, StrategyProvider>,
{
    fn all_co_visit(visitor: &mut CV, heap: &mut Heap) -> Self {
        let car = FieldsCar::co_visit(visitor, heap);
        // visitor.co_proceed();
        let cdr = <(FieldsCdrCar, FieldsCdrCdr)>::all_co_visit(visitor, heap);
        (car, cdr)
    }
}

impl<CV, Ctx, Heap, StrategyProvider, FieldsCar> AllCoVisitable<Heap, CV, Ctx, StrategyProvider>
    for (FieldsCar, ())
where
    // Self: Copy,
    // Ctx: Copy,
    // CV: CoVisitor<Ctx>,
    FieldsCar: CoVisitable<CV, StrategyProvider, Heap>,
    // StrategyProvider: crate::case_split::HasPatternMatchStrategyFor<FieldsCar>,
{
    fn all_co_visit(visitor: &mut CV, heap: &mut Heap) -> Self {
        let car = FieldsCar::co_visit(visitor, heap);
        (car, ())
    }
}

struct CoCallablefyCoVisitor<CV, MatchedType, StrategyProvider> {
    cv: CV,
    phantom: std::marker::PhantomData<(MatchedType, StrategyProvider)>,
}

impl<CV, MatchedType, StrategyProvider> CoCallablefyCoVisitor<CV, MatchedType, StrategyProvider> {
    fn new(cv: CV) -> Self {
        Self {
            cv,
            phantom: std::marker::PhantomData,
        }
    }
}

// impl<V, MatchedType, StrategyProvider> CoCallablefyCoVisitor<V, MatchedType, StrategyProvider> {
//     fn descend<Ctx, F: FnMut(&mut CoCallablefyCoVisitor<V, Ctx, StrategyProvider>)>(
//         &mut self,
//         ctx: Ctx,
//         mut f: F,
//     ) {
//         take_mut::take(self, |this| {
//             let CoCallablefyCoVisitor {
//                 cv, ctx: outer_ctx, ..
//             } = this;
//             let mut descended = CoCallablefyCoVisitor {
//                 cv,
//                 ctx,
//                 phantom: std::marker::PhantomData,
//             };
//             f(&mut descended);
//             Self {
//                 cv: descended.cv,
//                 ctx: outer_ctx,
//                 phantom: std::marker::PhantomData,
//             }
//         });
//     }
// }

impl<V: Heaped, MatchedType, StrategyProvider> Heaped
    for CoCallablefyCoVisitor<V, MatchedType, StrategyProvider>
{
    type Heap = V::Heap;
}

impl<V, MatchedType, Heap, CaseCar, CaseCdr, StrategyProvider> CoCallable<Heap, (CaseCar, CaseCdr)>
    for CoCallablefyCoVisitor<V, MatchedType, StrategyProvider>
where
    MatchedType: Copy,
    // V: CoVisitor<MatchedType> + CoVisitor<CaseCar>,
    (CaseCar, CaseCdr): Copy + AllCoVisitable<Heap, V, MatchedType, StrategyProvider>,
{
    fn call(&mut self, heap: &mut Heap) -> (CaseCar, CaseCdr) {
        <(CaseCar, CaseCdr)>::all_co_visit(&mut self.cv, heap)
    }
}

impl<T, V, M, S> AdmitNoMatchingCase<T> for CoCallablefyCoVisitor<V, M, S>
where
    V: AdmitNoMatchingCase<T>,
    T: Heaped,
{
    fn no_matching_case(&self, heap: &mut <T as Heaped>::Heap) -> T {
        self.cv.no_matching_case(heap)
    }
}
// impl<V, PatternMatchStrategyProvider, T> Visitable<V, PatternMatchStrategyProvider> for T
// where
//     T: Heaped<Heap = V::Heap>
//         + CaseSplittable<
//             CallablefyVisitor<V, T, PatternMatchStrategyProvider>,
//             PatternMatchStrategyProvider::Strategy,
//         > + Copy,
//     V: VisitorDfs<T>,
//     PatternMatchStrategyProvider: HasPatternMatchStrategyFor<T>,
// {
//     fn visit(
//         &self,
//         visitor: &mut V,
//         heap: &mut <V as HasBorrowedHeapRef>::Borrowed<'_, Self::Heap>,
//     ) {
//         let mut heap = heap;
//         if let MaybeAbortThisSubtree::Proceed = visitor.push(*self, &mut heap) {
//             take_mut::take(visitor, |visitor| {
//                 let mut callable = CallablefyVisitor {
//                     visitor,
//                     ctx: *self,
//                     phantom: std::marker::PhantomData,
//                 };
//                 <T as CaseSplittable<_, _>>::case_split(self, &mut callable, &mut heap);
//                 callable.visitor
//             });
//         }
//         visitor.pop(*self, heap);
//     }
// }

// co_visit::CoCallablefyCoVisitor<CV, T, PatternMatchStrategyProvider>: select::AcceptingCases<<PatternMatchStrategyProvider as case_split::HasPatternMatchStrategyFor<T>>::Strategy>

impl<CV, T, PatternMatchStrategyProvider, Cases> AcceptingCases<Cases>
    for CoCallablefyCoVisitor<CV, T, PatternMatchStrategyProvider>
where
    Cases: ConsList,
    CV: AcceptingCases<Cases>,
{
    type ShortCircuitsTo =
        CoCallablefyCoVisitor<CV::ShortCircuitsTo, T, PatternMatchStrategyProvider>;
    type AcceptingRemainingCases =
        CoCallablefyCoVisitor<CV::AcceptingRemainingCases, T, PatternMatchStrategyProvider>;

    fn try_case(&mut self) -> Result<Self::ShortCircuitsTo, Self::AcceptingRemainingCases> {
        match self.cv.try_case() {
            Ok(short_circuited) => Ok(CoCallablefyCoVisitor::new(short_circuited)),
            Err(remaining_cases) => {
                let remaining_cases = CoCallablefyCoVisitor {
                    cv: remaining_cases,
                    phantom: std::marker::PhantomData,
                };
                Err(remaining_cases)
            }
        }
    }

    // fn stop_with_external_success(self, proof_of_success: Cases::Car) -> Self::ShortCircuitsTo {
    //     self.cv.stop_with_external_success(proof_of_success)
    // }
}

// impl<Heap, CV, T> CoVisitable<CV, T::PatternMatchStrategyProvider, Heap> for T
// where
//     T: Heaped<Heap = Heap>
//         + CoCaseSplittable<
//             CoCallablefyCoVisitor<CV, T, T::PatternMatchStrategyProvider>,
//             <T::PatternMatchStrategyProvider as HasPatternMatchStrategyFor<T>>::Strategy,
//         > + Copy
//         + Adt,
//     CV: CoVisitor<T> + AdmitNoMatchingCase<T>, //  + AcceptingCases<PatternMatchStrategyProvider::Strategy>
//                                                // PatternMatchStrategyProvider: crate::case_split::HasPatternMatchStrategyFor<T>,
// {
//     fn co_visit(visitor: &mut CV, heap: &mut Heap) -> Self {
//         let mut ret: Option<Self> = None;
//         visitor.co_push();
//         take_mut::take(visitor, |visitor| {
//             let mut callable = CoCallablefyCoVisitor {
//                 cv: visitor,
//                 phantom: std::marker::PhantomData,
//             };
//             ret = Some(<T as CoCaseSplittable<_, _>>::co_case_split(
//                 &mut callable,
//                 heap,
//             ));
//             callable.cv
//         });
//         visitor.co_pop();
//         ret.unwrap()
//     }
// }

impl<Heap, CV, PatternMatchStrategyProvider, T> CoVisitable<CV, PatternMatchStrategyProvider, Heap>
    for T
where
    T: Heaped<Heap = Heap>
        + CoCaseSplittable<
            CoCallablefyCoVisitor<CV, T, PatternMatchStrategyProvider>,
            PatternMatchStrategyProvider::Strategy,
        > + Copy,
    CV: CoVisitor<T> + AdmitNoMatchingCase<T>, //  + AcceptingCases<PatternMatchStrategyProvider::Strategy>
    PatternMatchStrategyProvider: crate::case_split::HasPatternMatchStrategyFor<T>,
{
    fn co_visit(visitor: &mut CV, heap: &mut Heap) -> Self {
        let mut ret: Option<Self> = None;
        visitor.co_push();
        take_mut::take(visitor, |visitor| {
            let mut callable = CoCallablefyCoVisitor {
                cv: visitor,
                phantom: std::marker::PhantomData,
            };
            ret = Some(<T as CoCaseSplittable<_, _>>::co_case_split(
                &mut callable,
                heap,
            ));
            callable.cv
        });
        visitor.co_pop();
        ret.unwrap()
    }
}
