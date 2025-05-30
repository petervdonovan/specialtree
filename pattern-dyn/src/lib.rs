use langspec::langspec::{LangSpec, SortIdOf};
use pmsp::AdtMetadata;
use thiserror::Error;
use visit::Visit;
use visitor::{PatternBuilder, PbResultError};

pub mod tmfscore;
pub mod visitor;
// #[derive(Debug)]
// pub struct NamedDynPattern<SortId> {
//     pub snake_ident: String,
//     pub pattern: DynPattern<SortId>,
// }
#[derive(Debug)]
pub enum DynPattern<SortId> {
    Composite(CompositePattern<SortId>),
    Ignored(SortId),
    ZeroOrMore(Variable<SortId>),
    Variable(Variable<SortId>),
    Literal(LiteralPattern<SortId>),
    Named(String, Box<DynPattern<SortId>>),
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

// pub fn to_pattern<L, Heap, Ls, T>(
//     heap: &Heap,
//     ls: &Ls,
//     t: &T,
// ) -> Result<DynPattern<SortIdOf<Ls>>, PbResultError>
// where
//     PatternBuilder<L, SortIdOf<Ls>>: Visit<AdtMetadata, T, Heap, L>,
//     Ls: LangSpec,
// {
//     let sids = ls.all_sort_ids().collect::<Vec<_>>();
//     let mut pb = PatternBuilder::new(sids.clone());
//     <PatternBuilder<L, SortIdOf<Ls>> as Visit<_, _, _, _>>::visit(&mut pb, heap, t);
//     pb.result()
// }

pub fn to_pattern<L, LSub, Heap, T>(heap: &Heap, t: &T) -> Result<DynPattern<u32>, PbResultError>
where
    PatternBuilder<L, LSub, u32>: Visit<AdtMetadata, T, Heap, L>,
    // L: LangSpec,
{
    let mut pb = PatternBuilder::new((0..100).collect());
    <PatternBuilder<L, LSub, u32> as Visit<_, _, _, _>>::visit(&mut pb, heap, t);
    pb.result()
}
