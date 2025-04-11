use crate::{
    Heaped,
    case_split::ConsList,
    co_case_split::{CoCallable, CoCaseSplittable},
    select::AcceptingCases,
};

pub trait CoVisitor<T>: Heaped {
    fn co_push(&mut self, heap: &mut Self::Heap);
    fn co_proceed(&mut self, heap: &mut Self::Heap);
    fn co_pop(&mut self, heap: &mut Self::Heap);
}

pub trait CoVisitable<CV, PatternMatchStrategyProvider>: Sized + Heaped
where
    CV: Heaped<Heap = <Self as Heaped>::Heap>,
{
    fn co_visit(visitor: &mut CV, heap: &mut Self::Heap) -> Self;
}

pub trait AllCoVisitable<CV: Heaped, Ctx, PatternMatchStrategyProvider>: Sized
where
    CV: Heaped,
{
    fn all_co_visit(visitor: &mut CV, heap: &mut CV::Heap) -> Self;
}

impl<CV, Ctx, StrategyProvider, FieldsCar, FieldsCdrCar, FieldsCdrCdr>
    AllCoVisitable<CV, Ctx, StrategyProvider> for (FieldsCar, (FieldsCdrCar, FieldsCdrCdr))
where
    Self: Copy,
    Ctx: Copy,
    CV: CoVisitor<Ctx>,
    FieldsCar: CoVisitable<CV, StrategyProvider>,
    CV: Heaped<Heap = FieldsCar::Heap>,
    StrategyProvider: crate::case_split::HasPatternMatchStrategyFor<FieldsCar>,
    (FieldsCdrCar, FieldsCdrCdr): AllCoVisitable<CV, Ctx, StrategyProvider>,
{
    fn all_co_visit(visitor: &mut CV, heap: &mut CV::Heap) -> Self {
        let car = FieldsCar::co_visit(visitor, heap);
        visitor.co_proceed(heap);
        let cdr = <(FieldsCdrCar, FieldsCdrCdr)>::all_co_visit(visitor, heap);
        (car, cdr)
    }
}

impl<CV, Ctx, StrategyProvider, FieldsCar> AllCoVisitable<CV, Ctx, StrategyProvider>
    for (FieldsCar, ())
where
    Self: Copy,
    Ctx: Copy,
    CV: CoVisitor<Ctx>,
    FieldsCar: CoVisitable<CV, StrategyProvider>,
    CV: Heaped<Heap = FieldsCar::Heap>,
    StrategyProvider: crate::case_split::HasPatternMatchStrategyFor<FieldsCar>,
{
    fn all_co_visit(visitor: &mut CV, heap: &mut CV::Heap) -> Self {
        let car = FieldsCar::co_visit(visitor, heap);
        (car, ())
    }
}

struct CoCallablefyCoVisitor<CV, MatchedType, StrategyProvider> {
    cv: CV,
    phantom: std::marker::PhantomData<(MatchedType, StrategyProvider)>,
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

impl<V, MatchedType, CaseCar, CaseCdr, StrategyProvider> CoCallable<(CaseCar, CaseCdr)>
    for CoCallablefyCoVisitor<V, MatchedType, StrategyProvider>
where
    MatchedType: Copy,
    V: CoVisitor<MatchedType> + CoVisitor<CaseCar>,
    (CaseCar, CaseCdr): Copy + AllCoVisitable<V, MatchedType, StrategyProvider>,
{
    fn call(&mut self, heap: &mut Self::Heap) -> (CaseCar, CaseCdr) {
        <(CaseCar, CaseCdr)>::all_co_visit(&mut self.cv, heap)
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
    type ShortCircuitsTo = CV::ShortCircuitsTo;
    type AcceptingRemainingCases =
        CoCallablefyCoVisitor<CV::AcceptingRemainingCases, T, PatternMatchStrategyProvider>;

    fn try_case(&mut self) -> Result<Self::ShortCircuitsTo, Self::AcceptingRemainingCases> {
        match self.cv.try_case() {
            Ok(short_circuited) => Ok(short_circuited),
            Err(remaining_cases) => {
                let remaining_cases = CoCallablefyCoVisitor {
                    cv: remaining_cases,
                    phantom: std::marker::PhantomData,
                };
                Err(remaining_cases)
            }
        }
    }

    fn stop_with_external_success(self, proof_of_success: Cases::Car) -> Self::ShortCircuitsTo {
        self.cv.stop_with_external_success(proof_of_success)
    }
}

impl<CV, PatternMatchStrategyProvider, T> CoVisitable<CV, PatternMatchStrategyProvider> for T
where
    T: Heaped<Heap = CV::Heap>
        + CoCaseSplittable<
            CoCallablefyCoVisitor<CV, T, PatternMatchStrategyProvider>,
            PatternMatchStrategyProvider::Strategy,
        > + Copy,
    CV: CoVisitor<T> + AcceptingCases<PatternMatchStrategyProvider::Strategy>,
    PatternMatchStrategyProvider: crate::case_split::HasPatternMatchStrategyFor<T>,
{
    fn co_visit(visitor: &mut CV, heap: &mut <CV as Heaped>::Heap) -> Self {
        let mut ret: Option<Self> = None;
        visitor.co_push(heap);
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
        visitor.co_pop(heap);
        ret.unwrap()
    }
}
