use crate::ad::{AData, AFingerprint};

/// Adapted data cacher interface definition.
///
/// Caching helps to avoid regeneration of adapted data on every run, which is a very expensive
/// process.
///
/// Any methods which read/write data have mutable self in signature to allow implementations of
/// data handlers which store data on themselves.
pub trait AdaptedDataCacher: std::fmt::Debug + Send + Sync {
    /// Get cached data fingerprint.
    fn get_cache_fingerprint(&mut self) -> Result<AFingerprint, Box<dyn std::error::Error>>;
    /// Load cache from persistent storage.
    fn load_from_cache(&mut self) -> Result<AData, Box<dyn std::error::Error>>;
    /// Store passed data in cache.
    fn write_cache(&mut self, data: &AData, fingerprint: AFingerprint) -> Result<(), Box<dyn std::error::Error>>;
    /// Get adapted data cacher version.
    ///
    /// Change in version triggers adapted data cache rebuild, even if source data and core library
    /// version stayed the same.
    fn get_cacher_version(&self) -> String;
}
