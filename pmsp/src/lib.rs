#![allow(incomplete_features)]
#![feature(lazy_type_alias)]
#![feature(fundamental)]

use conslist::ConsList;
use words::Implements;

pub trait NamesPatternMatchStrategyGivenContext<Heap> {
    type Strategy: Strategy;
}

pub type StrategyOf<T, Heap, L>
    = <<T as Implements<Heap, L>>::LWord as NamesPatternMatchStrategyGivenContext<Heap>>::Strategy
where
    T: Implements<Heap, L>,
    <T as Implements<Heap, L>>::LWord: NamesPatternMatchStrategyGivenContext<Heap>;

pub trait Strategy {
    type Car: ConsList;
    type Cdr: Strategy;
}
#[fundamental]
pub trait NonemptyStrategy: Strategy {}
impl<Car, Cdr> NonemptyStrategy for (Car, Cdr)
where
    Cdr: Strategy,
    Car: ConsList,
{
}
impl Strategy for () {
    type Car = ();

    type Cdr = ();
}
impl<T, U> Strategy for (T, U)
where
    T: ConsList,
    U: Strategy,
{
    type Car = T;

    type Cdr = U;
}
