use crate::case_split::ConsList;

pub trait SelectCase
where
    Self: Sized,
{
    type AC<CasesConsList: ConsList>: InSelectCase<EndSelectCase = Self>;
    fn start_cases<CasesConsList: ConsList>(self) -> Self::AC<CasesConsList>;
}
pub trait InSelectCase {
    type EndSelectCase;
}
pub trait AcceptingCases<CasesConsList>: InSelectCase
where
    CasesConsList: ConsList,
{
    type AcceptingRemainingCases: AcceptingCases<CasesConsList::Cdr, EndSelectCase = Self::EndSelectCase>;
    fn try_case(self) -> Result<Self::EndSelectCase, Self::AcceptingRemainingCases>;
}
