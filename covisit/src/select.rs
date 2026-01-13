use aspect::Aspect;
use pmsp::Strategy;

pub trait SelectCase
where
    Self: Sized,
{
    type A: Aspect;
    type AC<Cases: Strategy>: FromSelectCase<Done = Self>;
    fn start_cases<Cases: Strategy>(self) -> Self::AC<Cases>;
}
pub trait FromSelectCase {
    type Done;
}
pub trait AcceptingCases<Cases>: FromSelectCase
where
    Cases: Strategy,
{
    type AcceptingRemainingCases: AcceptingCases<Cases::Cdr, Done = Self::Done>;
    fn try_case(self) -> Result<Self::Done, Self::AcceptingRemainingCases>;
}
pub trait AdmitNoMatchingCase<LWord, L, T, Heap>
where
    Self: AcceptingCases<()>,
{
    fn admit(self, heap: &mut Heap) -> (Self::Done, T);
}
