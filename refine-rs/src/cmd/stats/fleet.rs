use crate::stat::{FleetStats, StatOption};

pub struct GetFleetStatsCmd {
    pub default: bool = true,
    pub dmg: StatOption<bool> = StatOption::Default,
    pub mps: StatOption<bool> = StatOption::Default,
    pub outgoing_nps: StatOption<bool> = StatOption::Default,
    pub outgoing_rps: StatOption<bool> = StatOption::Default,
    pub outgoing_cps: StatOption<bool> = StatOption::Default,
    pub mass: StatOption<bool> = StatOption::Default,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl GetFleetStatsCmd {
    pub(crate) fn execute(self, core_fleet: &mut rc::FleetMut) -> FleetStats {
        FleetStats {}
    }
}
