use crate::src::{SrcAlias, SrcInfoMode, SrcInfoModes, SrcOrigin, SrcWarnings};

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
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "SrcWarnings::is_empty"))]
    pub warnings: SrcWarnings,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl SrcInfo {
    pub(crate) fn from_alias_and_core(
        alias: SrcAlias,
        time_created: time::UtcDateTime,
        core_info: &rc::src::SrcInfo,
        modes: SrcInfoModes,
    ) -> Self {
        Self {
            alias,
            time_created,
            origin: core_info.origin.clone(),
            extended: match modes.src {
                SrcInfoMode::Partial => None,
                SrcInfoMode::Full => Some(SrcInfoExt {
                    warnings: core_info.warnings.clone(),
                }),
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
