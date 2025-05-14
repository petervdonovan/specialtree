use pmsp::AdtMetadata;
use thiserror::Error;
use visit::Visit;
use visitor::{PatternBuilder, PbResultError};

pub mod tmfscore;
pub mod visitor;
#[derive(Debug)]
pub struct NamedDynPattern<SortId> {
    pub snake_ident: String,
    pub pattern: DynPattern<SortId>,
}
#[derive(Debug)]
pub enum DynPattern<SortId> {
    Composite(CompositePattern<SortId>),
    Ignored(SortId),
    ZeroOrMore(Box<DynPattern<SortId>>),
    Variable(Variable<SortId>),
    Literal(LiteralPattern<SortId>),
}
#[derive(Debug)]
pub struct Variable<SortId> {
    pub sid: SortId,
    pub ident: String,
}
#[derive(Debug)]
pub struct CompositePattern<SortId> {
    pub rs_ty: SortId,
    pub components: Vec<DynPattern<SortId>>,
}
#[derive(Debug)]
pub struct LiteralPattern<SortId> {
    pub sid: SortId,
    pub equal_to: syn::Expr,
}
// impl<SortId> std::fmt::Debug for LiteralPattern<SortId>
// where
//     SortId: std::fmt::Debug,
// {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         // let et = format!("{:?}", &self.equal_to);
//         Box::new(self.equal_to.clone()).fmt(f);
//         f.debug_struct("LiteralPattern")
//             .field("sid", &self.sid)
//             .field("equal_to", &*et)
//             .finish()
//     }
// }

#[derive(Error, Debug)]
pub enum ToDynPatternError {
    #[error("invalid sequence of pattern match components")]
    InvalidSequenceOfComponents(),
}

pub fn to_pattern<L, Heap, T>(heap: &Heap, t: &T) -> Result<DynPattern<u32>, PbResultError>
where
    PatternBuilder<L, u32>: Visit<AdtMetadata, T, Heap, L>,
    // L: LangSpec,
{
    let mut pb = PatternBuilder::new((0..100).collect());
    <PatternBuilder<L, u32> as Visit<_, _, _, _>>::visit(&mut pb, heap, t);
    pb.result()
}

// pub trait ToDynPattern<L, SortId> {
//     fn pattern(&self) -> Result<DynPattern<SortId>, ToDynPatternError>;
// }

// impl ToDyn
