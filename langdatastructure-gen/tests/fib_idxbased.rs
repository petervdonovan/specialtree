pub struct fiblang {
    pub fib: Vec<fib>,
    pub Nat: Vec<Nat>,
}
pub struct fib(pub usize, pub usize);
pub enum Nat {
    NatLit(usize),
    fib(Box<usize>),
}
