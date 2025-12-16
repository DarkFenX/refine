use std::str::FromStr;

use crate::cacher_json::data::{CCustomAttrId, CEveAttrId};

const E_PREFIX: &str = "e";
const C_PREFIX: &str = "c";

#[derive(Eq, PartialEq, Hash)]
pub(in crate::cacher_json) enum CAttrId {
    Eve(CEveAttrId),
    Custom(CCustomAttrId),
}
impl From<&rc::ad::AAttrId> for CAttrId {
    fn from(a_attr_id: &rc::ad::AAttrId) -> Self {
        match a_attr_id {
            rc::ad::AAttrId::Eve(id) => Self::Eve(*id),
            rc::ad::AAttrId::Custom(id) => Self::Custom(*id),
        }
    }
}
impl From<&CAttrId> for rc::ad::AAttrId {
    fn from(c_attr_id: &CAttrId) -> Self {
        match c_attr_id {
            CAttrId::Eve(id) => Self::Eve(*id),
            CAttrId::Custom(id) => Self::Custom(*id),
        }
    }
}
impl serde::Serialize for CAttrId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        let string = match self {
            Self::Eve(id) => format!("{E_PREFIX}{id}"),
            Self::Custom(id) => format!("{C_PREFIX}{id}"),
        };
        serializer.serialize_str(&string)
    }
}
impl<'de> serde::Deserialize<'de> for CAttrId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct CAttrIdVisitor;

        impl<'de> serde::de::Visitor<'de> for CAttrIdVisitor {
            type Value = CAttrId;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("attr type-prefixed int CAttrId")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                if let Some(id_str) = v.strip_prefix(E_PREFIX) {
                    let id = CEveAttrId::from_str(id_str).map_err(|v| serde::de::Error::custom(v))?;
                    return Ok(Self::Value::Eve(id));
                }
                if let Some(id_str) = v.strip_prefix(C_PREFIX) {
                    let id = CCustomAttrId::from_str(id_str).map_err(|v| serde::de::Error::custom(v))?;
                    return Ok(Self::Value::Custom(id));
                }
                let msg = format!("expected an int prefixed by \"{E_PREFIX}\" or \"{C_PREFIX}\", got \"{v}\"");
                Err(serde::de::Error::custom(msg))
            }
        }
        deserializer.deserialize_str(CAttrIdVisitor)
    }
}
