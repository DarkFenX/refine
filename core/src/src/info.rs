use crate::ad::AData;

/// Exposes info about data source: how it was made, and what warnings were encountered while making
/// it.
#[derive(Clone)]
pub struct SrcInfo {
    pub origin: SrcOrigin,
    pub warnings: SrcWarnings,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Origin
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Clone)]
pub enum SrcOrigin {
    Generated(SrcOriginGenReason),
    Cached(SrcOriginCached),
}

#[derive(Clone)]
pub enum SrcOriginGenReason {
    NoCacher,
    NoEveDataVersion(String),
    NoCachedFingerprint(String),
    FingerprintMismatch(SrcOriginGenFpMismatch),
    CacheLoadFailed(String),
}

#[derive(Clone)]
pub struct SrcOriginGenFpMismatch {
    pub needed: String,
    pub cached: String,
}

#[derive(Clone)]
pub struct SrcOriginCached {
    pub fingerprint: String,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Warnings
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Clone)]
pub struct SrcWarnings {
    pub eve_data_fetch: Vec<String>,
    pub adg_pk_duplicates: Vec<String>,
    pub adg_cleanup: Vec<String>,
    pub adg_validation: Vec<String>,
    pub adg_conversion_main: Vec<String>,
    pub adg_customization: Vec<String>,
    pub adg_conversion_aux: Vec<String>,
    pub cache_write: Option<String>,
}
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
