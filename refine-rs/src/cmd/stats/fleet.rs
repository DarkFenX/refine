use crate::{
    PValue,
    stats::{
        FleetStats, StatDmg, StatMining, StatOptionExt, StatOptionFitDmg, StatOptionFitMining, StatOptionFitOutCps,
        StatOptionFitOutNps, StatOptionFitOutRps, StatOptionMass, StatOutReps,
    },
};

#[derive(Default)]
pub struct GetFleetStatsCmd {
    pub default: bool = true,
    pub dmg: StatOptionExt<StatOptionFitDmg> = StatOptionExt::Default,
    pub mps: StatOptionExt<StatOptionFitMining> = StatOptionExt::Default,
    pub outgoing_nps: StatOptionExt<StatOptionFitOutNps> = StatOptionExt::Default,
    pub outgoing_rps: StatOptionExt<StatOptionFitOutRps> = StatOptionExt::Default,
    pub outgoing_cps: StatOptionExt<StatOptionFitOutCps> = StatOptionExt::Default,
    pub mass: StatOptionExt<StatOptionMass> = StatOptionExt::Default,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl GetFleetStatsCmd {
    pub(crate) fn execute(self, core_fleet: &mut rc::FleetMut) -> FleetStats {
        let mut stats = FleetStats { .. };
        if let Some(options) = self.dmg.into_enabled(self.default) {
            stats.dmg = Some(get_dmg_stats(core_fleet, options));
        }
        if let Some(options) = self.mps.into_enabled(self.default) {
            stats.mps = Some(get_mps_stats(core_fleet, options));
        }
        if let Some(options) = self.outgoing_nps.into_enabled(self.default) {
            stats.outgoing_nps = Some(get_outgoing_nps_stats(core_fleet, options));
        }
        if let Some(options) = self.outgoing_cps.into_enabled(self.default) {
            stats.outgoing_cps = Some(get_outgoing_cps_stats(core_fleet, options));
        }
        if let Some(options) = self.outgoing_rps.into_enabled(self.default) {
            stats.outgoing_rps = Some(get_outgoing_rps_stats(core_fleet, options));
        }
        if let Some(options) = self.mass.into_enabled(self.default) {
            stats.mass = Some(get_mass_stats(core_fleet, options));
        }
        stats
    }
}

fn get_dmg_stats(core_fleet: &mut rc::FleetMut, options: Vec<StatOptionFitDmg>) -> Vec<Option<StatDmg>> {
    let mut stats = Vec::with_capacity(options.len());
    for option in options.into_iter() {
        match option.projectee_item_id {
            Some(projectee_item_id) => {
                match core_fleet.get_stat_dmg_applied(option.item_kinds, option.time_options, &projectee_item_id) {
                    Ok(core_stat) => stats.push(Some(StatDmg::from_core_applied(core_stat))),
                    Err(_) => stats.push(None),
                };
            }
            None => {
                let core_stat = core_fleet.get_stat_dmg(option.item_kinds, option.time_options);
                stats.push(Some(StatDmg::from_core(core_stat)));
            }
        }
    }
    stats
}
fn get_mps_stats(core_fleet: &mut rc::FleetMut, options: Vec<StatOptionFitMining>) -> Vec<StatMining> {
    let mut stats = Vec::with_capacity(options.len());
    for option in options.into_iter() {
        let stat = core_fleet.get_stat_mps(option.item_kinds, option.time_options, option.mission);
        stats.push(stat);
    }
    stats
}
fn get_outgoing_nps_stats(core_fleet: &mut rc::FleetMut, options: Vec<StatOptionFitOutNps>) -> Vec<Option<PValue>> {
    let mut stats = Vec::with_capacity(options.len());
    for option in options.into_iter() {
        match option.projectee_item_id {
            Some(projectee_item_id) => {
                match core_fleet.get_stat_outgoing_nps_applied(
                    option.item_kinds,
                    option.time_options,
                    &projectee_item_id,
                ) {
                    Ok(stat) => stats.push(Some(stat)),
                    Err(_) => stats.push(None),
                }
            }
            None => {
                let stat = core_fleet.get_stat_outgoing_nps(option.item_kinds, option.time_options);
                stats.push(Some(stat));
            }
        }
    }
    stats
}
fn get_outgoing_rps_stats(
    core_fleet: &mut rc::FleetMut,
    options: Vec<StatOptionFitOutRps>,
) -> Vec<Option<StatOutReps>> {
    let mut stats = Vec::with_capacity(options.len());
    for option in options.into_iter() {
        match option.projectee_item_id {
            Some(projectee_item_id) => {
                match core_fleet.get_stat_outgoing_rps_applied(
                    option.item_kinds,
                    option.time_options,
                    &projectee_item_id,
                ) {
                    Ok(stat) => stats.push(Some(stat)),
                    Err(_) => stats.push(None),
                }
            }
            None => {
                let stat = core_fleet.get_stat_outgoing_rps(option.item_kinds, option.time_options);
                stats.push(Some(stat));
            }
        }
    }
    stats
}
fn get_outgoing_cps_stats(core_fleet: &mut rc::FleetMut, options: Vec<StatOptionFitOutCps>) -> Vec<Option<PValue>> {
    let mut stats = Vec::with_capacity(options.len());
    for option in options.into_iter() {
        match option.projectee_item_id {
            Some(projectee_item_id) => {
                match core_fleet.get_stat_outgoing_cps_applied(option.time_options, &projectee_item_id) {
                    Ok(stat) => stats.push(Some(stat)),
                    Err(_) => stats.push(None),
                }
            }
            None => {
                let stat = core_fleet.get_stat_outgoing_cps(option.time_options);
                stats.push(Some(stat));
            }
        }
    }
    stats
}
fn get_mass_stats(core_fleet: &mut rc::FleetMut, options: Vec<StatOptionMass>) -> Vec<PValue> {
    let mut stats = Vec::with_capacity(options.len());
    for option in options.into_iter() {
        let stat = core_fleet.get_stat_mass(option.affectors);
        stats.push(stat);
    }
    stats
}
