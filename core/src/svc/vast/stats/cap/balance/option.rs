use crate::num::UnitInterval;

/// Capacitor change sources which will be considered for cap balance stats.
#[derive(Copy, Clone)]
pub struct StatCapBlcSrcKinds {
    pub regen: StatCapBlcRegen,
    pub cap_injectors: bool,
    pub nosfs: bool,
    pub consumers: bool,
    pub incoming_transfers: bool,
    pub incoming_neuts: bool,
}
impl StatCapBlcSrcKinds {
    /// Include all capacitor change sources.
    pub fn all_enabled() -> Self {
        Self {
            regen: StatCapBlcRegen::Enabled(StatCapBlcRegenOptions { .. }),
            cap_injectors: true,
            nosfs: true,
            consumers: true,
            incoming_transfers: true,
            incoming_neuts: true,
        }
    }
    /// Exclude all capacitor change sources.
    pub fn all_disabled() -> Self {
        Self {
            regen: StatCapBlcRegen::Disabled,
            cap_injectors: false,
            nosfs: false,
            consumers: false,
            incoming_transfers: false,
            incoming_neuts: false,
        }
    }
}

#[derive(Copy, Clone)]
pub enum StatCapBlcRegen {
    Enabled(StatCapBlcRegenOptions),
    Disabled,
}

#[derive(Copy, Clone)]
pub struct StatCapBlcRegenOptions {
    pub cap_perc: UnitInterval = UnitInterval::from_f64_clamped(0.25),
}
