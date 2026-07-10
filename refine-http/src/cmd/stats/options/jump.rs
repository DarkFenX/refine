use serde::Deserialize;
use serde_with::{DisplayFromStr, serde_as};

use super::shared_affectors::HStatAffectors;

#[serde_as]
#[derive(Clone, Default, Deserialize)]
pub(in crate::cmd) struct HStatOptionJump {
    #[serde(default)]
    pub(in crate::cmd) range: HStatOptionJumpRange,
    #[serde(default)]
    #[serde_as(as = "Vec<DisplayFromStr>")]
    pub(in crate::cmd) passenger_fit_ids: Vec<rc::FitId>,
    #[serde(default)]
    pub(in crate::cmd) passenger_fuel_affectors: HStatAffectors,
}

#[derive(Copy, Clone, Default)]
pub enum HStatOptionJumpRange {
    LightYears(f64),
    #[default]
    Max,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Serialization support
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'de> Deserialize<'de> for HStatOptionJumpRange {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct HStatJumpRangeVisitor;

        impl<'de> serde::de::Visitor<'de> for HStatJumpRangeVisitor {
            type Value = HStatOptionJumpRange;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("distance in light-years as a number, or \"max\"")
            }

            fn visit_i8<E>(self, v: i8) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Self::Value::LightYears(v as f64))
            }
            fn visit_u8<E>(self, v: u8) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Self::Value::LightYears(v as f64))
            }
            fn visit_i16<E>(self, v: i16) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Self::Value::LightYears(v as f64))
            }
            fn visit_u16<E>(self, v: u16) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Self::Value::LightYears(v as f64))
            }
            fn visit_i32<E>(self, v: i32) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Self::Value::LightYears(v as f64))
            }
            fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Self::Value::LightYears(v as f64))
            }
            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Self::Value::LightYears(v as f64))
            }
            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Self::Value::LightYears(v as f64))
            }
            fn visit_i128<E>(self, v: i128) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Self::Value::LightYears(v as f64))
            }
            fn visit_u128<E>(self, v: u128) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Self::Value::LightYears(v as f64))
            }

            fn visit_f32<E>(self, v: f32) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Self::Value::LightYears(v as f64))
            }
            fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Self::Value::LightYears(v))
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match v {
                    "max" => Ok(Self::Value::Max),
                    _ => Err(serde::de::Error::custom("unexpected string value")),
                }
            }
        }
        deserializer.deserialize_any(HStatJumpRangeVisitor)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HStatOptionJumpRange {
    pub(in crate::cmd::stats) fn into_core(self) -> rc::stats::StatJumpRange {
        match self {
            Self::LightYears(range) => rc::stats::StatJumpRange::LightYears(rc::PValue::from_f64_clamped(range)),
            Self::Max => rc::stats::StatJumpRange::Max,
        }
    }
}
