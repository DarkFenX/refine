use crate::{
    PValue,
    stats::{
        FitStats, StatDmg, StatEhp, StatErps, StatMining, StatOption, StatOptionEhp, StatOptionErps, StatOptionExt,
        StatOptionFitDmg, StatOptionFitMining, StatOptionFitOutCps, StatOptionFitOutNps, StatOptionFitOutRps,
        StatOptionRps, StatOutReps, StatRps,
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
    // Fit slots
    pub high_slots: StatOption = StatOption::Default,
    pub mid_slots: StatOption = StatOption::Default,
    pub low_slots: StatOption = StatOption::Default,
    pub turret_slots: StatOption = StatOption::Default,
    pub launcher_slots: StatOption = StatOption::Default,
    pub rig_slots: StatOption = StatOption::Default,
    pub service_slots: StatOption = StatOption::Default,
    pub subsystem_slots: StatOption = StatOption::Default,
    pub launched_drones: StatOption = StatOption::Default,
    pub launched_fighters: StatOption = StatOption::Default,
    pub launched_light_fighters: StatOption = StatOption::Default,
    pub launched_heavy_fighters: StatOption = StatOption::Default,
    pub launched_support_fighters: StatOption = StatOption::Default,
    pub launched_st_light_fighters: StatOption = StatOption::Default,
    pub launched_st_heavy_fighters: StatOption = StatOption::Default,
    pub launched_st_support_fighters: StatOption = StatOption::Default,
    // Ship tank
    pub resists: StatOption = StatOption::Default,
    pub hp: StatOption = StatOption::Default,
    pub ehp: StatOptionExt<StatOptionEhp> = StatOptionExt::Default,
    pub wc_ehp: StatOption = StatOption::Default,
    pub rps: StatOptionExt<StatOptionRps> = StatOptionExt::Default,
    pub erps: StatOptionExt<StatOptionErps> = StatOptionExt::Default,
    pub breach_resist: StatOption = StatOption::Default,
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
        ////////////////////////////////////////////////////////////////////////////////////////////
        // Fit slots
        ////////////////////////////////////////////////////////////////////////////////////////////
        if self.high_slots.into_enabled(self.default) {
            stats.high_slots = Some(vec![core_fit.get_stat_high_slots()]);
        }
        if self.mid_slots.into_enabled(self.default) {
            stats.mid_slots = Some(vec![core_fit.get_stat_mid_slots()]);
        }
        if self.low_slots.into_enabled(self.default) {
            stats.low_slots = Some(vec![core_fit.get_stat_low_slots()]);
        }
        if self.turret_slots.into_enabled(self.default) {
            stats.turret_slots = Some(vec![core_fit.get_stat_turret_slots()]);
        }
        if self.launcher_slots.into_enabled(self.default) {
            stats.launcher_slots = Some(vec![core_fit.get_stat_launcher_slots()]);
        }
        if self.rig_slots.into_enabled(self.default) {
            stats.rig_slots = Some(vec![core_fit.get_stat_rig_slots()]);
        }
        if self.service_slots.into_enabled(self.default) {
            stats.service_slots = Some(vec![core_fit.get_stat_service_slots()]);
        }
        if self.subsystem_slots.into_enabled(self.default) {
            stats.subsystem_slots = Some(vec![core_fit.get_stat_subsystem_slots()]);
        }
        if self.launched_drones.into_enabled(self.default) {
            stats.launched_drones = Some(vec![core_fit.get_stat_launched_drones()]);
        }
        if self.launched_fighters.into_enabled(self.default) {
            stats.launched_fighters = Some(vec![core_fit.get_stat_launched_fighters()]);
        }
        if self.launched_light_fighters.into_enabled(self.default) {
            stats.launched_light_fighters = Some(vec![core_fit.get_stat_launched_light_fighters()]);
        }
        if self.launched_heavy_fighters.into_enabled(self.default) {
            stats.launched_heavy_fighters = Some(vec![core_fit.get_stat_launched_heavy_fighters()]);
        }
        if self.launched_support_fighters.into_enabled(self.default) {
            stats.launched_support_fighters = Some(vec![core_fit.get_stat_launched_support_fighters()]);
        }
        if self.launched_st_light_fighters.into_enabled(self.default) {
            stats.launched_st_light_fighters = Some(vec![core_fit.get_stat_launched_st_light_fighters()]);
        }
        if self.launched_st_heavy_fighters.into_enabled(self.default) {
            stats.launched_st_heavy_fighters = Some(vec![core_fit.get_stat_launched_st_heavy_fighters()]);
        }
        if self.launched_st_support_fighters.into_enabled(self.default) {
            stats.launched_st_support_fighters = Some(vec![core_fit.get_stat_launched_st_support_fighters()]);
        }
        ////////////////////////////////////////////////////////////////////////////////////////////
        // Ship tank
        ////////////////////////////////////////////////////////////////////////////////////////////
        if self.resists.into_enabled(self.default) {
            stats.resists = core_fit.get_stat_resists().into();
        }
        if self.hp.into_enabled(self.default) {
            stats.hp = core_fit.get_stat_hp().into();
        }
        if let Some(options) = self.ehp.into_enabled(self.default) {
            stats.ehp = get_ehp_stats(core_fit, options).into();
        }
        if self.wc_ehp.into_enabled(self.default) {
            stats.wc_ehp = core_fit.get_stat_wc_ehp().into();
        }
        if let Some(options) = self.rps.into_enabled(self.default) {
            stats.rps = get_rps_stats(core_fit, options).into();
        }
        if let Some(options) = self.erps.into_enabled(self.default) {
            stats.erps = get_erps_stats(core_fit, options).into();
        }
        if self.breach_resist.into_enabled(self.default) {
            stats.breach_resist = core_fit.get_stat_breach_resist().into();
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

////////////////////////////////////////////////////////////////////////////////////////////////////
// Ship tank
////////////////////////////////////////////////////////////////////////////////////////////////////
fn get_ehp_stats(core_fit: &mut rc::FitMut, options: Vec<StatOptionEhp>) -> Option<Vec<StatEhp>> {
    let mut stats = Vec::with_capacity(options.len());
    for option in options.into_iter() {
        match core_fit.get_stat_ehp(option.incoming_dps) {
            Ok(stat) => stats.push(stat),
            Err(_) => return None,
        }
    }
    Some(stats)
}
fn get_rps_stats(core_fit: &mut rc::FitMut, options: Vec<StatOptionRps>) -> Option<Vec<StatRps>> {
    let mut stats = Vec::with_capacity(options.len());
    for option in options.into_iter() {
        match core_fit.get_stat_rps(option.time_options, option.shield_perc) {
            Ok(stat) => stats.push(stat),
            Err(_) => return None,
        }
    }
    Some(stats)
}
fn get_erps_stats(core_fit: &mut rc::FitMut, options: Vec<StatOptionErps>) -> Option<Vec<StatErps>> {
    let mut stats = Vec::with_capacity(options.len());
    for option in options.into_iter() {
        match core_fit.get_stat_erps(option.incoming_dps, option.time_options, option.shield_perc) {
            Ok(stat) => stats.push(stat),
            Err(_) => return None,
        }
    }
    Some(stats)
}
