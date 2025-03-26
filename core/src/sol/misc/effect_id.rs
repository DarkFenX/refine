use crate::{
    ad,
    sol::{CustomEffectId, DogmaEffectId, ItemTypeId},
};

/// ID of an effect.
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum EffectId {
    /// ID of a general EVE effect.
    Dogma(DogmaEffectId),
    /// Space component effect attached to an item, system buff emitter part.
    ScSystemEmitter(ItemTypeId),
    /// Space component effect attached to an item, proximity effect part.
    ScProxyEffect(ItemTypeId),
    /// Space component effect attached to an item, proximity trigger/trap part.
    ScProxyTrigger(ItemTypeId),
    /// Space component effect attached to an item, ship link part.
    ScShipLink(ItemTypeId),
    /// ID of an effect created by the library.
    Custom(CustomEffectId),
}
impl std::fmt::Display for EffectId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Dogma(id) => write!(f, "d{}", id),
            Self::ScSystemEmitter(id) => write!(f, "scse{}", id),
            Self::ScProxyEffect(id) => write!(f, "scpe{}", id),
            Self::ScProxyTrigger(id) => write!(f, "scpt{}", id),
            Self::ScShipLink(id) => write!(f, "scsl{}", id),
            Self::Custom(id) => write!(f, "c{}", id),
        }
    }
}
impl From<ad::AEffectId> for EffectId {
    fn from(a_effect_id: ad::AEffectId) -> Self {
        match a_effect_id {
            ad::AEffectId::Dogma(id) => Self::Dogma(id),
            ad::AEffectId::ScSystemEmitter(id) => Self::ScSystemEmitter(id),
            ad::AEffectId::ScProxyEffect(id) => Self::ScProxyEffect(id),
            ad::AEffectId::ScProxyTrigger(id) => Self::ScProxyTrigger(id),
            ad::AEffectId::ScShipLink(id) => Self::ScShipLink(id),
            ad::AEffectId::Custom(id) => Self::Custom(id),
        }
    }
}
impl From<&ad::AEffectId> for EffectId {
    fn from(a_effect_id: &ad::AEffectId) -> Self {
        match a_effect_id {
            ad::AEffectId::Dogma(id) => Self::Dogma(*id),
            ad::AEffectId::ScSystemEmitter(id) => Self::ScSystemEmitter(*id),
            ad::AEffectId::ScProxyEffect(id) => Self::ScProxyEffect(*id),
            ad::AEffectId::ScProxyTrigger(id) => Self::ScProxyTrigger(*id),
            ad::AEffectId::ScShipLink(id) => Self::ScShipLink(*id),
            ad::AEffectId::Custom(id) => Self::Custom(*id),
        }
    }
}
impl From<EffectId> for ad::AEffectId {
    fn from(effect_id: EffectId) -> Self {
        match effect_id {
            EffectId::Dogma(id) => Self::Dogma(id),
            EffectId::ScSystemEmitter(id) => Self::ScSystemEmitter(id),
            EffectId::ScProxyEffect(id) => Self::ScProxyEffect(id),
            EffectId::ScProxyTrigger(id) => Self::ScProxyTrigger(id),
            EffectId::ScShipLink(id) => Self::ScShipLink(id),
            EffectId::Custom(id) => Self::Custom(id),
        }
    }
}
impl From<&EffectId> for ad::AEffectId {
    fn from(effect_id: &EffectId) -> Self {
        match effect_id {
            EffectId::Dogma(id) => Self::Dogma(*id),
            EffectId::ScSystemEmitter(id) => Self::ScSystemEmitter(*id),
            EffectId::ScProxyEffect(id) => Self::ScProxyEffect(*id),
            EffectId::ScProxyTrigger(id) => Self::ScProxyTrigger(*id),
            EffectId::ScShipLink(id) => Self::ScShipLink(*id),
            EffectId::Custom(id) => Self::Custom(*id),
        }
    }
}
