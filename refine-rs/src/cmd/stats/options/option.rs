#[derive(Copy, Clone, Default)]
pub enum StatOption {
    #[default]
    Default,
    Disabled,
    Enabled,
}

#[derive(Clone, Default)]
pub enum StatOptionExt<T> {
    #[default]
    Default,
    Disabled,
    Enabled,
    EnabledExtended(Vec<T>),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Non-public
////////////////////////////////////////////////////////////////////////////////////////////////////
impl StatOption {
    pub(in crate::cmd::stats) fn into_enabled(self, default: bool) -> bool {
        match self {
            Self::Default => default,
            Self::Disabled => false,
            Self::Enabled => true,
        }
    }
}

impl<T> StatOptionExt<T> {
    pub(in crate::cmd::stats) fn into_enabled(self, default: bool) -> Option<Vec<T>>
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

    impl<'de> Deserialize<'de> for StatOption {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            struct VisitorImpl;

            impl<'de> Visitor<'de> for VisitorImpl {
                type Value = StatOption;

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

    impl<'de, T> Deserialize<'de> for StatOptionExt<T>
    where
        T: Deserialize<'de>,
    {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            Ok(match StatOptionExtFormats::deserialize(deserializer)? {
                StatOptionExtFormats::Simple(StatOption::Default) => Self::Default,
                StatOptionExtFormats::Simple(StatOption::Disabled) => Self::Disabled,
                StatOptionExtFormats::Simple(StatOption::Enabled) => Self::Enabled,
                StatOptionExtFormats::Extended(data) => Self::EnabledExtended(data),
            })
        }
    }

    #[derive(serde::Deserialize)]
    #[serde(untagged)]
    enum StatOptionExtFormats<T> {
        Simple(StatOption),
        Extended(T),
    }
}
