use crate::{DpsProfile, UnitInterval, stats::StatTimeOptions};

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Copy, Clone, Default)]
pub struct StatOptionErps {
    pub incoming_dps: Option<DpsProfile> = None,
    #[cfg_attr(feature = "serde", serde(default))]
    pub time_options: StatTimeOptions = StatTimeOptions::default(),
    #[cfg_attr(feature = "serde", serde(default = "shield_perc_default"))]
    pub shield_perc: UnitInterval = UnitInterval::from_f64_clamped(0.25),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Private helpers
////////////////////////////////////////////////////////////////////////////////////////////////////
#[cfg(feature = "serde")]
fn shield_perc_default() -> UnitInterval {
    UnitInterval::from_f64_clamped(0.25)
}
