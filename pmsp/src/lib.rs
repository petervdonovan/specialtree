#![allow(incomplete_features)]
#![feature(lazy_type_alias)]
#![feature(fundamental)]

use conslist::ConsList;
use words::Implements;

pub trait NamesPatternMatchStrategy<L> {
    type Strategy: Strategy;
    // type TyMetadatas: Strategy;
}
// #[fundamental] // opt TmfMetadata out of the "covers" rule for when OgType is a local type
// pub struct TmfMetadata<OgType, ArgMetadatas>(std::marker::PhantomData<(OgType, ArgMetadatas)>);
// pub struct AdtMetadata;

pub type StrategyOf<T, Heap, L>
    = <<T as Implements<Heap, L>>::LWord as NamesPatternMatchStrategy<L>>::Strategy
where
    T: Implements<Heap, L>,
    <T as Implements<Heap, L>>::LWord: NamesPatternMatchStrategy<L>;

// pub type TyMetadataOf<T, Heap, L>
//     = <<T as Implements<Heap, L>>::LWord as NamesPatternMatchStrategy>::TyMetadatas
// where
//     T: Implements<Heap, L>,
//     <T as Implements<Heap, L>>::LWord: NamesPatternMatchStrategy;

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
