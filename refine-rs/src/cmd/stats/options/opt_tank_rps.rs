use crate::{UnitInterval, stats::StatTimeOptions};

#[derive(Copy, Clone, Default)]
pub struct StatOptionRps {
    pub time_options: StatTimeOptions = StatTimeOptions::default(),
    pub shield_perc: UnitInterval = UnitInterval::from_f64_clamped(0.25),
}
