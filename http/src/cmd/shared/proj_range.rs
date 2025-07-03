use std::str::FromStr;

const S2S_PREFIX: &str = "s";
const C2C_PREFIX: &str = "c";

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(in crate::cmd) enum HProjRange {
    // S2S is default when type of number is not specified
    S2S(rc::AttrVal),
    C2C(rc::AttrVal),
    None,
}
impl From<HProjRange> for rc::ProjRange {
    fn from(h_prange: HProjRange) -> Self {
        match h_prange {
            HProjRange::S2S(range) => Self::S2S(range),
            HProjRange::C2C(range) => Self::C2C(range),
            HProjRange::None => Self::None,
        }
    }
}
impl<'de> serde::Deserialize<'de> for HProjRange {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct HEffectIdVisitor;

        impl<'de> serde::de::Visitor<'de> for HEffectIdVisitor {
            type Value = HProjRange;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("float, int, string with range float with optional \"c\"/\"s\" prefix, or null")
            }

            fn visit_unit<E>(self) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Self::Value::None)
            }

            fn visit_none<E>(self) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Self::Value::None)
            }

            fn visit_i8<E>(self, v: i8) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Self::Value::S2S(rc::AttrVal::from(v)))
            }
            fn visit_u8<E>(self, v: u8) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Self::Value::S2S(rc::AttrVal::from(v)))
            }
            fn visit_i16<E>(self, v: i16) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Self::Value::S2S(rc::AttrVal::from(v)))
            }
            fn visit_u16<E>(self, v: u16) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Self::Value::S2S(rc::AttrVal::from(v)))
            }
            fn visit_i32<E>(self, v: i32) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Self::Value::S2S(rc::AttrVal::from(v)))
            }
            fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Self::Value::S2S(rc::AttrVal::from(v)))
            }
            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Self::Value::S2S(rc::AttrVal::from(v as f64)))
            }
            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Self::Value::S2S(rc::AttrVal::from(v as f64)))
            }
            fn visit_i128<E>(self, v: i128) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Self::Value::S2S(rc::AttrVal::from(v as f64)))
            }
            fn visit_u128<E>(self, v: u128) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Self::Value::S2S(rc::AttrVal::from(v as f64)))
            }

            fn visit_f32<E>(self, v: f32) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Self::Value::S2S(rc::AttrVal::from(v as f64)))
            }

            fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Self::Value::S2S(rc::AttrVal::from(v)))
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                if let Some(prange_str) = v.strip_prefix(S2S_PREFIX) {
                    let prange = rc::AttrVal::from_str(prange_str).map_err(|v| serde::de::Error::custom(v))?;
                    return Ok(Self::Value::S2S(prange));
                }
                if let Some(prange_str) = v.strip_prefix(C2C_PREFIX) {
                    let prange = rc::AttrVal::from_str(prange_str).map_err(|v| serde::de::Error::custom(v))?;
                    return Ok(Self::Value::C2C(prange));
                }
                let prange = rc::AttrVal::from_str(v).map_err(|v| serde::de::Error::custom(v))?;
                Ok(Self::Value::S2S(prange))
            }
        }
        deserializer.deserialize_str(HEffectIdVisitor)
    }
}
