use std::str::FromStr;

const CYCLES_PREFIX: &str = "c";
const TIME_PREFIX: &str = "t";
const SPOOL_SCALE_PREFIX: &str = "ss";
const CYCLE_SCALE_PREFIX: &str = "cs";

#[derive(Copy, Clone)]
pub(crate) enum HSpool {
    Cycles(rc::Count),
    Time(rc::AttrVal),
    SpoolScale(rc::AttrVal),
    CycleScale(rc::AttrVal),
}
impl From<rc::Spool> for HSpool {
    fn from(core_spool: rc::Spool) -> Self {
        match core_spool {
            rc::Spool::Cycles(count) => Self::Cycles(count),
            rc::Spool::Time(time) => Self::Time(time),
            rc::Spool::SpoolScale(value) => Self::SpoolScale(value.get_inner()),
            rc::Spool::CycleScale(value) => Self::CycleScale(value.get_inner()),
        }
    }
}
impl From<HSpool> for rc::Spool {
    fn from(h_spool: HSpool) -> Self {
        match h_spool {
            HSpool::Cycles(count) => Self::Cycles(count),
            HSpool::Time(count) => Self::Time(count),
            HSpool::SpoolScale(value) => Self::SpoolScale(rc::UnitInterval::new_clamped(value)),
            HSpool::CycleScale(value) => Self::CycleScale(rc::UnitInterval::new_clamped(value)),
        }
    }
}
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
impl<'de> serde::Deserialize<'de> for HSpool {
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
                    let value = rc::AttrVal::from_str(value_str).map_err(|v| serde::de::Error::custom(v))?;
                    return Ok(Self::Value::SpoolScale(value));
                }
                if let Some(value_str) = v.strip_prefix(CYCLE_SCALE_PREFIX) {
                    let value = rc::AttrVal::from_str(value_str).map_err(|v| serde::de::Error::custom(v))?;
                    return Ok(Self::Value::CycleScale(value));
                }
                if let Some(count_str) = v.strip_prefix(CYCLES_PREFIX) {
                    let count = rc::Count::from_str(count_str).map_err(|v| serde::de::Error::custom(v))?;
                    return Ok(Self::Value::Cycles(count));
                }
                if let Some(time_str) = v.strip_prefix(TIME_PREFIX) {
                    let time = rc::AttrVal::from_str(time_str).map_err(|v| serde::de::Error::custom(v))?;
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
