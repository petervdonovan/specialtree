pub struct fib(pub Nat, pub Nat);
pub enum Nat {
    NatLit(usize),
    fib(Box<fib>),
}
