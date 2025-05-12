pub mod traits;
pub mod types;

pub trait Predicate<T, Heap> {
    fn eval(heap: &Heap, t: &T) -> bool;
}
