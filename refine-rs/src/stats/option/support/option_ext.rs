use crate::{
    CmdResps,
    err::BrResolveError,
    shared::{BrResolveFallible, BrResolveInfallible},
};

/// A stat option which can have extended settings.
#[derive(Clone)]
pub enum StatOptionExt<T> {
    Disabled,
    Enabled,
    EnabledExtended(Vec<T>),
}

// Internal counterpart of public option, which can carry per-stat-option error
#[derive(Clone)]
pub(in crate::stats) enum StatOptionInt<T> {
    Disabled,
    Enabled,
    EnabledExtended(Vec<Result<T, BrResolveError>>),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<T> StatOptionExt<T> {
    pub(in crate::stats) fn into_internal(self) -> StatOptionInt<T> {
        match self {
            Self::Disabled => StatOptionInt::Disabled,
            Self::Enabled => StatOptionInt::Enabled,
            Self::EnabledExtended(options) => StatOptionInt::EnabledExtended(options.into_iter().map(Ok).collect()),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Backref resolution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<B> StatOptionExt<B> {
    pub(in crate::stats) fn br_resolve_infallible<I>(self, resps: &CmdResps) -> StatOptionExt<I>
    where
        B: BrResolveInfallible<Target = I>,
    {
        match self {
            Self::Disabled => StatOptionExt::Disabled,
            Self::Enabled => StatOptionExt::Enabled,
            Self::EnabledExtended(inner) => StatOptionExt::EnabledExtended(
                inner
                    .into_iter()
                    .map(|option| option.br_resolve_infallible(resps))
                    .collect(),
            ),
        }
    }
}

impl<B> StatOptionInt<B> {
    pub(in crate::stats) fn br_resolve_fallible<I>(self, resps: &CmdResps) -> StatOptionInt<I>
    where
        B: BrResolveFallible<Target = I>,
    {
        match self {
            Self::Disabled => StatOptionInt::Disabled,
            Self::Enabled => StatOptionInt::Enabled,
            Self::EnabledExtended(inner) => StatOptionInt::EnabledExtended(
                inner
                    .into_iter()
                    .map(|option| option.and_then(|option| option.br_resolve_fallible(resps)))
                    .collect(),
            ),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Default + stat resolution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<T> StatOptionExt<T>
where
    T: Default,
{
    pub(in crate::stats) fn from_default(default: bool) -> Vec<T> {
        match default {
            true => vec![T::default()],
            false => Vec::new(),
        }
    }
    pub(in crate::stats) fn stat_resolve(self) -> Vec<T> {
        match self {
            Self::Disabled => Vec::new(),
            Self::Enabled => vec![T::default()],
            Self::EnabledExtended(options) => options,
        }
    }
}

impl<T> StatOptionInt<T>
where
    T: Default,
{
    pub(in crate::stats) fn from_default(default: bool) -> Vec<Result<T, BrResolveError>> {
        match default {
            true => vec![Ok(T::default())],
            false => Vec::new(),
        }
    }
    pub(in crate::stats) fn stat_resolve(self) -> Vec<Result<T, BrResolveError>> {
        match self {
            Self::Disabled => Vec::new(),
            Self::Enabled => vec![Ok(T::default())],
            Self::EnabledExtended(options) => options,
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
    pub(super) enum StatOptionExtFormats<T> {
        Simple(bool),
        Extended(Vec<T>),
    }

    impl<'de, T> Deserialize<'de> for StatOptionInt<T>
    where
        T: Deserialize<'de>,
    {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            Ok(StatOptionExt::deserialize(deserializer)?.into_internal())
        }
    }
}
