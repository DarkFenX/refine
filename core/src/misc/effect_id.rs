use crate::{
    ad::AEffectId,
    def::{CustomEffectId, DogmaEffectId, ItemTypeId},
};

/// ID of an effect.
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
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
impl From<AEffectId> for EffectId {
    fn from(a_effect_id: AEffectId) -> Self {
        match a_effect_id {
            AEffectId::Dogma(id) => Self::Dogma(id),
            AEffectId::ScSystemWide(id) => Self::ScSystemWide(id),
            AEffectId::ScSystemEmitter(id) => Self::ScSystemEmitter(id),
            AEffectId::ScProxyEffect(id) => Self::ScProxyEffect(id),
            AEffectId::ScProxyTrap(id) => Self::ScProxyTrap(id),
            AEffectId::ScShipLink(id) => Self::ScShipLink(id),
            AEffectId::Custom(id) => Self::Custom(id),
        }
    }
}
impl From<&AEffectId> for EffectId {
    fn from(a_effect_id: &AEffectId) -> Self {
        match a_effect_id {
            AEffectId::Dogma(id) => Self::Dogma(*id),
            AEffectId::ScSystemWide(id) => Self::ScSystemWide(*id),
            AEffectId::ScSystemEmitter(id) => Self::ScSystemEmitter(*id),
            AEffectId::ScProxyEffect(id) => Self::ScProxyEffect(*id),
            AEffectId::ScProxyTrap(id) => Self::ScProxyTrap(*id),
            AEffectId::ScShipLink(id) => Self::ScShipLink(*id),
            AEffectId::Custom(id) => Self::Custom(*id),
        }
    }
}
impl From<EffectId> for AEffectId {
    fn from(effect_id: EffectId) -> Self {
        match effect_id {
            EffectId::Dogma(id) => Self::Dogma(id),
            EffectId::ScSystemWide(id) => Self::ScSystemWide(id),
            EffectId::ScSystemEmitter(id) => Self::ScSystemEmitter(id),
            EffectId::ScProxyEffect(id) => Self::ScProxyEffect(id),
            EffectId::ScProxyTrap(id) => Self::ScProxyTrap(id),
            EffectId::ScShipLink(id) => Self::ScShipLink(id),
            EffectId::Custom(id) => Self::Custom(id),
        }
    }
}
impl From<&EffectId> for AEffectId {
    fn from(effect_id: &EffectId) -> Self {
        match effect_id {
            EffectId::Dogma(id) => Self::Dogma(*id),
            EffectId::ScSystemWide(id) => Self::ScSystemWide(*id),
            EffectId::ScSystemEmitter(id) => Self::ScSystemEmitter(*id),
            EffectId::ScProxyEffect(id) => Self::ScProxyEffect(*id),
            EffectId::ScProxyTrap(id) => Self::ScProxyTrap(*id),
            EffectId::ScShipLink(id) => Self::ScShipLink(*id),
            EffectId::Custom(id) => Self::Custom(*id),
        }
    }
}
