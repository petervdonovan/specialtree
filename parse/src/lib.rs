use miette::{Diagnostic, SourceSpan};
use serde::{Deserialize, Serialize};
use thiserror::Error;

pub use miette;
pub use unicode_segmentation;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Keyword(&'static str);

impl std::fmt::Display for Keyword {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "`{}`", self.0)
    }
}

impl Keyword {
    pub const fn new(s: &'static str) -> Self {
        // validate on read instead of on construction so that constructor is const
        Keyword(s)
    }
    pub fn get(&self) -> &'static str {
        if self.0.is_empty() {
            panic!("Keyword cannot be empty");
        }
        if self.0.chars().any(|it| it.is_whitespace()) {
            panic!("Keyword cannot start with whitespace");
        }
        self.0
    }
}
#[derive(Debug)]
pub struct KeywordSequence(pub &'static [Keyword]);

pub enum TokenKind {
    FunctionSymbol,
    Literal,
    SanityCheck,
}
#[derive(Debug, Diagnostic, Error, Clone, Copy, Serialize, Deserialize)]
pub enum ParseError {
    #[diagnostic(transparent)]
    UnexpectedToken(UnexpectedTokenError),
    UnexpectedEndOfInput(#[label("here")] SourceSpan),
    TmfsParseFailure(#[label("here")] SourceSpan),
    RecursionLimitExceeded(#[label("here")] SourceSpan),
}
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ParseMetadata {
    pub location: SourceSpan,
    // todo: sourcespans of parts?
}

#[derive(Debug, Diagnostic, Error, Clone, Copy, Serialize, Deserialize)]
pub struct UnexpectedTokenError {
    // pub expected_any_of: Vec<Keyword>,
    #[label("here")]
    pub at: SourceSpan,
}

impl UnexpectedTokenError {
    pub fn merge_over(
        self,
        previous: Option<UnexpectedTokenError>,
    ) -> Option<UnexpectedTokenError> {
        match previous {
            None => Some(self),
            Some(previous) => {
                // for kw in self.expected_any_of {
                //     if !previous.expected_any_of.iter().any(|it| it.0 == kw.0) {
                //         previous.expected_any_of.push(kw);
                //     }
                // }
                Some(previous)
            }
        }
    }
}

impl ParseError {
    pub fn merge_over(self, previous: Option<ParseError>) -> Option<ParseError> {
        match previous {
            None => Some(self),
            Some(ParseError::UnexpectedToken(previous)) => match self {
                ParseError::UnexpectedToken(current) => Some(ParseError::UnexpectedToken(
                    current.merge_over(Some(previous)).unwrap(),
                )),
                _ => Some(self),
            },
            _ => Some(self),
        }
    }
}

impl std::fmt::Display for UnexpectedTokenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "expected one of ")?;
        // for (i, keyword) in self.expected_any_of.iter().enumerate() {
        //     if i > 0 {
        //         write!(f, ", ")?;
        //     }
        //     write!(f, "{}", keyword)?;
        // }
        Ok(())
    }
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::UnexpectedToken(unexpected_token_error) => unexpected_token_error.fmt(f),
            ParseError::UnexpectedEndOfInput(_) => {
                write!(f, "unexpected end of input")
            }
            ParseError::TmfsParseFailure(_) => {
                write!(f, "failed to parse tmfs")
            }
            ParseError::RecursionLimitExceeded(source_span) => {
                write!(
                    f,
                    "recursion limit exceeded at offset {}",
                    source_span.offset()
                )
            }
        }
    }
}

// pub trait Parse<Heap>: term::Heaped<Heap = Heap> + Sized {
//     fn parse(
//         source: &str,
//         offset: miette::SourceOffset,
//         heap: &mut Self::Heap,
//         errors: &mut Vec<ParseError>,
//     ) -> (Self, miette::SourceOffset);
//     fn unparse(&self, heap: &Self::Heap) -> Unparse;
// }
