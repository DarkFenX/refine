use crate::ad::{ACustomEffectId, ADogmaEffectId, AItemId};

/// ID of an adapted effect.
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum AEffectId {
    /// ID of a general EVE effect.
    Dogma(ADogmaEffectId),
    /// ID of an effect generated from space component attached to an item.
    SpaceComponent(AItemId),
    /// ID of an effect created by the library.
    Custom(ACustomEffectId),
}
impl std::fmt::Display for AEffectId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Dogma(id) => write!(f, "d{}", id),
            Self::SpaceComponent(id) => write!(f, "sc{}", id),
            Self::Custom(id) => write!(f, "c{}", id),
        }
    }
}
