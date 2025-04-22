use crate::{CanonicallyConstructibleFrom, Heaped};

#[fundamental]
pub trait Adt: Sized {
    type PatternMatchStrategyProvider: HasPatternMatchStrategyFor<Self>;
}

pub trait ConsList {
    type Car;
    type Cdr: ConsList;
    fn deconstruct(self) -> (Self::Car, Self::Cdr);
}
impl ConsList for () {
    type Car = ();
    type Cdr = ();
    fn deconstruct(self) -> (Self::Car, Self::Cdr) {
        ((), ())
    }
}
impl<T, Cdr> ConsList for (T, Cdr)
where
    Cdr: ConsList,
{
    type Car = T;
    type Cdr = Cdr;
    fn deconstruct(self) -> (Self::Car, Self::Cdr) {
        self
    }
}
pub trait CaseSplittable<F: HasBorrowedHeapRef, CcfConsList: ConsList>: Sized + Heaped
where
    F: Heaped<Heap = <Self as Heaped>::Heap>,
{
    fn case_split(&self, callable: &mut F, heap: &mut F::Borrowed<'_, Self::Heap>);
}
pub trait HasBorrowedHeapRef {
    type Borrowed<'a, Heap: 'a>: 'a;
    fn with_decayed_heapref<Heap, T, F: Fn(&Heap) -> T>(
        heap: &mut Self::Borrowed<'_, Heap>,
        f: F,
    ) -> T;
}
pub trait AdmitNoMatchingCase<T>: Heaped + HasBorrowedHeapRef {
    fn no_matching_case(&self, t: &T, heap: &mut Self::Borrowed<'_, Self::Heap>);
}
pub trait Callable<Case>: Heaped + HasBorrowedHeapRef {
    fn call(&mut self, t: Case, heap: &mut Self::Borrowed<'_, Self::Heap>);
    // fn do_not_call(&mut self);
}
impl<F: AdmitNoMatchingCase<T> + Heaped<Heap = T::Heap>, T: Heaped> CaseSplittable<F, ()> for T {
    fn case_split(&self, callable: &mut F, heap: &mut F::Borrowed<'_, Self::Heap>) {
        callable.no_matching_case(self, heap);
    }
}
impl<F, T: Heaped, CasesCar, CasesCdr: ConsList> CaseSplittable<F, (CasesCar, CasesCdr)> for T
where
    F: Callable<CasesCar> + Heaped<Heap = T::Heap>,
    T: CanonicallyConstructibleFrom<T::Heap, CasesCar>,
    T: CaseSplittable<F, CasesCdr>,
    T: Copy,
{
    fn case_split(
        &self,
        callable: &mut F,
        heap: &mut <F as HasBorrowedHeapRef>::Borrowed<'_, Self::Heap>,
    ) {
        if <F as HasBorrowedHeapRef>::with_decayed_heapref(heap, |hr| {
            <Self as CanonicallyConstructibleFrom<T::Heap, CasesCar>>::deconstruct_succeeds(
                self, hr,
            )
        }) {
            let t = <F as HasBorrowedHeapRef>::with_decayed_heapref(heap, |hr| {
                <Self as CanonicallyConstructibleFrom<T::Heap, CasesCar>>::deconstruct(*self, hr)
            });
            callable.call(t, heap);
        } else {
            // callable.do_not_call();
            <Self as CaseSplittable<F, CasesCdr>>::case_split(self, callable, heap);
        }
    }
}
pub trait HasPatternMatchStrategyFor<T> {
    type Strategy: ConsList;
}
pub trait PatternMatchable<F: HasBorrowedHeapRef, Strategies>: Heaped {
    fn do_match(&self, callable: &mut F, heap: &mut F::Borrowed<'_, <Self as Heaped>::Heap>);
}
impl<F, T, StrategyProvider> PatternMatchable<F, StrategyProvider> for T
where
    StrategyProvider: HasPatternMatchStrategyFor<T>,
    F: HasBorrowedHeapRef + Heaped<Heap = T::Heap>,
    T: CaseSplittable<F, StrategyProvider::Strategy>,
{
    fn do_match(&self, callable: &mut F, heap: &mut F::Borrowed<'_, <Self as Heaped>::Heap>) {
        <Self as CaseSplittable<F, StrategyProvider::Strategy>>::case_split(self, callable, heap);
    }
}
