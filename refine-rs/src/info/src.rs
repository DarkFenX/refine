use crate::src::{SrcAlias, SrcInfoMode};

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Clone)]
pub struct SrcInfo {
    pub alias: SrcAlias,
    /// Time when the request to create the source was received, not when it has been completed.
    #[cfg_attr(feature = "serde", serde(serialize_with = "custom_serde::format_time"))]
    pub time_created: time::UtcDateTime,
    pub origin: SrcOrigin,
    #[cfg_attr(feature = "serde", serde(flatten, skip_serializing_if = "Option::is_none"))]
    pub extended: Option<SrcInfoExt>,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Clone)]
pub struct SrcInfoExt {
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "rc::src::SrcWarnings::is_empty"))]
    pub warnings: rc::src::SrcWarnings,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(tag = "type", rename_all = "snake_case"))]
#[derive(Clone)]
pub enum SrcOrigin {
    Generated { reason: SrcOriginGeneratedReason },
    Cached { fingerprint: String },
}

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(tag = "type", rename_all = "snake_case"))]
#[derive(Clone)]
pub enum SrcOriginGeneratedReason {
    NoCacher,
    NoEveDataVersion { message: String },
    NoCachedFingerprint { message: String },
    FingerprintMismatch { message: String },
    CacheLoadFailed { message: String },
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl SrcInfo {
    pub(crate) fn from_alias_and_core(
        alias: SrcAlias,
        time_created: time::UtcDateTime,
        core_info: &rc::src::SrcInfo,
        src_mode: SrcInfoMode,
    ) -> Self {
        Self {
            alias,
            time_created,
            origin: SrcOrigin::from_core(&core_info.origin),
            extended: match src_mode {
                SrcInfoMode::Partial => None,
                SrcInfoMode::Full => Some(SrcInfoExt {
                    warnings: core_info.warnings.clone(),
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

////////////////////////////////////////////////////////////////////////////////////////////////////
// Custom de/serialization
////////////////////////////////////////////////////////////////////////////////////////////////////
#[cfg(feature = "serde")]
mod custom_serde {
    use serde::ser::{Error, Serializer};
    use time::{format_description::FormatDescriptionV3, macros::format_description};

    const TIME_FORMAT: FormatDescriptionV3<'_> = format_description!(
        version = 3,
        "[year]-[month]-[day] [hour]:[minute]:[second].[subsecond digits:3]"
    );

    pub(super) fn format_time<S>(time: &time::UtcDateTime, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let formatted = time.format(&TIME_FORMAT).map_err(S::Error::custom)?;
        serializer.serialize_str(&formatted)
    }
}
