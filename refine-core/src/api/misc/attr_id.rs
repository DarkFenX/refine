use crate::ad::{AAttrId, ACustomAttrId, AEveAttrId};

const EVE_PREFIX: &str = "e";
const CUSTOM_PREFIX: &str = "c";

/// ID of an attribute.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum AttrId {
    /// ID of an EVE attribute.
    Eve(EveAttrId),
    /// ID of an attribute created by the library.
    Custom(CustomAttrId),
}
impl std::fmt::Display for AttrId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Eve(id) => write!(f, "{EVE_PREFIX}{id}"),
            Self::Custom(id) => write!(f, "{CUSTOM_PREFIX}{id}"),
        }
    }
}

#[cfg_attr(feature = "serde", derive(derive_more::FromStr))]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, derive_more::Display)]
pub struct EveAttrId(i32);
impl EveAttrId {
    pub const fn from_i32(id: i32) -> Self {
        Self(id)
    }
    pub const fn into_i32(self) -> i32 {
        self.0
    }
}

#[cfg_attr(feature = "serde", derive(derive_more::FromStr))]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, derive_more::Display)]
pub struct CustomAttrId(i32);
impl CustomAttrId {
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
impl AttrId {
    pub(crate) fn from_aid(attr_aid: AAttrId) -> Self {
        match attr_aid {
            AAttrId::Eve(id) => Self::Eve(EveAttrId(id.into_i32())),
            AAttrId::Custom(id) => Self::Custom(CustomAttrId(id.into_i32())),
        }
    }
    pub(in crate::api) fn into_aid(self) -> AAttrId {
        match self {
            AttrId::Eve(id) => AAttrId::Eve(AEveAttrId::from_i32(id.0)),
            AttrId::Custom(id) => AAttrId::Custom(ACustomAttrId::from_i32(id.0)),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Custom serialization/deserialization
////////////////////////////////////////////////////////////////////////////////////////////////////
#[cfg(feature = "serde")]
mod custom_serde {
    use std::str::FromStr;

    use super::*;

    impl FromStr for AttrId {
        type Err = AttrIdParseError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            if let Some(id_str) = s.strip_prefix(EVE_PREFIX) {
                return Ok(Self::Eve(EveAttrId::from_str(id_str)?));
            }
            if let Some(id_str) = s.strip_prefix(CUSTOM_PREFIX) {
                return Ok(Self::Custom(CustomAttrId::from_str(id_str)?));
            }
            Err(AttrIdParseError::InvalidPrefix)
        }
    }

    #[derive(thiserror::Error, Debug)]
    pub enum AttrIdParseError {
        #[error("invalid prefix, expected \"{eve}\" or \"{custom}\" prefix", eve = EVE_PREFIX, custom = CUSTOM_PREFIX)]
        InvalidPrefix,
        #[error("{0}")]
        InvalidInt(#[from] std::num::ParseIntError),
    }

    impl serde::Serialize for AttrId {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::ser::Serializer,
        {
            let string = format!("{self}");
            serializer.serialize_str(&string)
        }
    }

    impl<'de> serde::Deserialize<'de> for AttrId {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::de::Deserializer<'de>,
        {
            struct Visitor;

            impl<'de> serde::de::Visitor<'de> for Visitor {
                type Value = AttrId;

                fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                    formatter.write_str("string with attribute type-prefixed integer")
                }

                fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
                where
                    E: serde::de::Error,
                {
                    Self::Value::from_str(v).map_err(serde::de::Error::custom)
                }
            }

            deserializer.deserialize_str(Visitor)
        }
    }
}
