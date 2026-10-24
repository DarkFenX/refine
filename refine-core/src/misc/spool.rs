use std::str::FromStr;

use crate::num::{Count, CountParseError, PValue, PValueParseError, UnitInterval, UnitIntervalParseError};

/// Controls on which spool cycle spoolable modules will be set.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Spool {
    /// Module will use this number, or max spool cycles supported by module, whichever is lower.
    Cycles(Count),
    /// Module will use count of full cycles it finishes by this time, or max spool cycles supported
    /// by module, whichever is lower.
    Time(PValue),
    /// Specify a point on damage multiplier range, which is then used to choose count of cycles
    /// sufficient to reach it. For example, with max spool = 0.455 and spool step = 0.1, spool
    /// scale = 0.42:
    /// ⌈(0.455 ÷ 0.1) × 0.42⌉ = ⌈1.911⌉ = 2
    /// Result can be different from cycle scale only if max spool can be divided by spool step with
    /// remainder, due to cycle scale being wider. If there is no remainder, spool and cycle range
    /// effectively match.
    SpoolScale(UnitInterval),
    /// Specify a point on cycle number range, which is then used to choose count of cycles
    /// sufficient to reach it. For example, with max spool = 0.455 and spool step = 0.1, cycle
    /// scale = 0.42:
    /// ⌈⌈0.455 ÷ 0.1⌉ × 0.42⌉ = ⌈⌈4.55⌉ × 0.42⌉ = ⌈5 × 0.42⌉ = ⌈2.1⌉ = 3
    /// Result can be different from spool scale only if max spool can be divided by spool step with
    /// remainder, due to cycle scale being wider. If there is no remainder, spool and cycle range
    /// effectively match.
    CycleScale(UnitInterval),
}

#[cfg_attr(feature = "serde", derive(serde_tuple::Serialize_tuple))]
#[derive(Copy, Clone)]
pub struct ItemSpoolInfo {
    /// Count of cycles at which effect reaches current spool setting.
    pub current: Count,
    /// Count of cycles at which effect reaches max spool.
    pub max: Count,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Custom de/serialization
////////////////////////////////////////////////////////////////////////////////////////////////////
const CYCLES_PREFIX: &str = "c";
const TIME_PREFIX: &str = "t";
const SPOOL_SCALE_PREFIX: &str = "ss";
const CYCLE_SCALE_PREFIX: &str = "cs";

impl std::fmt::Display for Spool {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Cycles(count) => write!(f, "{CYCLES_PREFIX}{count}"),
            Self::Time(time) => write!(f, "{TIME_PREFIX}{time}"),
            Self::SpoolScale(value) => write!(f, "{SPOOL_SCALE_PREFIX}{value}"),
            Self::CycleScale(value) => write!(f, "{CYCLE_SCALE_PREFIX}{value}"),
        }
    }
}

impl FromStr for Spool {
    type Err = SpoolParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Process longer prefixes first in case of conflicting starting letters
        if let Some(value_str) = s.strip_prefix(SPOOL_SCALE_PREFIX) {
            return Ok(Self::SpoolScale(UnitInterval::from_str(value_str)?));
        }
        if let Some(value_str) = s.strip_prefix(CYCLE_SCALE_PREFIX) {
            return Ok(Self::CycleScale(UnitInterval::from_str(value_str)?));
        }
        if let Some(count_str) = s.strip_prefix(CYCLES_PREFIX) {
            return Ok(Self::Cycles(Count::from_str(count_str)?));
        }
        if let Some(time_str) = s.strip_prefix(TIME_PREFIX) {
            return Ok(Self::Time(PValue::from_str(time_str)?));
        }
        Err(SpoolParseError::InvalidPrefix)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum SpoolParseError {
    #[error(
        "invalid prefix, expected \"{CYCLES_PREFIX}\", \"{TIME_PREFIX}\", \"{SPOOL_SCALE_PREFIX}\", or \"{CYCLE_SCALE_PREFIX}\" prefix"
    )]
    InvalidPrefix,
    #[error(transparent)]
    InvalidCount(#[from] CountParseError),
    #[error(transparent)]
    InvalidTime(#[from] PValueParseError),
    #[error(transparent)]
    InvalidScale(#[from] UnitIntervalParseError),
}

#[cfg(feature = "serde")]
mod custom_serde {
    use serde::{
        de::{Deserialize, Deserializer, Error, Visitor},
        ser::{Serialize, Serializer},
    };

    use super::*;

    impl Serialize for Spool {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            serializer.serialize_str(&self.to_string())
        }
    }

    impl<'de> Deserialize<'de> for Spool {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            struct VisitorImpl;

            impl<'de> Visitor<'de> for VisitorImpl {
                type Value = Spool;

                fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                    formatter.write_str("string with spool type-prefixed number")
                }

                fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
                where
                    E: Error,
                {
                    Self::Value::from_str(v).map_err(Error::custom)
                }
            }

            deserializer.deserialize_string(VisitorImpl)
        }
    }
}
