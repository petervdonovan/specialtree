use memo_with_references::memo;

#[memo('a)]
fn vector<'a>(x: u32) -> Vec<u32> {
    vec![x, x]
}

#[memo('a)]
fn integer<'a>(x: u32) -> u32 {
    x
}

#[test]
fn test() {
    let arena = bumpalo::Bump::new();
    let cache: memo_cache::Cache<memo_cache::logger::DebugCacheLogger> =
        memo_cache::Cache::new(&arena);
    let i = integer(&cache, 32);
    let v = vector(&cache, 32);
    expect_test::expect![[r#"32"#]].assert_eq(&format!("{i:?}"));
    expect_test::expect![[r#"[32, 32]"#]].assert_eq(&format!("{v:?}"));
    let report = cache.report();
    expect_test::expect![[r#"
        MISS: u32
        MISS: alloc::vec::Vec<u32>"#]]
    .assert_eq(&report);
}
