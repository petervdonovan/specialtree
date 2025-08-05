use arbitrary::{Arbitrary, Result, Unstructured};
use thiserror::Error;

#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum LeafError {
    #[error("Leaf string must be lowercase")]
    NotLowercase,
    #[error(
        "Leaf string must start with an alphabetic character that has distinct uppercase and lowercase forms"
    )]
    InvalidStartChar,
    #[error("Leaf string must contain only alphanumeric characters")]
    NonAlphanumeric,
    #[error("Leaf string cannot be camel case open token")]
    CamelOpen,
    #[error("Leaf string cannot be camel case close token")]
    CamelClose,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Leaf(String);

impl Leaf {
    pub fn new(s: String) -> Result<Self, LeafError> {
        if s.is_empty() {
            return Err(LeafError::InvalidStartChar);
        }
        if s.to_lowercase() != s {
            return Err(LeafError::NotLowercase);
        }
        if !s.chars().next().is_some_and(|c| {
            c.is_alphabetic() && !(c.to_uppercase().zip(c.to_lowercase()).all(|(a, b)| a == b))
        }) {
            return Err(LeafError::InvalidStartChar);
        }
        if !s.chars().all(|c| c.is_alphanumeric()) {
            return Err(LeafError::NonAlphanumeric);
        }
        if s == OPEN_TOKEN {
            return Err(LeafError::CamelOpen);
        }
        if s == CLOSE_TOKEN {
            return Err(LeafError::CamelClose);
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TreeToken {
    Leaf(Leaf),
    List(Box<[TreeToken]>),
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
        let threshold = (list_probability * 256.0) as u8;
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

    pub fn from_camel_str(s: &str) -> Result<Self, String> {
        // Stage 1: Lexing - split on capital letters
        fn lex_camel_str(s: &str) -> Vec<String> {
            if s.is_empty() {
                return Vec::new();
            }

            let mut tokens = Vec::new();
            let mut current_token = String::new();

            for ch in s.chars() {
                if ch.is_uppercase() {
                    // Start new token
                    if !current_token.is_empty() {
                        tokens.push(current_token);
                    }
                    current_token = String::new();
                    current_token.push(ch.to_lowercase().next().unwrap());
                } else {
                    // Continue current token
                    current_token.push(ch);
                }
            }

            // Don't forget the last token
            if !current_token.is_empty() {
                tokens.push(current_token);
            }

            tokens
        }

        // Stage 2: Parsing - interpret L/R as structural markers
        fn parse_tokens(tokens: &[String], pos: &mut usize) -> Result<Vec<TreeToken>, String> {
            let mut result = Vec::new();

            while *pos < tokens.len() {
                let token = &tokens[*pos];

                if token == "l" {
                    // L token (lowercase after lexing)
                    // Start of nested list
                    *pos += 1;
                    let nested = parse_tokens(tokens, pos)?;
                    result.push(TreeToken::List(nested.into_boxed_slice()));
                } else if token == "r" {
                    // R token (lowercase after lexing)
                    // End of current list
                    *pos += 1;
                    break;
                } else {
                    // Regular leaf token
                    let leaf =
                        Leaf::new(token.clone()).map_err(|e| format!("Invalid leaf: {e}"))?;
                    result.push(TreeToken::Leaf(leaf));
                    *pos += 1;
                }
            }

            Ok(result)
        }

        if s.is_empty() {
            return Ok(TreeToken::List(Box::new([])));
        }

        let tokens = lex_camel_str(s);
        let mut pos = 0;
        let parsed = parse_tokens(&tokens, &mut pos)?;
        Ok(TreeToken::List(parsed.into_boxed_slice()))
    }

    pub fn camel_str(&self) -> String {
        fn camel_str_rec(this: &TreeToken, s: &mut String) {
            match this {
                TreeToken::Leaf(leaf) => {
                    push_capitalized(leaf.as_str(), s);
                }
                TreeToken::List(tree_tokens) => {
                    push_capitalized(OPEN_TOKEN, s);
                    for tt in tree_tokens {
                        camel_str_rec(tt, s);
                    }
                    push_capitalized(CLOSE_TOKEN, s);
                }
            }
        }
        let mut ret = String::new();
        match self {
            TreeToken::Leaf(leaf) => push_capitalized(leaf.as_str(), &mut ret),
            TreeToken::List(tree_tokens) => {
                for tt in tree_tokens {
                    camel_str_rec(tt, &mut ret);
                }
            }
        }
        ret
    }
    pub fn snake_str(&self) -> String {
        todo!()
    }
    pub fn kebab_str(&self) -> String {
        todo!()
    }
    pub fn camel_ident(&self) -> syn::Ident {
        todo!()
    }
    pub fn snake_ident(&self) -> syn::Ident {
        todo!()
    }
    pub fn kebab_ident(&self) -> syn::Ident {
        todo!()
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
        // Test each validation error type
        assert_eq!(
            Leaf::new("Hello".to_string()).unwrap_err(),
            LeafError::NotLowercase
        );
        assert_eq!(
            Leaf::new("123abc".to_string()).unwrap_err(),
            LeafError::InvalidStartChar
        );
        assert_eq!(
            Leaf::new("hello_world".to_string()).unwrap_err(),
            LeafError::NonAlphanumeric
        );
        assert_eq!(
            Leaf::new("l".to_string()).unwrap_err(),
            LeafError::CamelOpen
        );
        assert_eq!(
            Leaf::new("r".to_string()).unwrap_err(),
            LeafError::CamelClose
        );
        assert_eq!(
            Leaf::new("".to_string()).unwrap_err(),
            LeafError::InvalidStartChar
        );
    }

    #[test]
    fn test_leaf_error_display() {
        // Verify error messages are human-readable
        assert_eq!(
            LeafError::NotLowercase.to_string(),
            "Leaf string must be lowercase"
        );
        assert_eq!(
            LeafError::CamelOpen.to_string(),
            "Leaf string cannot be camel case open token"
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
        // Simple leaf - now wrapped in a list since we always return lists
        let result = TreeToken::from_camel_str("Hello").unwrap();
        check_camel_str(result, expect!["Hello"]);

        // Multiple words
        let result = TreeToken::from_camel_str("HelloWorld").unwrap();
        check_camel_str(result, expect!["HelloWorld"]);

        // Nested structure
        let result = TreeToken::from_camel_str("OuterLInnerLeafREnd").unwrap();
        check_camel_str(result, expect!["OuterLInnerLeafREnd"]);

        // Empty list
        let result = TreeToken::from_camel_str("").unwrap();
        check_camel_str(result, expect![""]);
    }

    #[test]
    fn test_round_trip_correctness_and_diversity() {
        use arbitrary::{Arbitrary, Unstructured};
        use rand::rngs::StdRng;
        use rand::{Rng, SeedableRng};

        const SEED: u64 = 42;
        const TOTAL_ITERATIONS: usize = 50;
        const DIVERSITY_SAMPLES: usize = 20; // Show first 20 for diversity check
        const BYTES_PER_ITERATION: usize = 64;

        fn generate_tree_token(rng: &mut StdRng) -> Option<TreeToken> {
            let mut random_bytes = vec![0u8; BYTES_PER_ITERATION];
            rng.fill(&mut random_bytes[..]);
            let mut u = Unstructured::new(&random_bytes);
            TreeToken::arbitrary(&mut u).ok()
        }

        fn test_round_trip(token: &TreeToken) -> Result<(), String> {
            let camel_str = token.camel_str();
            let parsed =
                TreeToken::from_camel_str(&camel_str).map_err(|e| format!("Parse failed: {e}"))?;
            let reparsed_camel = parsed.camel_str();
            if camel_str != reparsed_camel {
                return Err(format!(
                    "Round trip failed: original={token:?}, camel_str={camel_str}, parsed={parsed:?}"
                ));
            }
            Ok(())
        }

        let mut rng = StdRng::seed_from_u64(SEED);
        let mut diversity_results = Vec::new();
        let mut round_trip_count = 0;

        for i in 0..TOTAL_ITERATIONS {
            if let Some(token) = generate_tree_token(&mut rng) {
                // Test round trip
                if let Err(msg) = test_round_trip(&token) {
                    panic!("{msg}");
                }
                round_trip_count += 1;

                // Collect diversity samples
                if i < DIVERSITY_SAMPLES {
                    diversity_results.push(token.camel_str());
                }
            }
        }

        // Ensure we tested a reasonable number of round trips
        assert!(
            round_trip_count >= TOTAL_ITERATIONS / 2,
            "Too few successful round trips: {round_trip_count}"
        );

        // Check diversity
        let joined = diversity_results.join(", ");
        expect!["LP0D1tky4R, LW7RLS77aS4vRLJgjkbR, V, XaLugf54Ka19, LZupoqa7I5t8v6R, Hjp0, X, SfrEc2Z, M, LRUg4c3Uwb, LLDghojUoznFkRHdlR, Edwio, U0Qa51Bwrxwmx, Kvlgb, LR, Ak1iLR, QD6pD630b, Eeh2zpw, LR, LDNRA80e67H6"]
            .assert_eq(&joined);
    }
}
