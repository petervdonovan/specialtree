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

pub trait VisitorDfs<Heap, T>: HasBorrowedHeapRef {
    fn push(&mut self, t: T, heap: &mut Self::Borrowed<'_, Heap>) -> MaybeAbortThisSubtree;
    fn proceed(&mut self, t: T, heap: &mut Self::Borrowed<'_, Heap>);
    fn pop(&mut self, t: T, heap: &mut Self::Borrowed<'_, Heap>);
}

pub trait Visitable<V: HasBorrowedHeapRef, Heap, PatternMatchStrategyProvider>: Sized {
    fn visit(&self, visitor: &mut V, heap: &mut V::Borrowed<'_, Heap>);
}

pub trait AllVisitable<V: HasBorrowedHeapRef, Heap, Ctx, PatternMatchStrategyProvider>:
    Sized
{
    fn all_visit(&self, visitor: &mut V, ctx: Ctx, heap: &mut V::Borrowed<'_, Heap>);
}

impl<V, Heap, Ctx, StrategyProvider, FieldsCar, FieldsCdrCar, FieldsCdrCdr>
    AllVisitable<V, Heap, Ctx, StrategyProvider> for (FieldsCar, (FieldsCdrCar, FieldsCdrCdr))
where
    Self: Copy,
    Ctx: Copy,
    V: VisitorDfs<Heap, Ctx>,
    FieldsCar: Visitable<V, Heap, StrategyProvider>,
    V: HasBorrowedHeapRef,
    StrategyProvider: HasPatternMatchStrategyFor<FieldsCar>,
    (FieldsCdrCar, FieldsCdrCdr): AllVisitable<V, Heap, Ctx, StrategyProvider>,
    FieldsCdrCdr: ConsList,
{
    fn all_visit(&self, visitor: &mut V, ctx: Ctx, heap: &mut V::Borrowed<'_, Heap>) {
        let (car, cdr) = self.deconstruct();
        car.visit(visitor, heap);
        visitor.proceed(ctx, heap);
        cdr.all_visit(visitor, ctx, heap);
    }
}
impl<V, Heap, Ctx, StrategyProvider, FieldsCar> AllVisitable<V, Heap, Ctx, StrategyProvider>
    for (FieldsCar, ())
where
    Self: Copy,
    Ctx: Copy,
    V: VisitorDfs<Heap, Ctx>,
    FieldsCar: Visitable<V, Heap, StrategyProvider>,
    V: HasBorrowedHeapRef,
    StrategyProvider: HasPatternMatchStrategyFor<FieldsCar>,
{
    fn all_visit(&self, visitor: &mut V, _: Ctx, heap: &mut V::Borrowed<'_, Heap>) {
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

impl<V, Heap, MatchedType, StrategyProvider> Callable<Heap, ()>
    for CallablefyVisitor<V, MatchedType, StrategyProvider>
where
    MatchedType: Copy,
    V: VisitorDfs<Heap, MatchedType>,
{
    fn call(&mut self, _: (), heap: &mut Self::Borrowed<'_, Heap>) {
        self.visitor.pop(self.ctx, heap);
    }
}

impl<V, Heap, MatchedType, CaseCar, CaseCdr, StrategyProvider> Callable<Heap, (CaseCar, CaseCdr)>
    for CallablefyVisitor<V, MatchedType, StrategyProvider>
where
    MatchedType: Copy,
    V: VisitorDfs<Heap, MatchedType> + VisitorDfs<Heap, CaseCar>,
    (CaseCar, CaseCdr): Copy + AllVisitable<V, Heap, MatchedType, StrategyProvider>,
{
    fn call(&mut self, case: (CaseCar, CaseCdr), heap: &mut Self::Borrowed<'_, Heap>) {
        case.all_visit(&mut self.visitor, self.ctx, heap);
    }
}

impl<V, Heap, PatternMatchStrategyProvider, T> Visitable<V, Heap, PatternMatchStrategyProvider>
    for T
where
    T: CaseSplittable<
            CallablefyVisitor<V, T, PatternMatchStrategyProvider>,
            Heap,
            PatternMatchStrategyProvider::Strategy,
        > + Copy,
    V: VisitorDfs<Heap, T>,
    PatternMatchStrategyProvider: HasPatternMatchStrategyFor<T>,
{
    fn visit(&self, visitor: &mut V, heap: &mut <V as HasBorrowedHeapRef>::Borrowed<'_, Heap>) {
        if let MaybeAbortThisSubtree::Proceed = visitor.push(*self, heap) {
            take_mut::take(visitor, |visitor| {
                let mut callable = CallablefyVisitor {
                    visitor,
                    ctx: *self,
                    phantom: std::marker::PhantomData,
                };
                <T as CaseSplittable<_, _, _>>::case_split(self, &mut callable, heap);
                callable.visitor
            });
        }
        visitor.pop(*self, heap);
    }
}
