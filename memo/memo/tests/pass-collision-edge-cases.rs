use memo_with_references::memo;

// Test the core collision case: same signature, same parameter types, different generics
#[memo('a)]
fn same_signature_different_generics<
    'a,
    T: Clone + std::fmt::Debug + 'static + Eq + std::hash::Hash,
>(
    x: u32,
) -> u32 {
    // Even though T is not used in the signature, different T should create different cache keys
    let _type_marker: std::marker::PhantomData<T> = std::marker::PhantomData;
    x * 2
}

#[test]
fn test_same_signature_different_generics() {
    let arena = bumpalo::Bump::new();
    let cache: memo_cache::Cache<memo_cache::logger::DebugCacheLogger> =
        memo_cache::Cache::new(&arena);

    // Identical parameters (u32) and return type (u32), but different generic type T
    let result_with_string = same_signature_different_generics::<String>(&cache, 42);
    let result_with_i32 = same_signature_different_generics::<i32>(&cache, 42);

    assert_eq!(result_with_string, 84);
    assert_eq!(result_with_i32, 84);

    let report = cache.report();
    let miss_count = report.matches("MISS").count();
    // Should have 2 different monomorphizations for String and i32
    assert_eq!(
        miss_count, 2,
        "Expected 2 cache misses for different type instantiations with same signature"
    );
}

fn main() {}
