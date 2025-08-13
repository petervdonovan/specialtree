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

pub struct Cache<'arena> {
    arena: &'arena bumpalo::Bump,
    cache: RefCell<std::collections::HashMap<MemoCacheKey, &'arena dyn Any>>,
}

impl<'arena> Cache<'arena> {
    pub fn new(arena: &'arena bumpalo::Bump) -> Self {
        Cache {
            arena,
            cache: RefCell::new(Default::default()),
        }
    }

    pub fn lookup<T: 'static>(&self, key: &MemoCacheKey) -> Option<&T> {
        self.cache
            .borrow()
            .get(key)
            .and_then(|value| value.downcast_ref::<T>())
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
}
