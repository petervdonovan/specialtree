use memo_with_references::memo;

// This test demonstrates a memory error when impl block generics are used
// but not captured in the function's own generic parameters
struct Container<T> {
    value: std::marker::PhantomData<T>,
}

impl<T: Default + 'static> Container<T> {
    // This function uses the impl block's generic T but doesn't declare it itself
    // This should cause a memory error because different T instantiations
    // will have the same cache key, leading to type confusion
    #[memo('a)]
    fn get_cloned<'a>() -> T {
        T::default()
    }
}

#[test]
fn test_impl_block_generic_memory_error() {
    let arena = bumpalo::Bump::new();
    let cache: memo_cache::Cache<memo_cache::logger::DebugCacheLogger> =
        memo_cache::Cache::new(&arena);

    let result_int = Container::<u32>::get_cloned(&cache); // Returns u32
    let result_vec = Container::<Vec<u32>>::get_cloned(&cache); // Should return Vec<u32> but gets u32 memory

    // This should now work correctly - different cache keys for different types
    assert_eq!(*result_int, 0u32); // u32::default() = 0
    assert_eq!(*result_vec, Vec::<u32>::new()); // Vec::default() = empty vec
}
