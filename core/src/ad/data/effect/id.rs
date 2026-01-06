use crate::{ad::AItemId, ed::EEffectId};

const DOGMA_PREFIX: &str = "d";
const SC_SYSWIDE_PREFIX: &str = "scsw";
const SC_SYSEMIT_PREFIX: &str = "scse";
const SC_PROXYEFF_PREFIX: &str = "scpe";
const SC_PROXYTRAP_PREFIX: &str = "scpt";
const SC_SHIPLINK_PREFIX: &str = "scsl";
const CUSTOM_PREFIX: &str = "c";

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum AEffectId {
    // ID of a general EVE effect
    Dogma(ADogmaEffectId),
    // Space component effect attached to an item, system-wide effect part
    ScSystemWide(AItemId),
    // Space component effect attached to an item, system buff emitter part
    ScSystemEmitter(AItemId),
    // Space component effect attached to an item, proximity effect part
    ScProxyEffect(AItemId),
    // Space component effect attached to an item, proximity trap/trigger part
    ScProxyTrap(AItemId),
    // Space component effect attached to an item, ship link part
    ScShipLink(AItemId),
    // ID of an effect created by the library
    Custom(ACustomEffectId),
}
impl std::fmt::Display for AEffectId {
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
impl std::str::FromStr for AEffectId {
    type Err = AEffectIdParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Process longer prefixes first in case of conflicting starting letters
        if let Some(id_str) = s.strip_prefix(SC_SYSWIDE_PREFIX) {
            return Ok(Self::ScSystemWide(AItemId::from_str(id_str)?));
        }
        if let Some(id_str) = s.strip_prefix(SC_SYSEMIT_PREFIX) {
            return Ok(Self::ScSystemEmitter(AItemId::from_str(id_str)?));
        }
        if let Some(id_str) = s.strip_prefix(SC_PROXYEFF_PREFIX) {
            return Ok(Self::ScProxyEffect(AItemId::from_str(id_str)?));
        }
        if let Some(id_str) = s.strip_prefix(SC_PROXYTRAP_PREFIX) {
            return Ok(Self::ScProxyTrap(AItemId::from_str(id_str)?));
        }
        if let Some(id_str) = s.strip_prefix(SC_SHIPLINK_PREFIX) {
            return Ok(Self::ScShipLink(AItemId::from_str(id_str)?));
        }
        if let Some(id_str) = s.strip_prefix(DOGMA_PREFIX) {
            return Ok(Self::Dogma(ADogmaEffectId::from_str(id_str)?));
        }
        if let Some(id_str) = s.strip_prefix(CUSTOM_PREFIX) {
            return Ok(Self::Custom(ACustomEffectId::from_str(id_str)?));
        }
        Err(AEffectIdParseError::InvalidPrefix)
    }
}
#[derive(thiserror::Error, Debug)]
pub enum AEffectIdParseError {
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
pub struct ADogmaEffectId(i32);
impl ADogmaEffectId {
    pub const fn from_i32(id: i32) -> Self {
        Self(id)
    }
    pub const fn into_i32(self) -> i32 {
        self.0
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, derive_more::Display, derive_more::FromStr)]
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
}
