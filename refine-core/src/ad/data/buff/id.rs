use crate::{ed::EBuffId, util::round_f64_to_i32};

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum ABuffId {
    Eve(AEveBuffId),
    Custom(ACustomBuffId),
}

#[cfg_attr(
    feature = "serde-ad",
    derive(serde::Serialize, serde::Deserialize),
    serde(transparent)
)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, derive_more::Display)]
pub struct AEveBuffId(i32);
impl AEveBuffId {
    pub const fn from_i32(id: i32) -> Self {
        Self(id)
    }
    pub const fn into_i32(self) -> i32 {
        self.0
    }
}

#[cfg_attr(
    feature = "serde-ad",
    derive(serde::Serialize, serde::Deserialize),
    serde(transparent)
)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, derive_more::Display)]
pub struct ACustomBuffId(i32);
impl ACustomBuffId {
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
impl ABuffId {
    pub(in crate::ad) const fn from_eid(buff_eid: EBuffId) -> Self {
        Self::Eve(AEveBuffId(buff_eid.into_i32()))
    }
    pub(crate) fn try_eve_from_f64_rounded(id: f64) -> Option<Self> {
        Some(Self::Eve(AEveBuffId::try_from_f64_rounded(id)?))
    }
    pub(in crate::ad) fn dc_eve(&self) -> Option<EBuffId> {
        match self {
            Self::Eve(eve_buff_aid) => Some(EBuffId::from_i32(eve_buff_aid.into_i32())),
            Self::Custom(..) => None,
        }
    }
}
impl AEveBuffId {
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

impl std::fmt::Display for ABuffId {
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

    // Human-readable representation
    struct StrVisitor;
    impl<'de> Visitor<'de> for StrVisitor {
        type Value = ABuffId;
        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("string with buff type-prefixed integer")
        }
        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: Error,
        {
            if let Some(id_str) = v.strip_prefix(EVE_PREFIX) {
                let id = i32::from_str(id_str).map_err(Error::custom)?;
                return Ok(Self::Value::Eve(AEveBuffId::from_i32(id)));
            }
            if let Some(id_str) = v.strip_prefix(CUSTOM_PREFIX) {
                let id = i32::from_str(id_str).map_err(Error::custom)?;
                return Ok(Self::Value::Custom(ACustomBuffId::from_i32(id)));
            }
            let msg = format!("expected an int prefixed by \"{EVE_PREFIX}\" or \"{CUSTOM_PREFIX}\", received \"{v}\"");
            Err(Error::custom(msg))
        }
    }

    // Binary representation
    #[derive(serde::Serialize, serde::Deserialize)]
    #[serde(remote = "ABuffId")]
    enum ABuffIdDef {
        Eve(AEveBuffId),
        Custom(ACustomBuffId),
    }

    // Serialization
    impl Serialize for ABuffId {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match serializer.is_human_readable() {
                true => serializer.serialize_str(&self.to_string()),
                false => ABuffIdDef::serialize(self, serializer),
            }
        }
    }

    // Deserialization
    impl<'de> Deserialize<'de> for ABuffId {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            match deserializer.is_human_readable() {
                true => deserializer.deserialize_string(StrVisitor),
                false => ABuffIdDef::deserialize(deserializer),
            }
        }
    }
}
