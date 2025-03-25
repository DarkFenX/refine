use crate::{
    ad,
    sol::{CustomEffectId, DogmaEffectId, ItemTypeId},
};

/// ID of an effect.
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum EffectId {
    /// ID of a general EVE effect.
    Dogma(DogmaEffectId),
    /// ID of an effect generated from space component attached to an item.
    SpaceComponent(ItemTypeId),
    /// ID of an effect created by the library.
    Custom(CustomEffectId),
}
impl std::fmt::Display for EffectId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Dogma(id) => write!(f, "d{}", id),
            Self::SpaceComponent(id) => write!(f, "sc{}", id),
            Self::Custom(id) => write!(f, "c{}", id),
        }
    }
}
impl From<ad::AEffectId> for EffectId {
    fn from(a_effect_id: ad::AEffectId) -> Self {
        match a_effect_id {
            ad::AEffectId::Dogma(id) => Self::Dogma(id),
            ad::AEffectId::SpaceComponent(id) => Self::SpaceComponent(id),
            ad::AEffectId::Custom(id) => Self::Custom(id),
        }
    }
}
impl From<&ad::AEffectId> for EffectId {
    fn from(a_effect_id: &ad::AEffectId) -> Self {
        match a_effect_id {
            ad::AEffectId::Dogma(id) => Self::Dogma(*id),
            ad::AEffectId::SpaceComponent(id) => Self::SpaceComponent(*id),
            ad::AEffectId::Custom(id) => Self::Custom(*id),
        }
    }
}
impl From<EffectId> for ad::AEffectId {
    fn from(effect_id: EffectId) -> Self {
        match effect_id {
            EffectId::Dogma(id) => Self::Dogma(id),
            EffectId::SpaceComponent(id) => Self::SpaceComponent(id),
            EffectId::Custom(id) => Self::Custom(id),
        }
    }
}
impl From<&EffectId> for ad::AEffectId {
    fn from(effect_id: &EffectId) -> Self {
        match effect_id {
            EffectId::Dogma(id) => Self::Dogma(*id),
            EffectId::SpaceComponent(id) => Self::SpaceComponent(*id),
            EffectId::Custom(id) => Self::Custom(*id),
        }
    }
}
