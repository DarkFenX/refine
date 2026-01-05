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
    pub const fn new(id: i32) -> Self {
        Self(id)
    }
    pub const fn into_inner(self) -> i32 {
        self.0
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, derive_more::Display, derive_more::FromStr)]
pub struct CustomEffectId(i32);
impl CustomEffectId {
    pub const fn new(id: i32) -> Self {
        Self(id)
    }
    pub const fn into_inner(self) -> i32 {
        self.0
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl From<AEffectId> for EffectId {
    fn from(effect_aid: AEffectId) -> Self {
        match effect_aid {
            AEffectId::Dogma(id) => Self::Dogma(DogmaEffectId::new(id.into_inner())),
            AEffectId::ScSystemWide(id) => Self::ScSystemWide(ItemTypeId::new(id.into_inner())),
            AEffectId::ScSystemEmitter(id) => Self::ScSystemEmitter(id.into()),
            AEffectId::ScProxyEffect(id) => Self::ScProxyEffect(id.into()),
            AEffectId::ScProxyTrap(id) => Self::ScProxyTrap(id.into()),
            AEffectId::ScShipLink(id) => Self::ScShipLink(id.into()),
            AEffectId::Custom(id) => Self::Custom(CustomEffectId::new(id.into_inner())),
        }
    }
}
impl From<&AEffectId> for EffectId {
    fn from(effect_aid: &AEffectId) -> Self {
        match effect_aid {
            AEffectId::Dogma(id) => Self::Dogma(DogmaEffectId::new(id.into_inner())),
            AEffectId::ScSystemWide(id) => Self::ScSystemWide(id.into()),
            AEffectId::ScSystemEmitter(id) => Self::ScSystemEmitter(id.into()),
            AEffectId::ScProxyEffect(id) => Self::ScProxyEffect(id.into()),
            AEffectId::ScProxyTrap(id) => Self::ScProxyTrap(id.into()),
            AEffectId::ScShipLink(id) => Self::ScShipLink(id.into()),
            AEffectId::Custom(id) => Self::Custom(CustomEffectId::new(id.into_inner())),
        }
    }
}
impl From<EffectId> for AEffectId {
    fn from(effect_id: EffectId) -> Self {
        match effect_id {
            EffectId::Dogma(id) => Self::Dogma(ADogmaEffectId::new(id.into_inner())),
            EffectId::ScSystemWide(id) => Self::ScSystemWide(id.into()),
            EffectId::ScSystemEmitter(id) => Self::ScSystemEmitter(id.into()),
            EffectId::ScProxyEffect(id) => Self::ScProxyEffect(id.into()),
            EffectId::ScProxyTrap(id) => Self::ScProxyTrap(id.into()),
            EffectId::ScShipLink(id) => Self::ScShipLink(id.into()),
            EffectId::Custom(id) => Self::Custom(ACustomEffectId::new(id.into_inner())),
        }
    }
}
impl From<&EffectId> for AEffectId {
    fn from(effect_id: &EffectId) -> Self {
        match effect_id {
            EffectId::Dogma(id) => Self::Dogma(ADogmaEffectId::new(id.into_inner())),
            EffectId::ScSystemWide(id) => Self::ScSystemWide(id.into()),
            EffectId::ScSystemEmitter(id) => Self::ScSystemEmitter(id.into()),
            EffectId::ScProxyEffect(id) => Self::ScProxyEffect(id.into()),
            EffectId::ScProxyTrap(id) => Self::ScProxyTrap(id.into()),
            EffectId::ScShipLink(id) => Self::ScShipLink(id.into()),
            EffectId::Custom(id) => Self::Custom(ACustomEffectId::new(id.into_inner())),
        }
    }
}
