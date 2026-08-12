use crate::{
    ad::AItemId,
    ed::{EEffectId, EItemId},
};

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum AEffectId {
    // ID of a general EVE effect
    Dogma(ADogmaEffectId),
    // Buff effect attached to an item, system-wide effect part
    SystemWide(AItemId),
    // Buff effect attached to an item, system buff emitter part
    SystemEmitter(AItemId),
    // Buff effect attached to an item, proximity effect part
    ProxyEffect(AItemId),
    // Buff effect attached to an item, proximity trap/trigger part
    ProxyTrigger(AItemId),
    // Buff effect attached to an item, ship link part
    ShipLink(AItemId),
    // ID of an effect created by the library
    Custom(ACustomEffectId),
}

#[cfg_attr(
    feature = "serde-ad",
    derive(serde::Serialize, serde::Deserialize),
    serde(transparent)
)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, derive_more::Display)]
pub struct ADogmaEffectId(i32);
impl ADogmaEffectId {
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
pub struct ACustomEffectId(i32);
impl ACustomEffectId {
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
impl AEffectId {
    pub(crate) const fn from_eid(effect_eid: EEffectId) -> Self {
        Self::Dogma(ADogmaEffectId(effect_eid.into_i32()))
    }
    pub(in crate::ad) fn dc_sc_item(&self) -> Option<EItemId> {
        match self {
            Self::SystemWide(item_aid)
            | Self::SystemEmitter(item_aid)
            | Self::ProxyEffect(item_aid)
            | Self::ProxyTrigger(item_aid)
            | Self::ShipLink(item_aid) => Some(EItemId::from_i32(item_aid.into_i32())),
            Self::Dogma(..) | Self::Custom(..) => None,
        }
    }
    pub(in crate::ad) fn dc_dogma_effect(&self) -> Option<EEffectId> {
        match self {
            Self::Dogma(dogma_effect_aid) => Some(EEffectId::from_i32(dogma_effect_aid.into_i32())),
            Self::SystemWide(..)
            | Self::SystemEmitter(..)
            | Self::ProxyEffect(..)
            | Self::ProxyTrigger(..)
            | Self::ShipLink(..)
            | Self::Custom(..) => None,
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
const PROXYTRIG_PREFIX: &str = "pt";
const SHIPLINK_PREFIX: &str = "sl";
const CUSTOM_PREFIX: &str = "c";

impl std::fmt::Display for AEffectId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Dogma(id) => write!(f, "{DOGMA_PREFIX}{id}"),
            Self::SystemWide(id) => write!(f, "{SYSWIDE_PREFIX}{id}"),
            Self::SystemEmitter(id) => write!(f, "{SYSEMIT_PREFIX}{id}"),
            Self::ProxyEffect(id) => write!(f, "{PROXYEFF_PREFIX}{id}"),
            Self::ProxyTrigger(id) => write!(f, "{PROXYTRIG_PREFIX}{id}"),
            Self::ShipLink(id) => write!(f, "{SHIPLINK_PREFIX}{id}"),
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
        type Value = AEffectId;
        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("string with effect type-prefixed integer")
        }
        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: Error,
        {
            // Process longer prefixes first in case of conflicting starting letters
            if let Some(id_str) = v.strip_prefix(SYSWIDE_PREFIX) {
                let id = i32::from_str(id_str).map_err(Error::custom)?;
                return Ok(Self::Value::SystemWide(AItemId::from_i32(id)));
            }
            if let Some(id_str) = v.strip_prefix(SYSEMIT_PREFIX) {
                let id = i32::from_str(id_str).map_err(Error::custom)?;
                return Ok(Self::Value::SystemEmitter(AItemId::from_i32(id)));
            }
            if let Some(id_str) = v.strip_prefix(PROXYEFF_PREFIX) {
                let id = i32::from_str(id_str).map_err(Error::custom)?;
                return Ok(Self::Value::ProxyEffect(AItemId::from_i32(id)));
            }
            if let Some(id_str) = v.strip_prefix(PROXYTRIG_PREFIX) {
                let id = i32::from_str(id_str).map_err(Error::custom)?;
                return Ok(Self::Value::ProxyTrigger(AItemId::from_i32(id)));
            }
            if let Some(id_str) = v.strip_prefix(SHIPLINK_PREFIX) {
                let id = i32::from_str(id_str).map_err(Error::custom)?;
                return Ok(Self::Value::ShipLink(AItemId::from_i32(id)));
            }
            if let Some(id_str) = v.strip_prefix(DOGMA_PREFIX) {
                let id = i32::from_str(id_str).map_err(Error::custom)?;
                return Ok(Self::Value::Dogma(ADogmaEffectId::from_i32(id)));
            }
            if let Some(id_str) = v.strip_prefix(CUSTOM_PREFIX) {
                let id = i32::from_str(id_str).map_err(Error::custom)?;
                return Ok(Self::Value::Custom(ACustomEffectId::from_i32(id)));
            }
            let msg = format!(
                "expected an int prefixed by \"{DOGMA_PREFIX}\", \"{SYSWIDE_PREFIX}\", \"{SYSEMIT_PREFIX}\", \"{PROXYEFF_PREFIX}\", \"{PROXYTRIG_PREFIX}\", \"{SHIPLINK_PREFIX}\", or \"{CUSTOM_PREFIX}\", received \"{v}\""
            );
            Err(Error::custom(msg))
        }
    }

    // Binary representation
    #[derive(serde::Serialize, serde::Deserialize)]
    #[serde(remote = "AEffectId")]
    enum AEffectIdDef {
        Dogma(ADogmaEffectId),
        SystemWide(AItemId),
        SystemEmitter(AItemId),
        ProxyEffect(AItemId),
        ProxyTrigger(AItemId),
        ShipLink(AItemId),
        Custom(ACustomEffectId),
    }

    // Serialization
    impl Serialize for AEffectId {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match serializer.is_human_readable() {
                true => serializer.serialize_str(&self.to_string()),
                false => AEffectIdDef::serialize(self, serializer),
            }
        }
    }

    // Deserialization
    impl<'de> Deserialize<'de> for AEffectId {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            match deserializer.is_human_readable() {
                true => deserializer.deserialize_string(StrVisitor),
                false => AEffectIdDef::deserialize(deserializer),
            }
        }
    }
}
