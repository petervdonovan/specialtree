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
    // Predicate(PredicatePattern<SortId>),
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

// pub struct PredicatePattern<SortId> {
//     pub sid: SortId,
//     pub f: syn::Expr,
//     pub subtree: Box<Pattern<SortId>>,
// }

pub mod dynamic {

    use crate::DynPattern;

    use thiserror::Error;

    #[derive(Error, Debug)]
    pub enum ToDynPatternError {
        #[error("invalid sequence of pattern match components")]
        InvalidSequenceOfComponents(),
    }

    pub trait ToDynPattern<L, SortId> {
        fn pattern(&self) -> Result<DynPattern<SortId>, ToDynPatternError>;
    }
}
