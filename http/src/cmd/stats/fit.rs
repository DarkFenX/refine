use crate::{
    cmd::{
        shared::get_primary_fit,
        stats::options::{
            HStatOption, HStatOptionCapBalance, HStatOptionCapSim, HStatOptionEhp, HStatOptionErps, HStatOptionFitDps,
            HStatOptionFitMining, HStatOptionFitOutNps, HStatOptionFitOutRps, HStatOptionFitVolley, HStatOptionRps,
            HStatResolvedOption,
        },
    },
    info::{
        HFitStats,
        stats::{
            HStatCapSim, HStatDmg, HStatLayerEhp, HStatLayerErps, HStatLayerErpsRegen, HStatLayerRps,
            HStatLayerRpsRegen, HStatMining, HStatTank, HStatTankRegen,
        },
    },
    util::{HExecError, default_true},
};

#[derive(educe::Educe, serde::Deserialize)]
#[educe(Default)]
pub(crate) struct HGetFitStatsCmd {
    #[serde(default = "default_true")]
    #[educe(Default = true)]
    default: bool,
    // Fit output stats
    dps: Option<HStatOption<HStatOptionFitDps>>,
    volley: Option<HStatOption<HStatOptionFitVolley>>,
    mps: Option<HStatOption<HStatOptionFitMining>>,
    outgoing_nps: Option<HStatOption<HStatOptionFitOutNps>>,
    outgoing_rps: Option<HStatOption<HStatOptionFitOutRps>>,
    outgoing_cps: Option<bool>,
    // Fit resources
    cpu: Option<bool>,
    powergrid: Option<bool>,
    calibration: Option<bool>,
    drone_bay_volume: Option<bool>,
    drone_bandwidth: Option<bool>,
    fighter_bay_volume: Option<bool>,
    // Fit slots
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
    // Ship tank
    resists: Option<bool>,
    hp: Option<bool>,
    ehp: Option<HStatOption<HStatOptionEhp>>,
    wc_ehp: Option<bool>,
    rps: Option<HStatOption<HStatOptionRps>>,
    erps: Option<HStatOption<HStatOptionErps>>,
    // Ship cap
    cap_amount: Option<bool>,
    cap_balance: Option<HStatOption<HStatOptionCapBalance>>,
    cap_sim: Option<HStatOption<HStatOptionCapSim>>,
    neut_resist: Option<bool>,
    // Ship sensors
    locks: Option<bool>,
    lock_range: Option<bool>,
    scan_res: Option<bool>,
    sensors: Option<bool>,
    dscan_range: Option<bool>,
    probing_size: Option<bool>,
    incoming_jam: Option<bool>,
    // Ship mobility
    speed: Option<bool>,
    agility: Option<bool>,
    align_time: Option<bool>,
    sig_radius: Option<bool>,
    mass: Option<bool>,
    warp_speed: Option<bool>,
    max_warp_range: Option<bool>,
    // Ship misc stats
    drone_control_range: Option<bool>,
    can_warp: Option<bool>,
}
impl HGetFitStatsCmd {
    pub(crate) fn execute(&self, core_sol: &mut rc::SolarSystem, fit_id: &rc::FitId) -> Result<HFitStats, HExecError> {
        let mut core_fit = get_primary_fit(core_sol, fit_id)?;
        let mut stats = HFitStats::new();
        ////////////////////////////////////////////////////////////////////////////////////////////
        // Fit output stats
        ////////////////////////////////////////////////////////////////////////////////////////////
        let dps_opt = HStatResolvedOption::new(&self.dps, self.default);
        if dps_opt.enabled {
            stats.dps = Some(get_dps_stats(&mut core_fit, dps_opt.options));
        }
        let volley_opt = HStatResolvedOption::new(&self.volley, self.default);
        if volley_opt.enabled {
            stats.volley = Some(get_volley_stats(&mut core_fit, volley_opt.options));
        }
        let mps_opt = HStatResolvedOption::new(&self.mps, self.default);
        if mps_opt.enabled {
            stats.mps = Some(get_mps_stats(&mut core_fit, mps_opt.options));
        }
        let out_nps_opt = HStatResolvedOption::new(&self.outgoing_nps, self.default);
        if out_nps_opt.enabled {
            stats.outgoing_nps = Some(get_outgoing_nps_stats(&mut core_fit, out_nps_opt.options));
        }
        let out_rps_opt = HStatResolvedOption::new(&self.outgoing_rps, self.default);
        if out_rps_opt.enabled {
            stats.outgoing_rps = Some(get_outgoing_rps_stats(&mut core_fit, out_rps_opt.options));
        }
        if self.outgoing_cps.unwrap_or(self.default) {
            stats.outgoing_cps = Some(core_fit.get_stat_outgoing_cps());
        }
        ////////////////////////////////////////////////////////////////////////////////////////////
        // Fit resources
        ////////////////////////////////////////////////////////////////////////////////////////////
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
        ////////////////////////////////////////////////////////////////////////////////////////////
        // Fit slots
        ////////////////////////////////////////////////////////////////////////////////////////////
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
        ////////////////////////////////////////////////////////////////////////////////////////////
        // Ship tank
        ////////////////////////////////////////////////////////////////////////////////////////////
        if self.resists.unwrap_or(self.default) {
            stats.resists = core_fit.get_stat_resists().into();
        }
        if self.hp.unwrap_or(self.default) {
            stats.hp = core_fit.get_stat_hp().into();
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
        ////////////////////////////////////////////////////////////////////////////////////////////
        // Ship cap
        ////////////////////////////////////////////////////////////////////////////////////////////
        if self.cap_amount.unwrap_or(self.default) {
            stats.cap_amount = core_fit.get_stat_cap_amount().into();
        }
        let cap_blc_opt = HStatResolvedOption::new(&self.cap_balance, self.default);
        if cap_blc_opt.enabled {
            stats.cap_balance = get_cap_balance_stats(&mut core_fit, cap_blc_opt.options).into();
        }
        let cap_sim_opt = HStatResolvedOption::new(&self.cap_sim, self.default);
        if cap_sim_opt.enabled {
            stats.cap_sim = get_cap_sim_stats(&mut core_fit, cap_sim_opt.options).into();
        }
        if self.neut_resist.unwrap_or(self.default) {
            stats.neut_resist = core_fit.get_stat_neut_resist().into();
        }
        ////////////////////////////////////////////////////////////////////////////////////////////
        // Ship sensors
        ////////////////////////////////////////////////////////////////////////////////////////////
        if self.locks.unwrap_or(self.default) {
            stats.locks = core_fit.get_stat_locks().into();
        }
        if self.lock_range.unwrap_or(self.default) {
            stats.lock_range = core_fit.get_stat_lock_range().into();
        }
        if self.scan_res.unwrap_or(self.default) {
            stats.scan_res = core_fit.get_stat_scan_res().into();
        }
        if self.sensors.unwrap_or(self.default) {
            stats.sensors = core_fit.get_stat_sensors().into();
        }
        if self.dscan_range.unwrap_or(self.default) {
            stats.dscan_range = core_fit.get_stat_dscan_range().into();
        }
        if self.probing_size.unwrap_or(self.default) {
            stats.probing_size = core_fit.get_stat_probing_size().unwrap_or_default().into();
        }
        if self.incoming_jam.unwrap_or(self.default) {
            stats.incoming_jam = core_fit.get_stat_incoming_jam().into();
        }
        ////////////////////////////////////////////////////////////////////////////////////////////
        // Ship mobility
        ////////////////////////////////////////////////////////////////////////////////////////////
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
        ////////////////////////////////////////////////////////////////////////////////////////////
        // Ship misc stats
        ////////////////////////////////////////////////////////////////////////////////////////////
        if self.drone_control_range.unwrap_or(self.default) {
            stats.drone_control_range = core_fit.get_stat_drone_control_range().into();
        }
        if self.can_warp.unwrap_or(self.default) {
            stats.can_warp = core_fit.get_stat_can_warp().into();
        }
        Ok(stats)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Fit output stats
////////////////////////////////////////////////////////////////////////////////////////////////////
fn get_dps_stats(core_fit: &mut rc::FitMut, options: Vec<HStatOptionFitDps>) -> Vec<Option<HStatDmg>> {
    let mut results = Vec::with_capacity(options.len());
    for option in options {
        let core_item_kinds = (&option.item_kinds).into();
        let core_spool = option.spool.map(Into::into);
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
        let core_spool = option.spool.map(Into::into);
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
fn get_mps_stats(core_fit: &mut rc::FitMut, options: Vec<HStatOptionFitMining>) -> Vec<HStatMining> {
    let mut results = Vec::with_capacity(options.len());
    for option in options {
        let core_result = core_fit.get_stat_mps((&option.item_kinds).into(), option.reload);
        results.push(core_result.into());
    }
    results
}
fn get_outgoing_nps_stats(core_fit: &mut rc::FitMut, options: Vec<HStatOptionFitOutNps>) -> Vec<Option<rc::AttrVal>> {
    let mut results = Vec::with_capacity(options.len());
    for option in options {
        let core_item_kinds = (&option.item_kinds).into();
        match option.projectee_item_id {
            Some(projectee_item_id) => {
                match core_fit.get_stat_outgoing_nps_applied(core_item_kinds, &projectee_item_id) {
                    Ok(result) => results.push(Some(result)),
                    Err(_) => results.push(None),
                }
            }
            None => {
                let result = core_fit.get_stat_outgoing_nps(core_item_kinds);
                results.push(Some(result));
            }
        }
    }
    results
}
fn get_outgoing_rps_stats(
    core_fit: &mut rc::FitMut,
    options: Vec<HStatOptionFitOutRps>,
) -> Vec<HStatTank<rc::AttrVal>> {
    options
        .iter()
        .map(|option| {
            let core_item_kinds = (&option.item_kinds).into();
            let core_spool = option.spool.map(Into::into);
            core_fit.get_stat_outgoing_rps(core_item_kinds, core_spool).into()
        })
        .collect()
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Ship tank
////////////////////////////////////////////////////////////////////////////////////////////////////
fn get_ehp_stats(
    core_fit: &mut rc::FitMut,
    options: Vec<HStatOptionEhp>,
) -> Option<Vec<HStatTank<Option<HStatLayerEhp>>>> {
    let mut results = Vec::with_capacity(options.len());
    for option in options {
        let core_incoming_dps = option.incoming_dps.map(Into::into);
        match core_fit.get_stat_ehp(core_incoming_dps) {
            Ok(core_result) => results.push(HStatTank::from_opt(core_result)),
            Err(_) => return None,
        }
    }
    Some(results)
}
fn get_rps_stats(
    core_fit: &mut rc::FitMut,
    options: Vec<HStatOptionRps>,
) -> Option<Vec<HStatTankRegen<HStatLayerRps, HStatLayerRpsRegen>>> {
    let mut results = Vec::with_capacity(options.len());
    for option in options {
        let core_shield = rc::UnitInterval::new_clamped(option.shield_perc);
        let core_spool = option.spool.map(Into::into);
        match core_fit.get_stat_rps(core_shield, core_spool) {
            Ok(core_result) => results.push(core_result.into()),
            Err(_) => return None,
        }
    }
    Some(results)
}
fn get_erps_stats(
    core_fit: &mut rc::FitMut,
    options: Vec<HStatOptionErps>,
) -> Option<Vec<HStatTankRegen<Option<HStatLayerErps>, Option<HStatLayerErpsRegen>>>> {
    let mut results = Vec::with_capacity(options.len());
    for option in options {
        let core_incoming_dps = option.incoming_dps.map(Into::into);
        let core_shield = rc::UnitInterval::new_clamped(option.shield_perc);
        let core_spool = option.spool.map(Into::into);
        match core_fit.get_stat_erps(core_incoming_dps, core_shield, core_spool) {
            Ok(core_result) => results.push(HStatTankRegen::from_opt(core_result)),
            Err(_) => return None,
        }
    }
    Some(results)
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Ship cap
////////////////////////////////////////////////////////////////////////////////////////////////////
fn get_cap_balance_stats(core_fit: &mut rc::FitMut, options: Vec<HStatOptionCapBalance>) -> Option<Vec<rc::AttrVal>> {
    let mut results = Vec::with_capacity(options.len());
    for option in options {
        let core_src_kinds = (&option.src_kinds).into();
        match core_fit.get_stat_cap_balance(core_src_kinds) {
            Ok(result) => results.push(result),
            Err(_) => return None,
        }
    }
    Some(results)
}
fn get_cap_sim_stats(core_fit: &mut rc::FitMut, options: Vec<HStatOptionCapSim>) -> Option<Vec<HStatCapSim>> {
    let mut results = Vec::with_capacity(options.len());
    for option in options {
        let cap_perc = rc::UnitInterval::new_clamped(option.cap_perc);
        let stagger = (&option.stagger).into();
        match core_fit.get_stat_cap_sim(cap_perc, stagger) {
            Ok(result) => results.push(result.into()),
            Err(_) => return None,
        }
    }
    Some(results)
}
