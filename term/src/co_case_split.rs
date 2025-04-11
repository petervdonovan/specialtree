use crate::{CanonicallyConstructibleFrom, Heaped, case_split::ConsList, select::AcceptingCases};

pub trait CoCaseSplittable<F, CcfConsList>: Sized + Heaped {
    fn co_case_split(callable: &mut F, heap: &mut Self::Heap) -> Self;
}

pub trait CoCallable<Case>: Heaped {
    fn call(&mut self, heap: &mut Self::Heap) -> Case;
}
pub trait AdmitNoMatchingCase<T>: Heaped {
    fn no_matching_case(&self, heap: &mut Self::Heap) -> T;
}
impl<F, T> CoCaseSplittable<F, ()> for T
where
    F: AdmitNoMatchingCase<T> + AcceptingCases<()> + Heaped<Heap = T::Heap>,
    T: Heaped,
{
    fn co_case_split(callable: &mut F, heap: &mut Self::Heap) -> Self {
        callable.no_matching_case(heap)
    }
}
impl<F, T, CasesCar, CasesCdr> CoCaseSplittable<F, (CasesCar, CasesCdr)> for T
where
    CasesCdr: ConsList,
    T: CanonicallyConstructibleFrom<CasesCar>,
    <F as AcceptingCases<(CasesCar, CasesCdr)>>::ShortCircuitsTo:
        CoCallable<CasesCar> + AcceptingCases<(CasesCar, CasesCdr)> + Heaped<Heap = T::Heap>,
    F: AcceptingCases<(CasesCar, CasesCdr)>,
    T: CoCaseSplittable<
            <F as AcceptingCases<(CasesCar, CasesCdr)>>::AcceptingRemainingCases,
            CasesCdr,
        >,
    T: Copy,
    T: Heaped,
{
    fn co_case_split(callable: &mut F, heap: &mut Self::Heap) -> Self {
        match <F as AcceptingCases<(CasesCar, CasesCdr)>>::try_case(callable) {
            Ok(mut short_circuited) => {
                let ok = short_circuited.call(heap);
                T::construct(heap, ok)
            }
            Err(mut arc) => <T as CoCaseSplittable<
                <F as AcceptingCases<(CasesCar, CasesCdr)>>::AcceptingRemainingCases,
                CasesCdr,
            >>::co_case_split(&mut arc, heap),
        }
    }
}
