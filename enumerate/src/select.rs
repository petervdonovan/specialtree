use rand::Rng as _;
use term::{
    case_split::ConsList,
    select::{AcceptingCases, FromSelectCase, SelectCase},
};

use crate::{ChoosingEnumerator, Enumerator};

impl<Dp> SelectCase for Enumerator<Dp> {
    type AC<CasesConsList: term::case_split::ConsList> = ChoosingEnumerator<CasesConsList, Dp>;

    fn start_cases<CasesConsList: term::case_split::ConsList>(self) -> Self::AC<CasesConsList> {
        let mut self_ = self;
        let chosen_case = self_.random_state.random_range(0..CasesConsList::LENGTH);
        ChoosingEnumerator {
            chosen_case,
            bak: self_,
            phantom: std::marker::PhantomData,
        }
    }
}

impl<AllCurrentCases, Dp> FromSelectCase for ChoosingEnumerator<AllCurrentCases, Dp> {
    type ShortCircuitsTo = Enumerator<Dp>;
}

impl<AllCurrentCases, Dp> AcceptingCases<()> for ChoosingEnumerator<AllCurrentCases, Dp>
where
    AllCurrentCases: term::case_split::NonemptyConsList,
{
    type AcceptingRemainingCases = Self;

    fn try_case(self) -> Result<Self::ShortCircuitsTo, Self::AcceptingRemainingCases> {
        panic!()
    }
}

impl<RemainingCasesCar, RemainingCasesCdr, AllCurrentCases, Dp>
    AcceptingCases<(RemainingCasesCar, RemainingCasesCdr)>
    for ChoosingEnumerator<AllCurrentCases, Dp>
where
    AllCurrentCases: ConsList,
    RemainingCasesCdr: ConsList,
    Self: AcceptingCases<RemainingCasesCdr, ShortCircuitsTo = Enumerator<Dp>>,
{
    type AcceptingRemainingCases = Self;

    fn try_case(self) -> Result<Self::ShortCircuitsTo, Self::AcceptingRemainingCases> {
        if RemainingCasesCdr::LENGTH == self.chosen_case {
            Ok(self.bak)
        } else {
            Err(self)
        }
    }
}
