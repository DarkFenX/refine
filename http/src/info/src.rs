use serde::Serialize;

use crate::info::HSrcInfoMode;

#[derive(Serialize)]
pub(crate) struct HSrcInfo {
    origin: HSrcOrigin,
    #[serde(skip_serializing_if = "HSrcWarnings::is_empty")]
    warnings: HSrcWarnings,
}

#[derive(Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
enum HSrcOrigin {
    Generated { reason: HSrcOriginGeneratedReason },
    Cached { fingerprint: String },
}

#[derive(Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
enum HSrcOriginGeneratedReason {
    NoCacher,
    NoEveDataVersion { message: String },
    NoCachedFingerprint { message: String },
    FingerprintMismatch { message: String },
    CacheLoadFailed { message: String },
}

#[derive(Default, Serialize)]
struct HSrcWarnings {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    eve_data_fetch: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    adg_pk_duplicates: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    adg_cleanup: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    adg_validation: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    adg_conversion_main: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    adg_customization: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    adg_conversion_aux: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cache_write: Option<String>,
}
impl HSrcWarnings {
    fn is_empty(&self) -> bool {
        self.eve_data_fetch.is_empty()
            && self.adg_pk_duplicates.is_empty()
            && self.adg_cleanup.is_empty()
            && self.adg_validation.is_empty()
            && self.adg_conversion_main.is_empty()
            && self.adg_customization.is_empty()
            && self.adg_conversion_aux.is_empty()
            && self.cache_write.is_none()
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HSrcInfo {
    pub(crate) fn from_core(core_info: &rc::src::SrcInfo, src_mode: HSrcInfoMode) -> Self {
        Self {
            origin: HSrcOrigin::from_core(&core_info.origin),
            warnings: match src_mode {
                HSrcInfoMode::Partial => HSrcWarnings::default(),
                HSrcInfoMode::Full => HSrcWarnings::from_core(&core_info.warnings),
            },
        }
    }
}

impl HSrcOrigin {
    fn from_core(core_origin: &rc::src::SrcOrigin) -> Self {
        match core_origin {
            rc::src::SrcOrigin::Generated(core_reason) => Self::Generated {
                reason: HSrcOriginGeneratedReason::from_core(core_reason),
            },
            rc::src::SrcOrigin::Cached(fingerprint) => Self::Cached {
                fingerprint: fingerprint.to_string(),
            },
        }
    }
}

impl HSrcOriginGeneratedReason {
    fn from_core(core_reason: &rc::src::SrcOriginGeneratedReason) -> Self {
        match core_reason {
            rc::src::SrcOriginGeneratedReason::NoCacher => Self::NoCacher,
            rc::src::SrcOriginGeneratedReason::NoEveDataVersion(message) => Self::NoEveDataVersion {
                message: message.to_string(),
            },
            rc::src::SrcOriginGeneratedReason::NoCachedFingerprint(message) => Self::NoCachedFingerprint {
                message: message.to_string(),
            },
            rc::src::SrcOriginGeneratedReason::FingerprintMismatch(message) => Self::FingerprintMismatch {
                message: message.to_string(),
            },
            rc::src::SrcOriginGeneratedReason::CacheLoadFailed(message) => Self::CacheLoadFailed {
                message: message.to_string(),
            },
        }
    }
}

impl HSrcWarnings {
    fn from_core(core_warnings: &rc::src::SrcWarnings) -> Self {
        Self {
            eve_data_fetch: core_warnings.eve_data_fetch.iter().cloned().collect(),
            adg_pk_duplicates: core_warnings.adg_pk_duplicates.iter().cloned().collect(),
            adg_cleanup: core_warnings.adg_cleanup.iter().cloned().collect(),
            adg_validation: core_warnings.adg_validation.iter().cloned().collect(),
            adg_conversion_main: core_warnings.adg_conversion_main.iter().cloned().collect(),
            adg_customization: core_warnings.adg_customization.iter().cloned().collect(),
            adg_conversion_aux: core_warnings.adg_conversion_aux.iter().cloned().collect(),
            cache_write: core_warnings.cache_write.clone(),
        }
    }
}
