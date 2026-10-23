use crate::{
    PValue, Value,
    err::BrResolveError,
    stats::{
        FitStats, StatBrFallibleError, StatCapSim, StatDmg, StatEhp, StatErps, StatInJam, StatJump, StatMining,
        StatOptionCapBlc, StatOptionCapSim, StatOptionEhp, StatOptionErps, StatOptionFitDmg, StatOptionFitMining,
        StatOptionFitOutCps, StatOptionFitOutNps, StatOptionFitOutRps, StatOptionIncomingJam, StatOptionJump,
        StatOptionMass, StatOptionRps, StatOutReps, StatResult, StatRps,
        err::{StatFitAppliedError, StatFitShipAppliedError, StatFitShipError, StatJumpError},
        exec_shared::{collect_stats_err_both_br, collect_stats_err_inner, collect_stats_err_outer_br},
        fit::FitStatsOptionsResolved,
    },
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl FitStatsOptionsResolved {
    pub(in crate::stats) fn execute(&self, core_fit: &mut rc::FitMut) -> FitStats {
        let Self {
            dmg,
            mps,
            outgoing_nps,
            outgoing_rps,
            outgoing_cps,
            cpu,
            powergrid,
            calibration,
            drone_bay_volume,
            drone_bandwidth,
            fighter_bay_volume,
            high_slots,
            mid_slots,
            low_slots,
            turret_slots,
            launcher_slots,
            rig_slots,
            service_slots,
            subsystem_slots,
            launched_drones,
            launched_fighters,
            launched_light_fighters,
            launched_heavy_fighters,
            launched_support_fighters,
            launched_st_light_fighters,
            launched_st_heavy_fighters,
            launched_st_support_fighters,
            resists,
            hp,
            ehp,
            wc_ehp,
            rps,
            erps,
            breach_resist,
            cap_amount,
            cap_balance,
            cap_sim,
            neut_resist,
            locks,
            lock_range,
            scan_res,
            sensors,
            dscan_range,
            probing_size,
            incoming_jam,
            speed,
            agility,
            align_time,
            sig_radius,
            mass,
            warp_speed,
            max_warp_range,
            jump,
            drone_control_range,
            can_warp,
            can_jump_gate,
            can_jump_wormhole,
            can_jump_drive,
            can_dock_station,
            can_dock_citadel,
            can_tether,
        } = self;
        let mut stats = FitStats { .. };
        // Fit output stats
        if let Some(options) = dmg {
            stats.dmg = get_dmg_stats(core_fit, options);
        }
        if let Some(options) = mps {
            stats.mps = get_mps_stats(core_fit, options);
        }
        if let Some(options) = outgoing_nps {
            stats.outgoing_nps = get_outgoing_nps_stats(core_fit, options);
        }
        if let Some(options) = outgoing_cps {
            stats.outgoing_cps = get_outgoing_cps_stats(core_fit, options);
        }
        if let Some(options) = outgoing_rps {
            stats.outgoing_rps = get_outgoing_rps_stats(core_fit, options);
        }
        // Fit resources
        if *cpu {
            stats.cpu = StatResult::from_stat(core_fit.get_stat_cpu());
        }
        if *powergrid {
            stats.powergrid = StatResult::from_stat(core_fit.get_stat_powergrid());
        }
        if *calibration {
            stats.calibration = StatResult::from_stat(core_fit.get_stat_calibration());
        }
        if *drone_bay_volume {
            stats.drone_bay_volume = StatResult::from_stat(core_fit.get_stat_drone_bay_volume());
        }
        if *drone_bandwidth {
            stats.drone_bandwidth = StatResult::from_stat(core_fit.get_stat_drone_bandwidth());
        }
        if *fighter_bay_volume {
            stats.fighter_bay_volume = StatResult::from_stat(core_fit.get_stat_fighter_bay_volume());
        }
        // Fit slots
        if *high_slots {
            stats.high_slots = StatResult::from_stat(core_fit.get_stat_high_slots());
        }
        if *mid_slots {
            stats.mid_slots = StatResult::from_stat(core_fit.get_stat_mid_slots());
        }
        if *low_slots {
            stats.low_slots = StatResult::from_stat(core_fit.get_stat_low_slots());
        }
        if *turret_slots {
            stats.turret_slots = StatResult::from_stat(core_fit.get_stat_turret_slots());
        }
        if *launcher_slots {
            stats.launcher_slots = StatResult::from_stat(core_fit.get_stat_launcher_slots());
        }
        if *rig_slots {
            stats.rig_slots = StatResult::from_stat(core_fit.get_stat_rig_slots());
        }
        if *service_slots {
            stats.service_slots = StatResult::from_stat(core_fit.get_stat_service_slots());
        }
        if *subsystem_slots {
            stats.subsystem_slots = StatResult::from_stat(core_fit.get_stat_subsystem_slots());
        }
        if *launched_drones {
            stats.launched_drones = StatResult::from_stat(core_fit.get_stat_launched_drones());
        }
        if *launched_fighters {
            stats.launched_fighters = StatResult::from_stat(core_fit.get_stat_launched_fighters());
        }
        if *launched_light_fighters {
            stats.launched_light_fighters = StatResult::from_stat(core_fit.get_stat_launched_light_fighters());
        }
        if *launched_heavy_fighters {
            stats.launched_heavy_fighters = StatResult::from_stat(core_fit.get_stat_launched_heavy_fighters());
        }
        if *launched_support_fighters {
            stats.launched_support_fighters = StatResult::from_stat(core_fit.get_stat_launched_support_fighters());
        }
        if *launched_st_light_fighters {
            stats.launched_st_light_fighters = StatResult::from_stat(core_fit.get_stat_launched_st_light_fighters());
        }
        if *launched_st_heavy_fighters {
            stats.launched_st_heavy_fighters = StatResult::from_stat(core_fit.get_stat_launched_st_heavy_fighters());
        }
        if *launched_st_support_fighters {
            stats.launched_st_support_fighters =
                StatResult::from_stat(core_fit.get_stat_launched_st_support_fighters());
        }
        // Ship tank
        if *resists {
            stats.resists = StatResult::from_result_outer(core_fit.get_stat_resists());
        }
        if *hp {
            stats.hp = StatResult::from_result_outer(core_fit.get_stat_hp());
        }
        if let Some(options) = ehp {
            stats.ehp = get_ehp_stats(core_fit, options);
        }
        if *wc_ehp {
            stats.wc_ehp = StatResult::from_result_outer(core_fit.get_stat_wc_ehp());
        }
        if let Some(options) = rps {
            stats.rps = get_rps_stats(core_fit, options);
        }
        if let Some(options) = erps {
            stats.erps = get_erps_stats(core_fit, options);
        }
        if *breach_resist {
            stats.breach_resist = StatResult::from_result_outer(core_fit.get_stat_breach_resist());
        }
        // Ship cap
        if *cap_amount {
            stats.cap_amount = StatResult::from_result_outer(core_fit.get_stat_cap_amount());
        }
        if let Some(options) = cap_balance {
            stats.cap_balance = get_cap_balance_stats(core_fit, options);
        }
        if let Some(options) = cap_sim {
            stats.cap_sim = get_cap_sim_stats(core_fit, options);
        }
        if *neut_resist {
            stats.neut_resist = StatResult::from_result_outer(core_fit.get_stat_neut_resist());
        }
        // Ship sensors
        if *locks {
            stats.locks = StatResult::from_result_outer(core_fit.get_stat_locks());
        }
        if *lock_range {
            stats.lock_range = StatResult::from_result_outer(core_fit.get_stat_lock_range());
        }
        if *scan_res {
            stats.scan_res = StatResult::from_result_outer(core_fit.get_stat_scan_res());
        }
        if *sensors {
            stats.sensors = StatResult::from_result_outer(core_fit.get_stat_sensors());
        }
        if *dscan_range {
            stats.dscan_range = StatResult::from_result_outer(core_fit.get_stat_dscan_range());
        }
        if *probing_size {
            stats.probing_size = StatResult::from_result_outer(core_fit.get_stat_probing_size());
        }
        if let Some(options) = incoming_jam {
            stats.incoming_jam = get_incoming_jam_stats(core_fit, options);
        }
        // Ship mobility
        if *speed {
            stats.speed = StatResult::from_result_outer(core_fit.get_stat_speed());
        }
        if *agility {
            stats.agility = StatResult::from_result_outer(core_fit.get_stat_agility());
        }
        if *align_time {
            stats.align_time = StatResult::from_result_outer(core_fit.get_stat_align_time());
        }
        if *sig_radius {
            stats.sig_radius = StatResult::from_result_outer(core_fit.get_stat_sig_radius());
        }
        if let Some(options) = mass {
            stats.mass = get_mass_stats(core_fit, options);
        }
        if *warp_speed {
            stats.warp_speed = StatResult::from_result_outer(core_fit.get_stat_warp_speed());
        }
        if *max_warp_range {
            stats.max_warp_range = StatResult::from_result_outer(core_fit.get_stat_max_warp_range());
        }
        if let Some(options) = jump {
            stats.jump = get_jump_stats(core_fit, options);
        }
        // Ship misc stats
        if *drone_control_range {
            stats.drone_control_range = StatResult::from_result_outer(core_fit.get_stat_drone_control_range());
        }
        if *can_warp {
            stats.can_warp = StatResult::from_result_outer(core_fit.get_stat_can_warp());
        }
        if *can_jump_gate {
            stats.can_jump_gate = StatResult::from_result_outer(core_fit.get_stat_can_jump_gate());
        }
        if *can_jump_wormhole {
            stats.can_jump_wormhole = StatResult::from_result_outer(core_fit.get_stat_can_jump_wormhole());
        }
        if *can_jump_drive {
            stats.can_jump_drive = StatResult::from_result_outer(core_fit.get_stat_can_jump_drive());
        }
        if *can_dock_station {
            stats.can_dock_station = StatResult::from_result_outer(core_fit.get_stat_can_dock_station());
        }
        if *can_dock_citadel {
            stats.can_dock_citadel = StatResult::from_result_outer(core_fit.get_stat_can_dock_citadel());
        }
        if *can_tether {
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
    options: &[Result<StatOptionFitDmg, BrResolveError>],
) -> StatResult<StatDmg, !, StatBrFallibleError<StatFitAppliedError>> {
    collect_stats_err_outer_br(options, |option| match option.projectee_item_id {
        Some(projectee_item_id) => core_fit
            .get_stat_dmg_applied(option.item_kinds, option.time, option.crits, &projectee_item_id)
            .map(StatDmg::from_core_applied),
        None => Ok(StatDmg::from_core(core_fit.get_stat_dmg(
            option.item_kinds,
            option.time,
            option.crits,
        ))),
    })
}
fn get_mps_stats(core_fit: &mut rc::FitMut, options: &[StatOptionFitMining]) -> StatResult<StatMining, !, !> {
    collect_stats_err_inner(options, |option| {
        Ok(core_fit.get_stat_mps(option.item_kinds, option.time, option.resource_kind))
    })
}
fn get_outgoing_nps_stats(
    core_fit: &mut rc::FitMut,
    options: &[Result<StatOptionFitOutNps, BrResolveError>],
) -> StatResult<PValue, !, StatBrFallibleError<StatFitAppliedError>> {
    collect_stats_err_outer_br(options, |option| match option.projectee_item_id {
        Some(projectee_item_id) => {
            core_fit.get_stat_outgoing_nps_applied(option.item_kinds, option.time, &projectee_item_id)
        }
        None => Ok(core_fit.get_stat_outgoing_nps(option.item_kinds, option.time)),
    })
}
fn get_outgoing_rps_stats(
    core_fit: &mut rc::FitMut,
    options: &[Result<StatOptionFitOutRps, BrResolveError>],
) -> StatResult<StatOutReps, !, StatBrFallibleError<StatFitAppliedError>> {
    collect_stats_err_outer_br(options, |option| match option.projectee_item_id {
        Some(projectee_item_id) => {
            core_fit.get_stat_outgoing_rps_applied(option.item_kinds, option.time, &projectee_item_id)
        }
        None => Ok(core_fit.get_stat_outgoing_rps(option.item_kinds, option.time)),
    })
}
fn get_outgoing_cps_stats(
    core_fit: &mut rc::FitMut,
    options: &[Result<StatOptionFitOutCps, BrResolveError>],
) -> StatResult<PValue, !, StatBrFallibleError<StatFitAppliedError>> {
    collect_stats_err_outer_br(options, |option| match option.projectee_item_id {
        Some(projectee_item_id) => core_fit.get_stat_outgoing_cps_applied(option.time, &projectee_item_id),
        None => Ok(core_fit.get_stat_outgoing_cps(option.time)),
    })
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Ship tank
////////////////////////////////////////////////////////////////////////////////////////////////////
fn get_ehp_stats(core_fit: &mut rc::FitMut, options: &[StatOptionEhp]) -> StatResult<StatEhp, StatFitShipError<!>, !> {
    collect_stats_err_inner(options, |option| core_fit.get_stat_ehp(option.incoming_dps))
}
fn get_rps_stats(core_fit: &mut rc::FitMut, options: &[StatOptionRps]) -> StatResult<StatRps, StatFitShipError<!>, !> {
    collect_stats_err_inner(options, |option| core_fit.get_stat_rps(option.time, option.shield_perc))
}
fn get_erps_stats(
    core_fit: &mut rc::FitMut,
    options: &[StatOptionErps],
) -> StatResult<StatErps, StatFitShipError<!>, !> {
    collect_stats_err_inner(options, |option| {
        core_fit.get_stat_erps(option.incoming_dps, option.time, option.shield_perc)
    })
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Ship cap
////////////////////////////////////////////////////////////////////////////////////////////////////
fn get_cap_balance_stats(
    core_fit: &mut rc::FitMut,
    options: &[Result<StatOptionCapBlc, BrResolveError>],
) -> StatResult<Value, StatFitShipAppliedError<!>, StatBrFallibleError<StatFitShipAppliedError<!>>> {
    collect_stats_err_both_br(options, |option| {
        core_fit.get_stat_cap_balance(option.src_kinds, option.time)
    })
}
fn get_cap_sim_stats(
    core_fit: &mut rc::FitMut,
    options: &[Result<StatOptionCapSim, BrResolveError>],
) -> StatResult<StatCapSim, StatFitShipAppliedError<!>, StatBrFallibleError<StatFitShipAppliedError<!>>> {
    collect_stats_err_both_br(options, |option| {
        core_fit.get_stat_cap_sim(
            option.cap_perc,
            option.optional_reloads,
            &option.stagger,
            option.nosf_projectee_item_id.as_ref(),
        )
    })
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Ship sensors
////////////////////////////////////////////////////////////////////////////////////////////////////
fn get_incoming_jam_stats(
    core_fit: &mut rc::FitMut,
    options: &[StatOptionIncomingJam],
) -> StatResult<StatInJam, StatFitShipError<!>, !> {
    collect_stats_err_inner(options, |option| core_fit.get_stat_incoming_jam(option.time))
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Ship mobility
////////////////////////////////////////////////////////////////////////////////////////////////////
fn get_mass_stats(core_fit: &mut rc::FitMut, options: &[StatOptionMass]) -> StatResult<PValue, StatFitShipError<!>, !> {
    collect_stats_err_inner(options, |option| core_fit.get_stat_mass(option.affectors))
}
fn get_jump_stats(
    core_fit: &mut rc::FitMut,
    options: &[StatOptionJump],
) -> StatResult<StatJump, StatFitShipError<StatJumpError>, !> {
    collect_stats_err_inner(options, |option| {
        core_fit.get_stat_jump(option.range, &option.passenger_fit_ids, option.passenger_fuel_affectors)
    })
}
