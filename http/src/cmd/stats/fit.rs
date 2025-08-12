use crate::{
    cmd::{
        shared::get_primary_fit,
        stats::options::{
            HStatOption, HStatOptionEhp, HStatOptionErps, HStatOptionFitDps, HStatOptionFitRemoteRps,
            HStatOptionFitVolley, HStatOptionRps, HStatResolvedOption,
        },
    },
    info::{
        HFitStats,
        stats::{HStatDmg, HStatLayerEhp, HStatLayerErps, HStatLayerRps, HStatTank},
    },
    util::HExecError,
};

#[derive(educe::Educe, serde::Deserialize)]
#[educe(Default)]
pub(crate) struct HGetFitStatsCmd {
    #[serde(default)]
    #[educe(Default = true)]
    default: bool,
    high_slots: Option<bool>,
    mid_slots: Option<bool>,
    low_slots: Option<bool>,
    turret_slots: Option<bool>,
    launcher_slots: Option<bool>,
    rig_slots: Option<bool>,
    service_slots: Option<bool>,
    subsystem_slots: Option<bool>,
    launched_drones: Option<bool>,
    launched_fighters: Option<bool>,
    launched_light_fighters: Option<bool>,
    launched_heavy_fighters: Option<bool>,
    launched_support_fighters: Option<bool>,
    launched_st_light_fighters: Option<bool>,
    launched_st_heavy_fighters: Option<bool>,
    launched_st_support_fighters: Option<bool>,
    cpu: Option<bool>,
    powergrid: Option<bool>,
    calibration: Option<bool>,
    drone_bay_volume: Option<bool>,
    drone_bandwidth: Option<bool>,
    fighter_bay_volume: Option<bool>,
    agility: Option<bool>,
    align_time: Option<bool>,
    speed: Option<bool>,
    hp: Option<bool>,
    dps: Option<HStatOption<HStatOptionFitDps>>,
    volley: Option<HStatOption<HStatOptionFitVolley>>,
    ehp: Option<HStatOption<HStatOptionEhp>>,
    wc_ehp: Option<bool>,
    rps: Option<HStatOption<HStatOptionRps>>,
    erps: Option<HStatOption<HStatOptionErps>>,
    resists: Option<bool>,
    remote_rps: Option<HStatOption<HStatOptionFitRemoteRps>>,
    remote_cps: Option<bool>,
}
impl HGetFitStatsCmd {
    pub(crate) fn execute(&self, core_sol: &mut rc::SolarSystem, fit_id: &rc::FitId) -> Result<HFitStats, HExecError> {
        let mut core_fit = get_primary_fit(core_sol, fit_id)?;
        let mut stats = HFitStats::new();
        if self.high_slots.unwrap_or(self.default) {
            stats.high_slots = Some(core_fit.get_stat_high_slots().into());
        }
        if self.mid_slots.unwrap_or(self.default) {
            stats.mid_slots = Some(core_fit.get_stat_mid_slots().into());
        }
        if self.low_slots.unwrap_or(self.default) {
            stats.low_slots = Some(core_fit.get_stat_low_slots().into());
        }
        if self.turret_slots.unwrap_or(self.default) {
            stats.turret_slots = Some(core_fit.get_stat_turret_slots().into());
        }
        if self.launcher_slots.unwrap_or(self.default) {
            stats.launcher_slots = Some(core_fit.get_stat_launcher_slots().into());
        }
        if self.rig_slots.unwrap_or(self.default) {
            stats.rig_slots = Some(core_fit.get_stat_rig_slots().into());
        }
        if self.service_slots.unwrap_or(self.default) {
            stats.service_slots = Some(core_fit.get_stat_service_slots().into());
        }
        if self.subsystem_slots.unwrap_or(self.default) {
            stats.subsystem_slots = Some(core_fit.get_stat_subsystem_slots().into());
        }
        if self.launched_drones.unwrap_or(self.default) {
            stats.launched_drones = Some(core_fit.get_stat_launched_drones().into());
        }
        if self.launched_fighters.unwrap_or(self.default) {
            stats.launched_fighters = Some(core_fit.get_stat_launched_fighters().into());
        }
        if self.launched_light_fighters.unwrap_or(self.default) {
            stats.launched_light_fighters = Some(core_fit.get_stat_launched_light_fighters().into());
        }
        if self.launched_heavy_fighters.unwrap_or(self.default) {
            stats.launched_heavy_fighters = Some(core_fit.get_stat_launched_heavy_fighters().into());
        }
        if self.launched_support_fighters.unwrap_or(self.default) {
            stats.launched_support_fighters = Some(core_fit.get_stat_launched_support_fighters().into());
        }
        if self.launched_st_light_fighters.unwrap_or(self.default) {
            stats.launched_st_light_fighters = Some(core_fit.get_stat_launched_st_light_fighters().into());
        }
        if self.launched_st_heavy_fighters.unwrap_or(self.default) {
            stats.launched_st_heavy_fighters = Some(core_fit.get_stat_launched_st_heavy_fighters().into());
        }
        if self.launched_st_support_fighters.unwrap_or(self.default) {
            stats.launched_st_support_fighters = Some(core_fit.get_stat_launched_st_support_fighters().into());
        }
        if self.cpu.unwrap_or(self.default) {
            stats.cpu = Some(core_fit.get_stat_cpu().into());
        }
        if self.powergrid.unwrap_or(self.default) {
            stats.powergrid = Some(core_fit.get_stat_powergrid().into());
        }
        if self.calibration.unwrap_or(self.default) {
            stats.calibration = Some(core_fit.get_stat_calibration().into());
        }
        if self.drone_bay_volume.unwrap_or(self.default) {
            stats.drone_bay_volume = Some(core_fit.get_stat_drone_bay_volume().into());
        }
        if self.drone_bandwidth.unwrap_or(self.default) {
            stats.drone_bandwidth = Some(core_fit.get_stat_drone_bandwidth().into());
        }
        if self.fighter_bay_volume.unwrap_or(self.default) {
            stats.fighter_bay_volume = Some(core_fit.get_stat_fighter_bay_volume().into());
        }
        if self.agility.unwrap_or(self.default) {
            stats.agility = core_fit.get_stat_agility().unwrap_or_default().into();
        }
        if self.align_time.unwrap_or(self.default) {
            stats.align_time = core_fit.get_stat_align_time().unwrap_or_default().into();
        }
        if self.speed.unwrap_or(self.default) {
            stats.speed = core_fit.get_stat_speed().into();
        }
        if self.hp.unwrap_or(self.default) {
            stats.hp = core_fit.get_stat_hp().into();
        }
        let dps_opt = HStatResolvedOption::new(&self.dps, self.default);
        if dps_opt.enabled {
            stats.dps = get_dps_stats(&mut core_fit, dps_opt.options);
        }
        let volley_opt = HStatResolvedOption::new(&self.volley, self.default);
        if volley_opt.enabled {
            stats.volley = get_volley_stats(&mut core_fit, volley_opt.options);
        }
        let ehp_opt = HStatResolvedOption::new(&self.ehp, self.default);
        if ehp_opt.enabled {
            stats.ehp = get_ehp_stats(&mut core_fit, ehp_opt.options).into();
        }
        if self.wc_ehp.unwrap_or(self.default) {
            stats.wc_ehp = core_fit.get_stat_wc_ehp().ok().map(HStatTank::from_opt).into();
        }
        let rps_opt = HStatResolvedOption::new(&self.rps, self.default);
        if rps_opt.enabled {
            stats.rps = get_rps_stats(&mut core_fit, rps_opt.options).into();
        }
        let erps_opt = HStatResolvedOption::new(&self.erps, self.default);
        if erps_opt.enabled {
            stats.erps = get_erps_stats(&mut core_fit, erps_opt.options).into();
        }
        if self.resists.unwrap_or(self.default) {
            stats.resists = core_fit.get_stat_resists().into();
        }
        let rrps_opt = HStatResolvedOption::new(&self.remote_rps, self.default);
        if rrps_opt.enabled {
            stats.remote_rps = Some(get_remote_rps_stats(&mut core_fit, rrps_opt.options));
        }
        if self.remote_cps.unwrap_or(self.default) {
            stats.remote_cps = Some(core_fit.get_stat_remote_cps());
        }
        Ok(stats)
    }
}

fn get_dps_stats(core_fit: &mut rc::FitMut, options: Vec<HStatOptionFitDps>) -> Option<Vec<HStatDmg>> {
    let mut results = Vec::with_capacity(options.len());
    for option in options {
        let core_item_kinds = (&option.item_kinds).into();
        let core_spool = option.spool.map(|h_spool| h_spool.into());
        let core_stat = core_fit.get_stat_dps(core_item_kinds, option.reload, core_spool);
        results.push(core_stat.into());
    }
    Some(results)
}

fn get_volley_stats(core_fit: &mut rc::FitMut, options: Vec<HStatOptionFitVolley>) -> Option<Vec<HStatDmg>> {
    let mut results = Vec::with_capacity(options.len());
    for option in options {
        let core_item_kinds = (&option.item_kinds).into();
        let core_spool = option.spool.map(|h_spool| h_spool.into());
        let core_stat = core_fit.get_stat_volley(core_item_kinds, core_spool);
        results.push(core_stat.into());
    }
    Some(results)
}

fn get_ehp_stats(
    core_fit: &mut rc::FitMut,
    options: Vec<HStatOptionEhp>,
) -> Option<Vec<HStatTank<Option<HStatLayerEhp>>>> {
    let mut results = Vec::with_capacity(options.len());
    for option in options {
        let core_incoming_dps = option.incoming_dps.map(|h_incoming_dps| h_incoming_dps.into());
        match core_fit.get_stat_ehp(core_incoming_dps) {
            Ok(core_result) => results.push(HStatTank::from_opt(core_result)),
            Err(_) => return None,
        }
    }
    Some(results)
}

fn get_rps_stats(core_fit: &mut rc::FitMut, options: Vec<HStatOptionRps>) -> Option<Vec<HStatTank<HStatLayerRps>>> {
    let mut results = Vec::with_capacity(options.len());
    for option in options {
        match core_fit.get_stat_rps(option.spool.map(|v| v.into())) {
            Ok(core_result) => results.push(core_result.into()),
            Err(_) => return None,
        }
    }
    Some(results)
}

fn get_erps_stats(
    core_fit: &mut rc::FitMut,
    options: Vec<HStatOptionErps>,
) -> Option<Vec<HStatTank<Option<HStatLayerErps>>>> {
    let mut results = Vec::with_capacity(options.len());
    for option in options {
        let core_incoming_dps = option.incoming_dps.map(|h_incoming_dps| h_incoming_dps.into());
        let core_spool = option.spool.map(|h_spool| h_spool.into());
        match core_fit.get_stat_erps(core_incoming_dps, core_spool) {
            Ok(core_result) => results.push(HStatTank::from_opt(core_result)),
            Err(_) => return None,
        }
    }
    Some(results)
}

fn get_remote_rps_stats(
    core_fit: &mut rc::FitMut,
    options: Vec<HStatOptionFitRemoteRps>,
) -> Vec<HStatTank<rc::AttrVal>> {
    options
        .iter()
        .map(|option| {
            let core_item_kinds = rc::stats::StatRrItemKinds::all_enabled();
            let core_spool = option.spool.map(|h_spool| h_spool.into());
            core_fit.get_stat_remote_rps(core_item_kinds, core_spool).into()
        })
        .collect()
}
