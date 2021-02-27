/// Options for creating `Engine`
#[derive(Debug, Copy, Clone)]
pub struct EngineOptions {
    /// Delimiter of csv
    pub delimiter: u8,
    /// Indexing buffer, only used at creation. Number of bytes.
    pub buf_capacity: usize,
    /// Capacity for lru cache. Number of items.
    pub cache_cap: usize,
}

impl Default for EngineOptions {
    #[inline]
    fn default() -> Self {
        Self { delimiter: b',', buf_capacity: 10 * 1024 * 1024, cache_cap: 1024 }
    }
}