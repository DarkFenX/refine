use crate::{
    PValue, Value,
    stats::{
        FitStats, StatCapSim, StatDmg, StatEhp, StatErps, StatInJam, StatJump, StatMining, StatOptionCapBlc,
        StatOptionCapSim, StatOptionEhp, StatOptionErps, StatOptionFitDmg, StatOptionFitMining, StatOptionFitOutCps,
        StatOptionFitOutNps, StatOptionFitOutRps, StatOptionIncomingJam, StatOptionJump, StatOptionMass, StatOptionRps,
        StatOutReps, StatResult, StatRps,
        err::{StatFitAppliedError, StatFitShipAppliedError, StatFitShipError, StatJumpError},
        fatal::StatErrorFatality,
        fit::FitStatsOptions,
        option_support::StatOptionResolved,
    },
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl FitStatsOptions<StatOptionResolved> {
    pub(in crate::stats) fn execute(&self, core_fit: &mut rc::FitMut) -> FitStats {
        let mut stats = FitStats { .. };
        // Fit output stats
        if let Some(options) = self.dmg.as_ref() {
            stats.dmg = get_dmg_stats(core_fit, options);
        }
        if let Some(options) = self.mps.as_ref() {
            stats.mps = get_mps_stats(core_fit, options);
        }
        if let Some(options) = self.outgoing_nps.as_ref() {
            stats.outgoing_nps = get_outgoing_nps_stats(core_fit, options);
        }
        if let Some(options) = self.outgoing_cps.as_ref() {
            stats.outgoing_cps = get_outgoing_cps_stats(core_fit, options);
        }
        if let Some(options) = self.outgoing_rps.as_ref() {
            stats.outgoing_rps = get_outgoing_rps_stats(core_fit, options);
        }
        // Fit resources
        if self.cpu {
            stats.cpu = StatResult::from_stat(core_fit.get_stat_cpu());
        }
        if self.powergrid {
            stats.powergrid = StatResult::from_stat(core_fit.get_stat_powergrid());
        }
        if self.calibration {
            stats.calibration = StatResult::from_stat(core_fit.get_stat_calibration());
        }
        if self.drone_bay_volume {
            stats.drone_bay_volume = StatResult::from_stat(core_fit.get_stat_drone_bay_volume());
        }
        if self.drone_bandwidth {
            stats.drone_bandwidth = StatResult::from_stat(core_fit.get_stat_drone_bandwidth());
        }
        if self.fighter_bay_volume {
            stats.fighter_bay_volume = StatResult::from_stat(core_fit.get_stat_fighter_bay_volume());
        }
        // Fit slots
        if self.high_slots {
            stats.high_slots = StatResult::from_stat(core_fit.get_stat_high_slots());
        }
        if self.mid_slots {
            stats.mid_slots = StatResult::from_stat(core_fit.get_stat_mid_slots());
        }
        if self.low_slots {
            stats.low_slots = StatResult::from_stat(core_fit.get_stat_low_slots());
        }
        if self.turret_slots {
            stats.turret_slots = StatResult::from_stat(core_fit.get_stat_turret_slots());
        }
        if self.launcher_slots {
            stats.launcher_slots = StatResult::from_stat(core_fit.get_stat_launcher_slots());
        }
        if self.rig_slots {
            stats.rig_slots = StatResult::from_stat(core_fit.get_stat_rig_slots());
        }
        if self.service_slots {
            stats.service_slots = StatResult::from_stat(core_fit.get_stat_service_slots());
        }
        if self.subsystem_slots {
            stats.subsystem_slots = StatResult::from_stat(core_fit.get_stat_subsystem_slots());
        }
        if self.launched_drones {
            stats.launched_drones = StatResult::from_stat(core_fit.get_stat_launched_drones());
        }
        if self.launched_fighters {
            stats.launched_fighters = StatResult::from_stat(core_fit.get_stat_launched_fighters());
        }
        if self.launched_light_fighters {
            stats.launched_light_fighters = StatResult::from_stat(core_fit.get_stat_launched_light_fighters());
        }
        if self.launched_heavy_fighters {
            stats.launched_heavy_fighters = StatResult::from_stat(core_fit.get_stat_launched_heavy_fighters());
        }
        if self.launched_support_fighters {
            stats.launched_support_fighters = StatResult::from_stat(core_fit.get_stat_launched_support_fighters());
        }
        if self.launched_st_light_fighters {
            stats.launched_st_light_fighters = StatResult::from_stat(core_fit.get_stat_launched_st_light_fighters());
        }
        if self.launched_st_heavy_fighters {
            stats.launched_st_heavy_fighters = StatResult::from_stat(core_fit.get_stat_launched_st_heavy_fighters());
        }
        if self.launched_st_support_fighters {
            stats.launched_st_support_fighters =
                StatResult::from_stat(core_fit.get_stat_launched_st_support_fighters());
        }
        // Ship tank
        if self.resists {
            stats.resists = StatResult::from_result_outer(core_fit.get_stat_resists());
        }
        if self.hp {
            stats.hp = StatResult::from_result_outer(core_fit.get_stat_hp());
        }
        if let Some(options) = self.ehp.as_ref() {
            stats.ehp = get_ehp_stats(core_fit, options);
        }
        if self.wc_ehp {
            stats.wc_ehp = StatResult::from_result_outer(core_fit.get_stat_wc_ehp());
        }
        if let Some(options) = self.rps.as_ref() {
            stats.rps = get_rps_stats(core_fit, options);
        }
        if let Some(options) = self.erps.as_ref() {
            stats.erps = get_erps_stats(core_fit, options);
        }
        if self.breach_resist {
            stats.breach_resist = StatResult::from_result_outer(core_fit.get_stat_breach_resist());
        }
        // Ship cap
        if self.cap_amount {
            stats.cap_amount = StatResult::from_result_outer(core_fit.get_stat_cap_amount());
        }
        if let Some(options) = self.cap_balance.as_ref() {
            stats.cap_balance = get_cap_balance_stats(core_fit, options);
        }
        if let Some(options) = self.cap_sim.as_ref() {
            stats.cap_sim = get_cap_sim_stats(core_fit, options);
        }
        if self.neut_resist {
            stats.neut_resist = StatResult::from_result_outer(core_fit.get_stat_neut_resist());
        }
        // Ship sensors
        if self.locks {
            stats.locks = StatResult::from_result_outer(core_fit.get_stat_locks());
        }
        if self.lock_range {
            stats.lock_range = StatResult::from_result_outer(core_fit.get_stat_lock_range());
        }
        if self.scan_res {
            stats.scan_res = StatResult::from_result_outer(core_fit.get_stat_scan_res());
        }
        if self.sensors {
            stats.sensors = StatResult::from_result_outer(core_fit.get_stat_sensors());
        }
        if self.dscan_range {
            stats.dscan_range = StatResult::from_result_outer(core_fit.get_stat_dscan_range());
        }
        if self.probing_size {
            stats.probing_size = StatResult::from_result_outer(core_fit.get_stat_probing_size());
        }
        if let Some(options) = self.incoming_jam.as_ref() {
            stats.incoming_jam = get_incoming_jam_stats(core_fit, options);
        }
        // Ship mobility
        if self.speed {
            stats.speed = StatResult::from_result_outer(core_fit.get_stat_speed());
        }
        if self.agility {
            stats.agility = StatResult::from_result_outer(core_fit.get_stat_agility());
        }
        if self.align_time {
            stats.align_time = StatResult::from_result_outer(core_fit.get_stat_align_time());
        }
        if self.sig_radius {
            stats.sig_radius = StatResult::from_result_outer(core_fit.get_stat_sig_radius());
        }
        if let Some(options) = self.mass.as_ref() {
            stats.mass = get_mass_stats(core_fit, options);
        }
        if self.warp_speed {
            stats.warp_speed = StatResult::from_result_outer(core_fit.get_stat_warp_speed());
        }
        if self.max_warp_range {
            stats.max_warp_range = StatResult::from_result_outer(core_fit.get_stat_max_warp_range());
        }
        if let Some(options) = self.jump.as_ref() {
            stats.jump = get_jump_stats(core_fit, options);
        }
        // Ship misc stats
        if self.drone_control_range {
            stats.drone_control_range = StatResult::from_result_outer(core_fit.get_stat_drone_control_range());
        }
        if self.can_warp {
            stats.can_warp = StatResult::from_result_outer(core_fit.get_stat_can_warp());
        }
        if self.can_jump_gate {
            stats.can_jump_gate = StatResult::from_result_outer(core_fit.get_stat_can_jump_gate());
        }
        if self.can_jump_wormhole {
            stats.can_jump_wormhole = StatResult::from_result_outer(core_fit.get_stat_can_jump_wormhole());
        }
        if self.can_jump_drive {
            stats.can_jump_drive = StatResult::from_result_outer(core_fit.get_stat_can_jump_drive());
        }
        if self.can_dock_station {
            stats.can_dock_station = StatResult::from_result_outer(core_fit.get_stat_can_dock_station());
        }
        if self.can_dock_citadel {
            stats.can_dock_citadel = StatResult::from_result_outer(core_fit.get_stat_can_dock_citadel());
        }
        if self.can_tether {
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
        match core_fit.get_stat_cap_balance(&option.src_kinds, option.time) {
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
            // Core takes it by value, and borrowed options are the only thing available here
            option.stagger.clone(),
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
