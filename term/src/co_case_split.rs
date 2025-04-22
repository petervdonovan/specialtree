use crate::{
    CanonicallyConstructibleFrom, Heaped,
    case_split::ConsList,
    select::{AcceptingCases, FromSelectCase},
};

pub trait CoCaseSplittable<F, CcfConsList>: Sized + Heaped
where
    F: FromSelectCase,
{
    fn co_case_split(callable: F, heap: &mut Self::Heap) -> (Self, F::ShortCircuitsTo);
}

pub trait CoCallable<Heap, Case> {
    fn call(&mut self, heap: &mut Heap) -> Case;
}
pub trait AdmitNoMatchingCase<T>
where
    T: Heaped,
    Self: FromSelectCase,
{
    fn no_matching_case(&self, heap: &mut T::Heap) -> (T, Self::ShortCircuitsTo);
}
impl<F, T> CoCaseSplittable<F, ()> for T
where
    F: AdmitNoMatchingCase<T> + FromSelectCase,
    T: Heaped,
{
    fn co_case_split(callable: F, heap: &mut Self::Heap) -> (Self, F::ShortCircuitsTo) {
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
    T: CanonicallyConstructibleFrom<T::Heap, CasesCar>,
    F: AdmitNoMatchingCase<T>,
    F: AcceptingCases<(CasesCar, CasesCdr)>,
    <F as FromSelectCase>::ShortCircuitsTo: CoCallable<T::Heap, CasesCar>, //  + AcceptingCases<(CasesCar, CasesCdr)>
    T: CoCaseSplittable<
            <F as AcceptingCases<(CasesCar, CasesCdr)>>::AcceptingRemainingCases,
            CasesCdr,
        >,
{
    fn co_case_split(callable: F, heap: &mut Self::Heap) -> (Self, F::ShortCircuitsTo) {
        match <F as AcceptingCases<(CasesCar, CasesCdr)>>::try_case(callable) {
            Ok(mut short_circuited) => {
                let ok = short_circuited.call(heap);
                (T::construct(heap, ok), short_circuited)
            }
            Err(mut arc) => <T as CoCaseSplittable<
                <F as AcceptingCases<(CasesCar, CasesCdr)>>::AcceptingRemainingCases,
                CasesCdr,
            >>::co_case_split(arc, heap),
        }
    }
}
