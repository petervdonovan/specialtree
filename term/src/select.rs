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

// mod test {
//     struct A0<A1>(std::marker::PhantomData<A1>);
//     struct B0<B1>(std::marker::PhantomData<B1>);
//     trait Ind {
//         fn run();
//     }
//     trait WouldBeIndIfWasInd<T> {}

//     impl<A1> Ind for A0<A1>
//     where
//         B0<A1>: Ind,
//     {
//         fn run() {
//             println!("A0");
//             B0::<A1>::run();
//         }
//     }
//     impl<B1> Ind for B0<B1>
//     where
//         // A0<B1>: WouldBeIndIfWasInd<Self>,
//         Self: std::fmt::Display,
//     {
//         fn run() {
//             println!("B0");
//             A0::<B1>::run();
//         }
//     }
//     fn run() {
//         A0::<()>::run();
//     }
// }
