use crate::{CmdResps, err::BrResolveError, shared::BrResolvable};

/// A stat option which can have extended settings.
#[derive(Clone)]
pub enum StatOptionExt<T> {
    Disabled,
    Enabled,
    EnabledExtended(Vec<T>),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Backref resolution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<B> StatOptionExt<B> {
    pub(in crate::stats) fn br_resolve<I>(self, resps: &CmdResps) -> Result<StatOptionExt<I>, BrResolveError>
    where
        B: BrResolvable<Target = I>,
    {
        Ok(match self {
            Self::Disabled => StatOptionExt::Disabled,
            Self::Enabled => StatOptionExt::Enabled,
            Self::EnabledExtended(inner) => {
                let mut resolved_inner = Vec::with_capacity(inner.len());
                for option in inner.into_iter() {
                    resolved_inner.push(option.br_resolve(resps)?);
                }
                StatOptionExt::EnabledExtended(resolved_inner)
            }
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Default + stat resolution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<T> StatOptionExt<T>
where
    T: Default,
{
    pub(in crate::stats) fn stat_resolve(self) -> Vec<T> {
        match self {
            Self::Disabled => Vec::new(),
            Self::Enabled => vec![T::default()],
            Self::EnabledExtended(options) => options,
        }
    }
    pub(in crate::stats) fn stat_default(default: bool) -> Vec<T> {
        match default {
            true => vec![T::default()],
            false => Vec::new(),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Custom de/serialization
////////////////////////////////////////////////////////////////////////////////////////////////////
#[cfg(feature = "serde")]
mod custom_serde {
    use serde::de::{Deserialize, Deserializer};

    use super::*;

    impl<'de, T> Deserialize<'de> for StatOptionExt<T>
    where
        T: Deserialize<'de>,
    {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            Ok(match StatOptionExtFormats::deserialize(deserializer)? {
                StatOptionExtFormats::Simple(false) => Self::Disabled,
                StatOptionExtFormats::Simple(true) => Self::Enabled,
                StatOptionExtFormats::Extended(options) => Self::EnabledExtended(options),
            })
        }
    }

    #[derive(serde::Deserialize)]
    #[serde(untagged)]
    enum StatOptionExtFormats<T> {
        Simple(bool),
        Extended(Vec<T>),
    }
}
