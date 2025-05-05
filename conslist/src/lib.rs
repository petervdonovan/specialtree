#![feature(fundamental)]

pub trait ConsList {
    type Car;
    type Cdr: ConsList;
    const LENGTH: u32;
    fn deconstruct(self) -> (Self::Car, Self::Cdr);
    fn reconstruct(car: Self::Car, cdr: Self::Cdr) -> Self
    where
        Self: Sized;
}
impl ConsList for () {
    type Car = ();
    type Cdr = ();
    const LENGTH: u32 = 0;
    fn deconstruct(self) -> (Self::Car, Self::Cdr) {
        ((), ())
    }

    fn reconstruct(_: Self::Car, _: Self::Cdr) -> Self
    where
        Self: Sized,
    {
    }
}
#[fundamental]
pub trait NonemptyConsList: ConsList {}
impl<Car, Cdr> NonemptyConsList for (Car, Cdr) where Cdr: ConsList {}
#[fundamental]
pub trait AtLeastTwoConsList: ConsList {}
impl<Car, Cdr> AtLeastTwoConsList for (Car, Cdr) where Cdr: NonemptyConsList {}
impl<T, Cdr> ConsList for (T, Cdr)
where
    Cdr: ConsList,
{
    type Car = T;
    type Cdr = Cdr;
    const LENGTH: u32 = Cdr::LENGTH + 1;
    fn deconstruct(self) -> (Self::Car, Self::Cdr) {
        self
    }

    fn reconstruct(car: Self::Car, cdr: Self::Cdr) -> Self
    where
        Self: Sized,
    {
        (car, cdr)
    }
}
