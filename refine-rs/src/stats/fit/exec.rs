use crate::{
    PValue, Value,
    stats::{
        FitStats, StatCapSim, StatDmg, StatEhp, StatErps, StatInJam, StatJump, StatMining, StatOptionCapBlc,
        StatOptionCapSim, StatOptionEhp, StatOptionErps, StatOptionFitDmg, StatOptionFitMining, StatOptionFitOutCps,
        StatOptionFitOutNps, StatOptionFitOutRps, StatOptionIncomingJam, StatOptionJump, StatOptionMass, StatOptionRps,
        StatOutReps, StatResult, StatRps,
        err::{StatFitAppliedError, StatFitShipAppliedError, StatFitShipError, StatJumpError},
        fatal::StatErrorFatality,
        fit::FitStatsOptionsResolved,
    },
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl FitStatsOptionsResolved {
    pub(super) fn execute(&self, core_fit: &mut rc::FitMut) -> FitStats {
        let mut stats = FitStats { .. };
        // Fit output stats
        if let Some(options) = self.dmg.get() {
            stats.dmg = get_dmg_stats(core_fit, options);
        }
        if let Some(options) = self.mps.get() {
            stats.mps = get_mps_stats(core_fit, options);
        }
        if let Some(options) = self.outgoing_nps.get() {
            stats.outgoing_nps = get_outgoing_nps_stats(core_fit, options);
        }
        if let Some(options) = self.outgoing_cps.get() {
            stats.outgoing_cps = get_outgoing_cps_stats(core_fit, options);
        }
        if let Some(options) = self.outgoing_rps.get() {
            stats.outgoing_rps = get_outgoing_rps_stats(core_fit, options);
        }
        // Fit resources
        if self.cpu.is_enabled() {
            stats.cpu = StatResult::from_stat(core_fit.get_stat_cpu());
        }
        if self.powergrid.is_enabled() {
            stats.powergrid = StatResult::from_stat(core_fit.get_stat_powergrid());
        }
        if self.calibration.is_enabled() {
            stats.calibration = StatResult::from_stat(core_fit.get_stat_calibration());
        }
        if self.drone_bay_volume.is_enabled() {
            stats.drone_bay_volume = StatResult::from_stat(core_fit.get_stat_drone_bay_volume());
        }
        if self.drone_bandwidth.is_enabled() {
            stats.drone_bandwidth = StatResult::from_stat(core_fit.get_stat_drone_bandwidth());
        }
        if self.fighter_bay_volume.is_enabled() {
            stats.fighter_bay_volume = StatResult::from_stat(core_fit.get_stat_fighter_bay_volume());
        }
        // Fit slots
        if self.high_slots.is_enabled() {
            stats.high_slots = StatResult::from_stat(core_fit.get_stat_high_slots());
        }
        if self.mid_slots.is_enabled() {
            stats.mid_slots = StatResult::from_stat(core_fit.get_stat_mid_slots());
        }
        if self.low_slots.is_enabled() {
            stats.low_slots = StatResult::from_stat(core_fit.get_stat_low_slots());
        }
        if self.turret_slots.is_enabled() {
            stats.turret_slots = StatResult::from_stat(core_fit.get_stat_turret_slots());
        }
        if self.launcher_slots.is_enabled() {
            stats.launcher_slots = StatResult::from_stat(core_fit.get_stat_launcher_slots());
        }
        if self.rig_slots.is_enabled() {
            stats.rig_slots = StatResult::from_stat(core_fit.get_stat_rig_slots());
        }
        if self.service_slots.is_enabled() {
            stats.service_slots = StatResult::from_stat(core_fit.get_stat_service_slots());
        }
        if self.subsystem_slots.is_enabled() {
            stats.subsystem_slots = StatResult::from_stat(core_fit.get_stat_subsystem_slots());
        }
        if self.launched_drones.is_enabled() {
            stats.launched_drones = StatResult::from_stat(core_fit.get_stat_launched_drones());
        }
        if self.launched_fighters.is_enabled() {
            stats.launched_fighters = StatResult::from_stat(core_fit.get_stat_launched_fighters());
        }
        if self.launched_light_fighters.is_enabled() {
            stats.launched_light_fighters = StatResult::from_stat(core_fit.get_stat_launched_light_fighters());
        }
        if self.launched_heavy_fighters.is_enabled() {
            stats.launched_heavy_fighters = StatResult::from_stat(core_fit.get_stat_launched_heavy_fighters());
        }
        if self.launched_support_fighters.is_enabled() {
            stats.launched_support_fighters = StatResult::from_stat(core_fit.get_stat_launched_support_fighters());
        }
        if self.launched_st_light_fighters.is_enabled() {
            stats.launched_st_light_fighters = StatResult::from_stat(core_fit.get_stat_launched_st_light_fighters());
        }
        if self.launched_st_heavy_fighters.is_enabled() {
            stats.launched_st_heavy_fighters = StatResult::from_stat(core_fit.get_stat_launched_st_heavy_fighters());
        }
        if self.launched_st_support_fighters.is_enabled() {
            stats.launched_st_support_fighters =
                StatResult::from_stat(core_fit.get_stat_launched_st_support_fighters());
        }
        // Ship tank
        if self.resists.is_enabled() {
            stats.resists = StatResult::from_result_outer(core_fit.get_stat_resists());
        }
        if self.hp.is_enabled() {
            stats.hp = StatResult::from_result_outer(core_fit.get_stat_hp());
        }
        if let Some(options) = self.ehp.get() {
            stats.ehp = get_ehp_stats(core_fit, options);
        }
        if self.wc_ehp.is_enabled() {
            stats.wc_ehp = StatResult::from_result_outer(core_fit.get_stat_wc_ehp());
        }
        if let Some(options) = self.rps.get() {
            stats.rps = get_rps_stats(core_fit, options);
        }
        if let Some(options) = self.erps.get() {
            stats.erps = get_erps_stats(core_fit, options);
        }
        if self.breach_resist.is_enabled() {
            stats.breach_resist = StatResult::from_result_outer(core_fit.get_stat_breach_resist());
        }
        // Ship cap
        if self.cap_amount.is_enabled() {
            stats.cap_amount = StatResult::from_result_outer(core_fit.get_stat_cap_amount());
        }
        if let Some(options) = self.cap_balance.get() {
            stats.cap_balance = get_cap_balance_stats(core_fit, options);
        }
        if let Some(options) = self.cap_sim.get() {
            stats.cap_sim = get_cap_sim_stats(core_fit, options);
        }
        if self.neut_resist.is_enabled() {
            stats.neut_resist = StatResult::from_result_outer(core_fit.get_stat_neut_resist());
        }
        // Ship sensors
        if self.locks.is_enabled() {
            stats.locks = StatResult::from_result_outer(core_fit.get_stat_locks());
        }
        if self.lock_range.is_enabled() {
            stats.lock_range = StatResult::from_result_outer(core_fit.get_stat_lock_range());
        }
        if self.scan_res.is_enabled() {
            stats.scan_res = StatResult::from_result_outer(core_fit.get_stat_scan_res());
        }
        if self.sensors.is_enabled() {
            stats.sensors = StatResult::from_result_outer(core_fit.get_stat_sensors());
        }
        if self.dscan_range.is_enabled() {
            stats.dscan_range = StatResult::from_result_outer(core_fit.get_stat_dscan_range());
        }
        if self.probing_size.is_enabled() {
            stats.probing_size = StatResult::from_result_outer(core_fit.get_stat_probing_size());
        }
        if let Some(options) = self.incoming_jam.get() {
            stats.incoming_jam = get_incoming_jam_stats(core_fit, options);
        }
        // Ship mobility
        if self.speed.is_enabled() {
            stats.speed = StatResult::from_result_outer(core_fit.get_stat_speed());
        }
        if self.agility.is_enabled() {
            stats.agility = StatResult::from_result_outer(core_fit.get_stat_agility());
        }
        if self.align_time.is_enabled() {
            stats.align_time = StatResult::from_result_outer(core_fit.get_stat_align_time());
        }
        if self.sig_radius.is_enabled() {
            stats.sig_radius = StatResult::from_result_outer(core_fit.get_stat_sig_radius());
        }
        if let Some(options) = self.mass.get() {
            stats.mass = get_mass_stats(core_fit, options);
        }
        if self.warp_speed.is_enabled() {
            stats.warp_speed = StatResult::from_result_outer(core_fit.get_stat_warp_speed());
        }
        if self.max_warp_range.is_enabled() {
            stats.max_warp_range = StatResult::from_result_outer(core_fit.get_stat_max_warp_range());
        }
        if let Some(options) = self.jump.get() {
            stats.jump = get_jump_stats(core_fit, options);
        }
        // Ship misc stats
        if self.drone_control_range.is_enabled() {
            stats.drone_control_range = StatResult::from_result_outer(core_fit.get_stat_drone_control_range());
        }
        if self.can_warp.is_enabled() {
            stats.can_warp = StatResult::from_result_outer(core_fit.get_stat_can_warp());
        }
        if self.can_jump_gate.is_enabled() {
            stats.can_jump_gate = StatResult::from_result_outer(core_fit.get_stat_can_jump_gate());
        }
        if self.can_jump_wormhole.is_enabled() {
            stats.can_jump_wormhole = StatResult::from_result_outer(core_fit.get_stat_can_jump_wormhole());
        }
        if self.can_jump_drive.is_enabled() {
            stats.can_jump_drive = StatResult::from_result_outer(core_fit.get_stat_can_jump_drive());
        }
        if self.can_dock_station.is_enabled() {
            stats.can_dock_station = StatResult::from_result_outer(core_fit.get_stat_can_dock_station());
        }
        if self.can_dock_citadel.is_enabled() {
            stats.can_dock_citadel = StatResult::from_result_outer(core_fit.get_stat_can_dock_citadel());
        }
        if self.can_tether.is_enabled() {
            stats.can_tether = StatResult::from_result_outer(core_fit.get_stat_can_tether());
        }
        stats
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Fit output stats
////////////////////////////////////////////////////////////////////////////////////////////////////
fn get_dmg_stats(
    core_fit: &mut rc::FitMut,
    options: &[StatOptionFitDmg],
) -> StatResult<StatDmg, !, StatFitAppliedError> {
    let mut stats = Vec::with_capacity(options.len());
    for option in options.iter() {
        match option.projectee_item_id {
            Some(projectee_item_id) => {
                let stat = core_fit
                    .get_stat_dmg_applied(option.item_kinds, option.time, option.crits, &projectee_item_id)
                    .map(StatDmg::from_core_applied);
                stats.push(stat);
            }
            None => {
                let stat = StatDmg::from_core(core_fit.get_stat_dmg(option.item_kinds, option.time, option.crits));
                stats.push(Ok(stat));
            }
        }
    }
    StatResult::Result(stats)
}
fn get_mps_stats(core_fit: &mut rc::FitMut, options: &[StatOptionFitMining]) -> StatResult<StatMining, !, !> {
    let mut stats = Vec::with_capacity(options.len());
    for option in options.iter() {
        let stat = core_fit.get_stat_mps(option.item_kinds, option.time, option.resource_kind);
        stats.push(Ok(stat));
    }
    StatResult::Result(stats)
}
fn get_outgoing_nps_stats(
    core_fit: &mut rc::FitMut,
    options: &[StatOptionFitOutNps],
) -> StatResult<PValue, !, StatFitAppliedError> {
    let mut stats = Vec::with_capacity(options.len());
    for option in options.iter() {
        match option.projectee_item_id {
            Some(projectee_item_id) => {
                let stat = core_fit.get_stat_outgoing_nps_applied(option.item_kinds, option.time, &projectee_item_id);
                stats.push(stat);
            }
            None => {
                let stat = core_fit.get_stat_outgoing_nps(option.item_kinds, option.time);
                stats.push(Ok(stat));
            }
        }
    }
    StatResult::Result(stats)
}
fn get_outgoing_rps_stats(
    core_fit: &mut rc::FitMut,
    options: &[StatOptionFitOutRps],
) -> StatResult<StatOutReps, !, StatFitAppliedError> {
    let mut stats = Vec::with_capacity(options.len());
    for option in options.iter() {
        match option.projectee_item_id {
            Some(projectee_item_id) => {
                let stat = core_fit.get_stat_outgoing_rps_applied(option.item_kinds, option.time, &projectee_item_id);
                stats.push(stat);
            }
            None => {
                let stat = core_fit.get_stat_outgoing_rps(option.item_kinds, option.time);
                stats.push(Ok(stat));
            }
        }
    }
    StatResult::Result(stats)
}
fn get_outgoing_cps_stats(
    core_fit: &mut rc::FitMut,
    options: &[StatOptionFitOutCps],
) -> StatResult<PValue, !, StatFitAppliedError> {
    let mut stats = Vec::with_capacity(options.len());
    for option in options.iter() {
        match option.projectee_item_id {
            Some(projectee_item_id) => {
                let stat = core_fit.get_stat_outgoing_cps_applied(option.time, &projectee_item_id);
                stats.push(stat);
            }
            None => {
                let stat = core_fit.get_stat_outgoing_cps(option.time);
                stats.push(Ok(stat));
            }
        }
    }
    StatResult::Result(stats)
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Ship tank
////////////////////////////////////////////////////////////////////////////////////////////////////
fn get_ehp_stats(core_fit: &mut rc::FitMut, options: &[StatOptionEhp]) -> StatResult<StatEhp, StatFitShipError<!>, !> {
    let mut stats = Vec::with_capacity(options.len());
    for option in options.iter() {
        match core_fit.get_stat_ehp(option.incoming_dps) {
            Ok(stat) => stats.push(Ok(stat)),
            Err(err) => return StatResult::Error(err),
        }
    }
    StatResult::Result(stats)
}
fn get_rps_stats(core_fit: &mut rc::FitMut, options: &[StatOptionRps]) -> StatResult<StatRps, StatFitShipError<!>, !> {
    let mut stats = Vec::with_capacity(options.len());
    for option in options.iter() {
        match core_fit.get_stat_rps(option.time, option.shield_perc) {
            Ok(stat) => stats.push(Ok(stat)),
            Err(err) => return StatResult::Error(err),
        }
    }
    StatResult::Result(stats)
}
fn get_erps_stats(
    core_fit: &mut rc::FitMut,
    options: &[StatOptionErps],
) -> StatResult<StatErps, StatFitShipError<!>, !> {
    let mut stats = Vec::with_capacity(options.len());
    for option in options.iter() {
        match core_fit.get_stat_erps(option.incoming_dps, option.time, option.shield_perc) {
            Ok(stat) => stats.push(Ok(stat)),
            Err(err) => return StatResult::Error(err),
        }
    }
    StatResult::Result(stats)
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Ship cap
////////////////////////////////////////////////////////////////////////////////////////////////////
fn get_cap_balance_stats(
    core_fit: &mut rc::FitMut,
    options: &[StatOptionCapBlc],
) -> StatResult<Value, StatFitShipAppliedError<!>, StatFitShipAppliedError<!>> {
    let mut stats = Vec::with_capacity(options.len());
    for option in options.iter() {
        match core_fit.get_stat_cap_balance(option.src_kinds, option.time) {
            Ok(stat) => stats.push(Ok(stat)),
            Err(err) => match err.is_fatal() {
                true => return StatResult::Error(err),
                false => stats.push(Err(err)),
            },
        }
    }
    StatResult::Result(stats)
}
fn get_cap_sim_stats(
    core_fit: &mut rc::FitMut,
    options: &[StatOptionCapSim],
) -> StatResult<StatCapSim, StatFitShipAppliedError<!>, StatFitShipAppliedError<!>> {
    let mut stats = Vec::with_capacity(options.len());
    for option in options.iter() {
        match core_fit.get_stat_cap_sim(
            option.cap_perc,
            option.optional_reloads,
            &option.stagger,
            option.nosf_projectee_item_id.as_ref(),
        ) {
            Ok(stat) => stats.push(Ok(stat)),
            Err(err) => match err.is_fatal() {
                true => return StatResult::Error(err),
                false => stats.push(Err(err)),
            },
        }
    }
    StatResult::Result(stats)
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Ship sensors
////////////////////////////////////////////////////////////////////////////////////////////////////
fn get_incoming_jam_stats(
    core_fit: &mut rc::FitMut,
    options: &[StatOptionIncomingJam],
) -> StatResult<StatInJam, StatFitShipError<!>, !> {
    let mut stats = Vec::with_capacity(options.len());
    for option in options.iter() {
        match core_fit.get_stat_incoming_jam(option.time) {
            Ok(stat) => stats.push(Ok(stat)),
            Err(err) => return StatResult::Error(err),
        }
    }
    StatResult::Result(stats)
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Ship mobility
////////////////////////////////////////////////////////////////////////////////////////////////////
fn get_mass_stats(core_fit: &mut rc::FitMut, options: &[StatOptionMass]) -> StatResult<PValue, StatFitShipError<!>, !> {
    let mut stats = Vec::with_capacity(options.len());
    for option in options.iter() {
        match core_fit.get_stat_mass(option.affectors) {
            Ok(stat) => stats.push(Ok(stat)),
            Err(err) => return StatResult::Error(err),
        }
    }
    StatResult::Result(stats)
}
fn get_jump_stats(
    core_fit: &mut rc::FitMut,
    options: &[StatOptionJump],
) -> StatResult<StatJump, StatFitShipError<StatJumpError>, !> {
    let mut stats = Vec::with_capacity(options.len());
    for option in options.iter() {
        match core_fit.get_stat_jump(option.range, &option.passenger_fit_ids, option.passenger_fuel_affectors) {
            Ok(stat) => stats.push(Ok(stat)),
            Err(err) => return StatResult::Error(err),
        }
    }
    StatResult::Result(stats)
}
