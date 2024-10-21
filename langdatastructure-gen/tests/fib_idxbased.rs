pub struct Fib {
    pub f: Vec<F>,
    pub nat: Vec<Nat>,
}
pub struct F(pub usize, pub usize);
pub enum Nat {
    NatLit(usize),
    F(Box<usize>),
}
