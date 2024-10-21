pub struct F(pub Nat, pub Nat);
pub enum Nat {
    NatLit(usize),
    F(Box<F>),
}
