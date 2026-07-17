use crate::{
    PValue,
    stats::{FleetStats, StatOptionExt, StatOptionMass},
};

#[derive(Default)]
pub struct GetFleetStatsCmd {
    pub default: bool = true,
    pub dmg: StatOptionExt<bool> = StatOptionExt::Default,
    pub mps: StatOptionExt<bool> = StatOptionExt::Default,
    pub outgoing_nps: StatOptionExt<bool> = StatOptionExt::Default,
    pub outgoing_rps: StatOptionExt<bool> = StatOptionExt::Default,
    pub outgoing_cps: StatOptionExt<bool> = StatOptionExt::Default,
    pub mass: StatOptionExt<StatOptionMass> = StatOptionExt::Default,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl GetFleetStatsCmd {
    pub(crate) fn execute(self, core_fleet: &mut rc::FleetMut) -> FleetStats {
        let mut stats = FleetStats { .. };
        if let Some(options) = self.mass.into_enabled(self.default) {
            stats.mass = Some(get_mass_stats(core_fleet, options));
        }
        stats
    }
}

fn get_mass_stats(core_fleet: &mut rc::FleetMut, options: Vec<StatOptionMass>) -> Vec<PValue> {
    let mut results = Vec::with_capacity(options.len());
    for option in options.into_iter() {
        let stat = core_fleet.get_stat_mass(option.affectors);
        results.push(stat);
    }
    results
}
