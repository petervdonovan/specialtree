#![feature(fundamental)]
use ccf::CanonicallyConstructibleFrom;
pub use type_equals;

pub mod drop;
pub mod visit;

#[macro_export]
macro_rules! auto_impl_ccf {
    ($heapty:ty, $ty:ty) => {
        impl ccf::CanonicallyConstructibleFrom<$heapty, ($ty, ())> for $ty {
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

// #[repr(transparent)]
// pub struct Owned<T>(std::mem::ManuallyDrop<T>);
// // impl<T> Heaped for Owned<T>
// // where
// //     T: Heaped,
// // {
// //     type Heap = T::Heap;
// // }
// impl<Heap, TOwned: AllOwned, U: CanonicallyConstructibleFrom<Heap, TOwned::Bak>>
//     CanonicallyConstructibleFrom<Heap, (TOwned, ())> for Owned<U>
// {
//     fn construct(heap: &mut Heap, t: (TOwned, ())) -> Self {
//         // safe because bak and construct are inverse to each other wrt freeing
//         unsafe {
//             let bak = t.0.bak();
//             let u = U::construct(heap, bak);
//             Owned(std::mem::ManuallyDrop::new(u))
//         }
//     }

//     fn deconstruct_succeeds(&self, heap: &Heap) -> bool {
//         self.0.deconstruct_succeeds(heap)
//     }

//     fn deconstruct(self, heap: &Heap) -> (TOwned, ()) {
//         // safe because .0 and new are inverse to each other wrt freeing
//         unsafe {
//             let bak = std::mem::ManuallyDrop::<U>::into_inner(self.0).deconstruct(heap);
//             (TOwned::new(bak), ())
//         }
//     }
// }

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
#[derive(PartialEq, Eq, Hash, Clone, Debug, PartialOrd, Ord)]
pub struct CcfRelation<SortId> {
    pub from: Vec<SortId>,
    pub to: SortId,
}
