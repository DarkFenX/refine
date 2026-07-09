use serde::Deserialize;
use serde_with::{DisplayFromStr, serde_as};

#[derive(Copy, Clone, Default)]
pub enum HStatJumpRange {
    LightYears(f64),
    #[default]
    Max,
}

#[serde_as]
#[derive(Clone, Default, Deserialize)]
pub(in crate::cmd) struct HStatOptionJump {
    pub(in crate::cmd) range: HStatJumpRange,
    #[serde_as(as = "Vec<DisplayFromStr>")]
    pub(in crate::cmd) passenger_fit_ids: Vec<rc::FitId>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Serialization support
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'de> Deserialize<'de> for HStatJumpRange {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct HStatJumpRangeVisitor;

        impl<'de> serde::de::Visitor<'de> for HStatJumpRangeVisitor {
            type Value = HStatJumpRange;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("distance in light-years as a float, or \"max\"")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(match v {
                    "max" => HStatJumpRange::Max,
                    _ => Self::Value::LightYears(v.parse().map_err(|e| serde::de::Error::custom(e))?),
                })
            }
        }
        deserializer.deserialize_str(HStatJumpRangeVisitor)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HStatJumpRange {
    pub(in crate::cmd::stats) fn into_core(self) -> rc::stats::StatJumpRange {
        match self {
            Self::LightYears(range) => rc::stats::StatJumpRange::LightYears(rc::PValue::from_f64_clamped(range)),
            Self::Max => rc::stats::StatJumpRange::Max,
        }
    }
}
