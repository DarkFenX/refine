use crate::EffectMode;

/// Data on item's effect.
#[cfg_attr(feature = "serde", derive(serde_tuple::Serialize_tuple))]
#[derive(Copy, Clone)]
pub struct ItemEffectInfo {
    /// Is effect running in current configuration or not.
    pub running: bool,
    /// Run mode this effect on this item has
    pub mode: EffectMode,
}
