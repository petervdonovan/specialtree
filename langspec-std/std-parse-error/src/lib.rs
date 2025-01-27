use langspec::{flat::LangSpecFlat, humanreadable::LangSpecHuman, langspec::TerminalLangSpec};

pub fn parse_error() -> LangSpecFlat<tymetafuncspec_core::Core> {
    let lsh: LangSpecHuman<tymetafuncspec_core::Core> = serde_json::from_str(
        r#"
    {
        "name": {
            "human": "ParseError",
            "camel": "ParseError",
            "snake": "parse_error"
        },
        "products": [
            {
                "name": {
                    "human": "error",
                    "camel": "Error",
                    "snake": "error"
                },
                "sorts": []
            }
        ],
        "sums": []
    }
    "#,
    )
    .unwrap();
    LangSpecFlat::canonical_from(&lsh)
}
