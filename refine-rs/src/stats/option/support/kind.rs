use super::containers::{StatDefOption, StatDefOptionExt, StatOptionExt};
use crate::{CmdResps, err::BrResolveError, shared::BrResolvable};

////////////////////////////////////////////////////////////////////////////////////////////////////
// HKT for stats options
////////////////////////////////////////////////////////////////////////////////////////////////////
// Needed to make containers with per-entity options to be usable for storing them "raw" and
// resolved form. Raw form is the format which is stored in public-facing entities, which have
// "default" field defined elsewhere. Resolved form is default value + stored option combined.
pub(in crate::stats) trait StatOptionKind {
    type Regular: Clone + Default;
    type Extended<T>: Clone + Default
    where
        T: Clone;
}

#[derive(Copy, Clone)]
pub(in crate::stats) struct StatOptionRaw;
impl StatOptionKind for StatOptionRaw {
    type Regular = StatDefOption;
    type Extended<T>
        = StatDefOptionExt<T>
    where
        T: Clone;
}

#[derive(Copy, Clone)]
pub(in crate::stats) struct StatOptionResolved;
impl StatOptionKind for StatOptionResolved {
    type Regular = bool;
    type Extended<T>
        = Option<Vec<T>>
    where
        T: Clone;
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Containers which take their representation from the kind
////////////////////////////////////////////////////////////////////////////////////////////////////
pub(in crate::stats) struct StatOptionRegular<O>(O::Regular)
where
    O: StatOptionKind;
impl<O> Clone for StatOptionRegular<O>
where
    O: StatOptionKind,
{
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl<O> Default for StatOptionRegular<O>
where
    O: StatOptionKind,
{
    fn default() -> Self {
        Self(Default::default())
    }
}

pub(in crate::stats) struct StatOptionExtended<O, T>(O::Extended<T>)
where
    O: StatOptionKind,
    T: Clone;
impl<O, T> Clone for StatOptionExtended<O, T>
where
    O: StatOptionKind,
    T: Clone,
{
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl<O, T> Default for StatOptionExtended<O, T>
where
    O: StatOptionKind,
    T: Clone,
{
    fn default() -> Self {
        Self(Default::default())
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl From<bool> for StatOptionRegular<StatOptionRaw> {
    fn from(enabled: bool) -> Self {
        Self(StatDefOption::from_bool(enabled))
    }
}

impl<T> From<StatOptionExt<T>> for StatOptionExtended<StatOptionRaw, T>
where
    T: Clone,
{
    fn from(option: StatOptionExt<T>) -> Self {
        Self(StatDefOptionExt::from_non_default(option))
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Backref resolution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<B, T> StatOptionExtended<StatOptionRaw, B>
where
    B: Clone + BrResolvable<Target = T>,
    T: Clone,
{
    pub(in crate::stats) fn br_resolve(
        self,
        resps: &CmdResps,
    ) -> Result<StatOptionExtended<StatOptionRaw, T>, BrResolveError> {
        Ok(StatOptionExtended(self.0.br_resolve(resps)?))
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Default + stat resolution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl StatOptionRegular<StatOptionRaw> {
    pub(in crate::stats) fn stat_resolve(self, default: bool) -> StatOptionRegular<StatOptionResolved> {
        StatOptionRegular(self.0.stat_resolve(default))
    }
}

impl<T> StatOptionExtended<StatOptionRaw, T>
where
    T: Clone + Default,
{
    pub(in crate::stats) fn stat_resolve(self, default: bool) -> StatOptionExtended<StatOptionResolved, T> {
        StatOptionExtended(self.0.stat_resolve(default))
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Resolved option access
////////////////////////////////////////////////////////////////////////////////////////////////////
impl StatOptionRegular<StatOptionResolved> {
    pub(in crate::stats) fn is_enabled(&self) -> bool {
        self.0
    }
}

impl<T> StatOptionExtended<StatOptionResolved, T>
where
    T: Clone,
{
    pub(in crate::stats) fn is_enabled(&self) -> bool {
        self.0.is_some()
    }
    pub(in crate::stats) fn get(&self) -> Option<&[T]> {
        self.0.as_deref()
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Custom de/serialization
////////////////////////////////////////////////////////////////////////////////////////////////////
#[cfg(feature = "serde")]
pub(in crate::stats) use custom_serde::DeStatOptionKind;

#[cfg(feature = "serde")]
mod custom_serde {
    use serde::{Deserialize, Deserializer};

    use super::*;

    pub(in crate::stats) trait DeStatOptionKind: StatOptionKind {
        fn deserialize_regular<'de, D>(deserializer: D) -> Result<Self::Regular, D::Error>
        where
            D: Deserializer<'de>;
        fn deserialize_extended<'de, T, D>(deserializer: D) -> Result<Self::Extended<T>, D::Error>
        where
            T: Clone + Deserialize<'de>,
            D: Deserializer<'de>;
    }

    impl DeStatOptionKind for StatOptionRaw {
        fn deserialize_regular<'de, D>(deserializer: D) -> Result<Self::Regular, D::Error>
        where
            D: Deserializer<'de>,
        {
            StatDefOption::deserialize(deserializer)
        }
        fn deserialize_extended<'de, T, D>(deserializer: D) -> Result<Self::Extended<T>, D::Error>
        where
            T: Clone + Deserialize<'de>,
            D: Deserializer<'de>,
        {
            StatDefOptionExt::deserialize(deserializer)
        }
    }

    impl<'de, O> Deserialize<'de> for StatOptionRegular<O>
    where
        O: DeStatOptionKind,
    {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            O::deserialize_regular(deserializer).map(Self)
        }
    }

    impl<'de, O, T> Deserialize<'de> for StatOptionExtended<O, T>
    where
        O: DeStatOptionKind,
        T: Clone + Deserialize<'de>,
    {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            O::deserialize_extended(deserializer).map(Self)
        }
    }
}
