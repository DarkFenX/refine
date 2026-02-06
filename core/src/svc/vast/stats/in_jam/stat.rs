use crate::num::UnitInterval;

#[derive(Copy, Clone)]
pub struct StatInJam {
    /// Chance to get jammed over passed duration.
    pub chance: UnitInterval,
    /// Percentage of time target is unable to lock.
    pub uptime: UnitInterval,
}
