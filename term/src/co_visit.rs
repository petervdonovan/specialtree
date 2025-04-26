use crate::{
    Heaped,
    case_split::ConsList,
    co_case_split::{AdmitNoMatchingCase, CoCallable, CoCaseSplittable},
    fnlut::HasFn,
    select::{AcceptingCases, FromSelectCase, SelectCase},
};

pub trait CoVisitor<T> {
    fn co_push(&mut self);
    fn co_proceed(&mut self);
    fn co_pop(&mut self);
}

pub trait CoVisitable<CV, PatternMatchStrategyProvider, Heap, Fnlut>: Sized {
    fn co_visit(visitor: &mut CV, heap: &mut Heap, fnlut: Fnlut) -> Self;
}

pub trait AllCoVisitable<Heap, CV, Ctx, PatternMatchStrategyProvider, Fnlut>: Sized {
    fn all_co_visit(visitor: &mut CV, heap: &mut Heap, fnlut: Fnlut) -> Self;
}

impl<CV, Ctx, Heap, StrategyProvider, Fnlut, FieldsCar, FieldsCdrCar, FieldsCdrCdr>
    AllCoVisitable<Heap, CV, Ctx, StrategyProvider, Fnlut>
    for (FieldsCar, (FieldsCdrCar, FieldsCdrCdr))
where
    // FieldsCar: CoVisitable<CV, StrategyProvider, Heap, Fnlut>,
    Fnlut: HasFn<FieldsCar, for<'a, 'b> fn(&'a mut CV, &'b mut Heap) -> FieldsCar>,
    (FieldsCdrCar, FieldsCdrCdr): AllCoVisitable<Heap, CV, Ctx, StrategyProvider, Fnlut>,
{
    fn all_co_visit(visitor: &mut CV, heap: &mut Heap, fnlut: Fnlut) -> Self {
        let car = (fnlut.get::<FieldsCar>())(visitor, heap);
        // visitor.co_proceed();
        let cdr = <(FieldsCdrCar, FieldsCdrCdr)>::all_co_visit(visitor, heap, fnlut);
        (car, cdr)
    }
}

impl<CV, Ctx, Heap, StrategyProvider, FieldsCar, Fnlut>
    AllCoVisitable<Heap, CV, Ctx, StrategyProvider, Fnlut> for (FieldsCar, ())
where
    FieldsCar: CoVisitable<CV, StrategyProvider, Heap, Fnlut>,
{
    fn all_co_visit(visitor: &mut CV, heap: &mut Heap, fnlut: Fnlut) -> Self {
        let car = FieldsCar::co_visit(visitor, heap, fnlut);
        (car, ())
    }
}

struct CoCallablefyCoVisitor<CV, MatchedType, StrategyProvider, Fnlut> {
    cv: CV,
    fnlut: Fnlut,
    phantom: std::marker::PhantomData<(MatchedType, StrategyProvider)>,
}

impl<CV, MatchedType, StrategyProvider, Fnlut>
    CoCallablefyCoVisitor<CV, MatchedType, StrategyProvider, Fnlut>
{
    fn new(cv: CV, fnlut: Fnlut) -> Self {
        Self {
            cv,
            fnlut,
            phantom: std::marker::PhantomData,
        }
    }
}

impl<V: Heaped, MatchedType, StrategyProvider, Fnlut> Heaped
    for CoCallablefyCoVisitor<V, MatchedType, StrategyProvider, Fnlut>
{
    type Heap = V::Heap;
}

impl<V, MatchedType, Heap, CaseCar, CaseCdr, StrategyProvider, Fnlut: Copy>
    CoCallable<Heap, (CaseCar, CaseCdr)>
    for CoCallablefyCoVisitor<V, MatchedType, StrategyProvider, Fnlut>
where
    MatchedType: Copy,
    (CaseCar, CaseCdr): Copy + AllCoVisitable<Heap, V, MatchedType, StrategyProvider, Fnlut>,
{
    fn call(&mut self, heap: &mut Heap) -> (CaseCar, CaseCdr) {
        <(CaseCar, CaseCdr)>::all_co_visit(&mut self.cv, heap, self.fnlut)
    }
}

impl<T, V, M, S, F> AdmitNoMatchingCase<T> for CoCallablefyCoVisitor<V, M, S, F>
where
    V: AdmitNoMatchingCase<T>,
    T: Heaped,
    F: Copy,
{
    fn no_matching_case(&self, heap: &mut <T as Heaped>::Heap) -> (T, Self::ShortCircuitsTo) {
        let (t, short) = self.cv.no_matching_case(heap);
        (t, CoCallablefyCoVisitor::new(short, self.fnlut))
    }
}

impl<CV, T, PatternMatchStrategyProvider, Fnlut> FromSelectCase
    for CoCallablefyCoVisitor<CV, T, PatternMatchStrategyProvider, Fnlut>
where
    CV: FromSelectCase,
{
    type ShortCircuitsTo =
        CoCallablefyCoVisitor<CV::ShortCircuitsTo, T, PatternMatchStrategyProvider, Fnlut>;
}

impl<CV, T, PatternMatchStrategyProvider, Cases, Fnlut> AcceptingCases<Cases>
    for CoCallablefyCoVisitor<CV, T, PatternMatchStrategyProvider, Fnlut>
where
    Cases: ConsList,
    CV: AcceptingCases<Cases> + FromSelectCase,
{
    type AcceptingRemainingCases =
        CoCallablefyCoVisitor<CV::AcceptingRemainingCases, T, PatternMatchStrategyProvider, Fnlut>;

    fn try_case(self) -> Result<Self::ShortCircuitsTo, Self::AcceptingRemainingCases> {
        match self.cv.try_case() {
            Ok(short_circuited) => Ok(CoCallablefyCoVisitor::new(short_circuited, self.fnlut)),
            Err(remaining_cases) => {
                let remaining_cases = CoCallablefyCoVisitor {
                    cv: remaining_cases,
                    fnlut: self.fnlut,
                    phantom: std::marker::PhantomData,
                };
                Err(remaining_cases)
            }
        }
    }
}

impl<Heap, CV, PatternMatchStrategyProvider, T, Fnlut>
    CoVisitable<CV, PatternMatchStrategyProvider, Heap, Fnlut> for T
where
    T: Heaped<Heap = Heap>
        + CoCaseSplittable<
            CoCallablefyCoVisitor<
                CV::AC<PatternMatchStrategyProvider::Strategy>,
                T,
                PatternMatchStrategyProvider,
                Fnlut,
            >,
            PatternMatchStrategyProvider::Strategy,
        > + Copy,
    CV: CoVisitor<T> + SelectCase + Default,
    CV::AC<PatternMatchStrategyProvider::Strategy>: AdmitNoMatchingCase<T>,
    PatternMatchStrategyProvider: crate::case_split::HasPatternMatchStrategyFor<T>,
{
    fn co_visit(visitor: &mut CV, heap: &mut Heap, fnlut: Fnlut) -> Self {
        visitor.co_push();
        let visitor_owned = std::mem::take(visitor);
        // take_mut::take(visitor, |visitor| {
        let visitor_co = visitor_owned.start_cases();
        let callable = CoCallablefyCoVisitor {
            cv: visitor_co,
            fnlut,
            phantom: std::marker::PhantomData,
        };
        let (new_ret, short) = <T as CoCaseSplittable<_, _>>::co_case_split(callable, heap);
        *visitor = short.cv;
        // });
        visitor.co_pop();
        new_ret
    }
}
