use crate::ad::AData;

/// Info about data source: how it was made, and what warnings were encountered while making it.
pub struct SrcInfo {
    pub origin: SrcOrigin,
    pub warnings: SrcWarnings,
}

/// Was data source read from cache or generated from passed EVE data.
pub enum SrcOrigin {
    /// Data was generated from scratch, with a reason why.
    Generated(SrcOriginGeneratedReason),
    /// Cached data was used, with fingerprint string.
    Cached(String),
}

/// Reason why data was generated from scratch.
pub enum SrcOriginGeneratedReason {
    NoCacher,
    /// EVE data handler did not return EVE data version, with error message which was returned.
    NoEveDataVersion(String),
    /// Adapted data cacher did not return cached data fingerprint, with error message which was
    /// returned.
    NoCachedFingerprint(String),
    /// Needed fingerprint and cached fingerprint were different, with message which mentions both.
    FingerprintMismatch(String),
    /// Adapted data cacher was unable to read cache data, with error message it returned.
    CacheLoadFailed(String),
}

pub struct SrcWarnings {
    /// Warnings recorded by EVE data handler.
    pub eve_data_fetch: Vec<String>,
    /// Info on removed data due to primary key collisions.
    pub adg_pk_duplicates: Vec<String>,
    /// Cleanup stats.
    ///
    /// Those are purely informative, real data is expected to have lots of cleaned entries.
    pub adg_cleanup: Vec<String>,
    /// Warnings encountered during data validation.
    pub adg_validation: Vec<String>,
    /// Warnings encountered during data conversion.
    pub adg_conversion_main: Vec<String>,
    /// Warnings encountered during data customization.
    pub adg_customization: Vec<String>,
    /// Warnings encountered during post-customization data conversion.
    pub adg_conversion_aux: Vec<String>,
    /// Errors recorded by adapted data cacher during writing.
    ///
    /// Those errors are not fatal for functioning of the lib, so might consider those as warnings.
    pub cache_write: Option<String>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl SrcWarnings {
    pub(super) fn from_adapted_warnings(a_data: &mut AData) -> Self {
        Self {
            eve_data_fetch: std::mem::take(&mut a_data.warnings.data_fetch),
            adg_pk_duplicates: std::mem::take(&mut a_data.warnings.pk_duplicates),
            adg_cleanup: std::mem::take(&mut a_data.warnings.cleanup),
            adg_validation: std::mem::take(&mut a_data.warnings.validation),
            adg_conversion_main: std::mem::take(&mut a_data.warnings.conversion_main),
            adg_customization: std::mem::take(&mut a_data.warnings.customization),
            adg_conversion_aux: std::mem::take(&mut a_data.warnings.conversion_aux),
            cache_write: None,
        }
    }
}
