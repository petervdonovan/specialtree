use crate::case_split::ConsList;

pub trait SelectCase<CasesConsList>
where
    Self: Sized,
    CasesConsList: ConsList,
{
    type AC: AcceptingCases<CasesConsList, ShortCircuitsTo = Self>;
    fn start_cases() -> Self::AC;
}
pub trait AcceptingCases<CasesConsList>
where
    CasesConsList: ConsList,
{
    type ShortCircuitsTo;
    type AcceptingRemainingCases: AcceptingCases<CasesConsList::Cdr, ShortCircuitsTo = Self::ShortCircuitsTo>;
    fn try_case(&mut self) -> Result<Self::ShortCircuitsTo, Self::AcceptingRemainingCases>;
    fn stop_with_external_success(
        self,
        proof_of_success: CasesConsList::Car,
    ) -> Self::ShortCircuitsTo;
}
