use crate::{CanonicallyConstructibleFrom, Heaped};

pub trait Adt: Sized {
    type PatternMatchStrategyProvider: HasPatternMatchStrategyFor<Self>;
}

pub trait ConsList {
    type Car;
    type Cdr;
}
impl ConsList for () {
    type Car = ();
    type Cdr = ();
}
impl<T, Cdr> ConsList for (T, Cdr) {
    type Car = T;
    type Cdr = Cdr;
}
pub trait CaseSplittable<F: HasBorrowedHeapRef, CcfConsList: ConsList>: Sized + Heaped
where
    F: Heaped<Heap = <Self as Heaped>::Heap>,
{
    fn case_split(&self, callable: &mut F, heap: F::Borrowed<'_, Self::Heap>);
}
pub trait HasBorrowedHeapRef {
    type Borrowed<'a, Heap: 'a>: 'a;
    fn with_decayed_heapref<Heap, T, F: Fn(&Heap) -> T>(
        heap: &mut Self::Borrowed<'_, Heap>,
        f: F,
    ) -> T;
}
pub trait Callable<T>: Heaped + HasBorrowedHeapRef {
    fn call(&mut self, t: T, heap: Self::Borrowed<'_, Self::Heap>);
}
impl<F: Callable<()> + Heaped<Heap = T::Heap>, T: Heaped> CaseSplittable<F, ()> for T {
    fn case_split(&self, _callable: &mut F, _heap: F::Borrowed<'_, Self::Heap>) {
        panic!("no matching case");
    }
}
impl<F, T: Heaped, Car, Cdr: ConsList> CaseSplittable<F, (Car, Cdr)> for T
where
    F: Callable<Car> + HasBorrowedHeapRef + Heaped<Heap = T::Heap>,
    T: CanonicallyConstructibleFrom<Car>,
    T: CaseSplittable<F, Cdr>,
    T: Copy,
{
    fn case_split(
        &self,
        callable: &mut F,
        heap: <F as HasBorrowedHeapRef>::Borrowed<'_, Self::Heap>,
    ) {
        let mut heap = heap;
        if <F as HasBorrowedHeapRef>::with_decayed_heapref(&mut heap, |hr| {
            <Self as CanonicallyConstructibleFrom<Car>>::deconstruct_succeeds(self, hr)
        }) {
            let t = <F as HasBorrowedHeapRef>::with_decayed_heapref(&mut heap, |hr| {
                <Self as CanonicallyConstructibleFrom<Car>>::deconstruct(*self, hr)
            });
            callable.call(t, heap);
        } else {
            <Self as CaseSplittable<F, Cdr>>::case_split(self, callable, heap);
        }
    }
}
pub trait HasPatternMatchStrategyFor<T> {
    type Strategy: ConsList;
}
pub trait PatternMatchable<F: HasBorrowedHeapRef, Strategies>: Heaped {
    fn do_match(&self, callable: &mut F, heap: F::Borrowed<'_, <Self as Heaped>::Heap>);
}
impl<F, T, StrategyProvider> PatternMatchable<F, StrategyProvider> for T
where
    StrategyProvider: HasPatternMatchStrategyFor<T>,
    F: HasBorrowedHeapRef + Heaped<Heap = T::Heap>,
    T: CaseSplittable<F, StrategyProvider::Strategy>,
{
    fn do_match(&self, callable: &mut F, heap: F::Borrowed<'_, <Self as Heaped>::Heap>) {
        <Self as CaseSplittable<F, StrategyProvider::Strategy>>::case_split(self, callable, heap);
    }
}
