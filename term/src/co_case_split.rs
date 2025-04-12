use crate::{CanonicallyConstructibleFrom, Heaped, case_split::ConsList, select::AcceptingCases};

pub trait CoCaseSplittable<F, CcfConsList>: Sized + Heaped {
    fn co_case_split(callable: &mut F, heap: &mut Self::Heap) -> Self;
}

pub trait CoCallable<Heap, Case> {
    fn call(&mut self, heap: &mut Heap) -> Case;
}
pub trait AdmitNoMatchingCase<T>
where
    T: Heaped,
{
    fn no_matching_case(&self, heap: &mut T::Heap) -> T;
}
impl<F, T> CoCaseSplittable<F, ()> for T
where
    F: AdmitNoMatchingCase<T>,
    T: Heaped,
{
    fn co_case_split(callable: &mut F, heap: &mut Self::Heap) -> Self {
        callable.no_matching_case(heap)
    }
}

// pub trait CoCallableForStrategy<Heap, Strategy>:
//     AcceptingCases<Strategy, ShortCircuitsTo: CoCallable<Heap, Strategy::Car>>
// where
//     Strategy: ConsList,
// {
// }
// impl<Heap, Strategy, T> CoCallableForStrategy<Heap, Strategy> for T
// where
//     Self: AcceptingCases<Strategy, ShortCircuitsTo: CoCallable<Heap, Strategy::Car>>,
//     Strategy: ConsList,
// {
// }

impl<F, T, CasesCar, CasesCdr> CoCaseSplittable<F, (CasesCar, CasesCdr)> for T
where
    CasesCdr: ConsList,
    T: Copy,
    T: Heaped,
    T: CanonicallyConstructibleFrom<CasesCar>,
    F: AdmitNoMatchingCase<T>,
    F: AcceptingCases<(CasesCar, CasesCdr)>,
    <F as AcceptingCases<(CasesCar, CasesCdr)>>::ShortCircuitsTo:
        CoCallable<T::Heap, CasesCar> + AcceptingCases<(CasesCar, CasesCdr)>,
    T: CoCaseSplittable<
            <F as AcceptingCases<(CasesCar, CasesCdr)>>::AcceptingRemainingCases,
            CasesCdr,
        >,
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
