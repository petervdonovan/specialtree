/// Logging functionality for memo cache operations
/// Trait for logging cache operations (hits and misses)
pub trait MemoCacheLogger: Default {
    /// Called when a cache hit occurs for type T
    fn hit<T>(&mut self);
    /// Called when a cache miss occurs for type T
    fn miss<T>(&mut self);
    /// Generate a report of cache activity
    fn report(&self) -> String;
    /// Reset/clear all logged data
    fn reset(&mut self);
}

/// A logger that performs no operations (null object pattern)
pub struct NoopCacheLogger;

/// A logger that records all cache events with type information
#[derive(Default)]
pub struct DebugCacheLogger {
    events: Vec<String>,
}

/// A logger that collects statistics about cache hit/miss rates
#[derive(Default)]
pub struct CollectingStatsCacheLogger {
    hits: u32,
    misses: u32,
}

/// A logger that chains two other loggers together
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

#[cfg(test)]
mod tests {
    use crate::{
        Cache, MemoCacheKey,
        logger::{ChainCacheLogger, CollectingStatsCacheLogger, DebugCacheLogger},
    };

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
        let cache: Cache<ChainCacheLogger<DebugCacheLogger, CollectingStatsCacheLogger>> =
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
