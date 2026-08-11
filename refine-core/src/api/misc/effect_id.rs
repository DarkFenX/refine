use crate::{
    ItemTypeId,
    ad::{ACustomEffectId, ADogmaEffectId, AEffectId},
};

/// ID of an effect.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum EffectId {
    /// ID of a general EVE effect.
    Dogma(DogmaEffectId),
    /// Buff effect attached to an item, system-wide effect part.
    SystemWide(ItemTypeId),
    /// Buff effect attached to an item, system buff emitter part.
    SystemEmitter(ItemTypeId),
    /// Buff effect attached to an item, proximity effect part.
    ProxyEffect(ItemTypeId),
    /// Buff effect attached to an item, proximity trigger/trap part.
    ProxyTrap(ItemTypeId),
    /// Buff effect attached to an item, ship link part.
    ShipLink(ItemTypeId),
    /// ID of an effect created by the library.
    Custom(CustomEffectId),
}

/// ID of an effect defined by EVE data.
#[cfg_attr(feature = "serde", derive(derive_more::FromStr))]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, derive_more::Display)]
pub struct DogmaEffectId(i32);
impl DogmaEffectId {
    pub const fn from_i32(id: i32) -> Self {
        Self(id)
    }
    pub const fn into_i32(self) -> i32 {
        self.0
    }
}

/// ID of an effect defined by the library, with no EVE counterpart.
#[cfg_attr(feature = "serde", derive(derive_more::FromStr))]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, derive_more::Display)]
pub struct CustomEffectId(i32);
impl CustomEffectId {
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
impl EffectId {
    pub(crate) fn from_aid(effect_aid: AEffectId) -> Self {
        match effect_aid {
            AEffectId::Dogma(id) => Self::Dogma(DogmaEffectId(id.into_i32())),
            AEffectId::SystemWide(id) => Self::SystemWide(ItemTypeId::from_aid(id)),
            AEffectId::SystemEmitter(id) => Self::SystemEmitter(ItemTypeId::from_aid(id)),
            AEffectId::ProxyEffect(id) => Self::ProxyEffect(ItemTypeId::from_aid(id)),
            AEffectId::ProxyTrap(id) => Self::ProxyTrap(ItemTypeId::from_aid(id)),
            AEffectId::ShipLink(id) => Self::ShipLink(ItemTypeId::from_aid(id)),
            AEffectId::Custom(id) => Self::Custom(CustomEffectId(id.into_i32())),
        }
    }
    pub(in crate::api) fn into_aid(self) -> AEffectId {
        match self {
            EffectId::Dogma(id) => AEffectId::Dogma(ADogmaEffectId::from_i32(id.0)),
            EffectId::SystemWide(id) => AEffectId::SystemWide(id.into_aid()),
            EffectId::SystemEmitter(id) => AEffectId::SystemEmitter(id.into_aid()),
            EffectId::ProxyEffect(id) => AEffectId::ProxyEffect(id.into_aid()),
            EffectId::ProxyTrap(id) => AEffectId::ProxyTrap(id.into_aid()),
            EffectId::ShipLink(id) => AEffectId::ShipLink(id.into_aid()),
            EffectId::Custom(id) => AEffectId::Custom(ACustomEffectId::from_i32(id.0)),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Custom de/serialization
////////////////////////////////////////////////////////////////////////////////////////////////////
const DOGMA_PREFIX: &str = "d";
const SYSWIDE_PREFIX: &str = "sw";
const SYSEMIT_PREFIX: &str = "se";
const PROXYEFF_PREFIX: &str = "pe";
const PROXYTRAP_PREFIX: &str = "pt";
const SHIPLINK_PREFIX: &str = "sl";
const CUSTOM_PREFIX: &str = "c";

impl std::fmt::Display for EffectId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Dogma(id) => write!(f, "{DOGMA_PREFIX}{id}"),
            Self::SystemWide(id) => write!(f, "{SYSWIDE_PREFIX}{id}"),
            Self::SystemEmitter(id) => write!(f, "{SYSEMIT_PREFIX}{id}"),
            Self::ProxyEffect(id) => write!(f, "{PROXYEFF_PREFIX}{id}"),
            Self::ProxyTrap(id) => write!(f, "{PROXYTRAP_PREFIX}{id}"),
            Self::ShipLink(id) => write!(f, "{SHIPLINK_PREFIX}{id}"),
            Self::Custom(id) => write!(f, "{CUSTOM_PREFIX}{id}"),
        }
    }
}

#[cfg(feature = "serde")]
mod custom_serde {
    use std::str::FromStr;

    use serde::{
        de::{Deserialize, Deserializer, Error, Visitor},
        ser::{Serialize, Serializer},
    };

    use super::*;

    impl FromStr for EffectId {
        type Err = EffectIdParseError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            // Process longer prefixes first in case of conflicting starting letters
            if let Some(id_str) = s.strip_prefix(SYSWIDE_PREFIX) {
                return Ok(Self::SystemWide(ItemTypeId::from_str(id_str)?));
            }
            if let Some(id_str) = s.strip_prefix(SYSEMIT_PREFIX) {
                return Ok(Self::SystemEmitter(ItemTypeId::from_str(id_str)?));
            }
            if let Some(id_str) = s.strip_prefix(PROXYEFF_PREFIX) {
                return Ok(Self::ProxyEffect(ItemTypeId::from_str(id_str)?));
            }
            if let Some(id_str) = s.strip_prefix(PROXYTRAP_PREFIX) {
                return Ok(Self::ProxyTrap(ItemTypeId::from_str(id_str)?));
            }
            if let Some(id_str) = s.strip_prefix(SHIPLINK_PREFIX) {
                return Ok(Self::ShipLink(ItemTypeId::from_str(id_str)?));
            }
            if let Some(id_str) = s.strip_prefix(DOGMA_PREFIX) {
                return Ok(Self::Dogma(DogmaEffectId::from_str(id_str)?));
            }
            if let Some(id_str) = s.strip_prefix(CUSTOM_PREFIX) {
                return Ok(Self::Custom(CustomEffectId::from_str(id_str)?));
            }
            Err(EffectIdParseError::InvalidPrefix)
        }
    }

    #[derive(Debug, thiserror::Error)]
    pub enum EffectIdParseError {
        #[error(
            "invalid prefix, expected \"{DOGMA_PREFIX}\", \"{SYSWIDE_PREFIX}\", \"{SYSEMIT_PREFIX}\", \"{PROXYEFF_PREFIX}\", \"{PROXYTRAP_PREFIX}\", \"{SHIPLINK_PREFIX}\", or \"{CUSTOM_PREFIX}\" prefix"
        )]
        InvalidPrefix,
        #[error(transparent)]
        InvalidInt(#[from] std::num::ParseIntError),
    }

    impl Serialize for EffectId {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            serializer.serialize_str(&self.to_string())
        }
    }

    impl<'de> Deserialize<'de> for EffectId {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            struct VisitorImpl;

            impl<'de> Visitor<'de> for VisitorImpl {
                type Value = EffectId;

                fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                    formatter.write_str("string with effect type-prefixed integer")
                }

                fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
                where
                    E: Error,
                {
                    Self::Value::from_str(v).map_err(Error::custom)
                }
            }

            deserializer.deserialize_string(VisitorImpl)
        }
    }
}
