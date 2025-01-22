pub struct ZeroBased(pub usize);

pub struct ParsePosition<'a> {
    s: &'a str,
    line: ZeroBased,
    col: ZeroBased,
}

pub struct Token(String);

pub enum TokenKind {
    FunctionSymbol,
    Literal,
}

pub struct Error {
    pub expected_single_tokens: Vec<String>,
    pub got_token: String,
}
