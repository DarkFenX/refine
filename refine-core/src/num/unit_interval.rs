use std::{mem::transmute, pat::pattern_type, str::FromStr};

use crate::{PValue, Value};

// Bits of non-negative floats are ordered the same way as the floats
const ZERO_BITS: u64 = 0x0000_0000_0000_0000;
const ONE_BITS: u64 = 0x3ff0_0000_0000_0000;
type Range = pattern_type!(u64 is ZERO_BITS..=ONE_BITS);

/// Float value within the unit interval `[0, 1]`.
///
/// Serialization notes: deserialization clamps out-of-range input instead of failing.
#[derive(Copy, Clone)]
pub struct UnitInterval(Range);
impl UnitInterval {
    pub fn from_f64_checked(v: f64) -> Result<Self, UnitIntervalError> {
        match (0.0..=1.0).contains(&v) {
            true => Ok(Self::from_f64_clamped(v)),
            false => Err(UnitIntervalError { value: v }),
        }
    }
    pub const fn from_f64_clamped(v: f64) -> Self {
        let bits = v.to_bits();
        let bits = match bits <= ONE_BITS {
            true => bits,
            // Negative values (including negative zero), values above 1, NaN
            false => match v > 1.0 {
                true => ONE_BITS,
                false => ZERO_BITS,
            },
        };
        // SAFETY: just ensured that the value is in range
        Self(unsafe { transmute::<u64, Range>(bits) })
    }
    pub const fn into_f64(self) -> f64 {
        f64::from_bits(self.into_bits())
    }
}

#[derive(Debug, thiserror::Error)]
#[error(
    "value {value} is out of allowed unit interval range [{}, {}]",
    UnitInterval::MIN,
    UnitInterval::MAX
)]
pub struct UnitIntervalError {
    pub value: f64,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Constants
////////////////////////////////////////////////////////////////////////////////////////////////////
impl UnitInterval {
    pub const DEFAULT: Self = Self::from_f64_clamped(0.0);
    pub const MIN: Self = Self::from_f64_clamped(0.0);
    pub const MAX: Self = Self::from_f64_clamped(1.0);
    pub(crate) const ZERO: Self = Self::from_f64_clamped(0.0);
    pub(crate) const ONE: Self = Self::from_f64_clamped(1.0);
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl UnitInterval {
    pub(crate) fn from_value_clamped(value: Value) -> Self {
        Self::from_f64_clamped(value.into_f64())
    }
    pub(crate) fn from_pvalue_clamped(value: PValue) -> Self {
        Self::from_f64_clamped(value.into_f64())
    }
    const fn into_bits(self) -> u64 {
        // SAFETY: any value in range can be represented by base type
        unsafe { transmute::<Range, u64>(self.0) }
    }
    pub(crate) fn into_value(self) -> Value {
        Value::from_f64_unchecked(self.into_f64())
    }
    pub(crate) fn into_pvalue(self) -> PValue {
        PValue::from_f64_unchecked(self.into_f64())
    }
}
impl From<UnitInterval> for f64 {
    fn from(v: UnitInterval) -> Self {
        v.into_f64()
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Ordering/equality/hash - pattern types do not provide those
////////////////////////////////////////////////////////////////////////////////////////////////////
impl Eq for UnitInterval {}
impl PartialEq for UnitInterval {
    fn eq(&self, other: &Self) -> bool {
        self.into_bits() == other.into_bits()
    }
}
impl std::marker::StructuralPartialEq for UnitInterval {}

impl Ord for UnitInterval {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.into_bits().cmp(&other.into_bits())
    }
}
impl PartialOrd for UnitInterval {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl std::hash::Hash for UnitInterval {
    fn hash<H>(&self, state: &mut H)
    where
        H: std::hash::Hasher,
    {
        self.into_bits().hash(state);
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Custom de/serialization
////////////////////////////////////////////////////////////////////////////////////////////////////
impl std::fmt::Display for UnitInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.into_f64(), f)
    }
}
impl std::fmt::Debug for UnitInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("UnitInterval").field(&self.into_f64()).finish()
    }
}

impl FromStr for UnitInterval {
    type Err = UnitIntervalParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value = f64::from_str(s)?;
        let value = Self::from_f64_clamped(value);
        Ok(value)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum UnitIntervalParseError {
    #[error(transparent)]
    InvalidFloat(#[from] std::num::ParseFloatError),
}

#[cfg(feature = "serde")]
mod custom_serde {
    use serde::{
        de::{Deserialize, Deserializer},
        ser::{Serialize, Serializer},
    };

    use super::*;

    impl Serialize for UnitInterval {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            serializer.serialize_f64(self.into_f64())
        }
    }

    impl<'de> Deserialize<'de> for UnitInterval {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            f64::deserialize(deserializer).map(UnitInterval::from_f64_clamped)
        }
    }
}
