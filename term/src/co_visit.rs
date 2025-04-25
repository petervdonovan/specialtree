use crate::{
    Heaped,
    case_split::{Adt, ConsList, HasPatternMatchStrategyFor},
    co_case_split::{AdmitNoMatchingCase, CoCallable, CoCaseSplittable},
    select::{AcceptingCases, FromSelectCase, SelectCase},
};

pub trait CoVisitor<T> {
    fn co_push(&mut self);
    fn co_proceed(&mut self);
    fn co_pop(&mut self);
}

pub trait CoVisitable<CV, PatternMatchStrategyProvider, Heap, DepthFuel>: Sized {
    fn co_visit(visitor: &mut CV, heap: &mut Heap) -> Self;
}

pub trait AllCoVisitable<Heap, CV, Ctx, PatternMatchStrategyProvider, DepthFuel>: Sized {
    fn all_co_visit(visitor: &mut CV, heap: &mut Heap) -> Self;
}

impl<CV, Ctx, Heap, StrategyProvider, DepthFuel, FieldsCar, FieldsCdrCar, FieldsCdrCdr>
    AllCoVisitable<Heap, CV, Ctx, StrategyProvider, DepthFuel>
    for (FieldsCar, (FieldsCdrCar, FieldsCdrCdr))
where
    // Self: Copy,
    // Ctx: Copy,
    // CV: CoVisitor<Ctx>,
    FieldsCar: CoVisitable<CV, StrategyProvider, Heap, DepthFuel>,
    // CV: Heaped<Heap = FieldsCar::Heap>,
    // StrategyProvider: crate::case_split::HasPatternMatchStrategyFor<FieldsCar>,
    (FieldsCdrCar, FieldsCdrCdr): AllCoVisitable<Heap, CV, Ctx, StrategyProvider, DepthFuel>,
{
    fn all_co_visit(visitor: &mut CV, heap: &mut Heap) -> Self {
        let car = FieldsCar::co_visit(visitor, heap);
        // visitor.co_proceed();
        let cdr = <(FieldsCdrCar, FieldsCdrCdr)>::all_co_visit(visitor, heap);
        (car, cdr)
    }
}

impl<CV, Ctx, Heap, StrategyProvider, FieldsCar, DepthFuel>
    AllCoVisitable<Heap, CV, Ctx, StrategyProvider, DepthFuel> for (FieldsCar, ())
where
    // Self: Copy,
    // Ctx: Copy,
    // CV: CoVisitor<Ctx>,
    FieldsCar: CoVisitable<CV, StrategyProvider, Heap, DepthFuel>,
    // StrategyProvider: crate::case_split::HasPatternMatchStrategyFor<FieldsCar>,
{
    fn all_co_visit(visitor: &mut CV, heap: &mut Heap) -> Self {
        let car = FieldsCar::co_visit(visitor, heap);
        (car, ())
    }
}

struct CoCallablefyCoVisitor<CV, MatchedType, StrategyProvider, DepthFuel> {
    cv: CV,
    phantom: std::marker::PhantomData<(MatchedType, StrategyProvider, DepthFuel)>,
}

impl<CV, MatchedType, StrategyProvider, DepthFuel>
    CoCallablefyCoVisitor<CV, MatchedType, StrategyProvider, DepthFuel>
{
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

impl<V: Heaped, MatchedType, StrategyProvider, DepthFuel> Heaped
    for CoCallablefyCoVisitor<V, MatchedType, StrategyProvider, DepthFuel>
{
    type Heap = V::Heap;
}

impl<V, MatchedType, Heap, CaseCar, CaseCdr, StrategyProvider, DepthFuel>
    CoCallable<Heap, (CaseCar, CaseCdr)>
    for CoCallablefyCoVisitor<V, MatchedType, StrategyProvider, DepthFuel>
where
    MatchedType: Copy,
    // V: CoVisitor<MatchedType> + CoVisitor<CaseCar>,
    (CaseCar, CaseCdr): Copy + AllCoVisitable<Heap, V, MatchedType, StrategyProvider, DepthFuel>,
{
    fn call(&mut self, heap: &mut Heap) -> (CaseCar, CaseCdr) {
        <(CaseCar, CaseCdr)>::all_co_visit(&mut self.cv, heap)
    }
}

impl<T, V, M, S, D> AdmitNoMatchingCase<T> for CoCallablefyCoVisitor<V, M, S, D>
where
    V: AdmitNoMatchingCase<T>,
    T: Heaped,
{
    fn no_matching_case(&self, heap: &mut <T as Heaped>::Heap) -> (T, Self::ShortCircuitsTo) {
        let (t, short) = self.cv.no_matching_case(heap);
        (t, CoCallablefyCoVisitor::new(short))
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

impl<CV, T, PatternMatchStrategyProvider, DepthFuel> FromSelectCase
    for CoCallablefyCoVisitor<CV, T, PatternMatchStrategyProvider, DepthFuel>
where
    CV: FromSelectCase,
{
    type ShortCircuitsTo =
        CoCallablefyCoVisitor<CV::ShortCircuitsTo, T, PatternMatchStrategyProvider, DepthFuel>;
}

impl<CV, T, PatternMatchStrategyProvider, Cases, DepthFuel> AcceptingCases<Cases>
    for CoCallablefyCoVisitor<CV, T, PatternMatchStrategyProvider, DepthFuel>
where
    Cases: ConsList,
    CV: AcceptingCases<Cases> + FromSelectCase,
{
    // type ShortCircuitsTo =
    //     CoCallablefyCoVisitor<CV::ShortCircuitsTo, T, PatternMatchStrategyProvider>;
    type AcceptingRemainingCases = CoCallablefyCoVisitor<
        CV::AcceptingRemainingCases,
        T,
        PatternMatchStrategyProvider,
        DepthFuel,
    >;

    fn try_case(self) -> Result<Self::ShortCircuitsTo, Self::AcceptingRemainingCases> {
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

impl<Heap, CV, PatternMatchStrategyProvider, T, DepthFuel>
    CoVisitable<CV, PatternMatchStrategyProvider, Heap, DepthFuel> for T
where
    DepthFuel: std::ops::Sub<typenum::B1>,
    T: Heaped<Heap = Heap>
        + CoCaseSplittable<
            CoCallablefyCoVisitor<
                CV::AC<PatternMatchStrategyProvider::Strategy>,
                T,
                PatternMatchStrategyProvider,
                typenum::Sub1<DepthFuel>,
            >,
            PatternMatchStrategyProvider::Strategy,
        > + Copy,
    CV: CoVisitor<T> + SelectCase + Default, //  + AcceptingCases<PatternMatchStrategyProvider::Strategy>
    CV::AC<PatternMatchStrategyProvider::Strategy>: AdmitNoMatchingCase<T>,
    PatternMatchStrategyProvider: crate::case_split::HasPatternMatchStrategyFor<T>,
{
    fn co_visit(visitor: &mut CV, heap: &mut Heap) -> Self {
        let mut ret: Option<Self> = None;
        visitor.co_push();
        let visitor_owned = std::mem::take(visitor);
        // take_mut::take(visitor, |visitor| {
        let visitor_co = visitor_owned.start_cases();
        let callable = CoCallablefyCoVisitor {
            cv: visitor_co,
            phantom: std::marker::PhantomData,
        };
        let (new_ret, short) = <T as CoCaseSplittable<_, _>>::co_case_split(callable, heap);
        ret = Some(new_ret);
        *visitor = short.cv;
        // });
        visitor.co_pop();
        ret.unwrap()
    }
}
