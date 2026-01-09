use std::str::FromStr;

use serde::Deserialize;

const CYCLES_PREFIX: &str = "c";
const TIME_PREFIX: &str = "t";
const SPOOL_SCALE_PREFIX: &str = "ss";
const CYCLE_SCALE_PREFIX: &str = "cs";

#[derive(Copy, Clone)]
pub(crate) enum HSpool {
    Cycles(u32),
    Time(f64),
    SpoolScale(f64),
    CycleScale(f64),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversion
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HSpool {
    pub(crate) fn from_core(core_spool: rc::Spool) -> Self {
        match core_spool {
            rc::Spool::Cycles(count) => Self::Cycles(count.into_u32()),
            rc::Spool::Time(time) => Self::Time(time.into_f64()),
            rc::Spool::SpoolScale(value) => Self::SpoolScale(value.into_f64()),
            rc::Spool::CycleScale(value) => Self::CycleScale(value.into_f64()),
        }
    }
    pub(crate) fn into_core(self) -> rc::Spool {
        match self {
            Self::Cycles(count) => rc::Spool::Cycles(rc::Count::from_u32(count)),
            Self::Time(count) => rc::Spool::Time(rc::PValue::from_f64_clamped(count)),
            Self::SpoolScale(value) => rc::Spool::SpoolScale(rc::UnitInterval::from_f64_clamped(value)),
            Self::CycleScale(value) => rc::Spool::CycleScale(rc::UnitInterval::from_f64_clamped(value)),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Serialization support
////////////////////////////////////////////////////////////////////////////////////////////////////
impl serde::Serialize for HSpool {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        let string = match self {
            Self::Cycles(count) => format!("{CYCLES_PREFIX}{count}"),
            Self::Time(time) => format!("{TIME_PREFIX}{time}"),
            Self::SpoolScale(value) => format!("{SPOOL_SCALE_PREFIX}{value}"),
            Self::CycleScale(value) => format!("{CYCLE_SCALE_PREFIX}{value}"),
        };
        serializer.serialize_str(&string)
    }
}
impl<'de> Deserialize<'de> for HSpool {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct HSpoolVisitor;

        impl<'de> serde::de::Visitor<'de> for HSpoolVisitor {
            type Value = HSpool;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("spool type-prefixed number HSpool")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                if let Some(value_str) = v.strip_prefix(SPOOL_SCALE_PREFIX) {
                    let value = f64::from_str(value_str).map_err(|e| serde::de::Error::custom(e))?;
                    return Ok(Self::Value::SpoolScale(value));
                }
                if let Some(value_str) = v.strip_prefix(CYCLE_SCALE_PREFIX) {
                    let value = f64::from_str(value_str).map_err(|e| serde::de::Error::custom(e))?;
                    return Ok(Self::Value::CycleScale(value));
                }
                if let Some(count_str) = v.strip_prefix(CYCLES_PREFIX) {
                    let count = u32::from_str(count_str).map_err(|e| serde::de::Error::custom(e))?;
                    return Ok(Self::Value::Cycles(count));
                }
                if let Some(time_str) = v.strip_prefix(TIME_PREFIX) {
                    let time = f64::from_str(time_str).map_err(|e| serde::de::Error::custom(e))?;
                    return Ok(Self::Value::Time(time));
                }
                let msg = format!(
                    "expected a number prefixed by \"{CYCLES_PREFIX}\", \"{TIME_PREFIX}\", \"{SPOOL_SCALE_PREFIX}\", \
                    or \"{CYCLE_SCALE_PREFIX}\", got \"{v}\""
                );
                Err(serde::de::Error::custom(msg))
            }
        }
        deserializer.deserialize_str(HSpoolVisitor)
    }
}
