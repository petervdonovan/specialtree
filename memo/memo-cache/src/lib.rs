pub use sha2;

pub mod logger;

use std::any::Any;
use std::cell::RefCell;
use std::hash::{Hash, Hasher};

use crate::logger::{MemoCacheLogger, NoopCacheLogger};

type DynEqChecker = dyn Fn(&dyn Any) -> bool;

pub struct MemoCacheKey {
    bak: Box<dyn Any>,
    eq: Box<DynEqChecker>,
    hash: u64,
    fingerprint: u128,
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
    cache: RefCell<std::collections::HashMap<MemoCacheKey, *const ()>>,
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

    pub fn lookup<T: 'arena>(&self, key: &MemoCacheKey) -> Option<&'arena T> {
        let result = self.cache.borrow().get(key).map(|result| {
            let casted = result.cast::<T>();
            unsafe { &*casted }
        });

        if result.is_some() {
            self.logger.borrow_mut().hit::<T>();
        } else {
            self.logger.borrow_mut().miss::<T>();
        }
        result
    }

    pub fn insert<T: 'arena>(&self, key: MemoCacheKey, value: T) -> &'arena T {
        let arena_value = self.arena.alloc(value);
        self.cache
            .borrow_mut()
            .insert(key, (arena_value as *const T) as *const ());
        arena_value
    }

    /// Insert a reference result directly (no extra allocation) when the caller already has
    /// a reference with the arena lifetime.
    pub fn insert_ref<T: 'arena>(&self, key: MemoCacheKey, value: &'arena T) -> &'arena T {
        self.cache
            .borrow_mut()
            .insert(key, (value as *const T) as *const ());
        value
    }

    /// Get the cache logger's report
    pub fn report(&self) -> String {
        self.logger.borrow().report()
    }

    /// Reset the cache logger data
    pub fn reset_logger(&self) {
        self.logger.borrow_mut().reset();
    }
}
