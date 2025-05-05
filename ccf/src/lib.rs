pub mod transitivity;

pub trait CanonicallyConstructibleFrom<Heap, T>: Sized {
    fn construct(heap: &mut Heap, t: T) -> Self;
    fn deconstruct_succeeds(&self, heap: &Heap) -> bool;
    fn deconstruct(self, heap: &Heap) -> T;
}
