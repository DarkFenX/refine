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
impl std::str::FromStr for EffectId {
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
    #[error("invalid int: {0}")]
    InvalidInt(#[from] std::num::ParseIntError),
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, derive_more::Display, derive_more::FromStr)]
pub struct DogmaEffectId(i32);
impl DogmaEffectId {
    pub const fn from_i32(id: i32) -> Self {
        Self(id)
    }
    pub const fn into_i32(self) -> i32 {
        self.0
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, derive_more::Display, derive_more::FromStr)]
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
    pub(in crate::api) fn from_aid(effect_aid: AEffectId) -> Self {
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
