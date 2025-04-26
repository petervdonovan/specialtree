#![feature(fundamental)]
pub use type_equals;

pub mod case_split;
pub mod co_case_split;
pub mod co_visit;
pub mod drop;
pub mod fnlut;
pub mod select;
pub mod visit;

pub trait CanonicallyConstructibleFrom<Heap, T>: Sized {
    fn construct(heap: &mut Heap, t: T) -> Self;
    fn deconstruct_succeeds(&self, heap: &Heap) -> bool;
    fn deconstruct(self, heap: &Heap) -> T;
}
pub trait DirectlyCanonicallyConstructibleFrom<Heap, T>: Sized {
    fn construct(heap: &mut Heap, t: T) -> Self;
    fn deconstruct_succeeds(&self, heap: &Heap) -> bool;
    fn deconstruct(self, heap: &Heap) -> T;
}
#[macro_export]
macro_rules! auto_impl_ccf {
    ($heapty:ty, $ty:ty) => {
        impl term::CanonicallyConstructibleFrom<$heapty, ($ty, ())> for $ty {
            fn construct(heap: &mut $heapty, t: ($ty, ())) -> Self {
                t.0
            }

            fn deconstruct_succeeds(&self, heap: &$heapty) -> bool {
                true
            }

            fn deconstruct(self, heap: &$heapty) -> ($ty, ()) {
                (self, ())
            }
        }
    };
}

pub trait TransitivelyUnitCcf<Heap, T> {
    type Intermediary;
}

pub trait TransitivelyAllCcf<Heap, T> {
    type Intermediary;
    type Intermediaries;
}

impl<Heap, T, U> CanonicallyConstructibleFrom<Heap, (T, ())> for U
where
    U: Copy,
    U: TransitivelyUnitCcf<Heap, T> + Heaped<Heap = Heap>,
    U: DirectlyCanonicallyConstructibleFrom<Heap, (U::Intermediary, ())>,
    U::Intermediary: CanonicallyConstructibleFrom<Heap, (T, ())>,
{
    fn construct(heap: &mut Heap, t: (T, ())) -> Self {
        let intermediary = U::Intermediary::construct(heap, t);
        Self::construct(heap, (intermediary, ()))
    }

    fn deconstruct_succeeds(&self, heap: &Heap) -> bool {
        self.deconstruct_succeeds(heap) && self.deconstruct(heap).0.deconstruct_succeeds(heap)
    }

    fn deconstruct(self, heap: &Heap) -> (T, ()) {
        self.deconstruct(heap).0.deconstruct(heap)
    }
}

pub trait AllCcf<Heap, FromConsList> {
    fn construct(heap: &mut Heap, t: FromConsList) -> Self;
    fn deconstruct_succeeds(&self, heap: &Heap) -> bool;
    fn deconstruct(self, heap: &Heap) -> FromConsList;
}
impl<Heap> AllCcf<Heap, ()> for () {
    fn construct(_heap: &mut Heap, _t: ()) -> Self {}

    fn deconstruct_succeeds(&self, _heap: &Heap) -> bool {
        true
    }

    fn deconstruct(self, _heap: &Heap) {}
}
impl<Heap, TCar, TCdr, UCar, UCdr> AllCcf<Heap, (TCar, TCdr)> for (UCar, UCdr)
where
    UCar: CanonicallyConstructibleFrom<Heap, (TCar, ())>,
    UCdr: AllCcf<Heap, TCdr>,
{
    fn construct(heap: &mut Heap, t: (TCar, TCdr)) -> Self {
        let (car, cdr) = t;
        let ucar = UCar::construct(heap, (car, ()));
        let ucdr = UCdr::construct(heap, cdr);
        (ucar, ucdr)
    }

    fn deconstruct_succeeds(&self, heap: &Heap) -> bool {
        self.0.deconstruct_succeeds(heap) && self.1.deconstruct_succeeds(heap)
    }

    fn deconstruct(self, heap: &Heap) -> (TCar, TCdr) {
        let (car, cdr) = self;
        let tcar = car.deconstruct(heap);
        let tcdr = cdr.deconstruct(heap);
        (tcar.0, tcdr)
    }
}

impl<Heap, TCar, TCdrCar, TCdrCdr, U> CanonicallyConstructibleFrom<Heap, (TCar, (TCdrCar, TCdrCdr))>
    for U
where
    U: Copy,
    U: TransitivelyAllCcf<Heap, (TCar, (TCdrCar, TCdrCdr))>,
    U::Intermediaries: AllCcf<Heap, (TCar, (TCdrCar, TCdrCdr))>,
    U: CanonicallyConstructibleFrom<Heap, (U::Intermediary, ())>,
    U::Intermediary: DirectlyCanonicallyConstructibleFrom<Heap, U::Intermediaries>,
{
    fn construct(heap: &mut Heap, t: (TCar, (TCdrCar, TCdrCdr))) -> Self {
        let (car, cdr) = t;
        let intermediaries = U::Intermediaries::construct(heap, (car, cdr));
        let intermediary = U::Intermediary::construct(heap, intermediaries);
        Self::construct(heap, (intermediary, ()))
    }

    fn deconstruct_succeeds(&self, heap: &Heap) -> bool {
        self.deconstruct_succeeds(heap) && self.deconstruct(heap).0.deconstruct_succeeds(heap)
    }

    fn deconstruct(self, heap: &Heap) -> (TCar, (TCdrCar, TCdrCdr)) {
        let intermediary = self.deconstruct(heap);
        let intermediaries = U::Intermediary::deconstruct(intermediary.0, heap);
        let (car, cdr) = intermediaries.deconstruct(heap);
        (car, cdr)
    }
}

#[repr(transparent)]
pub struct Owned<T>(std::mem::ManuallyDrop<T>);
impl<T> Heaped for Owned<T>
where
    T: Heaped,
{
    type Heap = T::Heap;
}
impl<Heap, TOwned: AllOwned, U: CanonicallyConstructibleFrom<Heap, TOwned::Bak>>
    CanonicallyConstructibleFrom<Heap, (TOwned, ())> for Owned<U>
{
    fn construct(heap: &mut Heap, t: (TOwned, ())) -> Self {
        // safe because bak and construct are inverse to each other wrt freeing
        unsafe {
            let bak = t.0.bak();
            let u = U::construct(heap, bak);
            Owned(std::mem::ManuallyDrop::new(u))
        }
    }

    fn deconstruct_succeeds(&self, heap: &Heap) -> bool {
        self.0.deconstruct_succeeds(heap)
    }

    fn deconstruct(self, heap: &Heap) -> (TOwned, ()) {
        // safe because .0 and new are inverse to each other wrt freeing
        unsafe {
            let bak = std::mem::ManuallyDrop::<U>::into_inner(self.0).deconstruct(heap);
            (TOwned::new(bak), ())
        }
    }
}

pub trait Heaped {
    type Heap;
}
pub trait MapsTmf<LWord, TmfMonomorphization>: Sized {
    type Tmf: CanonicallyConstructibleFrom<Self, (TmfMonomorphization, ())>;
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
#[derive(PartialEq, Eq, Hash, Clone, Debug, PartialOrd, Ord)]
pub struct CcfRelation<SortId> {
    pub from: Vec<SortId>,
    pub to: SortId,
}
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
// pub struct UsingIntermediary<T, Intermediary> {
//     t: T,
//     _intermediary: std::marker::PhantomData<Intermediary>,
// }
// impl<T, I> Heaped for UsingIntermediary<T, I>
// where
//     T: Heaped,
// {
//     type Heap = T::Heap;
// }
// impl<T, Intermediary> UsingIntermediary<T, Intermediary> {
//     pub fn new(t: T) -> Self {
//         UsingIntermediary {
//             t,
//             _intermediary: std::marker::PhantomData,
//         }
//     }
// }
// impl<'heap, T: Heaped, Intermediary: Heaped, U: Heaped, Fallibility>
//     CanonicallyMaybeConvertibleTo<'heap, U, Fallibility> for UsingIntermediary<T, Intermediary>
// where
//     T: CanonicallyMaybeConvertibleTo<'heap, Intermediary, Fallibility>,
//     Intermediary: CanonicallyMaybeConvertibleTo<'heap, U, Fallibility>,
// {
//     fn maybe_convert(self, heap: &'heap Self::Heap) -> Result<U, Fallibility> {
//         self.t
//             .maybe_convert(heap)
//             .and_then(|intermediary| intermediary.maybe_convert(heap))
//     }
// }
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
