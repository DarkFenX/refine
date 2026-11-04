use crate::PValue;

/// Stats simulation time in seconds, within `[0, 86400]` range (one day).
///
/// Serialization notes: deserialization clamps out-of-range input instead of failing.
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(transparent))]
#[derive(Copy, Clone, Debug, derive_more::Display)]
pub struct StatTime(f64);
impl StatTime {
    pub fn from_f64_checked(time: f64) -> Result<Self, StatTimeError> {
        match (Self::MIN.0..=Self::MAX.0).contains(&time) {
            true => Ok(Self(time)),
            false => Err(StatTimeError { time }),
        }
    }
    pub const fn from_f64_clamped(time: f64) -> Self {
        match time.is_nan() {
            true => Self::DEFAULT,
            false => Self::from_finite_f64_clamped(time),
        }
    }
    pub(crate) const fn from_finite_f64_clamped(time: f64) -> Self {
        Self(time.clamp(Self::MIN.0, Self::MAX.0))
    }
    pub const fn into_f64(self) -> f64 {
        self.0
    }
}

#[derive(Debug, thiserror::Error)]
#[error("stat time {time} is out of allowed range [{}, {}]", StatTime::MIN, StatTime::MAX)]
pub struct StatTimeError {
    pub time: f64,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Constants
////////////////////////////////////////////////////////////////////////////////////////////////////
impl StatTime {
    pub const DEFAULT: Self = Self(0.0);
    pub const MIN: Self = Self(0.0);
    pub const MAX: Self = Self(24.0 * 60.0 * 60.0);
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl StatTime {
    pub(crate) fn into_pvalue(self) -> PValue {
        PValue::from_f64_unchecked(self.0)
    }
}
impl From<StatTime> for f64 {
    fn from(v: StatTime) -> Self {
        v.0
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Custom de/serialization
////////////////////////////////////////////////////////////////////////////////////////////////////
#[cfg(feature = "serde")]
mod custom_serde {
    use serde::de::{Deserialize, Deserializer};

    use super::*;

    impl<'de> Deserialize<'de> for StatTime {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            f64::deserialize(deserializer).map(StatTime::from_f64_clamped)
        }
    }
}
