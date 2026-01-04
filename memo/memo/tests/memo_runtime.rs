use memo_with_references::memo;

// Simple non-Clone type we'll pass by reference (frozen argument)
#[derive(Hash, PartialEq, Eq)]
struct BigThing {
    val: u32,
}

// Non-copyable, non-cloneable type to ensure no hidden copies
#[derive(Debug)]
#[allow(dead_code)]
struct NoCopyThing {
    data: Vec<u32>,
    id: u32,
}

// Use a real cache type now (&Cache) even though the macro still doesn't perform memoization.
#[memo('a)]
fn add_clonearg<'a>(x: u32) -> NoCopyThing {
    NoCopyThing {
        data: vec![x * 2],
        id: x,
    }
}

// Function taking only a non-clone (reference) argument.
#[memo('a)]
fn ref_only<'a>(bt: &'a BigThing) -> u32 {
    bt.val * 3
}

// Function taking both a cloneable owned arg and a non-clone reference arg.
#[memo('a)]
fn clone_and_ref<'a>(x: u32, bt: &'a BigThing) -> u32 {
    x + bt.val
}

// Test function that returns a reference to the result of another memoized function
#[memo('a)]
fn nested_memo<'a>(x: u32) -> &'a NoCopyThing {
    add_clonearg(cache, x + 1)
}

fn test_add_clonearg<'a>(cache: &'a memo_cache::Cache<'a, memo_cache::logger::DebugCacheLogger>) {
    cache.reset_logger();
    let a1 = add_clonearg(cache, 21);
    let a2 = add_clonearg(cache, 22);
    let a3 = add_clonearg(cache, 21);

    expect_test::expect![[r#"
        NoCopyThing {
            data: [
                42,
            ],
            id: 21,
        }"#]]
    .assert_eq(&format!("{:#?}", *a1));

    expect_test::expect![[r#"
        NoCopyThing {
            data: [
                44,
            ],
            id: 22,
        }"#]]
    .assert_eq(&format!("{:#?}", *a2));

    expect_test::expect![[r#"
        NoCopyThing {
            data: [
                42,
            ],
            id: 21,
        }"#]]
    .assert_eq(&format!("{:#?}", *a3));

    let report = cache.report();
    // Should have 2 misses for unique values (21, 22) and 1 hit when 21 is repeated
    expect_test::expect![[r#"
        MISS: memo_runtime::NoCopyThing
        MISS: memo_runtime::NoCopyThing
        HIT: memo_runtime::NoCopyThing"#]]
    .assert_eq(&report);
}

fn test_ref_only<'a>(
    arena: &'a bumpalo::Bump,
    cache: &'a memo_cache::Cache<'a, memo_cache::logger::DebugCacheLogger>,
) {
    cache.reset_logger();
    let bt1 = arena.alloc(BigThing { val: 10 });
    let bt2 = arena.alloc(BigThing { val: 11 });
    // Two distinct references then repeat first.
    let r1 = ref_only(cache, bt1);
    let r2 = ref_only(cache, bt2);
    let r3 = ref_only(cache, bt1);
    assert_eq!(*r1, 30);
    assert_eq!(*r2, 33);
    assert_eq!(*r3, 30);

    let report = cache.report();
    expect_test::expect![[r#"
        MISS: u32
        MISS: u32
        HIT: u32"#]]
    .assert_eq(&report);
}

fn test_clone_and_ref<'a>(
    arena: &'a bumpalo::Bump,
    cache: &'a memo_cache::Cache<'a, memo_cache::logger::DebugCacheLogger>,
) {
    cache.reset_logger();
    let bt = arena.alloc(BigThing { val: 5 });
    // Distinct x values plus repeat of first x with same ref.
    let c1 = clone_and_ref(cache, 7, bt); // 12
    let c2 = clone_and_ref(cache, 8, bt); // 13
    let c3 = clone_and_ref(cache, 7, bt); // 12 again
    assert_eq!(*c1, 12);
    assert_eq!(*c2, 13);
    assert_eq!(*c3, 12);

    let report = cache.report();
    expect_test::expect![[r#"
        MISS: u32
        MISS: u32
        HIT: u32"#]]
    .assert_eq(&report);
}

fn test_nested_memo<'a>(cache: &'a memo_cache::Cache<'a, memo_cache::logger::DebugCacheLogger>) {
    cache.reset_logger();
    // Test a function that returns a reference to another memoized function's result
    let n1 = nested_memo(cache, 10); // calls add_clonearg(cache, 11) -> 22
    let n2 = nested_memo(cache, 15); // calls add_clonearg(cache, 16) -> 32
    let n3 = nested_memo(cache, 10); // should hit cache for nested_memo(10)
    let n4 = add_clonearg(cache, 11); // should hit cache for add_clonearg(11)

    expect_test::expect![[r#"
        NoCopyThing {
            data: [
                22,
            ],
            id: 11,
        }"#]]
    .assert_eq(&format!("{:#?}", *n1));

    expect_test::expect![[r#"
        NoCopyThing {
            data: [
                32,
            ],
            id: 16,
        }"#]]
    .assert_eq(&format!("{:#?}", *n2));

    expect_test::expect![[r#"
        NoCopyThing {
            data: [
                22,
            ],
            id: 11,
        }"#]]
    .assert_eq(&format!("{:#?}", *n3));

    expect_test::expect![[r#"
        NoCopyThing {
            data: [
                22,
            ],
            id: 11,
        }"#]]
    .assert_eq(&format!("{:#?}", *n4));

    let report = cache.report();
    // Should see misses for nested_memo and add_clonearg, then hits
    expect_test::expect![[r#"
        MISS: memo_runtime::NoCopyThing
        MISS: memo_runtime::NoCopyThing
        MISS: memo_runtime::NoCopyThing
        MISS: memo_runtime::NoCopyThing
        HIT: memo_runtime::NoCopyThing
        HIT: memo_runtime::NoCopyThing"#]]
    .assert_eq(&report);
}

#[test]
fn memo_runtime() {
    let arena = bumpalo::Bump::new();
    let cache: memo_cache::Cache<memo_cache::logger::DebugCacheLogger> =
        memo_cache::Cache::new(&arena);
    test_add_clonearg(&cache);
    // Additional scenarios:
    test_ref_only(&arena, &cache);
    test_clone_and_ref(&arena, &cache);
    test_nested_memo(&cache);
}
