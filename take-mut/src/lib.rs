pub trait Poisonable {
    fn poisoned() -> Self;
}

pub fn take<T: Poisonable, Ret>(x: &mut T, f: impl FnOnce(T) -> (T, Ret)) -> Ret {
    let contents = std::mem::replace(x, T::poisoned());
    let (xx, ret) = f(contents);
    *x = xx;
    ret
}
