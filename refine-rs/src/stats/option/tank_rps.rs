use crate::{UnitInterval, stats::StatTimeOptions};

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Copy, Clone, Default)]
pub struct StatOptionRps {
    #[cfg_attr(feature = "serde", serde(default))]
    pub(in crate::stats) time: StatTimeOptions = StatTimeOptions::default(),
    #[cfg_attr(feature = "serde", serde(default = "shield_perc_default"))]
    pub(in crate::stats) shield_perc: UnitInterval = UnitInterval::from_f64_clamped(0.25),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Construction
////////////////////////////////////////////////////////////////////////////////////////////////////
impl StatOptionRps {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_time(mut self, time: StatTimeOptions) -> Self {
        self.time = time;
        self
    }
    pub fn with_shield_perc(mut self, shield_perc: UnitInterval) -> Self {
        self.shield_perc = shield_perc;
        self
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Custom de/serialization
////////////////////////////////////////////////////////////////////////////////////////////////////
#[cfg(feature = "serde")]
fn shield_perc_default() -> UnitInterval {
    UnitInterval::from_f64_clamped(0.25)
}
