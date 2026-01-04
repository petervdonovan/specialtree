use crate::Cache;

thread_local! {
    static THREAD_LOCAL_CACHE: &'static Cache<'static> = {
        // Intentionally leak the bump and cache so stored values can have a thread-long
        // allocation lifetime (suitable for returning `&'static` references).
        let bump: &'static bumpalo::Bump = Box::leak(Box::new(bumpalo::Bump::new()));
        let cache: &'static Cache<'static> = Box::leak(Box::new(Cache::new(bump)));
        cache
    };
}

/// Get a reference to the per-thread global cache.
pub fn thread_local_cache() -> &'static Cache<'static> {
    THREAD_LOCAL_CACHE.with(|c| *c)
}
