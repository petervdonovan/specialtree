use memo_with_references::memo;

struct SimpleContainer {
    value: u32,
}

impl SimpleContainer {
    #[memo('a)]
    fn get_value<'a>(&self) -> u32 {
        self.value
    }
}

#[test]
fn test_self_parameter_works() {
    let arena = bumpalo::Bump::new();
    let cache: memo_cache::Cache<memo_cache::logger::DebugCacheLogger> =
        memo_cache::Cache::new(&arena);

    let container = SimpleContainer { value: 42 };
    let result = container.get_value(&cache);

    assert_eq!(*result, 42u32);
}

fn main() {}
