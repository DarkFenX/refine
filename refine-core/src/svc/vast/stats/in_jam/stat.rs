use crate::num::UnitInterval;

#[cfg_attr(feature = "serde", derive(serde_tuple::Serialize_tuple))]
#[derive(Copy, Clone)]
pub struct StatInJam {
    /// Chance to get jammed over passed duration.
    pub chance: UnitInterval,
    /// Percentage of time target is unable to lock.
    pub uptime: UnitInterval,
}
