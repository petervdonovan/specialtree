use memo_with_references::memo;

// Simple non-Clone type we'll pass by reference (frozen argument)
struct BigThing {
    val: u32,
}

// Use a real cache type now (&Cache) even though the macro still doesn't perform memoization.
#[memo('a)]
fn add_clonearg<'a>(x: u32) -> &'a u32 {
    cache.alloc(x * 2)
}

// Function taking only a non-clone (reference) argument.
#[memo('a)]
fn ref_only<'a>(bt: &'a BigThing) -> &'a u32 {
    // Need to return computed value (bt.val * 3) by allocating it so it attains the arena lifetime.
    cache.alloc(bt.val * 3)
}

// Function taking both a cloneable owned arg and a non-clone reference arg.
#[memo('a)]
fn clone_and_ref<'a>(x: u32, bt: &'a BigThing) -> &'a u32 {
    cache.alloc(x + bt.val)
}

fn test_add_clonearg<'a>(cache: &'a memo_cache::Cache<'a, memo_cache::DebugCacheLogger>) {
    cache.reset_logger();
    let a1 = add_clonearg(cache, 21);
    let a2 = add_clonearg(cache, 22);
    let a3 = add_clonearg(cache, 21);

    assert_eq!(*a1, 42);
    assert_eq!(*a2, 44);
    assert_eq!(*a3, 42);

    let report = cache.report();
    // Should have 2 misses for unique values (21, 22) and 1 hit when 21 is repeated
    expect_test::expect![[r#"
        MISS: u32
        MISS: u32
        HIT: u32"#]]
    .assert_eq(&report);
}

fn test_ref_only<'a>(
    arena: &'a bumpalo::Bump,
    cache: &'a memo_cache::Cache<'a, memo_cache::DebugCacheLogger>,
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
    cache: &'a memo_cache::Cache<'a, memo_cache::DebugCacheLogger>,
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

#[test]
fn memo_runtime() {
    let arena = bumpalo::Bump::new();
    let cache: memo_cache::Cache<memo_cache::DebugCacheLogger> = memo_cache::Cache::new(&arena);
    test_add_clonearg(&cache);
    // Additional scenarios:
    test_ref_only(&arena, &cache);
    test_clone_and_ref(&arena, &cache);
}
