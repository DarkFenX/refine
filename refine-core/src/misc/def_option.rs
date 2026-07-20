/// An option which can inherit .
#[derive(Copy, Clone, Default)]
pub enum DefOption {
    #[default]
    Default,
    Disabled,
    Enabled,
}

/// An option which can have some default value.
#[derive(Copy, Clone, Default)]
pub enum DefOptionExt<T> {
    #[default]
    Default,
    Disabled,
    Enabled,
    EnabledExtended(T),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Non-public
////////////////////////////////////////////////////////////////////////////////////////////////////
impl DefOption {
    pub(crate) fn into_enabled(self, default: bool) -> bool {
        match self {
            Self::Default => default,
            Self::Enabled => true,
            Self::Disabled => false,
        }
    }
    pub(crate) fn is_enabled(&self, default: bool) -> bool {
        match self {
            Self::Default => default,
            Self::Enabled => true,
            Self::Disabled => false,
        }
    }
}

impl<T> DefOptionExt<T> {
    pub(crate) fn into_enabled(self, default: bool) -> Option<T>
    where
        T: Default,
    {
        match self {
            Self::Default => match default {
                false => None,
                true => Some(Default::default()),
            },
            Self::Disabled => None,
            Self::Enabled => Some(T::default()),
            Self::EnabledExtended(settings) => Some(settings),
        }
    }
    pub(crate) fn is_enabled(&self, default: bool) -> Option<T>
    where
        T: Default + Copy,
    {
        match self {
            Self::Default => match default {
                false => None,
                true => Some(T::default()),
            },
            Self::Disabled => None,
            Self::Enabled => Some(T::default()),
            Self::EnabledExtended(settings) => Some(*settings),
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

    impl<'de> Deserialize<'de> for DefOption {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            struct VisitorState;

            impl<'de> Visitor<'de> for VisitorState {
                type Value = DefOption;

                fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                    formatter.write_str("bool or null")
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
            }

            deserializer.deserialize_any(VisitorState)
        }
    }

    impl<'de, T> Deserialize<'de> for DefOptionExt<T>
    where
        T: Deserialize<'de>,
    {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            Ok(match DefOptionExtFormats::deserialize(deserializer)? {
                DefOptionExtFormats::Simple(DefOption::Default) => Self::Default,
                DefOptionExtFormats::Simple(DefOption::Disabled) => Self::Disabled,
                DefOptionExtFormats::Simple(DefOption::Enabled) => Self::Enabled,
                DefOptionExtFormats::Extension(data) => Self::EnabledExtended(data),
            })
        }
    }

    #[derive(serde::Deserialize)]
    #[serde(untagged)]
    enum DefOptionExtFormats<T> {
        Simple(DefOption),
        Extension(T),
    }
}
