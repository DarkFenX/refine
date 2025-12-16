use std::str::FromStr;

use crate::cacher_json::data::{CCustomBuffId, CEveBuffId};

const E_PREFIX: &str = "e";
const C_PREFIX: &str = "c";

#[derive(Eq, PartialEq, Hash)]
pub(in crate::cacher_json) enum CBuffId {
    Eve(CEveBuffId),
    Custom(CCustomBuffId),
}
impl From<&rc::ad::ABuffId> for CBuffId {
    fn from(a_buff_id: &rc::ad::ABuffId) -> Self {
        match a_buff_id {
            rc::ad::ABuffId::Eve(id) => Self::Eve(*id),
            rc::ad::ABuffId::Custom(id) => Self::Custom(*id),
        }
    }
}
impl From<&CBuffId> for rc::ad::ABuffId {
    fn from(c_buff_id: &CBuffId) -> Self {
        match c_buff_id {
            CBuffId::Eve(id) => Self::Eve(*id),
            CBuffId::Custom(id) => Self::Custom(*id),
        }
    }
}
impl serde::Serialize for CBuffId {
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
impl<'de> serde::Deserialize<'de> for CBuffId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct CBuffIdVisitor;

        impl<'de> serde::de::Visitor<'de> for CBuffIdVisitor {
            type Value = CBuffId;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("buff type-prefixed int CBuffId")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                if let Some(id_str) = v.strip_prefix(E_PREFIX) {
                    let id = CEveBuffId::from_str(id_str).map_err(|e| serde::de::Error::custom(e))?;
                    return Ok(Self::Value::Eve(id));
                }
                if let Some(id_str) = v.strip_prefix(C_PREFIX) {
                    let id = CCustomBuffId::from_str(id_str).map_err(|e| serde::de::Error::custom(e))?;
                    return Ok(Self::Value::Custom(id));
                }
                let msg = format!("expected an int prefixed by \"{E_PREFIX}\" or \"{C_PREFIX}\", got \"{v}\"");
                Err(serde::de::Error::custom(msg))
            }
        }
        deserializer.deserialize_str(CBuffIdVisitor)
    }
}
