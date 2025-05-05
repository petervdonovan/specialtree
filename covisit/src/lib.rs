// see https://github.com/rust-lang/rust/pull/108033
#![allow(internal_features)] // this is research-quality software!
#![feature(rustc_attrs)]

// want: foreign crates can implement covisit for their concrete type and for a specific covisitor (no generics required), and
// foreign crates can implement a specific covisitor for all ADTs

#[rustc_coinductive]
pub trait Covisit<T> {
    fn covisit(&mut self) -> T;
}

pub trait Adt: Sized {
    type Strategy: Into<Self>;
}

impl<Covisitor, T> Covisit<T> for Covisitor
where
    T: Adt,
    // T::Strategy: Covisit<Covisitor, T>,
    Covisitor: Covisit<T::Strategy>,
    // T::Strategy: Adt,
{
    fn covisit(&mut self) -> T {
        <Self as Covisit<T::Strategy>>::covisit(self).into()
    }
}
#[derive(Debug)]
struct SomeAdt(SomeTmf<Self>);

#[derive(Debug)]
enum SomeTmf<X> {
    Adt(Box<X>),
    None,
}

impl Adt for SomeAdt {
    type Strategy = SomeTmf<Self>;
}

impl From<SomeTmf<Self>> for SomeAdt {
    fn from(it: SomeTmf<Self>) -> Self {
        SomeAdt(it)
    }
}

struct SomeCovisit {
    fuel: usize,
}

impl<SomeAdt2> Covisit<SomeTmf<SomeAdt2>> for SomeCovisit
where
    // SomeAdt2: Adt<Strategy = Self>,
    // SomeCovisit: Covisit<SomeAdt2::Strategy>,
    SomeCovisit: Covisit<SomeAdt2>,
{
    fn covisit(&mut self) -> SomeTmf<SomeAdt2> {
        self.fuel -= 1;
        if self.fuel == 0 {
            return SomeTmf::None;
        }
        SomeTmf::Adt(Box::new(<SomeCovisit as Covisit<SomeAdt2>>::covisit(self)))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let result = <SomeCovisit as Covisit<SomeAdt>>::covisit(&mut SomeCovisit { fuel: 10 });
        dbg!(&result);
    }
}
