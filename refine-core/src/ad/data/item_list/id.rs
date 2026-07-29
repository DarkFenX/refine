use crate::{ed::EItemListId, util::round_f64_to_i32};

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum AItemListId {
    Eve(AEveItemListId),
    Custom(ACustomItemListId),
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, derive_more::Display)]
pub struct AEveItemListId(i32);
impl AEveItemListId {
    pub const fn from_i32(id: i32) -> Self {
        Self(id)
    }
    pub const fn into_i32(self) -> i32 {
        self.0
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, derive_more::Display)]
pub struct ACustomItemListId(i32);
impl ACustomItemListId {
    pub const fn from_i32(id: i32) -> Self {
        Self(id)
    }
    pub const fn into_i32(self) -> i32 {
        self.0
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl AItemListId {
    pub(in crate::ad) const fn from_eid(item_list_eid: EItemListId) -> Self {
        Self::Eve(AEveItemListId(item_list_eid.into_i32()))
    }
    pub(crate) fn try_eve_from_f64_rounded(id: f64) -> Option<Self> {
        Some(Self::Eve(AEveItemListId::try_from_f64_rounded(id)?))
    }
    pub(in crate::ad) fn dc_eve(&self) -> Option<EItemListId> {
        match self {
            Self::Eve(eve_item_list_aid) => Some(EItemListId::from_i32(eve_item_list_aid.into_i32())),
            _ => None,
        }
    }
}
impl AEveItemListId {
    fn try_from_f64_rounded(id: f64) -> Option<Self> {
        match round_f64_to_i32(id) {
            // Reference to 0 is considered as no reference throughout EVE data
            0 => None,
            id => Some(Self(id)),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Custom de/serialization
////////////////////////////////////////////////////////////////////////////////////////////////////
const EVE_PREFIX: &str = "e";
const CUSTOM_PREFIX: &str = "c";

impl std::fmt::Display for AItemListId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Eve(id) => write!(f, "{EVE_PREFIX}{id}"),
            Self::Custom(id) => write!(f, "{CUSTOM_PREFIX}{id}"),
        }
    }
}

#[cfg(feature = "serde-ad")]
mod custom_serde_ad {
    use std::str::FromStr;

    use serde::{
        de::{Deserialize, Deserializer, Error, Visitor},
        ser::{Serialize, Serializer},
    };

    use super::*;

    impl Serialize for AItemListId {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            serializer.serialize_str(&self.to_string())
        }
    }

    impl<'de> Deserialize<'de> for AItemListId {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            struct VisitorImpl;

            impl<'de> Visitor<'de> for VisitorImpl {
                type Value = AItemListId;

                fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                    formatter.write_str("string with item list type-prefixed integer")
                }

                fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
                where
                    E: Error,
                {
                    if let Some(id_str) = v.strip_prefix(EVE_PREFIX) {
                        let id = i32::from_str(id_str).map_err(Error::custom)?;
                        return Ok(Self::Value::Eve(AEveItemListId::from_i32(id)));
                    }
                    if let Some(id_str) = v.strip_prefix(CUSTOM_PREFIX) {
                        let id = i32::from_str(id_str).map_err(Error::custom)?;
                        return Ok(Self::Value::Custom(ACustomItemListId::from_i32(id)));
                    }
                    let msg = format!(
                        "expected an int prefixed by \"{EVE_PREFIX}\" or \"{CUSTOM_PREFIX}\", received \"{v}\""
                    );
                    Err(Error::custom(msg))
                }
            }

            deserializer.deserialize_str(VisitorImpl)
        }
    }
}
