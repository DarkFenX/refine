use crate::num::UnitInterval;

#[derive(Copy, Clone)]
pub struct StatInJam {
    pub chance: UnitInterval,
    pub uptime: UnitInterval,
}
