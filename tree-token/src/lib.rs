use arbitrary::{Arbitrary, Result, Unstructured};
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TreeToken {
    Leaf(Leaf),
    List(Box<[TreeToken]>),
}

#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum LeafError {
    #[error("Leaf string '{0}' must be lowercase")]
    NotLowercase(String),
    #[error(
        "Leaf string '{0}' must start with an alphabetic character that has distinct uppercase and lowercase forms"
    )]
    InvalidStartChar(String),
    #[error("Leaf string '{0}' must contain only alphanumeric characters")]
    NonAlphanumeric(String),
    #[error("Leaf string '{0}' cannot be camel case open token")]
    CamelOpen(String),
    #[error("Leaf string '{0}' cannot be camel case close token")]
    CamelClose(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Leaf(String);

impl Leaf {
    pub fn new(s: String) -> Result<Self, LeafError> {
        if s.is_empty() {
            return Err(LeafError::InvalidStartChar(s));
        }
        if s.to_lowercase() != s {
            return Err(LeafError::NotLowercase(s));
        }
        if !s.chars().next().is_some_and(|c| {
            c.is_alphabetic() && !(c.to_uppercase().zip(c.to_lowercase()).all(|(a, b)| a == b))
        }) {
            return Err(LeafError::InvalidStartChar(s));
        }
        if !s.chars().all(|c| c.is_alphanumeric()) {
            return Err(LeafError::NonAlphanumeric(s));
        }
        if s == OPEN_TOKEN {
            return Err(LeafError::CamelOpen(s));
        }
        if s == CLOSE_TOKEN {
            return Err(LeafError::CamelClose(s));
        }
        Ok(Leaf(s))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn into_string(self) -> String {
        self.0
    }
}

impl<'a> Arbitrary<'a> for Leaf {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self> {
        const MAX_TOKEN_LEN: usize = 6;
        let valid_starts = [
            'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q',
            'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
        ];
        let start_char = *u.choose(&valid_starts)?;

        let len = u.int_in_range(0..=MAX_TOKEN_LEN)?;
        let valid_chars = [
            'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q',
            'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1', '2', '3', '4', '5', '6', '7',
            '8', '9',
        ];

        let mut s = String::new();
        s.push(start_char);

        for _ in 0..len {
            let c = *u.choose(&valid_chars)?;
            s.push(c);
        }
        if [OPEN_TOKEN, CLOSE_TOKEN].contains(&s.as_str()) {
            return Leaf::arbitrary(u);
        }

        Ok(Leaf(s))
    }
}

impl<'a> Arbitrary<'a> for TreeToken {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self> {
        const INITIAL_LIST_PROBABILITY: f64 = 2.0 / 3.0;
        Self::arbitrary_with_probability(u, INITIAL_LIST_PROBABILITY, true)
    }
}

impl TreeToken {
    fn arbitrary_with_probability<'a>(
        u: &mut Unstructured<'a>,
        list_probability: f64,
        is_top_level: bool,
    ) -> Result<Self> {
        // Use the probability to decide between leaf and list
        let threshold = (list_probability * 256.0).floor() as u8;
        let should_be_list = u.arbitrary::<u8>()? < threshold;

        if should_be_list {
            // For lists, ensure they're not empty at the top level
            let len = if is_top_level {
                u.int_in_range(1..=3)? // Top level: at least 1 element
            } else {
                u.int_in_range(0..=3)? // Nested: can be empty
            };

            // Divide probability by list length to keep expected size finite
            let new_probability = if len > 0 {
                list_probability / len as f64
            } else {
                0.0
            };

            let mut items = Vec::with_capacity(len);
            for _ in 0..len {
                items.push(Self::arbitrary_with_probability(u, new_probability, false)?);
            }
            Ok(TreeToken::List(items.into_boxed_slice()))
        } else {
            Ok(TreeToken::Leaf(Leaf::arbitrary(u)?))
        }
    }
}

const OPEN_TOKEN: &str = "l";
const CLOSE_TOKEN: &str = "r";

impl TreeToken {
    pub fn list(l: Box<[TreeToken]>) -> Self {
        TreeToken::List(l)
    }

    fn unparse_to_token_stream(&self) -> Vec<&str> {
        fn unparse_recursively<'a>(token: &'a TreeToken, tokens: &mut Vec<&'a str>) {
            match token {
                TreeToken::Leaf(leaf) => {
                    tokens.push(leaf.as_str());
                }
                TreeToken::List(tree_tokens) => {
                    tokens.push(OPEN_TOKEN);
                    for tt in tree_tokens {
                        unparse_recursively(tt, tokens);
                    }
                    tokens.push(CLOSE_TOKEN);
                }
            }
        }

        let mut tokens = Vec::new();
        match self {
            TreeToken::Leaf(leaf) => tokens.push(leaf.as_str()),
            TreeToken::List(tree_tokens) => {
                for tt in tree_tokens {
                    unparse_recursively(tt, &mut tokens);
                }
            }
        }
        tokens
    }

    fn parse_token_sequence(tokens: &[String]) -> Result<Vec<TreeToken>, String> {
        let mut result = Vec::new();
        let mut pos = 0;

        while pos < tokens.len() {
            let token = &tokens[pos];

            if token == OPEN_TOKEN {
                pos += 1;
                let nested = Self::parse_token_sequence_recursive(tokens, &mut pos)?;
                result.push(TreeToken::List(nested.into_boxed_slice()));
            } else if token == CLOSE_TOKEN {
                break;
            } else {
                let leaf = Leaf::new(token.clone()).map_err(|e| format!("Invalid leaf: {e}"))?;
                result.push(TreeToken::Leaf(leaf));
                pos += 1;
            }
        }

        Ok(result)
    }

    fn parse_token_sequence_recursive(
        tokens: &[String],
        pos: &mut usize,
    ) -> Result<Vec<TreeToken>, String> {
        let mut result = Vec::new();

        while *pos < tokens.len() {
            let token = &tokens[*pos];

            if token == OPEN_TOKEN {
                *pos += 1;
                let nested = Self::parse_token_sequence_recursive(tokens, pos)?;
                result.push(TreeToken::List(nested.into_boxed_slice()));
            } else if token == CLOSE_TOKEN {
                *pos += 1;
                break;
            } else {
                let leaf = Leaf::new(token.clone()).map_err(|e| format!("Invalid leaf: {e}"))?;
                result.push(TreeToken::Leaf(leaf));
                *pos += 1;
            }
        }

        Ok(result)
    }

    pub fn from_camel_str(s: &str) -> Result<Self, String> {
        fn lex_camel_str(s: &str) -> Vec<String> {
            if s.is_empty() {
                return Vec::new();
            }

            let mut tokens = Vec::new();
            let mut current_token = String::new();

            for ch in s.chars() {
                if ch.is_uppercase() {
                    if !current_token.is_empty() {
                        tokens.push(current_token);
                    }
                    current_token = String::new();
                    current_token.push(ch.to_lowercase().next().unwrap());
                } else {
                    current_token.push(ch);
                }
            }

            if !current_token.is_empty() {
                tokens.push(current_token);
            }

            tokens
        }

        if s.is_empty() {
            return Ok(TreeToken::List(Box::new([])));
        }

        let tokens = lex_camel_str(s);
        let parsed = Self::parse_token_sequence(&tokens)?;
        Ok(TreeToken::List(parsed.into_boxed_slice()))
    }

    pub fn from_snake_str(s: &str) -> Result<Self, String> {
        if s.is_empty() {
            return Ok(TreeToken::List(Box::new([])));
        }

        let tokens: Vec<String> = s.split('_').map(|s| s.to_string()).collect();
        let parsed = Self::parse_token_sequence(&tokens)?;
        Ok(TreeToken::List(parsed.into_boxed_slice()))
    }

    pub fn from_kebab_str(s: &str) -> Result<Self, String> {
        if s.is_empty() {
            return Ok(TreeToken::List(Box::new([])));
        }

        let tokens: Vec<String> = s.split('-').map(|s| s.to_string()).collect();
        let parsed = Self::parse_token_sequence(&tokens)?;
        Ok(TreeToken::List(parsed.into_boxed_slice()))
    }

    pub fn camel_str(&self) -> String {
        let tokens = self.unparse_to_token_stream();
        tokens
            .into_iter()
            .map(|token| {
                let mut result = String::new();
                push_capitalized(token, &mut result);
                result
            })
            .collect()
    }

    pub fn snake_str(&self) -> String {
        let tokens = self.unparse_to_token_stream();
        tokens.join("_")
    }

    pub fn kebab_str(&self) -> String {
        let tokens = self.unparse_to_token_stream();
        tokens.join("-")
    }
    pub fn camel_ident(&self) -> syn::Ident {
        syn::Ident::new(&self.camel_str(), proc_macro2::Span::call_site())
    }
    pub fn snake_ident(&self) -> syn::Ident {
        syn::Ident::new(&self.camel_str(), proc_macro2::Span::call_site())
    }
    pub fn kebab_ident(&self) -> syn::Ident {
        syn::Ident::new(&self.kebab_str(), proc_macro2::Span::call_site())
    }
}

fn push_capitalized(input: &str, output: &mut String) {
    let first_char = input.chars().next().unwrap();
    for uppercase in first_char.to_uppercase() {
        output.push(uppercase);
    }
    output.push_str(&input[first_char.len_utf8()..])
}

#[cfg(test)]
mod tests {
    use super::*;
    use expect_test::{Expect, expect};

    fn check_camel_str(tree_token: TreeToken, expected: Expect) {
        expected.assert_eq(&tree_token.camel_str());
    }

    #[test]
    fn test_leaf_new_valid() {
        // Test valid cases
        assert!(Leaf::new("hello".to_string()).is_ok());
        assert!(Leaf::new("test123".to_string()).is_ok());
        assert!(Leaf::new("a".to_string()).is_ok());
    }

    #[test]
    fn test_leaf_new_validation_errors() {
        assert_eq!(
            Leaf::new("Hello".to_string()).unwrap_err(),
            LeafError::NotLowercase("Hello".to_string())
        );
        assert_eq!(
            Leaf::new("123abc".to_string()).unwrap_err(),
            LeafError::InvalidStartChar("123abc".to_string())
        );
        assert_eq!(
            Leaf::new("hello_world".to_string()).unwrap_err(),
            LeafError::NonAlphanumeric("hello_world".to_string())
        );
        assert_eq!(
            Leaf::new("l".to_string()).unwrap_err(),
            LeafError::CamelOpen("l".to_string())
        );
        assert_eq!(
            Leaf::new("r".to_string()).unwrap_err(),
            LeafError::CamelClose("r".to_string())
        );
        assert_eq!(
            Leaf::new("".to_string()).unwrap_err(),
            LeafError::InvalidStartChar("".to_string())
        );
    }

    #[test]
    fn test_leaf_error_display() {
        assert_eq!(
            LeafError::NotLowercase("Hello".to_string()).to_string(),
            "Leaf string 'Hello' must be lowercase"
        );
        assert_eq!(
            LeafError::CamelOpen("l".to_string()).to_string(),
            "Leaf string 'l' cannot be camel case open token"
        );
    }

    #[test]
    fn test_camel_str_leaf() {
        let leaf = TreeToken::Leaf(Leaf::new("hello".to_string()).unwrap());
        check_camel_str(leaf, expect!["Hello"]);

        let leaf = TreeToken::Leaf(Leaf::new("test123".to_string()).unwrap());
        check_camel_str(leaf, expect!["Test123"]);

        let leaf = TreeToken::Leaf(Leaf::new("a".to_string()).unwrap());
        check_camel_str(leaf, expect!["A"]);
    }

    #[test]
    fn test_camel_str_list() {
        // Empty list
        let empty_list = TreeToken::List(Box::new([]));
        check_camel_str(empty_list, expect![""]);

        // Single leaf in list
        let single_leaf = TreeToken::List(Box::new([TreeToken::Leaf(
            Leaf::new("hello".to_string()).unwrap(),
        )]));
        check_camel_str(single_leaf, expect!["Hello"]);

        // Multiple leaves in list
        let multi_leaf = TreeToken::List(Box::new([
            TreeToken::Leaf(Leaf::new("hello".to_string()).unwrap()),
            TreeToken::Leaf(Leaf::new("world".to_string()).unwrap()),
        ]));
        check_camel_str(multi_leaf, expect!["HelloWorld"]);
    }

    #[test]
    fn test_camel_str_nested() {
        // Nested list structure
        let nested = TreeToken::List(Box::new([
            TreeToken::Leaf(Leaf::new("outer".to_string()).unwrap()),
            TreeToken::List(Box::new([
                TreeToken::Leaf(Leaf::new("inner".to_string()).unwrap()),
                TreeToken::Leaf(Leaf::new("leaf".to_string()).unwrap()),
            ])),
            TreeToken::Leaf(Leaf::new("end".to_string()).unwrap()),
        ]));
        check_camel_str(nested, expect!["OuterLInnerLeafREnd"]);
    }

    #[test]
    fn test_from_camel_str() {
        let result = TreeToken::from_camel_str("Hello").unwrap();
        check_camel_str(result, expect!["Hello"]);

        let result = TreeToken::from_camel_str("HelloWorld").unwrap();
        check_camel_str(result, expect!["HelloWorld"]);

        let result = TreeToken::from_camel_str("OuterLInnerLeafREnd").unwrap();
        check_camel_str(result, expect!["OuterLInnerLeafREnd"]);

        let result = TreeToken::from_camel_str("").unwrap();
        check_camel_str(result, expect![""]);
    }

    #[test]
    fn test_snake_str() {
        let leaf = TreeToken::Leaf(Leaf::new("hello".to_string()).unwrap());
        assert_eq!(leaf.snake_str(), "hello");

        let multi_leaf = TreeToken::List(Box::new([
            TreeToken::Leaf(Leaf::new("hello".to_string()).unwrap()),
            TreeToken::Leaf(Leaf::new("world".to_string()).unwrap()),
        ]));
        assert_eq!(multi_leaf.snake_str(), "hello_world");

        let nested = TreeToken::List(Box::new([
            TreeToken::Leaf(Leaf::new("outer".to_string()).unwrap()),
            TreeToken::List(Box::new([
                TreeToken::Leaf(Leaf::new("inner".to_string()).unwrap()),
                TreeToken::Leaf(Leaf::new("leaf".to_string()).unwrap()),
            ])),
            TreeToken::Leaf(Leaf::new("end".to_string()).unwrap()),
        ]));
        assert_eq!(nested.snake_str(), "outer_l_inner_leaf_r_end");
    }

    #[test]
    fn test_kebab_str() {
        let leaf = TreeToken::Leaf(Leaf::new("hello".to_string()).unwrap());
        assert_eq!(leaf.kebab_str(), "hello");

        let multi_leaf = TreeToken::List(Box::new([
            TreeToken::Leaf(Leaf::new("hello".to_string()).unwrap()),
            TreeToken::Leaf(Leaf::new("world".to_string()).unwrap()),
        ]));
        assert_eq!(multi_leaf.kebab_str(), "hello-world");

        let nested = TreeToken::List(Box::new([
            TreeToken::Leaf(Leaf::new("outer".to_string()).unwrap()),
            TreeToken::List(Box::new([
                TreeToken::Leaf(Leaf::new("inner".to_string()).unwrap()),
                TreeToken::Leaf(Leaf::new("leaf".to_string()).unwrap()),
            ])),
            TreeToken::Leaf(Leaf::new("end".to_string()).unwrap()),
        ]));
        assert_eq!(nested.kebab_str(), "outer-l-inner-leaf-r-end");
    }

    #[test]
    fn test_from_snake_str() {
        let result = TreeToken::from_snake_str("hello").unwrap();
        assert_eq!(result.snake_str(), "hello");

        let result = TreeToken::from_snake_str("hello_world").unwrap();
        assert_eq!(result.snake_str(), "hello_world");

        let result = TreeToken::from_snake_str("outer_l_inner_leaf_r_end").unwrap();
        assert_eq!(result.snake_str(), "outer_l_inner_leaf_r_end");

        let result = TreeToken::from_snake_str("").unwrap();
        assert_eq!(result.snake_str(), "");
    }

    #[test]
    fn test_from_kebab_str() {
        let result = TreeToken::from_kebab_str("hello").unwrap();
        assert_eq!(result.kebab_str(), "hello");

        let result = TreeToken::from_kebab_str("hello-world").unwrap();
        assert_eq!(result.kebab_str(), "hello-world");

        let result = TreeToken::from_kebab_str("outer-l-inner-leaf-r-end").unwrap();
        assert_eq!(result.kebab_str(), "outer-l-inner-leaf-r-end");

        let result = TreeToken::from_kebab_str("").unwrap();
        assert_eq!(result.kebab_str(), "");
    }

    #[test]
    fn test_round_trip_correctness_and_diversity() {
        use arbitrary::{Arbitrary, Unstructured};
        use rand::rngs::StdRng;
        use rand::{Rng, SeedableRng};

        const SEED: u64 = 42;
        const TOTAL_ITERATIONS: usize = 50;
        const DIVERSITY_SAMPLES: usize = 20;
        const BYTES_PER_ITERATION: usize = 64;

        fn generate_tree_token(rng: &mut StdRng) -> Option<TreeToken> {
            let mut random_bytes = vec![0u8; BYTES_PER_ITERATION];
            rng.fill(&mut random_bytes[..]);
            let mut u = Unstructured::new(&random_bytes);
            TreeToken::arbitrary(&mut u).ok()
        }

        fn test_camel_round_trip(token: &TreeToken) -> Result<(), String> {
            let camel_str = token.camel_str();
            let parsed = TreeToken::from_camel_str(&camel_str)
                .map_err(|e| format!("Camel parse failed: {e}"))?;
            let reparsed_camel = parsed.camel_str();
            if camel_str != reparsed_camel {
                return Err(format!(
                    "Camel round trip failed: original={token:?}, camel_str={camel_str}, parsed={parsed:?}"
                ));
            }
            Ok(())
        }

        fn test_snake_round_trip(token: &TreeToken) -> Result<(), String> {
            let snake_str = token.snake_str();
            let parsed = TreeToken::from_snake_str(&snake_str)
                .map_err(|e| format!("Snake parse failed: {e}"))?;
            let reparsed_snake = parsed.snake_str();
            if snake_str != reparsed_snake {
                return Err(format!(
                    "Snake round trip failed: original={token:?}, snake_str={snake_str}, parsed={parsed:?}"
                ));
            }
            Ok(())
        }

        fn test_kebab_round_trip(token: &TreeToken) -> Result<(), String> {
            let kebab_str = token.kebab_str();
            let parsed = TreeToken::from_kebab_str(&kebab_str)
                .map_err(|e| format!("Kebab parse failed: {e}"))?;
            let reparsed_kebab = parsed.kebab_str();
            if kebab_str != reparsed_kebab {
                return Err(format!(
                    "Kebab round trip failed: original={token:?}, kebab_str={kebab_str}, parsed={parsed:?}"
                ));
            }
            Ok(())
        }

        let mut rng = StdRng::seed_from_u64(SEED);
        let mut diversity_results = Vec::new();
        let mut round_trip_count = 0;

        for i in 0..TOTAL_ITERATIONS {
            if let Some(token) = generate_tree_token(&mut rng) {
                if let Err(msg) = test_camel_round_trip(&token) {
                    panic!("{msg}");
                }
                if let Err(msg) = test_snake_round_trip(&token) {
                    panic!("{msg}");
                }
                if let Err(msg) = test_kebab_round_trip(&token) {
                    panic!("{msg}");
                }
                round_trip_count += 1;

                if i < DIVERSITY_SAMPLES {
                    diversity_results.push(token.kebab_str());
                }
            }
        }

        assert!(
            round_trip_count >= TOTAL_ITERATIONS / 2,
            "Too few successful round trips: {round_trip_count}"
        );

        let joined = diversity_results.join(", ");
        expect!["l-p0-d1tky4-r, l-w7-r-l-s77a-s4v-r-l-jgjkb-r, v, xa-lugf54-ka19, l-zupoqa7-i5t8v6-r, hjp0, x, sfr-ec2-z, m, l-r-ug4c3-uwb, l-l-dghoj-uozn-fk-r-hdl-r, edwio, u0-qa51-bwrxwmx, kvlgb, l-r, ak1i-l-r, q-d6p-d630b, eeh2zpw, l-r, l-d-n-r-a80e67-h6"]
            .assert_eq(&joined);
    }
}
