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
    pub(super) fn new() -> Self {
        Self {
            eve_data_fetch: Vec::new(),
            adg_pk_duplicates: Vec::new(),
            adg_cleanup: Vec::new(),
            adg_validation: Vec::new(),
            adg_conversion_main: Vec::new(),
            adg_customization: Vec::new(),
            adg_conversion_aux: Vec::new(),
            cache_write: None,
        }
    }
}
