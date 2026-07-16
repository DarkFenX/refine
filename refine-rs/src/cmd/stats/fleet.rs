use crate::stats::{FleetStats, StatOptionExt};

#[derive(Default)]
pub struct GetFleetStatsCmd {
    pub default: bool = true,
    pub dmg: StatOptionExt<bool> = StatOptionExt::Default,
    pub mps: StatOptionExt<bool> = StatOptionExt::Default,
    pub outgoing_nps: StatOptionExt<bool> = StatOptionExt::Default,
    pub outgoing_rps: StatOptionExt<bool> = StatOptionExt::Default,
    pub outgoing_cps: StatOptionExt<bool> = StatOptionExt::Default,
    pub mass: StatOptionExt<bool> = StatOptionExt::Default,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl GetFleetStatsCmd {
    pub(crate) fn execute(self, core_fleet: &mut rc::FleetMut) -> FleetStats {
        FleetStats {}
    }
}
