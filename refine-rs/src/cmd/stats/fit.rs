use crate::{
    PValue,
    stats::{
        FitStats, StatDmg, StatMining, StatOption, StatOptionExt, StatOptionFitDmg, StatOptionFitMining,
        StatOptionFitOutCps, StatOptionFitOutNps, StatOptionFitOutRps, StatOutReps,
    },
};

#[derive(Default)]
pub struct GetFitStatsCmd {
    pub default: bool = true,
    // Fit output stats
    pub dmg: StatOptionExt<StatOptionFitDmg> = StatOptionExt::Default,
    pub mps: StatOptionExt<StatOptionFitMining> = StatOptionExt::Default,
    pub outgoing_nps: StatOptionExt<StatOptionFitOutNps> = StatOptionExt::Default,
    pub outgoing_rps: StatOptionExt<StatOptionFitOutRps> = StatOptionExt::Default,
    pub outgoing_cps: StatOptionExt<StatOptionFitOutCps> = StatOptionExt::Default,
    // Fit resources
    pub cpu: StatOption = StatOption::Default,
    pub powergrid: StatOption = StatOption::Default,
    pub calibration: StatOption = StatOption::Default,
    pub drone_bay_volume: StatOption = StatOption::Default,
    pub drone_bandwidth: StatOption = StatOption::Default,
    pub fighter_bay_volume: StatOption = StatOption::Default,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl GetFitStatsCmd {
    pub(crate) fn execute(self, core_fit: &mut rc::FitMut) -> FitStats {
        let mut stats = FitStats { .. };
        if self.cpu.into_enabled(self.default) {
            stats.cpu = Some(vec![core_fit.get_stat_cpu()])
        }
        ////////////////////////////////////////////////////////////////////////////////////////////
        // Fit output stats
        ////////////////////////////////////////////////////////////////////////////////////////////
        if let Some(options) = self.dmg.into_enabled(self.default) {
            stats.dmg = Some(get_dmg_stats(core_fit, options));
        }
        if let Some(options) = self.mps.into_enabled(self.default) {
            stats.mps = Some(get_mps_stats(core_fit, options));
        }
        if let Some(options) = self.outgoing_nps.into_enabled(self.default) {
            stats.outgoing_nps = Some(get_outgoing_nps_stats(core_fit, options));
        }
        if let Some(options) = self.outgoing_cps.into_enabled(self.default) {
            stats.outgoing_cps = Some(get_outgoing_cps_stats(core_fit, options));
        }
        if let Some(options) = self.outgoing_rps.into_enabled(self.default) {
            stats.outgoing_rps = Some(get_outgoing_rps_stats(core_fit, options));
        }
        ////////////////////////////////////////////////////////////////////////////////////////////
        // Fit resources
        ////////////////////////////////////////////////////////////////////////////////////////////
        if self.powergrid.into_enabled(self.default) {
            stats.powergrid = Some(vec![core_fit.get_stat_powergrid()])
        }
        if self.calibration.into_enabled(self.default) {
            stats.calibration = Some(vec![core_fit.get_stat_calibration()])
        }
        if self.drone_bay_volume.into_enabled(self.default) {
            stats.drone_bay_volume = Some(vec![core_fit.get_stat_drone_bay_volume()])
        }
        if self.drone_bandwidth.into_enabled(self.default) {
            stats.drone_bandwidth = Some(vec![core_fit.get_stat_drone_bandwidth()])
        }
        if self.fighter_bay_volume.into_enabled(self.default) {
            stats.fighter_bay_volume = Some(vec![core_fit.get_stat_fighter_bay_volume()])
        }
        stats
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Fit output stats
////////////////////////////////////////////////////////////////////////////////////////////////////
fn get_dmg_stats(core_fit: &mut rc::FitMut, options: Vec<StatOptionFitDmg>) -> Vec<Option<StatDmg>> {
    let mut stats = Vec::with_capacity(options.len());
    for option in options.into_iter() {
        match option.projectee_item_id {
            Some(projectee_item_id) => {
                match core_fit.get_stat_dmg_applied(option.item_kinds, option.time_options, &projectee_item_id) {
                    Ok(core_stat) => stats.push(Some(StatDmg::from_core_applied(core_stat))),
                    Err(_) => stats.push(None),
                };
            }
            None => {
                let core_stat = core_fit.get_stat_dmg(option.item_kinds, option.time_options);
                stats.push(Some(StatDmg::from_core(core_stat)));
            }
        }
    }
    stats
}
fn get_mps_stats(core_fit: &mut rc::FitMut, options: Vec<StatOptionFitMining>) -> Vec<StatMining> {
    let mut stats = Vec::with_capacity(options.len());
    for option in options.into_iter() {
        let stat = core_fit.get_stat_mps(option.item_kinds, option.time_options, option.mission);
        stats.push(stat);
    }
    stats
}
fn get_outgoing_nps_stats(core_fit: &mut rc::FitMut, options: Vec<StatOptionFitOutNps>) -> Vec<Option<PValue>> {
    let mut stats = Vec::with_capacity(options.len());
    for option in options.into_iter() {
        match option.projectee_item_id {
            Some(projectee_item_id) => {
                match core_fit.get_stat_outgoing_nps_applied(option.item_kinds, option.time_options, &projectee_item_id)
                {
                    Ok(stat) => stats.push(Some(stat)),
                    Err(_) => stats.push(None),
                }
            }
            None => {
                let stat = core_fit.get_stat_outgoing_nps(option.item_kinds, option.time_options);
                stats.push(Some(stat));
            }
        }
    }
    stats
}
fn get_outgoing_rps_stats(core_fit: &mut rc::FitMut, options: Vec<StatOptionFitOutRps>) -> Vec<Option<StatOutReps>> {
    let mut stats = Vec::with_capacity(options.len());
    for option in options.into_iter() {
        match option.projectee_item_id {
            Some(projectee_item_id) => {
                match core_fit.get_stat_outgoing_rps_applied(option.item_kinds, option.time_options, &projectee_item_id)
                {
                    Ok(stat) => stats.push(Some(stat)),
                    Err(_) => stats.push(None),
                }
            }
            None => {
                let stat = core_fit.get_stat_outgoing_rps(option.item_kinds, option.time_options);
                stats.push(Some(stat));
            }
        }
    }
    stats
}
fn get_outgoing_cps_stats(core_fit: &mut rc::FitMut, options: Vec<StatOptionFitOutCps>) -> Vec<Option<PValue>> {
    let mut stats = Vec::with_capacity(options.len());
    for option in options {
        match option.projectee_item_id {
            Some(projectee_item_id) => {
                match core_fit.get_stat_outgoing_cps_applied(option.time_options, &projectee_item_id) {
                    Ok(stat) => stats.push(Some(stat)),
                    Err(_) => stats.push(None),
                }
            }
            None => {
                let stat = core_fit.get_stat_outgoing_cps(option.time_options);
                stats.push(Some(stat));
            }
        }
    }
    stats
}
