// #![feature(never_type)]

pub use type_equals;

pub mod case_split;
pub mod co_case_split;
pub mod co_visit;
pub mod drop;
pub mod select;
pub mod visit;

pub trait CanonicallyConstructibleFrom<T>: Sized
where
    Self: Heaped,
{
    fn construct(heap: &mut Self::Heap, t: T) -> Self;
    fn deconstruct_succeeds(&self, heap: &Self::Heap) -> bool;
    fn deconstruct(self, heap: &Self::Heap) -> T;
}
#[repr(transparent)]
pub struct Owned<T>(std::mem::ManuallyDrop<T>);
impl<T> Heaped for Owned<T>
where
    T: Heaped,
{
    type Heap = T::Heap;
}
impl<TOwned: AllOwned, U: CanonicallyConstructibleFrom<TOwned::Bak>>
    CanonicallyConstructibleFrom<TOwned> for Owned<U>
{
    fn construct(heap: &mut Self::Heap, t: TOwned) -> Self {
        // safe because bak and construct are inverse to each other wrt freeing
        unsafe {
            let bak = t.bak();
            let u = U::construct(heap, bak);
            Owned(std::mem::ManuallyDrop::new(u))
        }
    }

    fn deconstruct_succeeds(&self, heap: &Self::Heap) -> bool {
        self.0.deconstruct_succeeds(heap)
    }

    fn deconstruct(self, heap: &Self::Heap) -> TOwned {
        // safe because .0 and new are inverse to each other wrt freeing
        unsafe {
            let bak = std::mem::ManuallyDrop::<U>::into_inner(self.0).deconstruct(heap);
            TOwned::new(bak)
        }
    }
}

pub trait Heaped {
    type Heap;
}
pub trait SuperHeap<SubHeap> {
    fn subheap<T>(&self) -> &SubHeap
    where
        T: type_equals::TypeEquals<Other = SubHeap>;
    fn subheap_mut<T>(&mut self) -> &mut SubHeap
    where
        T: type_equals::TypeEquals<Other = SubHeap>;
}
impl<T> SuperHeap<T> for T {
    fn subheap<U>(&self) -> &T
    where
        U: type_equals::TypeEquals<Other = T>,
    {
        self
    }
    fn subheap_mut<U>(&mut self) -> &mut T
    where
        U: type_equals::TypeEquals<Other = T>,
    {
        self
    }
}
#[macro_export]
macro_rules! impl_superheap {
    ($heapty:ty ; $subheapty:ty ; $($field_names:ident)*) => {
        impl term::SuperHeap<$subheapty> for $heapty {
            fn subheap<T>(&self) -> &$subheapty
            where
                T: term::type_equals::TypeEquals<Other = $subheapty>,
            {
                &self $( . $field_names)*.0
            }
            fn subheap_mut<T>(&mut self) -> &mut $subheapty
            where
                T: term::type_equals::TypeEquals<Other = $subheapty>,
            {
                &mut self $( . $field_names)*.0
            }
        }
    };
}
pub trait CanonicallyMaybeConvertibleTo<'heap, T: Heaped, Fallibility>
where
    Self: Heaped<Heap = T::Heap>,
{
    fn maybe_convert(self, heap: &'heap Self::Heap) -> Result<T, Fallibility>;
}
#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct TyFingerprint(u128);
impl TyFingerprint {
    pub fn combine(&self, other: &Self) -> Self {
        let mut hasher = twox_hash::XxHash3_128::new();
        hasher.write(&self.0.to_ne_bytes());
        hasher.write(&other.0.to_ne_bytes());
        TyFingerprint(hasher.finish_128())
    }
}
impl<'a> From<&'a str> for TyFingerprint {
    fn from(s: &'a str) -> Self {
        TyFingerprint(twox_hash::XxHash3_128::oneshot(s.as_bytes()))
    }
}
impl From<u128> for TyFingerprint {
    fn from(u: u128) -> Self {
        TyFingerprint(u)
    }
}
impl TyFingerprint {
    pub fn lit_int(&self) -> syn::LitInt {
        syn::LitInt::new(&format!("0x{:x}", &self.0), proc_macro2::Span::call_site())
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
// pub type IsoClassExpansionMaybeConversionFallibility = !; // cannot fail
// pub type ExpansionMaybeConversionFallibility = (); // failure is inhabited

pub trait AllOwned {
    type Bak;
    /// # Safety
    /// Leaks if result is not converted back to Owned and then dropped
    unsafe fn bak(self) -> Self::Bak;
    /// # Safety
    /// Use-after-free if `bak` involves a shared reference
    unsafe fn new(bak: Self::Bak) -> Self;
}
macro_rules! impl_to_refs {
    ($arg:ident $(,)? $($args:ident),*) => {
        #[allow(non_snake_case)]
        impl<T, $($args),*> AllOwned for (Owned<T>, $($args),*) {
            type Bak = (T, $($args),*);
            unsafe fn bak(self) -> Self::Bak {
                let (Owned(t), $($args),*) = self;
                (std::mem::ManuallyDrop::<T>::into_inner(t), $($args),*)
            }
            unsafe fn new(bak: Self::Bak) -> Self {
                let (t, $($args),*) = bak;
                (Owned(std::mem::ManuallyDrop::new(t)), $($args),*)
            }
        }
        impl_to_refs!($($args),*);
    };
    () => {};
}
impl_to_refs!(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11);
// impl<T, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> ToRefs
//     for (T, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11)
// {
//     type Refs<'a>
//         = (
//         &'a T,
//         &'a T1,
//         &'a T2,
//         &'a T3,
//         &'a T4,
//         &'a T5,
//         &'a T6,
//         &'a T7,
//         &'a T8,
//         &'a T9,
//         &'a T10,
//         &'a T11,
//     )
//     where
//         Self: 'a;
// }
