use langspec::{flat::LangSpecFlat, humanreadable::LangSpecHuman, langspec::TerminalLangSpec};

pub fn parse_metadata() -> LangSpecFlat<tymetafuncspec_core::Core> {
    let lsh: LangSpecHuman<tymetafuncspec_core::Core> = serde_json::from_str(
        r#"
    {
        "name": {
            "human": "ParseMetadata",
            "camel": "ParseMetadata",
            "snake": "parse_metadata"
        },
        "products": [
            {
                "name": {
                    "human": "line",
                    "camel": "Line",
                    "snake": "line"
                },
                "sorts": [
                    {"TyMetaFunc": {"f": 0, "a": []}}
                ]
            },
            {
                "name": {
                    "human": "column",
                    "camel": "Column",
                    "snake": "column"
                },
                "sorts": [
                    {"TyMetaFunc": {"f": 0, "a": []}}
                ]
            },
            {
                "name": {
                    "human": "end_position",
                    "camel": "EndPosition",
                    "snake": "end_position"
                },
                "sorts": [
                    {"Algebraic": {"Product": "line"}},
                    {"Algebraic": {"Product": "column"}}
                ]
            },
            {
                "name": {
                    "human": "start_position",
                    "camel": "StartPosition",
                    "snake": "start_position"
                },
                "sorts": [
                    {"Algebraic": {"Product": "line"}},
                    {"Algebraic": {"Product": "column"}}
                ]
            },
            {
                "name": {
                    "human": "metadata",
                    "camel": "Metadata",
                    "snake": "metadata"
                },
                "sorts": [
                    {"Algebraic": {"Product": "start_position"}},
                    {"Algebraic": {"Product": "end_position"}}
                ]
            }
        ],
        "sums": []
    }
    "#,
    )
    .unwrap();
    LangSpecFlat::canonical_from(&lsh)
}
