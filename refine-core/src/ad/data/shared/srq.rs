use crate::ad::AItemId;

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum AModifierSrq {
    SelfRef,
    ItemId(AItemId),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Custom de/serialization
////////////////////////////////////////////////////////////////////////////////////////////////////
#[cfg(feature = "serde-ad")]
mod custom_serde {
    use serde::{
        de::{Deserialize, Deserializer},
        ser::{Serialize, Serializer},
    };

    use super::*;

    impl Serialize for AModifierSrq {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::SelfRef => serializer.serialize_none(),
                Self::ItemId(id) => serializer.serialize_some(id),
            }
        }
    }

    impl<'de> Deserialize<'de> for AModifierSrq {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            match Option::deserialize(deserializer)? {
                Some(item_aid) => Ok(Self::ItemId(item_aid)),
                None => Ok(Self::SelfRef),
            }
        }
    }
}
