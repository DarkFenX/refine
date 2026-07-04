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
    pub cache_write: Option<String>,
}
impl SrcWarnings {
    pub(super) fn new() -> Self {
        Self { cache_write: None }
    }
}
