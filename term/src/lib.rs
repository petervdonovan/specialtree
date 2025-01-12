#![feature(never_type)]

pub trait CanonicallyConstructibleFrom<T>: Sized
where
    Self: Heaped,
{
    fn construct(heap: &mut Self::Heap, t: T) -> Self;
    fn deconstruct(self) -> T;
    fn reconstruct<F: Fn(T) -> T>(&mut self, heap: &mut Self::Heap, f: F) {
        take_mut::take(self, |s| Self::construct(heap, f(Self::deconstruct(s))))
    }
}
pub trait Heaped {
    type Heap;
}
pub trait CanonicallyMaybeConvertibleTo<'heap, T: Heaped, Fallibility>
where
    Self: Heaped<Heap = T::Heap>,
{
    fn maybe_convert(self, heap: &'heap Self::Heap) -> Result<T, Fallibility>;
}
#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct TyFingerprint(u128);
impl<'a> From<&'a str> for TyFingerprint {
    fn from(s: &'a str) -> Self {
        TyFingerprint(twox_hash::XxHash3_128::oneshot(s.as_bytes()))
    }
}
#[derive(PartialEq, Eq, Hash, Clone)]
pub struct CcfRelation {
    pub from: Vec<TyFingerprint>,
    pub to: TyFingerprint,
}
#[derive(PartialEq, Eq, Hash, Clone)]
pub struct MctRelation {
    pub from: TyFingerprint,
    pub to: TyFingerprint,
}
// pub trait SettableTo<'heap, T>: Heaped {
//     fn set_to(&mut self, heap: &'heap mut Self::Heap, t: T);
// }
pub trait MutableCollection<'a, 'heap: 'a>:
    Heaped
    + IntoIterator<Item = <Self as MutableCollection<'a, 'heap>>::Item>
    + FromIterator<<Self as MutableCollection<'a, 'heap>>::Item>
{
    type Item;
    fn insert(&mut self, item: <Self as MutableCollection<'a, 'heap>>::Item);
    // type Cursor: Cursor<'a, 'heap, C = Self>;
    // fn start(self, heap: &'heap Self::Heap) -> Self::Cursor;
}
// pub trait Cursor<'a, 'heap: 'a> {
//     type C: MutableCollection<'a, 'heap>;
//     fn following<'b>(
//         &'b self,
//     ) -> impl ExactSizeIterator<Item = &'b <Self::C as MutableCollection<'a, 'heap>>::Item>
//     where
//         'a: 'b;
//     fn replace_n_with(
//         &mut self,
//         n: usize,
//         items: impl ExactSizeIterator<Item = <Self::C as MutableCollection<'a, 'heap>>::Item>,
//     ) -> impl ExactSizeIterator<Item = <Self::C as MutableCollection<'a, 'heap>>::Item>;
//     fn advance(&mut self);
//     fn finish(self) -> Self::C;
// }
// pub trait MutableCollectionTyFunc {
//     type MC<'a, 'heap: 'a, Item>: MutableCollection<'a, 'heap, Item = Item>;
// }
// pub struct MutableSet<'a, 'heap: 'a, T: Heaped, Bak: MutableCollectionTyFunc>(
//     Bak::MC<'a, 'heap, T>,
// );
// impl<'a, 'heap: 'a, T: Heaped, Bak: MutableCollectionTyFunc> Heaped
//     for MutableSet<'a, 'heap, T, Bak>
// {
//     type Heap = T::Heap;
// }
// impl<'a, 'heap: 'a, Bak: MutableCollectionTyFunc, T: Heaped, U: Heaped, Fallibility>
//     CanonicallyMaybeConvertibleTo<'heap, MutableSet<'a, 'heap, U, Bak>, Fallibility>
//     for MutableSet<'a, 'heap, T, Bak>
// where
//     T: CanonicallyMaybeConvertibleTo<'heap, U, Fallibility>,
// {
//     fn maybe_convert(
//         self,
//         heap: &'heap Self::Heap,
//     ) -> Result<MutableSet<'a, 'heap, U, Bak>, Fallibility> {
//         let mut ret = <Bak::MC<'a, 'heap, U> as FromIterator<U>>::from_iter(std::iter::empty());
//         for t in self.0 {
//             ret.insert(t.maybe_convert(heap)?);
//         }
//         Ok(MutableSet(ret))
//     }
// }
// impl<'a, 'heap: 'a, Bak: MutableCollectionTyFunc, T: Heaped, U: Heaped>
//     CanonicallyConstructibleFrom<MutableSet<'a, 'heap, U, Bak>> for MutableSet<'a, 'heap, T, Bak>
// where
//     T: CanonicallyConstructibleFrom<(U,)>,
// {
//     fn construct(heap: &mut Self::Heap, t: MutableSet<'a, 'heap, U, Bak>) -> Self {
//         let mut ret = <Bak::MC<'a, 'heap, T> as FromIterator<T>>::from_iter(std::iter::empty());
//         for u in t.0 {
//             ret.insert(T::construct(heap, (u,)));
//         }
//         MutableSet(ret)
//     }
// }
pub struct UsingIntermediary<T, Intermediary> {
    t: T,
    _intermediary: std::marker::PhantomData<Intermediary>,
}
impl<T, I> Heaped for UsingIntermediary<T, I>
where
    T: Heaped,
{
    type Heap = T::Heap;
}
impl<T, Intermediary> UsingIntermediary<T, Intermediary> {
    pub fn new(t: T) -> Self {
        UsingIntermediary {
            t,
            _intermediary: std::marker::PhantomData,
        }
    }
}
impl<'heap, T: Heaped, Intermediary: Heaped, U: Heaped, Fallibility>
    CanonicallyMaybeConvertibleTo<'heap, U, Fallibility> for UsingIntermediary<T, Intermediary>
where
    T: CanonicallyMaybeConvertibleTo<'heap, Intermediary, Fallibility>,
    Intermediary: CanonicallyMaybeConvertibleTo<'heap, U, Fallibility>,
{
    fn maybe_convert(self, heap: &'heap Self::Heap) -> Result<U, Fallibility> {
        self.t
            .maybe_convert(heap)
            .and_then(|intermediary| intermediary.maybe_convert(heap))
    }
}
pub type IsoClassExpansionMaybeConversionFallibility = !; // cannot fail
pub type ExpansionMaybeConversionFallibility = (); // failure is inhabited
