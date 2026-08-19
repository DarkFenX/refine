/// An option which can inherit default value from elsewhere.
#[derive(Copy, Clone, Default)]
pub enum DefOption {
    #[default]
    Default,
    Disabled,
    Enabled,
}

/// An option which can inherit default value from elsewhere, plus extended setting support.
#[derive(Copy, Clone, Default)]
pub enum DefOptionExt<T> {
    #[default]
    Default,
    Disabled,
    Enabled,
    EnabledExtended(T),
}

/// An option which can have extended settings.
#[derive(Copy, Clone)]
pub enum OptionExt<T> {
    Disabled,
    Enabled,
    EnabledExtended(T),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<T> DefOptionExt<T> {
    pub(crate) fn into_option_ext(self) -> Option<OptionExt<T>> {
        match self {
            DefOptionExt::Default => None,
            DefOptionExt::Disabled => Some(OptionExt::Disabled),
            DefOptionExt::Enabled => Some(OptionExt::Enabled),
            DefOptionExt::EnabledExtended(inner) => Some(OptionExt::EnabledExtended(inner)),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Non-public
////////////////////////////////////////////////////////////////////////////////////////////////////
impl DefOption {
    pub(crate) fn is_enabled(&self, default: bool) -> bool {
        match self {
            Self::Default => default,
            Self::Enabled => true,
            Self::Disabled => false,
        }
    }
}

impl<T> DefOptionExt<T> {
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
            struct VisitorImpl;

            impl<'de> Visitor<'de> for VisitorImpl {
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

            deserializer.deserialize_any(VisitorImpl)
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
                DefOptionExtFormats::Extended(data) => Self::EnabledExtended(data),
            })
        }
    }

    #[derive(serde::Deserialize)]
    #[serde(untagged)]
    enum DefOptionExtFormats<T> {
        Simple(DefOption),
        Extended(T),
    }
}
