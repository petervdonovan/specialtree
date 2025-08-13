pub use sha2;
use std::any::Any;
use std::cell::RefCell;
use std::hash::{Hash, Hasher};

type DynEqChecker = dyn Fn(&dyn Any) -> bool;

pub struct MemoCacheKey {
    bak: Box<dyn Any>,
    eq: Box<DynEqChecker>,
    hash: u64,
    fingerprint: u128,
}

pub trait MemoCacheLogger: Default {
    fn hit<T>(&mut self);
    fn miss<T>(&mut self);
    fn report(&self) -> String;
    fn reset(&mut self);
}

pub struct NoopCacheLogger;
#[derive(Default)]
pub struct DebugCacheLogger {
    events: Vec<String>,
}
#[derive(Default)]
pub struct CollectingStatsCacheLogger {
    hits: u32,
    misses: u32,
}
pub struct ChainCacheLogger<First, Second> {
    first_logger: First,
    second_logger: Second,
}

impl Default for NoopCacheLogger {
    fn default() -> Self {
        NoopCacheLogger
    }
}

impl MemoCacheLogger for NoopCacheLogger {
    fn hit<T>(&mut self) {
        // No-op
    }

    fn miss<T>(&mut self) {
        // No-op
    }

    fn report(&self) -> String {
        String::new()
    }

    fn reset(&mut self) {
        // No-op
    }
}

impl MemoCacheLogger for DebugCacheLogger {
    fn hit<T>(&mut self) {
        self.events
            .push(format!("HIT: {}", std::any::type_name::<T>()));
    }

    fn miss<T>(&mut self) {
        self.events
            .push(format!("MISS: {}", std::any::type_name::<T>()));
    }

    fn report(&self) -> String {
        self.events.join("\n")
    }

    fn reset(&mut self) {
        self.events.clear();
    }
}

impl MemoCacheLogger for CollectingStatsCacheLogger {
    fn hit<T>(&mut self) {
        self.hits += 1;
    }

    fn miss<T>(&mut self) {
        self.misses += 1;
    }

    fn report(&self) -> String {
        format!(
            "Cache hits: {}, misses: {}, hit rate: {:.1}%",
            self.hits,
            self.misses,
            if self.hits + self.misses > 0 {
                (self.hits as f64 / (self.hits + self.misses) as f64) * 100.0
            } else {
                0.0
            }
        )
    }

    fn reset(&mut self) {
        self.hits = 0;
        self.misses = 0;
    }
}

impl<First: Default, Second: Default> Default for ChainCacheLogger<First, Second> {
    fn default() -> Self {
        ChainCacheLogger {
            first_logger: First::default(),
            second_logger: Second::default(),
        }
    }
}

impl<First: MemoCacheLogger, Second: MemoCacheLogger> MemoCacheLogger
    for ChainCacheLogger<First, Second>
{
    fn hit<T>(&mut self) {
        self.first_logger.hit::<T>();
        self.second_logger.hit::<T>();
    }

    fn miss<T>(&mut self) {
        self.first_logger.miss::<T>();
        self.second_logger.miss::<T>();
    }

    fn report(&self) -> String {
        let first_report = self.first_logger.report();
        let second_report = self.second_logger.report();

        if first_report.is_empty() && second_report.is_empty() {
            String::new()
        } else if first_report.is_empty() {
            second_report
        } else if second_report.is_empty() {
            first_report
        } else {
            format!("{first_report}\n{second_report}")
        }
    }

    fn reset(&mut self) {
        self.first_logger.reset();
        self.second_logger.reset();
    }
}

impl MemoCacheKey {
    pub fn new<T: 'static + Eq + Hash + Clone>(fingerprint: u128, value: T) -> Self {
        let hash = {
            let mut hasher = std::collections::hash_map::DefaultHasher::new();
            value.hash(&mut hasher);
            fingerprint.hash(&mut hasher);
            hasher.finish()
        };

        let value_for_eq = value.clone();
        let bak: Box<dyn Any> = Box::new(value);
        let eq = Box::new(move |other: &dyn Any| {
            other
                .downcast_ref::<T>()
                .is_some_and(|other_typed| &value_for_eq == other_typed)
        });

        Self {
            bak,
            eq,
            hash,
            fingerprint,
        }
    }
}

impl PartialEq for MemoCacheKey {
    fn eq(&self, other: &Self) -> bool {
        if (self.fingerprint != other.fingerprint) | (self.hash != other.hash) {
            return false;
        }
        (self.eq)(other.bak.as_ref())
    }
}

impl Eq for MemoCacheKey {}

impl Hash for MemoCacheKey {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.hash.hash(state);
    }
}

pub struct Cache<'arena, Logger = NoopCacheLogger> {
    arena: &'arena bumpalo::Bump,
    cache: RefCell<std::collections::HashMap<MemoCacheKey, &'arena dyn Any>>,
    logger: RefCell<Logger>,
}

impl<'arena, Logger: MemoCacheLogger> Cache<'arena, Logger> {
    pub fn new(arena: &'arena bumpalo::Bump) -> Self {
        Cache {
            arena,
            cache: Default::default(),
            logger: Default::default(),
        }
    }

    pub fn lookup<T: 'static>(&self, key: &MemoCacheKey) -> Option<&'arena T> {
        let result = self
            .cache
            .borrow()
            .get(key)
            .and_then(|value| value.downcast_ref::<T>());

        if result.is_some() {
            self.logger.borrow_mut().hit::<T>();
        } else {
            self.logger.borrow_mut().miss::<T>();
        }

        result
    }

    pub fn insert<T: 'static>(&self, key: MemoCacheKey, value: T) -> &'arena T {
        let arena_value = self.arena.alloc(value);
        self.cache.borrow_mut().insert(key, arena_value as &dyn Any);
        arena_value
    }

    // Insert a reference result directly (no extra allocation) when the caller already has
    // a reference with the arena lifetime.
    pub fn insert_ref<T: 'static>(&self, key: MemoCacheKey, value: &'arena T) -> &'arena T {
        self.cache.borrow_mut().insert(key, value as &dyn Any);
        value
    }

    // Convenience allocation helper exposed for memoized functions that need to allocate
    // their return values into the arena to satisfy the required lifetime.
    pub fn alloc<T>(&self, value: T) -> &'arena T {
        self.arena.alloc(value)
    }

    // Get the cache logger's report
    pub fn report(&self) -> String {
        self.logger.borrow().report()
    }

    // Reset the cache logger data
    pub fn reset_logger(&self) {
        self.logger.borrow_mut().reset();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_noop_logger() {
        let arena = bumpalo::Bump::new();
        let cache: Cache<NoopCacheLogger> = Cache::new(&arena);

        let key = MemoCacheKey::new(123, 42i32);
        let _ = cache.lookup::<i32>(&key); // miss
        let key2 = MemoCacheKey::new(123, 42i32);
        cache.insert(key2, 42i32);
        let _ = cache.lookup::<i32>(&key); // hit

        // NoopCacheLogger should produce empty report
        let report = cache.report();
        expect_test::expect![[r#""#]].assert_eq(&report);
    }

    #[test]
    fn test_debug_logger() {
        let arena = bumpalo::Bump::new();
        let cache: Cache<DebugCacheLogger> = Cache::new(&arena);

        let key = MemoCacheKey::new(123, 42i32);
        let _ = cache.lookup::<i32>(&key); // miss
        let key2 = MemoCacheKey::new(123, 42i32);
        cache.insert(key2, 42i32);
        let _ = cache.lookup::<i32>(&key); // hit

        let report = cache.report();
        expect_test::expect![[r#"
            MISS: i32
            HIT: i32"#]]
        .assert_eq(&report);
    }

    #[test]
    fn test_stats_logger() {
        let arena = bumpalo::Bump::new();
        let cache: Cache<CollectingStatsCacheLogger> = Cache::new(&arena);

        let key1 = MemoCacheKey::new(123, 42i32);
        let key2 = MemoCacheKey::new(456, 24i32);

        // Two misses
        let _ = cache.lookup::<i32>(&key1);
        let _ = cache.lookup::<i32>(&key2);

        // Insert both (need new keys with same values)
        let key1_insert = MemoCacheKey::new(123, 42i32);
        let key2_insert = MemoCacheKey::new(456, 24i32);
        cache.insert(key1_insert, 42i32);
        cache.insert(key2_insert, 24i32);

        // Two hits
        let _ = cache.lookup::<i32>(&key1);
        let _ = cache.lookup::<i32>(&key2);

        let report = cache.report();
        expect_test::expect![[r#"Cache hits: 2, misses: 2, hit rate: 50.0%"#]].assert_eq(&report);
    }

    #[test]
    fn test_chain_logger_reset() {
        let arena = bumpalo::Bump::new();
        let mut cache: Cache<ChainCacheLogger<DebugCacheLogger, CollectingStatsCacheLogger>> =
            Cache::new(&arena);

        let key = MemoCacheKey::new(123, 42i32);
        let _ = cache.lookup::<i32>(&key); // miss
        let key2 = MemoCacheKey::new(123, 42i32);
        cache.insert(key2, 42i32);
        let _ = cache.lookup::<i32>(&key); // hit

        let report_before = cache.report();
        expect_test::expect![[r#"
            MISS: i32
            HIT: i32
            Cache hits: 1, misses: 1, hit rate: 50.0%"#]]
        .assert_eq(&report_before);

        // Reset and verify both loggers are cleared
        cache.reset_logger();
        let report_after = cache.report();
        expect_test::expect![[r#"Cache hits: 0, misses: 0, hit rate: 0.0%"#]]
            .assert_eq(&report_after);
    }
}
