use std::str::FromStr;

use crate::cacher_json::data::{CCustomItemListId, CEveItemListId};

const E_PREFIX: &str = "e";
const C_PREFIX: &str = "c";

#[derive(Copy, Clone)]
pub(in crate::cacher_json) enum CItemListId {
    Eve(CEveItemListId),
    Custom(CCustomItemListId),
}
impl From<&rc::ad::AItemListId> for CItemListId {
    fn from(a_item_list_id: &rc::ad::AItemListId) -> Self {
        match a_item_list_id {
            rc::ad::AItemListId::Eve(id) => Self::Eve(*id),
            rc::ad::AItemListId::Custom(id) => Self::Custom(*id),
        }
    }
}
impl From<&CItemListId> for rc::ad::AItemListId {
    fn from(c_item_list_id: &CItemListId) -> Self {
        match c_item_list_id {
            CItemListId::Eve(id) => Self::Eve(*id),
            CItemListId::Custom(id) => Self::Custom(*id),
        }
    }
}
impl serde::Serialize for CItemListId {
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
impl<'de> serde::Deserialize<'de> for CItemListId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct CItemListIdVisitor;

        impl<'de> serde::de::Visitor<'de> for CItemListIdVisitor {
            type Value = CItemListId;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("item list type-prefixed int CItemListId")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                if let Some(id_str) = v.strip_prefix(E_PREFIX) {
                    let id = CEveItemListId::from_str(id_str).map_err(|v| serde::de::Error::custom(v))?;
                    return Ok(Self::Value::Eve(id));
                }
                if let Some(id_str) = v.strip_prefix(C_PREFIX) {
                    let id = CCustomItemListId::from_str(id_str).map_err(|v| serde::de::Error::custom(v))?;
                    return Ok(Self::Value::Custom(id));
                }
                let msg = format!("expected an int prefixed by \"{E_PREFIX}\" or \"{C_PREFIX}\", got \"{v}\"");
                Err(serde::de::Error::custom(msg))
            }
        }
        deserializer.deserialize_str(CItemListIdVisitor)
    }
}
