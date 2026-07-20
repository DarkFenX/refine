use crate::num::PValue;

/// Range for jump stats used for fuel use calculations.
#[derive(Copy, Clone, Default)]
pub enum StatJumpRange {
    LightYears(PValue),
    #[default]
    Max,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Custom de/serialization - attribute definition
////////////////////////////////////////////////////////////////////////////////////////////////////
#[cfg(feature = "serde")]
mod custom_serde_attr {
    use serde::de::{Deserialize, Deserializer, Error, Visitor};

    use super::*;

    const ROLL_PREFIX: &str = "r";
    const ABS_PREFIX: &str = "a";

    impl<'de> Deserialize<'de> for StatJumpRange {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            struct VisitorState;

            impl<'de> Visitor<'de> for VisitorState {
                type Value = StatJumpRange;

                fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                    formatter.write_str("distance in light-years as a number, or \"max\"")
                }

                fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
                where
                    E: Error,
                {
                    Ok(Self::Value::LightYears(PValue::from_f64_clamped(v as f64)))
                }
                fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
                where
                    E: Error,
                {
                    Ok(Self::Value::LightYears(PValue::from_f64_clamped(v as f64)))
                }
                fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
                where
                    E: Error,
                {
                    Ok(Self::Value::LightYears(PValue::from_f64_clamped(v)))
                }

                fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
                where
                    E: Error,
                {
                    match v {
                        "max" => Ok(Self::Value::Max),
                        _ => Err(Error::custom("unexpected string value")),
                    }
                }
            }
            deserializer.deserialize_any(VisitorState)
        }
    }
}
