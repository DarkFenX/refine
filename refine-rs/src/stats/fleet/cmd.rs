use crate::{
    PValue,
    stats::{
        FleetStats, StatDefOptionExt, StatDmg, StatMining, StatOptionExt, StatOptionFitDmg, StatOptionFitMining,
        StatOptionFitOutCps, StatOptionFitOutNps, StatOptionFitOutRps, StatOptionMass, StatOutReps, StatResult,
        err::StatFleetAppliedError,
    },
};

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Default)]
pub struct GetFleetStatsCmd {
    #[cfg_attr(feature = "serde", serde(default))]
    default: bool = true,
    #[cfg_attr(feature = "serde", serde(default))]
    dmg: StatDefOptionExt<StatOptionFitDmg> = StatDefOptionExt::Default,
    #[cfg_attr(feature = "serde", serde(default))]
    mps: StatDefOptionExt<StatOptionFitMining> = StatDefOptionExt::Default,
    #[cfg_attr(feature = "serde", serde(default))]
    outgoing_nps: StatDefOptionExt<StatOptionFitOutNps> = StatDefOptionExt::Default,
    #[cfg_attr(feature = "serde", serde(default))]
    outgoing_rps: StatDefOptionExt<StatOptionFitOutRps> = StatDefOptionExt::Default,
    #[cfg_attr(feature = "serde", serde(default))]
    outgoing_cps: StatDefOptionExt<StatOptionFitOutCps> = StatDefOptionExt::Default,
    #[cfg_attr(feature = "serde", serde(default))]
    mass: StatDefOptionExt<StatOptionMass> = StatDefOptionExt::Default,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Construction
////////////////////////////////////////////////////////////////////////////////////////////////////
impl GetFleetStatsCmd {
    /// True to have all supported stats enabled by default, false to have them disabled.
    pub fn new(default: bool) -> Self {
        Self { default, .. }
    }
    pub fn with_dmg(mut self, option: StatOptionExt<StatOptionFitDmg>) -> Self {
        self.dmg = option.into();
        self
    }
    pub fn with_mps(mut self, option: StatOptionExt<StatOptionFitMining>) -> Self {
        self.mps = option.into();
        self
    }
    pub fn with_outgoing_nps(mut self, option: StatOptionExt<StatOptionFitOutNps>) -> Self {
        self.outgoing_nps = option.into();
        self
    }
    pub fn with_outgoing_rps(mut self, option: StatOptionExt<StatOptionFitOutRps>) -> Self {
        self.outgoing_rps = option.into();
        self
    }
    pub fn with_outgoing_cps(mut self, option: StatOptionExt<StatOptionFitOutCps>) -> Self {
        self.outgoing_cps = option.into();
        self
    }
    pub fn with_mass(mut self, option: StatOptionExt<StatOptionMass>) -> Self {
        self.mass = option.into();
        self
    }
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
