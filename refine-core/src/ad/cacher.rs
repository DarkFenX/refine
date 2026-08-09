use std::fmt;

use crate::ad::{AData, AFingerprint};

/// Adapted data cacher.
///
/// Convenience wrapper to hide boxing necessary to house a cacher implementation.
pub struct AdaptedDataCacher(pub Box<dyn AdaptedDataCacherInterface>);
impl AdaptedDataCacher {
    pub fn new(cacher: impl AdaptedDataCacherInterface + 'static) -> Self {
        Self(Box::new(cacher))
    }
    pub(crate) fn get_impl(&self) -> &dyn AdaptedDataCacherInterface {
        self.0.as_ref()
    }
}
impl<T: AdaptedDataCacherInterface + 'static> From<T> for AdaptedDataCacher {
    fn from(cacher: T) -> Self {
        Self::new(cacher)
    }
}
impl fmt::Debug for AdaptedDataCacher {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.0, f)
    }
}

/// Adapted data cacher interface definition.
///
/// Caching helps to avoid regeneration of adapted data on every run, which is a relatively
/// expensive process.
pub trait AdaptedDataCacherInterface: fmt::Debug + Send + Sync {
    /// Get cached data fingerprint.
    fn get_cache_fingerprint(&self) -> Result<AFingerprint, AdaptedDataCacherError>;
    /// Load cache from persistent storage.
    fn load_from_cache(&self) -> Result<AData, AdaptedDataCacherError>;
    /// Store passed data in cache.
    fn write_cache(&self, data: &AData, fingerprint: AFingerprint) -> Result<(), AdaptedDataCacherError>;
    /// Get adapted data cacher version.
    ///
    /// Change in version triggers adapted data cache rebuild, even if source data and core library
    /// version stayed the same.
    fn get_cacher_version(&self) -> String;
}

#[derive(Debug, thiserror::Error)]
#[error(transparent)]
pub struct AdaptedDataCacherError(pub Box<dyn std::error::Error + Send + Sync>);
impl AdaptedDataCacherError {
    pub fn new(error: impl std::error::Error + Send + Sync + 'static) -> Self {
        Self(Box::new(error))
    }
}
