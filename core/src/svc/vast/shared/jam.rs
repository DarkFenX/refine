use crate::misc::UnitInterval;

#[derive(Copy, Clone)]
pub struct StatJamApplied {
    pub chance: UnitInterval,
    pub uptime: UnitInterval,
}
