use crate::ad::{AAttrVal, ACount, AItemId, AItemListId};

/// Stores item-specific effect data.
#[derive(Copy, Clone, Default)]
pub struct AItemEffectData {
    /// Type ID of autocharge this effect loads.
    pub autocharge: Option<AItemId> = None,
    /// Defines cooldown of the effect in seconds.
    pub cd: Option<AAttrVal> = None,
    /// Defines how many times the effect can be used before its parent item has to reload.
    pub charge_count: Option<ACount> = None,
    /// Defines how much time each charge of the effect takes to reload, in seconds.
    pub charge_reload_time: Option<AAttrVal> = None,
    /// Type IDs this effect is allowed to be projected to.
    pub projectee_filter: Option<AItemListId> = None,
}
