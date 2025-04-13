use miette::SourceSpan;
use tymetafuncspec_core::{Either, Maybe, Pair};

pub type Cstfy<Heap, T> =
    CstfyTransparent<Heap, Pair<Heap, T, Maybe<Heap, std_parse_metadata::ParseMetadata<Heap>>>>;
pub type CstfyTransparent<Heap, T> = Either<Heap, T, std_parse_error::ParseError<Heap>>;
pub fn cstfy_ok<Heap, T>(
    t: T,
    starting_offset: parse::miette::SourceOffset,
    ending_offset: parse::miette::SourceOffset,
) -> Cstfy<Heap, T> {
    Either::Left(
        Pair::new(
            t,
            Maybe::Just(
                std_parse_metadata::ParseMetadata::new(parse::ParseMetadata {
                    location: SourceSpan::new(
                        starting_offset,
                        ending_offset.offset() - starting_offset.offset(),
                    ),
                }),
                std::marker::PhantomData,
            ),
        ),
        std::marker::PhantomData,
    )
}
pub fn cstfy_transparent_ok<Heap, T>(t: T) -> CstfyTransparent<Heap, T> {
    Either::Left(t, std::marker::PhantomData)
}
pub fn uncstfy<Heap, T>(c: &Cstfy<Heap, T>) -> Option<&T> {
    match c {
        Either::Left(p, _) => Some(&p.l),
        Either::Right(_, _) => None,
    }
}
pub fn uncstfy_transparent<Heap, T>(c: &CstfyTransparent<Heap, T>) -> Option<&T> {
    match c {
        Either::Left(t, _) => Some(t),
        Either::Right(_, _) => None,
    }
}
#[macro_export]
macro_rules! return_if_err {
    ($result:ident) => {
        let $result = match $result {
            Ok(ok) => ok,
            Err(e) => {
                return tymetafuncspec_core::Either::Right(
                    std_parse_error::ParseError::new(e),
                    std::marker::PhantomData,
                )
            }
        };
    };
}
