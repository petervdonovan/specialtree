use thiserror::Error;

pub mod tmfscore;
pub mod visitor;

pub struct NamedDynPattern<SortId> {
    pub snake_ident: String,
    pub pattern: DynPattern<SortId>,
}

pub enum DynPattern<SortId> {
    Composite(CompositePattern<SortId>),
    Ignored(SortId),
    ZeroOrMore(Box<DynPattern<SortId>>),
    Variable(Variable<SortId>),
    Literal(LiteralPattern<SortId>),
}

pub struct Variable<SortId> {
    pub sid: SortId,
    pub ident: String,
}

pub struct CompositePattern<SortId> {
    pub rs_ty: SortId,
    pub components: Vec<DynPattern<SortId>>,
}

pub struct LiteralPattern<SortId> {
    pub sid: SortId,
    pub equal_to: syn::Expr,
}

#[derive(Error, Debug)]
pub enum ToDynPatternError {
    #[error("invalid sequence of pattern match components")]
    InvalidSequenceOfComponents(),
}

// pub trait ToDynPattern<L, SortId> {
//     fn pattern(&self) -> Result<DynPattern<SortId>, ToDynPatternError>;
// }

// impl ToDyn
