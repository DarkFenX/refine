use crate::num::{Count, PValue, UnitInterval};

/// Controls on which spool cycle spoolable modules will be set.
#[derive(Copy, Clone)]
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

#[derive(Copy, Clone)]
pub struct ItemSpoolInfo {
    /// Count of cycles at which effect reaches current spool setting.
    pub current: Count,
    /// Count of cycles at which effect reaches max spool.
    pub max: Count,
    /// True if spool parameters are defined directly on item, false if inherited from sol.
    pub overridden: bool,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Custom serialization/deserialization
////////////////////////////////////////////////////////////////////////////////////////////////////
#[cfg(feature = "serde")]
mod custom_serde {
    use std::str::FromStr;

    use super::*;

    const CYCLES_PREFIX: &str = "c";
    const TIME_PREFIX: &str = "t";
    const SPOOL_SCALE_PREFIX: &str = "ss";
    const CYCLE_SCALE_PREFIX: &str = "cs";

    impl serde::Serialize for Spool {
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

    impl<'de> serde::Deserialize<'de> for Spool {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::de::Deserializer<'de>,
        {
            struct SpoolVisitor;

            impl<'de> serde::de::Visitor<'de> for SpoolVisitor {
                type Value = Spool;

                fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                    formatter.write_str("spool type-prefixed number")
                }

                fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
                where
                    E: serde::de::Error,
                {
                    if let Some(value_str) = v.strip_prefix(SPOOL_SCALE_PREFIX) {
                        let value = UnitInterval::from_str(value_str).map_err(|e| serde::de::Error::custom(e))?;
                        return Ok(Self::Value::SpoolScale(value));
                    }
                    if let Some(value_str) = v.strip_prefix(CYCLE_SCALE_PREFIX) {
                        let value = UnitInterval::from_str(value_str).map_err(|e| serde::de::Error::custom(e))?;
                        return Ok(Self::Value::CycleScale(value));
                    }
                    if let Some(count_str) = v.strip_prefix(CYCLES_PREFIX) {
                        let count = Count::from_str(count_str).map_err(|e| serde::de::Error::custom(e))?;
                        return Ok(Self::Value::Cycles(count));
                    }
                    if let Some(time_str) = v.strip_prefix(TIME_PREFIX) {
                        let time = PValue::from_str(time_str).map_err(|e| serde::de::Error::custom(e))?;
                        return Ok(Self::Value::Time(time));
                    }
                    let msg = format!(
                        "expected a number prefixed by \"{CYCLES_PREFIX}\", \"{TIME_PREFIX}\", \"{SPOOL_SCALE_PREFIX}\", or \"{CYCLE_SCALE_PREFIX}\", got \"{v}\""
                    );
                    Err(serde::de::Error::custom(msg))
                }
            }
            deserializer.deserialize_str(SpoolVisitor)
        }
    }
}
