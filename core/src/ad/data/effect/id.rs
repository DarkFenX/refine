use crate::{ad::AItemId, def::DefId, ed::EEffectId};

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
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
impl const From<EEffectId> for AEffectId {
    fn from(effect_eid: EEffectId) -> Self {
        Self::Dogma(ADogmaEffectId(effect_eid.into_inner()))
    }
}
impl std::fmt::Display for AEffectId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Dogma(id) => write!(f, "d{id}"),
            Self::ScSystemWide(id) => write!(f, "scsw{id}"),
            Self::ScSystemEmitter(id) => write!(f, "scse{id}"),
            Self::ScProxyEffect(id) => write!(f, "scpe{id}"),
            Self::ScProxyTrap(id) => write!(f, "scpt{id}"),
            Self::ScShipLink(id) => write!(f, "scsl{id}"),
            Self::Custom(id) => write!(f, "c{id}"),
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, derive_more::Display)]
pub struct ADogmaEffectId(DefId);
impl ADogmaEffectId {
    pub const fn new(id: DefId) -> Self {
        Self(id)
    }
    pub const fn into_inner(self) -> DefId {
        self.0
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, derive_more::Display)]
pub struct ACustomEffectId(DefId);
impl ACustomEffectId {
    pub const fn new(id: DefId) -> Self {
        Self(id)
    }
    pub const fn into_inner(self) -> DefId {
        self.0
    }
}
