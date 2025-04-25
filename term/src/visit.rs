use crate::{
    Heaped,
    case_split::{
        Callable, CaseSplittable, ConsList, HasBorrowedHeapRef, HasPatternMatchStrategyFor,
    },
};

pub enum MaybeAbortThisSubtree {
    Abort,
    Proceed,
}

pub trait VisitorDfs<T>: Heaped + HasBorrowedHeapRef {
    fn push(&mut self, t: T, heap: &mut Self::Borrowed<'_, Self::Heap>) -> MaybeAbortThisSubtree;
    fn proceed(&mut self, t: T, heap: &mut Self::Borrowed<'_, Self::Heap>);
    fn pop(&mut self, t: T, heap: &mut Self::Borrowed<'_, Self::Heap>);
}

pub trait Visitable<V: HasBorrowedHeapRef, PatternMatchStrategyProvider>: Sized + Heaped
where
    V: Heaped<Heap = <Self as Heaped>::Heap>,
{
    fn visit(&self, visitor: &mut V, heap: &mut V::Borrowed<'_, Self::Heap>);
}

pub trait AllVisitable<V: HasBorrowedHeapRef, Ctx, PatternMatchStrategyProvider>: Sized
where
    V: Heaped,
{
    fn all_visit(&self, visitor: &mut V, ctx: Ctx, heap: &mut V::Borrowed<'_, V::Heap>);
}

impl<V, Ctx, StrategyProvider, FieldsCar, FieldsCdrCar, FieldsCdrCdr>
    AllVisitable<V, Ctx, StrategyProvider> for (FieldsCar, (FieldsCdrCar, FieldsCdrCdr))
where
    Self: Copy,
    Ctx: Copy,
    V: VisitorDfs<Ctx>,
    FieldsCar: Visitable<V, StrategyProvider>,
    V: Heaped<Heap = FieldsCar::Heap> + HasBorrowedHeapRef,
    StrategyProvider: HasPatternMatchStrategyFor<FieldsCar>,
    (FieldsCdrCar, FieldsCdrCdr): AllVisitable<V, Ctx, StrategyProvider>,
    FieldsCdrCdr: ConsList,
{
    fn all_visit(&self, visitor: &mut V, ctx: Ctx, heap: &mut V::Borrowed<'_, V::Heap>) {
        let (car, cdr) = self.deconstruct();
        car.visit(visitor, heap);
        visitor.proceed(ctx, heap);
        cdr.all_visit(visitor, ctx, heap);
    }
}
impl<V, Ctx, StrategyProvider, FieldsCar> AllVisitable<V, Ctx, StrategyProvider> for (FieldsCar, ())
where
    Self: Copy,
    Ctx: Copy,
    V: VisitorDfs<Ctx>,
    FieldsCar: Visitable<V, StrategyProvider>,
    V: Heaped<Heap = FieldsCar::Heap> + HasBorrowedHeapRef,
    StrategyProvider: HasPatternMatchStrategyFor<FieldsCar>,
{
    fn all_visit(&self, visitor: &mut V, _: Ctx, heap: &mut V::Borrowed<'_, V::Heap>) {
        let (car, _) = self.deconstruct();
        car.visit(visitor, heap);
    }
}

struct CallablefyVisitor<V, MatchedType, StrategyProvider> {
    visitor: V,
    ctx: MatchedType,
    phantom: std::marker::PhantomData<StrategyProvider>,
}

impl<V: HasBorrowedHeapRef, MatchedType, StrategyProvider> HasBorrowedHeapRef
    for CallablefyVisitor<V, MatchedType, StrategyProvider>
{
    type Borrowed<'a, Heap: 'a> = V::Borrowed<'a, Heap>;
    fn with_decayed_heapref<Heap, R, F: Fn(&Heap) -> R>(
        heap: &mut Self::Borrowed<'_, Heap>,
        f: F,
    ) -> R {
        V::with_decayed_heapref(heap, f)
    }
}

impl<V: Heaped, MatchedType, StrategyProvider> Heaped
    for CallablefyVisitor<V, MatchedType, StrategyProvider>
{
    type Heap = V::Heap;
}

impl<V, MatchedType, StrategyProvider> Callable<()>
    for CallablefyVisitor<V, MatchedType, StrategyProvider>
where
    MatchedType: Copy,
    V: VisitorDfs<MatchedType>,
{
    fn call(&mut self, _: (), heap: &mut Self::Borrowed<'_, Self::Heap>) {
        self.visitor.pop(self.ctx, heap);
    }
}

impl<V, MatchedType, CaseCar, CaseCdr, StrategyProvider> Callable<(CaseCar, CaseCdr)>
    for CallablefyVisitor<V, MatchedType, StrategyProvider>
where
    MatchedType: Copy,
    V: VisitorDfs<MatchedType> + VisitorDfs<CaseCar>,
    (CaseCar, CaseCdr): Copy + AllVisitable<V, MatchedType, StrategyProvider>,
{
    fn call(&mut self, case: (CaseCar, CaseCdr), heap: &mut Self::Borrowed<'_, Self::Heap>) {
        case.all_visit(&mut self.visitor, self.ctx, heap);
    }
}

impl<V, PatternMatchStrategyProvider, T> Visitable<V, PatternMatchStrategyProvider> for T
where
    T: Heaped<Heap = V::Heap>
        + CaseSplittable<
            CallablefyVisitor<V, T, PatternMatchStrategyProvider>,
            PatternMatchStrategyProvider::Strategy,
        > + Copy,
    V: VisitorDfs<T>,
    PatternMatchStrategyProvider: HasPatternMatchStrategyFor<T>,
{
    fn visit(
        &self,
        visitor: &mut V,
        heap: &mut <V as HasBorrowedHeapRef>::Borrowed<'_, Self::Heap>,
    ) {
        if let MaybeAbortThisSubtree::Proceed = visitor.push(*self, heap) {
            take_mut::take(visitor, |visitor| {
                let mut callable = CallablefyVisitor {
                    visitor,
                    ctx: *self,
                    phantom: std::marker::PhantomData,
                };
                <T as CaseSplittable<_, _>>::case_split(self, &mut callable, heap);
                callable.visitor
            });
        }
        visitor.pop(*self, heap);
    }
}
