use crate::{CanonicallyConstructibleFrom, Heaped, case_split::ConsList};

pub trait CoCaseSplittable<F, CcfConsList>: Sized + Heaped {
    fn co_case_split(callable: &mut F, heap: &mut Self::Heap) -> Option<Self>;
}

pub trait CoCallable<Case>: Heaped {
    fn call(&mut self, heap: &mut Self::Heap) -> Option<Case>;
}
pub trait AdmitNoMatchingCase<T>: Heaped {
    fn no_matching_case(&self, heap: &mut Self::Heap) -> Option<T>;
}
impl<F, T> CoCaseSplittable<F, ()> for T
where
    F: AdmitNoMatchingCase<T> + Heaped<Heap = T::Heap>,
    T: Heaped,
{
    fn co_case_split(callable: &mut F, heap: &mut Self::Heap) -> Option<Self> {
        callable.no_matching_case(heap)
    }
}
impl<F, T, CasesCar, CasesCdr> CoCaseSplittable<F, (CasesCar, CasesCdr)> for T
where
    CasesCdr: ConsList,
    F: CoCallable<CasesCar> + Heaped<Heap = T::Heap>,
    T: CanonicallyConstructibleFrom<CasesCar>,
    T: CoCaseSplittable<F, CasesCdr>,
    T: Copy,
    T: Heaped,
{
    fn co_case_split(callable: &mut F, heap: &mut Self::Heap) -> Option<Self> {
        if let Some(case) = callable.call(heap) {
            return Some(T::construct(heap, case));
        }
        T::co_case_split(callable, heap)
    }
}
