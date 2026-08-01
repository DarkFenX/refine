use crate::{
    PValue,
    stats::{
        FleetStats, StatDmg, StatMining, StatOptionExt, StatOptionFitDmg, StatOptionFitMining, StatOptionFitOutCps,
        StatOptionFitOutNps, StatOptionFitOutRps, StatOptionMass, StatOutReps, StatResult, err::StatFleetAppliedError,
    },
};

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Default)]
pub struct GetFleetStatsCmd {
    #[cfg_attr(feature = "serde", serde(default))]
    pub default: bool = true,
    #[cfg_attr(feature = "serde", serde(default))]
    pub dmg: StatOptionExt<StatOptionFitDmg> = StatOptionExt::Default,
    #[cfg_attr(feature = "serde", serde(default))]
    pub mps: StatOptionExt<StatOptionFitMining> = StatOptionExt::Default,
    #[cfg_attr(feature = "serde", serde(default))]
    pub outgoing_nps: StatOptionExt<StatOptionFitOutNps> = StatOptionExt::Default,
    #[cfg_attr(feature = "serde", serde(default))]
    pub outgoing_rps: StatOptionExt<StatOptionFitOutRps> = StatOptionExt::Default,
    #[cfg_attr(feature = "serde", serde(default))]
    pub outgoing_cps: StatOptionExt<StatOptionFitOutCps> = StatOptionExt::Default,
    #[cfg_attr(feature = "serde", serde(default))]
    pub mass: StatOptionExt<StatOptionMass> = StatOptionExt::Default,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl GetFleetStatsCmd {
    pub(crate) fn execute(self, core_fleet: &mut rc::FleetMut) -> FleetStats {
        let mut stats = FleetStats { .. };
        if let Some(options) = self.dmg.into_enabled(self.default) {
            stats.dmg = get_dmg_stats(core_fleet, options);
        }
        if let Some(options) = self.mps.into_enabled(self.default) {
            stats.mps = get_mps_stats(core_fleet, options);
        }
        if let Some(options) = self.outgoing_nps.into_enabled(self.default) {
            stats.outgoing_nps = get_outgoing_nps_stats(core_fleet, options);
        }
        if let Some(options) = self.outgoing_cps.into_enabled(self.default) {
            stats.outgoing_cps = get_outgoing_cps_stats(core_fleet, options);
        }
        if let Some(options) = self.outgoing_rps.into_enabled(self.default) {
            stats.outgoing_rps = get_outgoing_rps_stats(core_fleet, options);
        }
        if let Some(options) = self.mass.into_enabled(self.default) {
            stats.mass = get_mass_stats(core_fleet, options);
        }
        stats
    }
}

fn get_dmg_stats(
    core_fleet: &mut rc::FleetMut,
    options: Vec<StatOptionFitDmg>,
) -> StatResult<StatDmg, !, StatFleetAppliedError> {
    let mut stats = Vec::with_capacity(options.len());
    for option in options.into_iter() {
        match option.projectee_item_id {
            Some(projectee_item_id) => {
                let stat = core_fleet
                    .get_stat_dmg_applied(option.item_kinds, option.time_options, &projectee_item_id)
                    .map(StatDmg::from_core_applied);
                stats.push(stat);
            }
            None => {
                let stat = StatDmg::from_core(core_fleet.get_stat_dmg(option.item_kinds, option.time_options));
                stats.push(Ok(stat));
            }
        }
    }
    StatResult::Result(stats)
}
fn get_mps_stats(core_fleet: &mut rc::FleetMut, options: Vec<StatOptionFitMining>) -> StatResult<StatMining, !, !> {
    let mut stats = Vec::with_capacity(options.len());
    for option in options.into_iter() {
        let stat = core_fleet.get_stat_mps(option.item_kinds, option.time_options, option.mission);
        stats.push(Ok(stat));
    }
    StatResult::Result(stats)
}
fn get_outgoing_nps_stats(
    core_fleet: &mut rc::FleetMut,
    options: Vec<StatOptionFitOutNps>,
) -> StatResult<PValue, !, StatFleetAppliedError> {
    let mut stats = Vec::with_capacity(options.len());
    for option in options.into_iter() {
        match option.projectee_item_id {
            Some(projectee_item_id) => {
                let stat = core_fleet.get_stat_outgoing_nps_applied(
                    option.item_kinds,
                    option.time_options,
                    &projectee_item_id,
                );
                stats.push(stat)
            }
            None => {
                let stat = core_fleet.get_stat_outgoing_nps(option.item_kinds, option.time_options);
                stats.push(Ok(stat));
            }
        }
    }
    StatResult::Result(stats)
}
fn get_outgoing_rps_stats(
    core_fleet: &mut rc::FleetMut,
    options: Vec<StatOptionFitOutRps>,
) -> StatResult<StatOutReps, !, StatFleetAppliedError> {
    let mut stats = Vec::with_capacity(options.len());
    for option in options.into_iter() {
        match option.projectee_item_id {
            Some(projectee_item_id) => {
                let stat = core_fleet.get_stat_outgoing_rps_applied(
                    option.item_kinds,
                    option.time_options,
                    &projectee_item_id,
                );
                stats.push(stat);
            }
            None => {
                let stat = core_fleet.get_stat_outgoing_rps(option.item_kinds, option.time_options);
                stats.push(Ok(stat));
            }
        }
    }
    StatResult::Result(stats)
}
fn get_outgoing_cps_stats(
    core_fleet: &mut rc::FleetMut,
    options: Vec<StatOptionFitOutCps>,
) -> StatResult<PValue, !, StatFleetAppliedError> {
    let mut stats = Vec::with_capacity(options.len());
    for option in options.into_iter() {
        match option.projectee_item_id {
            Some(projectee_item_id) => {
                let stat = core_fleet.get_stat_outgoing_cps_applied(option.time_options, &projectee_item_id);
                stats.push(stat);
            }
            None => {
                let stat = core_fleet.get_stat_outgoing_cps(option.time_options);
                stats.push(Ok(stat));
            }
        }
    }
    StatResult::Result(stats)
}
fn get_mass_stats(core_fleet: &mut rc::FleetMut, options: Vec<StatOptionMass>) -> StatResult<PValue, !, !> {
    let mut stats = Vec::with_capacity(options.len());
    for option in options.into_iter() {
        let stat = core_fleet.get_stat_mass(option.affectors);
        stats.push(Ok(stat));
    }
    StatResult::Result(stats)
}
