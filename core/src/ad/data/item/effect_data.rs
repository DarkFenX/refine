use crate::defs::{Amount, AttrVal};

/// Stores item-specific effect data.
pub struct AItemEffData {
    /// Defines cooldown of the effect in seconds.
    pub cd: Option<AttrVal>,
    /// Defines how many times the effect can be used before its parent item has to reload.
    pub charge_amount: Option<Amount>,
    /// Defines how much time each charge of the effect takes to reload, in seconds.
    pub charge_reload_time: Option<AttrVal>,
}
impl AItemEffData {
    /// Make a new per-item effect data container out of passed data.
    pub(crate) fn new(cd: Option<AttrVal>, charge_amount: Option<Amount>, charge_reload_time: Option<AttrVal>) -> Self {
        Self {
            cd,
            charge_amount,
            charge_reload_time,
        }
    }
}
