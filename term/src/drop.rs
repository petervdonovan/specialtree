use crate::{
    Heaped, Owned, SuperHeap,
    case_split::{AdmitNoMatchingCase, Adt, Callable, HasBorrowedHeapRef, PatternMatchable},
};

pub trait UnsafeHeapDrop<Heap> {
    /// # Safety
    /// Self cannot be shared; if it is, there is a potential use-after-free.
    unsafe fn unsafe_heap_drop(self, heap: &mut Heap);
}
impl<Heap, T> UnsafeHeapDrop<Heap> for Owned<T>
where
    T: Heaped + UnsafeHeapDrop<T::Heap>,
    Heap: SuperHeap<T::Heap>,
{
    unsafe fn unsafe_heap_drop(self, heap: &mut Heap) {
        unsafe {
            std::mem::ManuallyDrop::<T>::into_inner(self.0)
                .unsafe_heap_drop(heap.subheap_mut::<T::Heap>());
        }
    }
}
pub struct OwnedWithHeapMutRef<'heap, T: Heaped + UnsafeHeapDrop<T::Heap>> {
    pub heap: &'heap mut T::Heap,
    pub t: Option<Owned<T>>,
}
impl<'heap, T: Heaped + UnsafeHeapDrop<T::Heap>> OwnedWithHeapMutRef<'heap, T> {
    pub fn new(heap: &'heap mut T::Heap, t: Owned<T>) -> Self {
        OwnedWithHeapMutRef { heap, t: Some(t) }
    }
}
impl<T: Heaped + UnsafeHeapDrop<T::Heap>> Drop for OwnedWithHeapMutRef<'_, T> {
    fn drop(&mut self) {
        unsafe {
            self.t.take().unwrap().unsafe_heap_drop(self.heap);
        }
    }
}

pub struct Dropper<Heap>(std::marker::PhantomData<Heap>);
impl<Heap> Dropper<Heap> {
    /// # Safety
    /// This is only safe if the calling code that uses this dropper as a callable
    /// is guaranteed to not drop any types that are not safe to drop.
    pub unsafe fn assert_dropped_types_shall_be_safe_to_drop() -> Self {
        Dropper(std::marker::PhantomData)
    }
}
impl<Heap> Heaped for Dropper<Heap> {
    type Heap = Heap;
}
impl<H> HasBorrowedHeapRef for Dropper<H> {
    type Borrowed<'a, Heap: 'a> = &'a mut Heap;

    fn with_decayed_heapref<Heap, T, F: Fn(&Heap) -> T>(
        heap: &mut Self::Borrowed<'_, Heap>,
        f: F,
    ) -> T {
        f(heap)
    }
}
impl<Heap, Case: UnsafeHeapDrop<Heap>> Callable<Case> for Dropper<Heap> {
    fn call(&mut self, t: Case, heap: &mut Self::Borrowed<'_, Heap>) {
        // safe if the safety guarantees described for construction of the Dropper are satisfied
        unsafe {
            t.unsafe_heap_drop(heap);
        }
    }
}
impl<Heap, T> AdmitNoMatchingCase<T> for Dropper<Heap> {
    fn no_matching_case(&self, _t: &T, _heap: &mut Self::Borrowed<'_, Heap>) {
        panic!("no matching case");
    }
}
impl<Heap, Car, Cdr> UnsafeHeapDrop<Heap> for (Car, Cdr)
where
    Car: UnsafeHeapDrop<Heap>,
    Cdr: UnsafeHeapDrop<Heap>,
{
    unsafe fn unsafe_heap_drop(self, heap: &mut Heap) {
        unsafe {
            self.0.unsafe_heap_drop(heap);
            self.1.unsafe_heap_drop(heap);
        }
    }
}
impl<
    Heap: SuperHeap<T::Heap>,
    T: Heaped
        + PatternMatchable<Dropper<<T as Heaped>::Heap>, <T as Adt>::PatternMatchStrategyProvider>
        + Adt,
> UnsafeHeapDrop<Heap> for T
{
    unsafe fn unsafe_heap_drop(self, heap: &mut Heap) {
        self.do_match(
            // propagate responsibility for safety upward
            &mut unsafe { Dropper::assert_dropped_types_shall_be_safe_to_drop() },
            &mut heap.subheap_mut::<T::Heap>(),
        );
    }
}
