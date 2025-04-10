use crate::{
    Heaped,
    case_split::{
        Callable, CaseSplittable, ConsList, HasBorrowedHeapRef, HasPatternMatchStrategyFor,
        PatternMatchable,
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
    fn all_visit(&self, visitor: &mut V, ctx: Ctx, heap: V::Borrowed<'_, V::Heap>);
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
{
    fn all_visit(&self, visitor: &mut V, ctx: Ctx, heap: V::Borrowed<'_, V::Heap>) {
        let mut heap = heap;
        let (car, cdr) = self.deconstruct();
        car.visit(visitor, &mut heap);
        visitor.proceed(ctx, &mut heap);
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
    fn all_visit(&self, visitor: &mut V, ctx: Ctx, heap: V::Borrowed<'_, V::Heap>) {
        let mut heap = heap;
        let (car, ()) = self.deconstruct();
        car.visit(visitor, &mut heap);
    }
}

struct CallablefyVisitor<V, MatchedType, StrategyProvider> {
    visitor: V,
    ctx: MatchedType,
    phantom: std::marker::PhantomData<StrategyProvider>,
}

impl<V, MatchedType, StrategyProvider> CallablefyVisitor<V, MatchedType, StrategyProvider> {
    fn descend<Ctx, F: FnMut(&mut CallablefyVisitor<V, Ctx, StrategyProvider>)>(
        &mut self,
        ctx: Ctx,
        mut f: F,
    ) {
        take_mut::take(self, |this| {
            let CallablefyVisitor {
                visitor,
                ctx: outer_ctx,
                ..
            } = this;
            let mut descended = CallablefyVisitor {
                visitor,
                ctx,
                phantom: std::marker::PhantomData,
            };
            f(&mut descended);
            Self {
                visitor: descended.visitor,
                ctx: outer_ctx,
                phantom: std::marker::PhantomData,
            }
        });
    }
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

// impl<V, StrategyProvider> Callable<()> for CallablefyVisitor<V, (), StrategyProvider>
// where
//     V: VisitorDfs<()>
//         + Heaped<Heap = Self::Heap>
//         + for<'a> HasBorrowedHeapRef<Borrowed<'a, Self::Heap> = Self::Borrowed<'a, Self::Heap>>,
// {
//     fn call(&mut self, _: (), mut heap: Self::Borrowed<'_, Self::Heap>) {
//         if let MaybeAbortThisSubtree::Abort = self.0.push((), &mut heap) {
//             return;
//         }
//         self.0.proceed((), &mut heap);
//         self.0.pop((), &mut heap);
//     }

//     fn do_not_call(&mut self) {
//         // do nothing
//     }
// }

impl<V, MatchedType, StrategyProvider> Callable<()>
    for CallablefyVisitor<V, MatchedType, StrategyProvider>
where
    MatchedType: Copy,
    V: VisitorDfs<MatchedType>,
{
    fn call(&mut self, _: (), heap: &mut Self::Borrowed<'_, Self::Heap>) {
        self.visitor.pop(self.ctx, heap);
    }

    // fn do_not_call(&mut self) {
    //     panic!("should be unreachable because this is the nil case");
    // }
}

impl<V, MatchedType, Case, StrategyProvider> Callable<Case>
    for CallablefyVisitor<V, MatchedType, StrategyProvider>
where
    MatchedType: Copy,
    V: VisitorDfs<MatchedType>
        + VisitorDfs<Case::Car>
        + Heaped<Heap = Self::Heap>
        + for<'a> HasBorrowedHeapRef<Borrowed<'a, Self::Heap> = Self::Borrowed<'a, Self::Heap>>,
    Self::Heap: 'static, // a technically unnecessary way to work around implied bounds for the HRTB not seeming to work
    Case: Copy + ConsList,
    StrategyProvider: HasPatternMatchStrategyFor<Case::Car>,
    Case::Car: AllVisitable<
            CallablefyVisitor<V, Case::Car, StrategyProvider>,
            MatchedType,
            StrategyProvider,
        > + Heaped<Heap = Self::Heap>
        + Copy,
    Self: Callable<Case::Cdr>,
{
    fn call(&mut self, case: Case, heap: &mut Self::Borrowed<'_, Self::Heap>) {
        // let heapref: &mut Self::Borrowed<'_, Self::Heap> = &mut heap;
        // let heapref: &mut Self::Borrowed<'_, V::Heap> = heapref;
        // the following would be safe because we defined Self::Borrowed to be the same as V::Borrowed
        // ... but rustc does not seem to deduce this
        // let heapref: &mut V::Borrowed<'_, V::Heap> = unsafe { std::mem::transmute(heapref) };
        let (car, cdr) = case.deconstruct();
        if let MaybeAbortThisSubtree::Proceed = self.visitor.push(car, heap) {
            // car.do_match(self.descend(car), heap);
            self.descend(car, |this| {
                // <Case::Car as PatternMatchable<
                //     CallablefyVisitor<V, Case::Car, StrategyProvider>,
                //     StrategyProvider,
                // >>::do_match(&car, this, heap);
            });
        }
        self.visitor.proceed(self.ctx, heap);
        self.call(cdr, heap);
    }

    // fn do_not_call(&mut self) {
    //     // do nothing
    // }
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
        let mut heap = heap;
        take_mut::take(visitor, |visitor| {
            let mut callable = CallablefyVisitor {
                visitor,
                ctx: *self,
                phantom: std::marker::PhantomData,
            };
            <T as CaseSplittable<_, _>>::case_split(self, &mut callable, &mut heap);
            callable.visitor
        });
    }
}
// The following demonstrates that mutually recursive trait implementations work Just Fine if you use a trait bound
// trait Loop0: Loop1 {
//     fn loop0(&self);
// }
// trait Loop1 {
//     fn loop1(&self);
// }

// impl<T> Loop0 for T
// {
//     fn loop0(&self) {
//         self.loop1();
//     }
// }

// impl<T> Loop1 for T
// where
//     T: Loop0,
// {
//     fn loop1(&self) {
//         self.loop0();
//     }
// }

// fn test() {
//     struct A();
//     (A()).loop0();
// }
