use crate::{
    ad::{ACustomEffectId, ADogmaEffectId, AEffectId},
    api::ItemTypeId,
};

const DOGMA_PREFIX: &str = "d";
const SC_SYSWIDE_PREFIX: &str = "scsw";
const SC_SYSEMIT_PREFIX: &str = "scse";
const SC_PROXYEFF_PREFIX: &str = "scpe";
const SC_PROXYTRAP_PREFIX: &str = "scpt";
const SC_SHIPLINK_PREFIX: &str = "scsl";
const CUSTOM_PREFIX: &str = "c";

/// ID of an effect.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum EffectId {
    /// ID of a general EVE effect.
    Dogma(DogmaEffectId),
    /// Space component effect attached to an item, system-wide effect part.
    ScSystemWide(ItemTypeId),
    /// Space component effect attached to an item, system buff emitter part.
    ScSystemEmitter(ItemTypeId),
    /// Space component effect attached to an item, proximity effect part.
    ScProxyEffect(ItemTypeId),
    /// Space component effect attached to an item, proximity trigger/trap part.
    ScProxyTrap(ItemTypeId),
    /// Space component effect attached to an item, ship link part.
    ScShipLink(ItemTypeId),
    /// ID of an effect created by the library.
    Custom(CustomEffectId),
}
impl std::fmt::Display for EffectId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Dogma(id) => write!(f, "{DOGMA_PREFIX}{id}"),
            Self::ScSystemWide(id) => write!(f, "{SC_SYSWIDE_PREFIX}{id}"),
            Self::ScSystemEmitter(id) => write!(f, "{SC_SYSEMIT_PREFIX}{id}"),
            Self::ScProxyEffect(id) => write!(f, "{SC_PROXYEFF_PREFIX}{id}"),
            Self::ScProxyTrap(id) => write!(f, "{SC_PROXYTRAP_PREFIX}{id}"),
            Self::ScShipLink(id) => write!(f, "{SC_SHIPLINK_PREFIX}{id}"),
            Self::Custom(id) => write!(f, "{CUSTOM_PREFIX}{id}"),
        }
    }
}

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
            AEffectId::ScSystemWide(id) => Self::ScSystemWide(ItemTypeId::from_aid(id)),
            AEffectId::ScSystemEmitter(id) => Self::ScSystemEmitter(ItemTypeId::from_aid(id)),
            AEffectId::ScProxyEffect(id) => Self::ScProxyEffect(ItemTypeId::from_aid(id)),
            AEffectId::ScProxyTrap(id) => Self::ScProxyTrap(ItemTypeId::from_aid(id)),
            AEffectId::ScShipLink(id) => Self::ScShipLink(ItemTypeId::from_aid(id)),
            AEffectId::Custom(id) => Self::Custom(CustomEffectId(id.into_i32())),
        }
    }
    pub(in crate::api) fn into_aid(self) -> AEffectId {
        match self {
            EffectId::Dogma(id) => AEffectId::Dogma(ADogmaEffectId::from_i32(id.0)),
            EffectId::ScSystemWide(id) => AEffectId::ScSystemWide(id.into_aid()),
            EffectId::ScSystemEmitter(id) => AEffectId::ScSystemEmitter(id.into_aid()),
            EffectId::ScProxyEffect(id) => AEffectId::ScProxyEffect(id.into_aid()),
            EffectId::ScProxyTrap(id) => AEffectId::ScProxyTrap(id.into_aid()),
            EffectId::ScShipLink(id) => AEffectId::ScShipLink(id.into_aid()),
            EffectId::Custom(id) => AEffectId::Custom(ACustomEffectId::from_i32(id.0)),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Custom serialization/deserialization
////////////////////////////////////////////////////////////////////////////////////////////////////
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
            if let Some(id_str) = s.strip_prefix(SC_SYSWIDE_PREFIX) {
                return Ok(Self::ScSystemWide(ItemTypeId::from_str(id_str)?));
            }
            if let Some(id_str) = s.strip_prefix(SC_SYSEMIT_PREFIX) {
                return Ok(Self::ScSystemEmitter(ItemTypeId::from_str(id_str)?));
            }
            if let Some(id_str) = s.strip_prefix(SC_PROXYEFF_PREFIX) {
                return Ok(Self::ScProxyEffect(ItemTypeId::from_str(id_str)?));
            }
            if let Some(id_str) = s.strip_prefix(SC_PROXYTRAP_PREFIX) {
                return Ok(Self::ScProxyTrap(ItemTypeId::from_str(id_str)?));
            }
            if let Some(id_str) = s.strip_prefix(SC_SHIPLINK_PREFIX) {
                return Ok(Self::ScShipLink(ItemTypeId::from_str(id_str)?));
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

    #[derive(thiserror::Error, Debug)]
    pub enum EffectIdParseError {
        #[error(
            "invalid prefix, expected \"{d}\", \"{scsw}\", \"{scse}\", \"{scpe}\", \"{scpt}\", \"{scsl}\", or \"{c}\" prefix",
            d = DOGMA_PREFIX,
            scsw = SC_SYSWIDE_PREFIX,
            scse = SC_SYSEMIT_PREFIX,
            scpe = SC_PROXYEFF_PREFIX,
            scpt = SC_PROXYTRAP_PREFIX,
            scsl = SC_SHIPLINK_PREFIX,
            c = CUSTOM_PREFIX,
        )]
        InvalidPrefix,
        #[error("{0}")]
        InvalidInt(#[from] std::num::ParseIntError),
    }

    impl Serialize for EffectId {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let string = format!("{self}");
            serializer.serialize_str(&string)
        }
    }

    impl<'de> Deserialize<'de> for EffectId {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            struct VisitorState;

            impl<'de> Visitor<'de> for VisitorState {
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

            deserializer.deserialize_str(VisitorState)
        }
    }
}
