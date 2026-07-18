use crate::{
    ad::AItemId,
    ed::{EEffectId, EItemId},
};

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
            Self::ScSystemWide(item_aid)
            | Self::ScSystemEmitter(item_aid)
            | Self::ScProxyEffect(item_aid)
            | Self::ScProxyTrap(item_aid)
            | Self::ScShipLink(item_aid) => Some(EItemId::from_i32(item_aid.into_i32())),
            Self::Dogma(_) | Self::Custom(_) => None,
        }
    }
    pub(in crate::ad) fn dc_dogma_effect(&self) -> Option<EEffectId> {
        match self {
            Self::Dogma(dogma_effect_aid) => Some(EEffectId::from_i32(dogma_effect_aid.into_i32())),
            Self::ScSystemWide(_)
            | Self::ScSystemEmitter(_)
            | Self::ScProxyEffect(_)
            | Self::ScProxyTrap(_)
            | Self::ScShipLink(_)
            | Self::Custom(_) => None,
        }
    }
}
