use crate::ad::{AData, AResult};

/// Adapted data cacher interface definition.
///
/// Caching helps to avoid regeneration of adapted data on every run, which is very expensive.
pub trait AdaptedDataCacher: std::fmt::Debug + Send + Sync {
    /// Get cached data fingerprint.
    fn get_cache_fingerprint(&mut self) -> Option<String>;
    /// Load cache from persistent storage.
    fn load_from_cache(&mut self) -> AResult<AData>;
    /// Store passed data in cache.
    fn write_cache(&mut self, data: &AData, fingerprint: String);
    /// Get adapted data cacher version.
    ///
    /// Change in version triggers adapted data cache rebuild, even if source data and core library
    /// version stayed the same.
    fn get_handler_version(&self) -> String;
}
