use miette::{Diagnostic, SourceSpan};
use serde::{Deserialize, Serialize};
use thiserror::Error;

pub use miette;
pub use unicode_segmentation;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Keyword(&'static str);
// pub struct Keyword(std::borrow::Cow<'static, str>);

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
    fn get(&self) -> &'static str {
        if self.0.is_empty() {
            panic!("Keyword cannot be empty");
        }
        if self.0.chars().any(|it| it.is_whitespace()) {
            panic!("Keyword cannot start with whitespace");
        }
        self.0
    }
    pub fn find(
        &self,
        source: &str,
        offset: miette::SourceOffset,
    ) -> Option<(miette::SourceOffset, miette::SourceOffset)> {
        match unicode_segmentation::UnicodeSegmentation::unicode_words(&source[offset.offset()..])
            .next()
        {
            Some(token) => {
                if token == self.get() {
                    let start = offset.offset() + source[offset.offset()..].find(token).unwrap();
                    let end = start + token.len();
                    Some((start.into(), end.into()))
                } else {
                    None
                }
            }
            None => None,
        }
    }
}

pub struct KeywordSequence(pub &'static [Keyword]);
impl KeywordSequence {
    pub fn find(
        &self,
        source: &str,
        offset: miette::SourceOffset,
    ) -> Option<(miette::SourceOffset, miette::SourceOffset)> {
        let mut start = None;
        let mut end = None;
        for kw in self.0.iter() {
            if let Some((kw_start, kw_end)) = kw.find(source, offset) {
                if start.is_none() {
                    start = Some(kw_start);
                }
                end = Some(kw_end);
            } else {
                return None;
            }
        }
        Some((start.unwrap(), end.unwrap()))
    }
}

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
            Some(mut previous) => {
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
        }
    }
}
#[derive(Debug)]
pub struct Unparse {
    lines: Vec<String>,
}

impl Unparse {
    pub fn new(start: &str) -> Unparse {
        if start.contains('\n') {
            panic!("UnparseResult::new only works on single-line strings");
        }
        if start.is_empty() {
            panic!("UnparseResult::new only works on non-empty strings");
        }
        if start.chars().next().unwrap().is_whitespace()
            || start.chars().last().unwrap().is_whitespace()
        {
            panic!(
                "UnparseResult::new only works on strings without leading or trailing whitespace"
            );
        }
        Unparse {
            lines: vec![start.to_string()],
        }
    }
    pub fn render(&self) -> String {
        self.lines.join("\n")
    }
    pub fn indent(&mut self) {
        for line in self.lines.iter_mut() {
            *line = format!("  {}", line);
        }
    }
    pub fn hstack(&mut self, other: Unparse) {
        if self.lines.len() != 1 || other.lines.len() != 1 {
            panic!("hstack only works on single-line unparse results");
        }
        self.lines[0].push(' ');
        self.lines[0].push_str(&other.lines[0]);
    }
    pub fn vstack(&mut self, other: Unparse) {
        self.lines.extend(other.lines);
    }
    pub fn width(&self) -> usize {
        self.lines.iter().map(|it| it.len()).max().unwrap_or(0)
    }
    pub fn height(&self) -> usize {
        self.lines.len()
    }
}

impl std::fmt::Display for Unparse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.render())
    }
}

pub trait Parse<Heap>: term::Heaped<Heap = Heap> + Sized {
    fn parse(
        source: &str,
        offset: miette::SourceOffset,
        heap: &mut Self::Heap,
        errors: &mut Vec<ParseError>,
    ) -> (Self, miette::SourceOffset);
    fn unparse(&self, heap: &Self::Heap) -> Unparse;
}
