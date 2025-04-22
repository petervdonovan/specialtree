use crate::case_split::ConsList;

pub trait SelectCase
where
    Self: Sized,
{
    type AC<CasesConsList: ConsList>: FromSelectCase<ShortCircuitsTo = Self>; //: AcceptingCases<CasesConsList, ShortCircuitsTo = Self>;
    fn start_cases<CasesConsList: ConsList>(self) -> Self::AC<CasesConsList>;
}
pub trait FromSelectCase {
    type ShortCircuitsTo;
}
pub trait AcceptingCases<CasesConsList>: FromSelectCase
where
    CasesConsList: ConsList,
{
    // type ShortCircuitsTo;
    type AcceptingRemainingCases: AcceptingCases<CasesConsList::Cdr, ShortCircuitsTo = Self::ShortCircuitsTo>;
    fn try_case(self) -> Result<Self::ShortCircuitsTo, Self::AcceptingRemainingCases>;
    // fn stop_with_external_success(
    //     self,
    //     proof_of_success: CasesConsList::Car,
    // ) -> Self::ShortCircuitsTo;
}
