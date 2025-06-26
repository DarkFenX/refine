use crate::ad::{AAttrVal, ACount};

/// Stores item-specific effect data.
#[derive(Default)]
pub struct AItemEffectData {
    /// Defines cooldown of the effect in seconds.
    pub cd: Option<AAttrVal> = None,
    /// Defines how many times the effect can be used before its parent item has to reload.
    pub charge_count: Option<ACount> = None,
    /// Defines how much time each charge of the effect takes to reload, in seconds.
    pub charge_reload_time: Option<AAttrVal> = None,
}
