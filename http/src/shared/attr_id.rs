use std::str::FromStr;

const E_PREFIX: &str = "e";
const C_PREFIX: &str = "c";

#[derive(Eq, PartialEq, Hash)]
pub(crate) enum HAttrId {
    Eve(rc::EveAttrId),
    Custom(rc::CustomAttrId),
}
impl From<&rc::AttrId> for HAttrId {
    fn from(core_attr_id: &rc::AttrId) -> Self {
        match core_attr_id {
            rc::AttrId::Eve(id) => Self::Eve(*id),
            rc::AttrId::Custom(id) => Self::Custom(*id),
        }
    }
}
impl From<rc::AttrId> for HAttrId {
    fn from(core_attr_id: rc::AttrId) -> Self {
        match core_attr_id {
            rc::AttrId::Eve(id) => Self::Eve(id),
            rc::AttrId::Custom(id) => Self::Custom(id),
        }
    }
}
impl From<&HAttrId> for rc::AttrId {
    fn from(h_attr_id: &HAttrId) -> Self {
        match h_attr_id {
            HAttrId::Eve(id) => Self::Eve(*id),
            HAttrId::Custom(id) => Self::Custom(*id),
        }
    }
}
impl From<HAttrId> for rc::AttrId {
    fn from(h_attr_id: HAttrId) -> Self {
        match h_attr_id {
            HAttrId::Eve(id) => Self::Eve(id),
            HAttrId::Custom(id) => Self::Custom(id),
        }
    }
}
impl serde::Serialize for HAttrId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        let string = match self {
            Self::Eve(id) => format!("{id}"),
            Self::Custom(id) => format!("{C_PREFIX}{id}"),
        };
        serializer.serialize_str(&string)
    }
}
impl<'de> serde::Deserialize<'de> for HAttrId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct HAttrIdVisitor;

        impl<'de> serde::de::Visitor<'de> for HAttrIdVisitor {
            type Value = HAttrId;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("number or string with number with optional type prefix")
            }

            fn visit_i8<E>(self, v: i8) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Self::Value::Eve(v as i32))
            }
            fn visit_u8<E>(self, v: u8) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Self::Value::Eve(v as i32))
            }
            fn visit_i16<E>(self, v: i16) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Self::Value::Eve(v as i32))
            }
            fn visit_u16<E>(self, v: u16) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Self::Value::Eve(v as i32))
            }
            fn visit_i32<E>(self, v: i32) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Self::Value::Eve(v))
            }
            fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Self::Value::Eve(v as i32))
            }
            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Self::Value::Eve(v as i32))
            }
            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Self::Value::Eve(v as i32))
            }
            fn visit_i128<E>(self, v: i128) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Self::Value::Eve(v as i32))
            }
            fn visit_u128<E>(self, v: u128) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Self::Value::Eve(v as i32))
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                if let Some(id_str) = v.strip_prefix(E_PREFIX) {
                    let id = rc::CustomAttrId::from_str(id_str).map_err(|e| serde::de::Error::custom(e))?;
                    return Ok(Self::Value::Eve(id));
                }
                if let Some(id_str) = v.strip_prefix(C_PREFIX) {
                    let id = rc::CustomAttrId::from_str(id_str).map_err(|e| serde::de::Error::custom(e))?;
                    return Ok(Self::Value::Custom(id));
                }
                let id = rc::CustomAttrId::from_str(v).map_err(|e| serde::de::Error::custom(e))?;
                Ok(Self::Value::Eve(id))
            }
        }
        deserializer.deserialize_str(HAttrIdVisitor)
    }
}
