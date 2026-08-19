use super::options::FleetStatsOptions;
use crate::{
    PValue,
    stats::{
        FleetStats, StatDmg, StatMining, StatOptionExt, StatOptionFitDmg, StatOptionFitMining, StatOptionFitOutCps,
        StatOptionFitOutNps, StatOptionFitOutRps, StatOptionMass, StatOutReps, StatResult, err::StatFleetAppliedError,
        option_support::StatOptionExtRaw,
    },
};

#[cfg_attr(feature = "serde", derive(serde::Deserialize), serde(default))]
#[derive(Default)]
pub struct GetFleetStatsCmd {
    #[cfg_attr(feature = "serde", serde(default = "custom_serde::stat_default"))]
    default: bool = true,
    #[cfg_attr(feature = "serde", serde(flatten))]
    options: FleetStatsOptions<StatOptionExtRaw>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Construction
////////////////////////////////////////////////////////////////////////////////////////////////////
impl GetFleetStatsCmd {
    /// True to have all supported stats enabled by default, false to have them disabled.
    pub fn new(default: bool) -> Self {
        Self {
            default,
            options: FleetStatsOptions::default(),
        }
    }
    pub fn with_dmg(mut self, option: StatOptionExt<StatOptionFitDmg>) -> Self {
        self.options.dmg = option.into();
        self
    }
    pub fn with_mps(mut self, option: StatOptionExt<StatOptionFitMining>) -> Self {
        self.options.mps = option.into();
        self
    }
    pub fn with_outgoing_nps(mut self, option: StatOptionExt<StatOptionFitOutNps>) -> Self {
        self.options.outgoing_nps = option.into();
        self
    }
    pub fn with_outgoing_rps(mut self, option: StatOptionExt<StatOptionFitOutRps>) -> Self {
        self.options.outgoing_rps = option.into();
        self
    }
    pub fn with_outgoing_cps(mut self, option: StatOptionExt<StatOptionFitOutCps>) -> Self {
        self.options.outgoing_cps = option.into();
        self
    }
    pub fn with_mass(mut self, option: StatOptionExt<StatOptionMass>) -> Self {
        self.options.mass = option.into();
        self
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl GetFleetStatsCmd {
    pub(crate) fn execute(self, core_fleet: &mut rc::FleetMut) -> FleetStats {
        let mut stats = FleetStats { .. };
        let options = self.options.resolve(self.default);
        if let Some(options) = options.dmg {
            stats.dmg = get_dmg_stats(core_fleet, options);
        }
        if let Some(options) = options.mps {
            stats.mps = get_mps_stats(core_fleet, options);
        }
        if let Some(options) = options.outgoing_nps {
            stats.outgoing_nps = get_outgoing_nps_stats(core_fleet, options);
        }
        if let Some(options) = options.outgoing_cps {
            stats.outgoing_cps = get_outgoing_cps_stats(core_fleet, options);
        }
        if let Some(options) = options.outgoing_rps {
            stats.outgoing_rps = get_outgoing_rps_stats(core_fleet, options);
        }
        if let Some(options) = options.mass {
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
fn get_mps_stats(core_fleet: &mut rc::FleetMut, options: Vec<StatOptionFitMining>) -> StatResult<StatMining, !, !> {
    let mut stats = Vec::with_capacity(options.len());
    for option in options.into_iter() {
        let stat = core_fleet.get_stat_mps(option.item_kinds, option.time, option.resource_kind);
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
    options: Vec<StatOptionFitOutRps>,
) -> StatResult<StatOutReps, !, StatFleetAppliedError> {
    let mut stats = Vec::with_capacity(options.len());
    for option in options.into_iter() {
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
    options: Vec<StatOptionFitOutCps>,
) -> StatResult<PValue, !, StatFleetAppliedError> {
    let mut stats = Vec::with_capacity(options.len());
    for option in options.into_iter() {
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
fn get_mass_stats(core_fleet: &mut rc::FleetMut, options: Vec<StatOptionMass>) -> StatResult<PValue, !, !> {
    let mut stats = Vec::with_capacity(options.len());
    for option in options.into_iter() {
        let stat = core_fleet.get_stat_mass(option.affectors);
        stats.push(Ok(stat));
    }
    StatResult::Result(stats)
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Custom de/serialization
////////////////////////////////////////////////////////////////////////////////////////////////////
#[cfg(feature = "serde")]
mod custom_serde {
    pub(super) fn stat_default() -> bool {
        true
    }
}
