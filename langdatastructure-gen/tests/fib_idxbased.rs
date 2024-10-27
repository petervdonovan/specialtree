pub struct Fib {
    pub plus: Vec<Plus>,
    pub f: Vec<F>,
    pub nat: Vec<Nat>,
}
pub struct Plus(pub usize, pub usize);
pub struct F(pub usize);
pub enum Nat {
    NatLit(usize),
    F(Box<usize>),
    Plus(Box<usize>),
}
