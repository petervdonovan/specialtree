pub struct Plus(pub Nat, pub Nat);
pub struct F(pub Nat);
pub enum Nat {
    NatLit(usize),
    F(Box<F>),
    Plus(Box<Plus>),
}
