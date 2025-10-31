use crate::{
    cmd::{
        shared::get_primary_fit,
        stats::options::{
            HStatOption, HStatOptionCapBalance, HStatOptionEhp, HStatOptionErps, HStatOptionFitDps,
            HStatOptionFitRemoteNps, HStatOptionFitRemoteRps, HStatOptionFitVolley, HStatOptionRps,
            HStatResolvedOption,
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
    speed: Option<bool>,
    agility: Option<bool>,
    align_time: Option<bool>,
    sig_radius: Option<bool>,
    mass: Option<bool>,
    warp_speed: Option<bool>,
    max_warp_range: Option<bool>,
    locks: Option<bool>,
    lock_range: Option<bool>,
    scan_res: Option<bool>,
    sensor: Option<bool>,
    dscan_range: Option<bool>,
    probing_size: Option<bool>,
    jam_chance: Option<bool>,
    drone_control_range: Option<bool>,
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
    remote_nps: Option<HStatOption<HStatOptionFitRemoteNps>>,
    cap_amount: Option<bool>,
    cap_balance: Option<HStatOption<HStatOptionCapBalance>>,
    neut_resist: Option<bool>,
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
        if self.speed.unwrap_or(self.default) {
            stats.speed = core_fit.get_stat_speed().into();
        }
        if self.agility.unwrap_or(self.default) {
            stats.agility = core_fit.get_stat_agility().unwrap_or_default().into();
        }
        if self.align_time.unwrap_or(self.default) {
            stats.align_time = core_fit.get_stat_align_time().unwrap_or_default().into();
        }
        if self.sig_radius.unwrap_or(self.default) {
            stats.sig_radius = core_fit.get_stat_sig_radius().into();
        }
        if self.mass.unwrap_or(self.default) {
            stats.mass = core_fit.get_stat_mass().into();
        }
        if self.warp_speed.unwrap_or(self.default) {
            stats.warp_speed = core_fit.get_stat_warp_speed().unwrap_or_default().into();
        }
        if self.max_warp_range.unwrap_or(self.default) {
            stats.max_warp_range = core_fit.get_stat_max_warp_range().unwrap_or_default().into();
        }
        if self.locks.unwrap_or(self.default) {
            stats.locks = core_fit.get_stat_locks().into();
        }
        if self.lock_range.unwrap_or(self.default) {
            stats.lock_range = core_fit.get_stat_lock_range().into();
        }
        if self.scan_res.unwrap_or(self.default) {
            stats.scan_res = core_fit.get_stat_scan_res().into();
        }
        if self.sensor.unwrap_or(self.default) {
            stats.sensor = core_fit.get_stat_sensor().into();
        }
        if self.dscan_range.unwrap_or(self.default) {
            stats.dscan_range = core_fit.get_stat_dscan_range().into();
        }
        if self.probing_size.unwrap_or(self.default) {
            stats.probing_size = core_fit.get_stat_probing_size().unwrap_or_default().into();
        }
        if self.jam_chance.unwrap_or(self.default) {
            stats.jam_chance = core_fit.get_stat_jam_chance().into();
        }
        if self.drone_control_range.unwrap_or(self.default) {
            stats.drone_control_range = core_fit.get_stat_drone_control_range().into();
        }
        if self.hp.unwrap_or(self.default) {
            stats.hp = core_fit.get_stat_hp().into();
        }
        let dps_opt = HStatResolvedOption::new(&self.dps, self.default);
        if dps_opt.enabled {
            stats.dps = Some(get_dps_stats(&mut core_fit, dps_opt.options));
        }
        let volley_opt = HStatResolvedOption::new(&self.volley, self.default);
        if volley_opt.enabled {
            stats.volley = Some(get_volley_stats(&mut core_fit, volley_opt.options));
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
        let rnps_opt = HStatResolvedOption::new(&self.remote_nps, self.default);
        if rnps_opt.enabled {
            stats.remote_nps = Some(get_remote_nps_stats(&mut core_fit, rnps_opt.options));
        }
        if self.cap_amount.unwrap_or(self.default) {
            stats.cap_amount = core_fit.get_stat_cap_amount().into();
        }
        let cblc_opt = HStatResolvedOption::new(&self.cap_balance, self.default);
        if cblc_opt.enabled {
            stats.cap_balance = get_cap_balance_stats(&mut core_fit, cblc_opt.options).into();
        }
        if self.neut_resist.unwrap_or(self.default) {
            stats.neut_resist = core_fit.get_stat_neut_resist().into();
        }
        Ok(stats)
    }
}

fn get_dps_stats(core_fit: &mut rc::FitMut, options: Vec<HStatOptionFitDps>) -> Vec<Option<HStatDmg>> {
    let mut results = Vec::with_capacity(options.len());
    for option in options {
        let core_item_kinds = (&option.item_kinds).into();
        let core_spool = option.spool.map(|h_spool| h_spool.into());
        match option.projectee_item_id {
            Some(projectee_item_id) => {
                match core_fit.get_stat_dps_applied(core_item_kinds, option.reload, core_spool, &projectee_item_id) {
                    Ok(core_stat) => results.push(Some(core_stat.into())),
                    Err(_) => results.push(None),
                };
            }
            None => {
                let core_stat = core_fit.get_stat_dps(core_item_kinds, option.reload, core_spool);
                results.push(Some(core_stat.into()));
            }
        }
    }
    results
}

fn get_volley_stats(core_fit: &mut rc::FitMut, options: Vec<HStatOptionFitVolley>) -> Vec<Option<HStatDmg>> {
    let mut results = Vec::with_capacity(options.len());
    for option in options {
        let core_item_kinds = (&option.item_kinds).into();
        let core_spool = option.spool.map(|h_spool| h_spool.into());
        match option.projectee_item_id {
            Some(projectee_item_id) => {
                match core_fit.get_stat_volley_applied(core_item_kinds, core_spool, &projectee_item_id) {
                    Ok(core_stat) => results.push(Some(core_stat.into())),
                    Err(_) => results.push(None),
                };
            }
            None => {
                let core_stat = core_fit.get_stat_volley(core_item_kinds, core_spool);
                results.push(Some(core_stat.into()));
            }
        }
    }
    results
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
            let core_item_kinds = (&option.item_kinds).into();
            let core_spool = option.spool.map(|h_spool| h_spool.into());
            core_fit.get_stat_remote_rps(core_item_kinds, core_spool).into()
        })
        .collect()
}

fn get_remote_nps_stats(core_fit: &mut rc::FitMut, options: Vec<HStatOptionFitRemoteNps>) -> Vec<Option<rc::AttrVal>> {
    let mut results = Vec::with_capacity(options.len());
    for option in options {
        let core_item_kinds = (&option.item_kinds).into();
        match option.projectee_item_id {
            Some(projectee_item_id) => {
                match core_fit.get_stat_remote_nps_applied(core_item_kinds, &projectee_item_id) {
                    Ok(result) => results.push(Some(result)),
                    Err(_) => results.push(None),
                }
            }
            None => {
                let result = core_fit.get_stat_remote_nps(core_item_kinds);
                results.push(Some(result));
            }
        }
    }
    results
}

fn get_cap_balance_stats(core_fit: &mut rc::FitMut, options: Vec<HStatOptionCapBalance>) -> Option<Vec<rc::AttrVal>> {
    let mut results = Vec::with_capacity(options.len());
    for option in options {
        let core_src_kinds = (&option.src_kinds).into();
        match core_fit.get_stat_cap_balance(core_src_kinds, option.regen_perc) {
            Ok(result) => results.push(result),
            Err(_) => return None,
        }
    }
    Some(results)
}
