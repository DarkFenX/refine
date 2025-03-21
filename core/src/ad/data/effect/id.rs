use crate::defs::{EEffectId, EItemId};

/// ID of an adapted effect.
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum AEffectId {
    /// ID of a general EVE effect.
    Dogma(EEffectId),
    /// ID of an effect generated from space component attached to an item.
    SpaceComponent(EItemId),
    /// ID of an effect created by the library.
    Custom(EEffectId),
}
