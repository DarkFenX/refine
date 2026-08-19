/// A stat option which can have extended settings.
#[derive(Clone)]
pub enum StatOptionExt<T> {
    Disabled,
    Enabled,
    EnabledExtended(Vec<T>),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Non-public
////////////////////////////////////////////////////////////////////////////////////////////////////
// Internal counterpart for public options where default field is present, bool version
#[derive(Copy, Clone, Default)]
pub(in crate::stats) enum StatDefOption {
    #[default]
    Default,
    Disabled,
    Enabled,
}

// Internal counterpart for public options where default field is present, StatOptionExt version
#[derive(Clone, Default)]
pub(in crate::stats) enum StatDefOptionExt<T> {
    #[default]
    Default,
    Disabled,
    Enabled,
    EnabledExtended(Vec<T>),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl From<bool> for StatDefOption {
    fn from(enabled: bool) -> Self {
        match enabled {
            true => Self::Enabled,
            false => Self::Disabled,
        }
    }
}

impl<T> From<StatOptionExt<T>> for StatDefOptionExt<T> {
    fn from(option: StatOptionExt<T>) -> Self {
        match option {
            StatOptionExt::Disabled => Self::Disabled,
            StatOptionExt::Enabled => Self::Enabled,
            StatOptionExt::EnabledExtended(inner) => Self::EnabledExtended(inner),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Non-public
////////////////////////////////////////////////////////////////////////////////////////////////////
impl StatDefOption {
    pub(in crate::stats) fn into_enabled(self, default: bool) -> bool {
        match self {
            Self::Default => default,
            Self::Disabled => false,
            Self::Enabled => true,
        }
    }
}

impl<T> StatDefOptionExt<T> {
    pub(in crate::stats) fn into_enabled(self, default: bool) -> Option<Vec<T>>
    where
        T: Default,
    {
        match self {
            Self::Default => match default {
                true => Some(vec![T::default()]),
                false => None,
            },
            Self::Disabled => None,
            Self::Enabled => Some(vec![T::default()]),
            Self::EnabledExtended(options) => Some(options),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Custom de/serialization
////////////////////////////////////////////////////////////////////////////////////////////////////
#[cfg(feature = "serde")]
mod custom_serde {
    use serde::de::{Deserialize, Deserializer, Error, Visitor};

    use super::*;

    impl<'de> Deserialize<'de> for StatDefOption {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            struct VisitorImpl;

            impl<'de> Visitor<'de> for VisitorImpl {
                type Value = StatDefOption;

                fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                    formatter.write_str("bool or null")
                }

                fn visit_unit<E>(self) -> Result<Self::Value, E>
                where
                    E: Error,
                {
                    Ok(Self::Value::Default)
                }
                fn visit_none<E>(self) -> Result<Self::Value, E>
                where
                    E: Error,
                {
                    Ok(Self::Value::Default)
                }
                fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
                where
                    E: Error,
                {
                    Ok(match v {
                        true => Self::Value::Enabled,
                        false => Self::Value::Disabled,
                    })
                }
            }

            deserializer.deserialize_any(VisitorImpl)
        }
    }

    impl<'de, T> Deserialize<'de> for StatDefOptionExt<T>
    where
        T: Deserialize<'de>,
    {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            Ok(match StatDefOptionExtFormats::deserialize(deserializer)? {
                StatDefOptionExtFormats::Simple(StatDefOption::Default) => Self::Default,
                StatDefOptionExtFormats::Simple(StatDefOption::Disabled) => Self::Disabled,
                StatDefOptionExtFormats::Simple(StatDefOption::Enabled) => Self::Enabled,
                StatDefOptionExtFormats::Extended(data) => Self::EnabledExtended(data),
            })
        }
    }

    #[derive(serde::Deserialize)]
    #[serde(untagged)]
    enum StatDefOptionExtFormats<T> {
        Simple(StatDefOption),
        Extended(T),
    }
}
