use crate::SrcInfoMode;

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct SrcInfo {
    pub origin: SrcOrigin,
    #[cfg_attr(feature = "serde", serde(flatten, skip_serializing_if = "Option::is_none"))]
    pub extended: Option<SrcInfoExt>,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct SrcInfoExt {
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "SrcWarnings::is_empty"))]
    pub warnings: SrcWarnings,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(tag = "type", rename_all = "snake_case"))]
pub enum SrcOrigin {
    Generated { reason: SrcOriginGeneratedReason },
    Cached { fingerprint: String },
}

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(tag = "type", rename_all = "snake_case"))]
pub enum SrcOriginGeneratedReason {
    NoCacher,
    NoEveDataVersion { message: String },
    NoCachedFingerprint { message: String },
    FingerprintMismatch { message: String },
    CacheLoadFailed { message: String },
}

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct SrcWarnings {
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub eve_data_fetch: Vec<String>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub adg_pk_duplicates: Vec<String>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub adg_cleanup: Vec<String>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub adg_validation: Vec<String>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub adg_conversion_main: Vec<String>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub adg_customization: Vec<String>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub adg_conversion_aux: Vec<String>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub cache_write: Option<String>,
}
#[cfg(feature = "serde")]
impl SrcWarnings {
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
impl SrcInfo {
    pub(crate) fn from_core(core_info: &rc::src::SrcInfo, src_mode: SrcInfoMode) -> Self {
        Self {
            origin: SrcOrigin::from_core(&core_info.origin),
            extended: match src_mode {
                SrcInfoMode::Partial => None,
                SrcInfoMode::Full => Some(SrcInfoExt {
                    warnings: SrcWarnings::from_core(&core_info.warnings),
                }),
            },
        }
    }
}

impl SrcOrigin {
    fn from_core(core_origin: &rc::src::SrcOrigin) -> Self {
        match core_origin {
            rc::src::SrcOrigin::Generated(core_reason) => Self::Generated {
                reason: SrcOriginGeneratedReason::from_core(core_reason),
            },
            rc::src::SrcOrigin::Cached(fingerprint) => Self::Cached {
                fingerprint: fingerprint.to_string(),
            },
        }
    }
}

impl SrcOriginGeneratedReason {
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

impl SrcWarnings {
    fn from_core(core_warnings: &rc::src::SrcWarnings) -> Self {
        Self {
            eve_data_fetch: core_warnings.eve_data_fetch.to_vec(),
            adg_pk_duplicates: core_warnings.adg_pk_duplicates.to_vec(),
            adg_cleanup: core_warnings.adg_cleanup.to_vec(),
            adg_validation: core_warnings.adg_validation.to_vec(),
            adg_conversion_main: core_warnings.adg_conversion_main.to_vec(),
            adg_customization: core_warnings.adg_customization.to_vec(),
            adg_conversion_aux: core_warnings.adg_conversion_aux.to_vec(),
            cache_write: core_warnings.cache_write.clone(),
        }
    }
}
