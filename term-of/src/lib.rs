use langspec::langspec::LangSpec;

pub enum Term<'a, L> {
    NatLiteral(u64),
    Algebraic(AlgebraicTerm<'a, L>),
    Set(Box<dyn Iterator<Item = AlgebraicTerm<'a, L>> + 'a>),
    Sequence(Box<dyn Iterator<Item = AlgebraicTerm<'a, L>> + 'a>),
}
pub enum AlgebraicTerm<'a, L> {
    Product(Box<dyn Product<'a, L> + 'a>),
    Sum(Box<dyn Sum<'a, L> + 'a>),
}

pub trait Product<'a, L: LangSpec> {
    fn ty_id(&self) -> L::ProductId;
    fn fields(&self) -> Box<dyn Iterator<Item = Term<'a, L>> + 'a>;
}
pub trait Sum<'a, L: LangSpec> {
    fn ty_id(&self) -> L::SumId;
    fn get(&self) -> Option<Term<'a, L>>;
}
