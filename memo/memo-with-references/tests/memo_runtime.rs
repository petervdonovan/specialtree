use memo_with_references::memo;
use std::sync::atomic::{AtomicUsize, Ordering};

static CALL_COUNT: AtomicUsize = AtomicUsize::new(0);

// Simple non-Clone type we'll pass by reference (frozen argument)
struct BigThing {
    val: u32,
}

// Use a real cache type now (&Cache) even though the macro still doesn't perform memoization.
#[memo(cache)]
fn add_clonearg<'a>(cache: &'a memo_cache::Cache<'a>, x: u32) -> &'a u32 {
    CALL_COUNT.fetch_add(1, Ordering::SeqCst);
    cache.alloc(x * 2)
}

// Function taking only a non-clone (reference) argument.
#[memo(cache)]
fn ref_only<'a>(cache: &'a memo_cache::Cache<'a>, bt: &'a BigThing) -> &'a u32 {
    CALL_COUNT.fetch_add(1, Ordering::SeqCst);
    // Need to return computed value (bt.val * 3) by allocating it so it attains the arena lifetime.
    cache.alloc(bt.val * 3)
}

// Function taking both a cloneable owned arg and a non-clone reference arg.
#[memo(cache)]
fn clone_and_ref<'a>(cache: &'a memo_cache::Cache<'a>, x: u32, bt: &'a BigThing) -> &'a u32 {
    CALL_COUNT.fetch_add(1, Ordering::SeqCst);
    cache.alloc(x + bt.val)
}

const EXPECTED_N_CALLS: usize = 2;

fn test_add_clonearg<'a>(cache: &'a memo_cache::Cache<'a>) {
    CALL_COUNT.store(0, Ordering::SeqCst);
    let a1 = add_clonearg(cache, 21);
    let a2 = add_clonearg(cache, 22);
    let a3 = add_clonearg(cache, 21);

    assert_eq!(*a1, 42);
    assert_eq!(*a2, 44);
    assert_eq!(*a3, 42);

    let n_calls = CALL_COUNT.load(Ordering::SeqCst);
    assert_eq!(
        n_calls, EXPECTED_N_CALLS,
        "memoization not applied; got {n_calls} calls",
    );
}

fn test_ref_only<'a>(arena: &'a bumpalo::Bump, cache: &'a memo_cache::Cache<'a>) {
    CALL_COUNT.store(0, Ordering::SeqCst);
    let bt1 = arena.alloc(BigThing { val: 10 });
    let bt2 = arena.alloc(BigThing { val: 11 });
    // Two distinct references then repeat first.
    let r1 = ref_only(cache, bt1);
    let r2 = ref_only(cache, bt2);
    let r3 = ref_only(cache, bt1);
    assert_eq!(*r1, 30);
    assert_eq!(*r2, 33);
    assert_eq!(*r3, 30);
    let n_calls = CALL_COUNT.load(Ordering::SeqCst);
    // If memoization worked we'd have 2 calls (bt1, bt2). Expect failure (will be 3).
    assert_eq!(n_calls, 2, "ref_only not memoized; got {n_calls} calls");
}

fn test_clone_and_ref<'a>(arena: &'a bumpalo::Bump, cache: &'a memo_cache::Cache<'a>) {
    CALL_COUNT.store(0, Ordering::SeqCst);
    let bt = arena.alloc(BigThing { val: 5 });
    // Distinct x values plus repeat of first x with same ref.
    let c1 = clone_and_ref(cache, 7, bt); // 12
    let c2 = clone_and_ref(cache, 8, bt); // 13
    let c3 = clone_and_ref(cache, 7, bt); // 12 again
    assert_eq!(*c1, 12);
    assert_eq!(*c2, 13);
    assert_eq!(*c3, 12);
    let n_calls = CALL_COUNT.load(Ordering::SeqCst);
    // If memoization worked we'd have 2 calls ( (7,bt) and (8,bt) ). Expect failure (will be 3 ).
    assert_eq!(
        n_calls, 2,
        "clone_and_ref not memoized; got {n_calls} calls"
    );
}

#[test]
fn memo_runtime() {
    let arena = bumpalo::Bump::new();
    let cache = memo_cache::Cache::new(&arena);
    test_add_clonearg(&cache);
    // Additional scenarios:
    test_ref_only(&arena, &cache);
    test_clone_and_ref(&arena, &cache);
}
