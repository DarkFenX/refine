use crate::{
    SrcAlias,
    src::{SrcOrigin, SrcWarnings},
    svc::SrcInnerGuarded,
};

#[cfg_attr(feature = "serde", derive(serde::Deserialize), serde(rename_all = "snake_case"))]
#[derive(Copy, Clone, Default)]
pub enum SrcInfoMode {
    #[default]
    Alias,
    Partial,
    Full,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Clone)]
pub struct SrcInfo {
    pub alias: SrcAlias,
    #[cfg_attr(feature = "serde", serde(flatten, skip_serializing_if = "Option::is_none"))]
    pub extended: Option<SrcInfoExt>,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Clone)]
pub struct SrcInfoExt {
    /// Time when the request to create the source was received, not when it has been completed.
    #[cfg_attr(feature = "serde", serde(serialize_with = "custom_serde::format_time"))]
    pub time_created: time::UtcDateTime,
    pub origin: SrcOrigin,
    pub default: bool,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "SrcWarnings::is_empty"))]
    pub warnings: SrcWarnings,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl SrcInfo {
    pub(crate) fn from_inner(inner_src: &SrcInnerGuarded, is_default: bool, src_info_mode: SrcInfoMode) -> Self {
        Self {
            alias: inner_src.get_alias(),
            extended: match src_info_mode {
                SrcInfoMode::Alias => None,
                SrcInfoMode::Partial | SrcInfoMode::Full => {
                    let core_info = inner_src.get_core().get_info();
                    Some(SrcInfoExt {
                        time_created: inner_src.get_time_created(),
                        origin: core_info.origin.clone(),
                        default: is_default,
                        warnings: match src_info_mode {
                            SrcInfoMode::Alias | SrcInfoMode::Partial => SrcWarnings::default(),
                            SrcInfoMode::Full => core_info.warnings.clone(),
                        },
                    })
                }
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
