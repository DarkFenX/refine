use std::fmt;

use crate::ad::{AData, AFingerprint};

/// Adapted data cacher.
///
/// Convenience wrapper to hide boxing necessary to house a cacher implementation.
pub struct AdaptedDataCacher(pub Box<dyn AdaptedDataCacherCore>);
impl AdaptedDataCacher {
    pub fn new<T>(cacher: T) -> Self
    where
        T: AdaptedDataCacherCore + 'static,
    {
        Self(Box::new(cacher))
    }
    pub(crate) fn get_impl(&self) -> &dyn AdaptedDataCacherCore {
        self.0.as_ref()
    }
}
impl<T> From<T> for AdaptedDataCacher
where
    T: AdaptedDataCacherCore + 'static,
{
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
pub trait AdaptedDataCacherCore: fmt::Debug + Send + Sync {
    /// Load cache with passed fingerprint from persistent storage.
    fn load_from_cache(&self, fingerprint: &AFingerprint) -> Result<AData, AdcReadError>;
    /// Store passed data in cache.
    fn write_cache(&self, data: &AData, fingerprint: AFingerprint) -> Result<(), AdcWriteError>;
    /// Get adapted data cacher version.
    ///
    /// Change in version triggers adapted data cache rebuild, even if source data and core library
    /// version stayed the same.
    fn get_cacher_version(&self) -> String;
}

#[derive(Debug, thiserror::Error)]
pub enum AdcReadError {
    #[error("unable to read cached fingerprint")]
    FingerprintRead(#[source] Box<dyn std::error::Error + Send + Sync>),
    #[error("cached fingerprint {0} is different")]
    FingerprintMismatch(AFingerprint),
    #[error("unable to read cached data")]
    DataRead(#[source] Box<dyn std::error::Error + Send + Sync>),
}

#[derive(Debug, thiserror::Error)]
#[error(transparent)]
pub struct AdcWriteError(pub Box<dyn std::error::Error + Send + Sync>);
