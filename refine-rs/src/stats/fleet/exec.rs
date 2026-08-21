use crate::{
    ItemId, PValue,
    stats::{
        FleetStats, StatDmg, StatMining, StatOptionFitDmg, StatOptionFitMining, StatOptionFitOutCps,
        StatOptionFitOutNps, StatOptionFitOutRps, StatOptionMass, StatOutReps, StatResult, err::StatFleetAppliedError,
        fleet::FleetStatsOptionsInt, option::StatOptionResolved,
    },
};

impl FleetStatsOptionsInt<StatOptionResolved, ItemId> {
    pub(in crate::stats) fn execute(&self, core_fleet: &mut rc::FleetMut) -> FleetStats {
        let mut stats = FleetStats { .. };
        if let Some(options) = self.dmg.get() {
            stats.dmg = get_dmg_stats(core_fleet, options);
        }
        if let Some(options) = self.mps.get() {
            stats.mps = get_mps_stats(core_fleet, options);
        }
        if let Some(options) = self.outgoing_nps.get() {
            stats.outgoing_nps = get_outgoing_nps_stats(core_fleet, options);
        }
        if let Some(options) = self.outgoing_cps.get() {
            stats.outgoing_cps = get_outgoing_cps_stats(core_fleet, options);
        }
        if let Some(options) = self.outgoing_rps.get() {
            stats.outgoing_rps = get_outgoing_rps_stats(core_fleet, options);
        }
        if let Some(options) = self.mass.get() {
            stats.mass = get_mass_stats(core_fleet, options);
        }
        stats
    }
}

fn get_dmg_stats(
    core_fleet: &mut rc::FleetMut,
    options: &[StatOptionFitDmg],
) -> StatResult<StatDmg, !, StatFleetAppliedError> {
    let mut stats = Vec::with_capacity(options.len());
    for option in options.iter() {
        match option.projectee_item_id {
            Some(projectee_item_id) => {
                let stat = core_fleet
                    .get_stat_dmg_applied(option.item_kinds, option.time, option.crits, &projectee_item_id)
                    .map(StatDmg::from_core_applied);
                stats.push(stat);
            }
            None => {
                let stat = StatDmg::from_core(core_fleet.get_stat_dmg(option.item_kinds, option.time, option.crits));
                stats.push(Ok(stat));
            }
        }
    }
    StatResult::Result(stats)
}
fn get_mps_stats(core_fleet: &mut rc::FleetMut, options: &[StatOptionFitMining]) -> StatResult<StatMining, !, !> {
    let mut stats = Vec::with_capacity(options.len());
    for option in options.iter() {
        let stat = core_fleet.get_stat_mps(option.item_kinds, option.time, option.resource_kind);
        stats.push(Ok(stat));
    }
    StatResult::Result(stats)
}
fn get_outgoing_nps_stats(
    core_fleet: &mut rc::FleetMut,
    options: &[StatOptionFitOutNps],
) -> StatResult<PValue, !, StatFleetAppliedError> {
    let mut stats = Vec::with_capacity(options.len());
    for option in options.iter() {
        match option.projectee_item_id {
            Some(projectee_item_id) => {
                let stat = core_fleet.get_stat_outgoing_nps_applied(option.item_kinds, option.time, &projectee_item_id);
                stats.push(stat)
            }
            None => {
                let stat = core_fleet.get_stat_outgoing_nps(option.item_kinds, option.time);
                stats.push(Ok(stat));
            }
        }
    }
    StatResult::Result(stats)
}
fn get_outgoing_rps_stats(
    core_fleet: &mut rc::FleetMut,
    options: &[StatOptionFitOutRps],
) -> StatResult<StatOutReps, !, StatFleetAppliedError> {
    let mut stats = Vec::with_capacity(options.len());
    for option in options.iter() {
        match option.projectee_item_id {
            Some(projectee_item_id) => {
                let stat = core_fleet.get_stat_outgoing_rps_applied(option.item_kinds, option.time, &projectee_item_id);
                stats.push(stat);
            }
            None => {
                let stat = core_fleet.get_stat_outgoing_rps(option.item_kinds, option.time);
                stats.push(Ok(stat));
            }
        }
    }
    StatResult::Result(stats)
}
fn get_outgoing_cps_stats(
    core_fleet: &mut rc::FleetMut,
    options: &[StatOptionFitOutCps],
) -> StatResult<PValue, !, StatFleetAppliedError> {
    let mut stats = Vec::with_capacity(options.len());
    for option in options.iter() {
        match option.projectee_item_id {
            Some(projectee_item_id) => {
                let stat = core_fleet.get_stat_outgoing_cps_applied(option.time, &projectee_item_id);
                stats.push(stat);
            }
            None => {
                let stat = core_fleet.get_stat_outgoing_cps(option.time);
                stats.push(Ok(stat));
            }
        }
    }
    StatResult::Result(stats)
}
fn get_mass_stats(core_fleet: &mut rc::FleetMut, options: &[StatOptionMass]) -> StatResult<PValue, !, !> {
    let mut stats = Vec::with_capacity(options.len());
    for option in options.iter() {
        let stat = core_fleet.get_stat_mass(option.affectors);
        stats.push(Ok(stat));
    }
    StatResult::Result(stats)
}
