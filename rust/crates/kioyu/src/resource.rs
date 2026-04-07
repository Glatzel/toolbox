use hashbrown::HashMap;

pub type ResourceKey = &'static str;
#[derive(Debug, PartialEq, thiserror::Error)]
pub enum ResourceError {
    // Resource errors
    #[error("resource already registered: {0}")]
    AlreadyRegistered(ResourceKey),

    #[error("resource not found: {0}")]
    NotFound(ResourceKey),

    #[error("insufficient capacity for: {0}")]
    InsufficientCapacity(ResourceKey),

    #[error("free would underflow resource: {0}")]
    Underflow(ResourceKey),
}

#[derive(Debug, Clone, PartialEq)]
struct ResourceEntry {
    capacity: usize,
    used: usize,
}

impl ResourceEntry {
    fn available(&self) -> usize { self.capacity.saturating_sub(self.used) }
}

#[derive(Debug, Default, PartialEq)]
pub struct ResourcePool {
    resources: HashMap<ResourceKey, ResourceEntry>,
}

impl ResourcePool {
    pub fn new() -> Self { Self::default() }

    /// Register a new resource key with a given capacity.
    /// Returns `Err` if the key is already registered.
    pub fn register(
        &mut self,
        key: ResourceKey,
        capacity: usize,
    ) -> Result<&mut Self, ResourceError> {
        if self.resources.contains_key(key) {
            return Err(ResourceError::AlreadyRegistered(key));
        }
        self.resources
            .insert(key, ResourceEntry { capacity, used: 0 });
        Ok(self)
    }

    /// Returns available units for a key, or `Err` if the key is not
    /// registered.
    pub(crate) fn available(&self, key: ResourceKey) -> Result<usize, ResourceError> {
        self.resources
            .get(key)
            .map(|r| r.available())
            .ok_or(ResourceError::NotFound(key))
    }

    /// Atomically allocates all requested resources.
    /// Returns `Ok(true)` on success, `Ok(false)` if any resource has
    /// insufficient capacity, or `Err` if any key is not registered.
    #[must_use = "check whether allocation succeeded"]
    pub(crate) fn allocate(&mut self, req: &[(ResourceKey, usize)]) -> Result<bool, ResourceError> {
        // Validate everything before mutating — keeps the operation atomic.
        for &(k, v) in req {
            let avail = self.available(k)?;
            if avail < v {
                clerk::info!(
                    "insufficient capacity for resource '{}': need {}, have {}",
                    k,
                    v,
                    avail
                );
                return Ok(false);
            }
        }
        for &(k, v) in req {
            if let Some(entry) = self.resources.get_mut(k) {
                entry.used += v;
            }
        }
        Ok(true)
    }

    /// Frees previously allocated resources.
    /// Returns `Err` if a key is not found or if freeing would underflow.
    /// Validates all entries before mutating — the operation is all-or-nothing.
    pub(crate) fn free(&mut self, req: &[(ResourceKey, usize)]) -> Result<(), ResourceError> {
        // Validate before mutating.
        for &(k, v) in req {
            let entry = self.resources.get(k).ok_or(ResourceError::NotFound(k))?;
            if v > entry.used {
                return Err(ResourceError::Underflow(k));
            }
        }
        for &(k, v) in req {
            if let Some(entry) = self.resources.get_mut(k) {
                entry.used -= v;
            }
        }
        Ok(())
    }

    /// Returns `(used, capacity)` for a key.
    pub fn utilization(&self, key: ResourceKey) -> Result<(usize, usize), ResourceError> {
        self.resources
            .get(key)
            .map(|e| (e.used, e.capacity))
            .ok_or(ResourceError::NotFound(key))
    }

    /// Returns an iterator over all keys and their `(used, capacity)`.
    pub fn iter(&self) -> impl Iterator<Item = (ResourceKey, usize, usize)> + '_ {
        self.resources.iter().map(|(&k, e)| (k, e.used, e.capacity))
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    fn pool_with(keys: &[(&'static str, usize)]) -> ResourcePool {
        let mut pool = ResourcePool::new();
        for &(k, cap) in keys {
            pool.register(k, cap).unwrap();
        }
        pool
    }

    // --- register ---

    #[test]
    fn register_ok() {
        let mut pool = ResourcePool::new();
        assert!(pool.register("cpu", 8).is_ok());
    }

    #[test]
    fn register_duplicate_errors() {
        let mut pool = pool_with(&[("cpu", 8)]);
        assert_eq!(
            pool.register("cpu", 4),
            Err(ResourceError::AlreadyRegistered("cpu"))
        );
    }

    // --- available ---

    #[test]
    fn available_full_capacity_initially() {
        let pool = pool_with(&[("mem", 1024)]);
        assert_eq!(pool.available("mem"), Ok(1024));
    }

    #[test]
    fn available_unknown_key_errors() {
        let pool = ResourcePool::new();
        assert_eq!(pool.available("gpu"), Err(ResourceError::NotFound("gpu")));
    }

    // --- allocate ---

    #[test]
    fn allocate_reduces_available() {
        let mut pool = pool_with(&[("cpu", 8)]);
        assert_eq!(pool.allocate(&[("cpu", 3)]), Ok(true));
        assert_eq!(pool.available("cpu"), Ok(5));
    }

    #[test]
    fn allocate_returns_false_when_insufficient() {
        let mut pool = pool_with(&[("cpu", 2)]);
        assert_eq!(pool.allocate(&[("cpu", 4)]), Ok(false));
        // No mutation occurred.
        assert_eq!(pool.available("cpu"), Ok(2));
    }

    #[test]
    fn allocate_is_atomic_on_partial_failure() {
        let mut pool = pool_with(&[("cpu", 8), ("mem", 1)]);
        // "mem" will fail; "cpu" must not be mutated.
        let result = pool.allocate(&[("cpu", 4), ("mem", 4)]);
        assert_eq!(result, Ok(false));
        assert_eq!(pool.available("cpu"), Ok(8));
        assert_eq!(pool.available("mem"), Ok(1));
    }

    #[test]
    fn allocate_unknown_key_errors_without_mutation() {
        let mut pool = pool_with(&[("cpu", 8)]);
        let result = pool.allocate(&[("cpu", 2), ("ghost", 1)]);
        assert_eq!(result, Err(ResourceError::NotFound("ghost")));
        assert_eq!(pool.available("cpu"), Ok(8));
    }

    #[test]
    fn allocate_exact_capacity_succeeds() {
        let mut pool = pool_with(&[("cpu", 4)]);
        assert_eq!(pool.allocate(&[("cpu", 4)]), Ok(true));
        assert_eq!(pool.available("cpu"), Ok(0));
    }

    // --- free ---

    #[test]
    fn free_restores_available() {
        let mut pool = pool_with(&[("cpu", 8)]);
        pool.allocate(&[("cpu", 4)]).unwrap();
        pool.free(&[("cpu", 4)]).unwrap();
        assert_eq!(pool.available("cpu"), Ok(8));
    }

    #[test]
    fn free_underflow_errors() {
        let mut pool = pool_with(&[("cpu", 8)]);
        pool.allocate(&[("cpu", 2)]).unwrap();
        assert_eq!(
            pool.free(&[("cpu", 4)]),
            Err(ResourceError::Underflow("cpu"))
        );
        // No mutation occurred.
        assert_eq!(pool.available("cpu"), Ok(6));
    }

    #[test]
    fn free_unknown_key_errors() {
        let mut pool = ResourcePool::new();
        assert_eq!(
            pool.free(&[("gpu", 1)]),
            Err(ResourceError::NotFound("gpu"))
        );
    }

    #[test]
    fn free_is_atomic_on_partial_failure() {
        let mut pool = pool_with(&[("cpu", 8), ("mem", 4)]);
        pool.allocate(&[("cpu", 4), ("mem", 2)]).unwrap();
        // "mem" free would underflow; "cpu" must not be mutated.
        let result = pool.free(&[("cpu", 2), ("mem", 4)]);
        assert_eq!(result, Err(ResourceError::Underflow("mem")));
        assert_eq!(pool.available("cpu"), Ok(4));
        assert_eq!(pool.available("mem"), Ok(2));
    }

    // --- utilization ---

    #[test]
    fn utilization_reflects_allocation() {
        let mut pool = pool_with(&[("cpu", 8)]);
        pool.allocate(&[("cpu", 3)]).unwrap();
        assert_eq!(pool.utilization("cpu"), Ok((3, 8)));
    }

    #[test]
    fn utilization_unknown_key_errors() {
        let pool = ResourcePool::new();
        assert_eq!(pool.utilization("cpu"), Err(ResourceError::NotFound("cpu")));
    }

    // --- iter ---

    #[test]
    fn iter_yields_all_keys() {
        let mut pool = pool_with(&[("cpu", 8), ("mem", 1024)]);
        pool.allocate(&[("cpu", 2)]).unwrap();

        let mut entries: Vec<_> = pool.iter().collect();
        entries.sort_by_key(|&(k, _, _)| k);

        assert_eq!(entries, vec![("cpu", 2, 8), ("mem", 0, 1024)]);
    }

    #[test]
    fn iter_empty_pool() {
        let pool = ResourcePool::new();
        assert_eq!(pool.iter().count(), 0);
    }
}
