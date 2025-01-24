use crate::defs::{AttrVal, Count};

/// Stores item-specific effect data.
pub struct AItemEffectData {
    /// Defines cooldown of the effect in seconds.
    pub cd: Option<AttrVal>,
    /// Defines how many times the effect can be used before its parent item has to reload.
    pub charge_count: Option<Count>,
    /// Defines how much time each charge of the effect takes to reload, in seconds.
    pub charge_reload_time: Option<AttrVal>,
}
impl AItemEffectData {
    /// Make a new per-item effect data container out of passed data.
    pub(crate) fn new(cd: Option<AttrVal>, charge_count: Option<Count>, charge_reload_time: Option<AttrVal>) -> Self {
        Self {
            cd,
            charge_count,
            charge_reload_time,
        }
    }
}
