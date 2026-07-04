#[derive(Clone)]
pub struct SrcInfo {
    pub origin: SrcOrigin,
    pub warnings: bool,
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
