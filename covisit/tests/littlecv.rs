// #[derive(Debug)]
// struct SomeAdt(SomeTmf<Self>);

// #[derive(Debug)]
// enum SomeTmf<X> {
//     Adt(Box<X>),
//     None,
// }

// impl Adt for SomeAdt {}

// impl From<SomeTmf<Self>> for SomeAdt {
//     fn from(it: SomeTmf<Self>) -> Self {
//         SomeAdt(it)
//     }
// }

// struct SomeCovisit {
//     fuel: usize,
// }

// impl<SomeAdt2> Covisit<SomeTmf<SomeAdt2>> for SomeCovisit
// where
//     // SomeAdt2: Adt<Strategy = Self>,
//     // SomeCovisit: Covisit<SomeAdt2::Strategy>,
//     SomeCovisit: Covisit<SomeAdt2>,
// {
//     fn covisit(&mut self) -> SomeTmf<SomeAdt2> {
//         self.fuel -= 1;
//         if self.fuel == 0 {
//             return SomeTmf::None;
//         }
//         SomeTmf::Adt(Box::new(<SomeCovisit as Covisit<SomeAdt2>>::covisit(self)))
//     }
// }
