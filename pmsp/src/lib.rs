#![allow(incomplete_features)]
#![feature(lazy_type_alias)]
#![feature(fundamental)]

use conslist::ConsList;
use words::Implements;

pub trait NamesPatternMatchStrategyGivenContext<Heap> {
    type Strategy: Strategy;
}

#[fundamental]
pub trait UsesStrategyForTraversal<VisitorOrCovisitor> {}

impl<T, VisitorOrCovisitor> UsesStrategyForTraversal<VisitorOrCovisitor> for T where T: words::Adt {}

pub type StrategyOf<T, Heap, L>
    = <<T as Implements<Heap, L>>::LWord as NamesPatternMatchStrategyGivenContext<Heap>>::Strategy
where
    T: Implements<Heap, L>,
    <T as Implements<Heap, L>>::LWord: NamesPatternMatchStrategyGivenContext<Heap>;

pub trait Strategy {
    type Car: ConsList;
    type Cdr: Strategy;
    const LENGTH: u32;
}
#[fundamental]
pub trait NonemptyStrategy: Strategy {}
#[fundamental]
pub trait AtLeastTwoStrategy: Strategy {}
impl<Car, Cdr> NonemptyStrategy for (Car, Cdr)
where
    Cdr: Strategy,
    Car: ConsList,
{
}
impl<Car, Cdr> AtLeastTwoStrategy for (Car, Cdr)
where
    Car: ConsList,
    Cdr: NonemptyStrategy,
{
}
impl Strategy for () {
    type Car = ();

    type Cdr = ();
    const LENGTH: u32 = 0;
}
impl<T, U> Strategy for (T, U)
where
    T: ConsList,
    U: Strategy,
{
    type Car = T;

    type Cdr = U;
    const LENGTH: u32 = Self::Cdr::LENGTH + 1;
}
