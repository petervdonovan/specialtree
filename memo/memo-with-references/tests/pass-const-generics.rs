use memo_with_references::memo;

// Test const generics with identical signatures but different const values
#[memo('a)]
fn const_generic_test<'a, const N: usize>(input: u32) -> u32 {
    // Same signature (u32 -> u32) but different const generic N
    input + N as u32
}

#[test]
fn test_const_generic_collision_prevention() {
    let arena = bumpalo::Bump::new();
    let cache: memo_cache::Cache<memo_cache::logger::DebugCacheLogger> =
        memo_cache::Cache::new(&arena);

    // Identical parameters and return types, but different const generic values
    let result_3 = const_generic_test::<3>(&cache, 10);
    let result_4 = const_generic_test::<4>(&cache, 10);

    assert_eq!(result_3, 13); // 10 + 3
    assert_eq!(result_4, 14); // 10 + 4

    let report = cache.report();
    let miss_count = report.matches("MISS").count();
    assert_eq!(
        miss_count, 2,
        "Expected 2 cache misses for different const generic values with same signature"
    );
}

fn main() {}
