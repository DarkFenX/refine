use rc::ItemMutCommon;

use crate::{
    PValue, Value,
    err::BrResolveError,
    stats::{
        ItemStats, StatBrFallibleError, StatCapSim, StatDmg, StatEhp, StatErps, StatInJam, StatJump, StatMining,
        StatOptionCapBlc, StatOptionCapSim, StatOptionEhp, StatOptionErps, StatOptionIncomingJam, StatOptionItemDmg,
        StatOptionItemMining, StatOptionItemOutCps, StatOptionItemOutNps, StatOptionItemOutRps, StatOptionJump,
        StatOptionMass, StatOptionRps, StatOutReps, StatResult, StatRps,
        err::{StatItemAppliedError, StatItemError, StatJumpError},
        exec_shared::{collect_stats_err_both_br, collect_stats_err_inner},
        item::ItemStatsOptionsResolved,
    },
};

impl ItemStatsOptionsResolved {
    pub(in crate::stats) fn execute(&self, core_item: &mut rc::ItemMut) -> ItemStats {
        let Self {
            dmg,
            mps,
            outgoing_nps,
            outgoing_rps,
            outgoing_cps,
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
        let mut stats = ItemStats { .. };
        // Output
        if let Some(options) = dmg {
            stats.dmg = get_dmg_stats(core_item, options);
        }
        if let Some(options) = mps {
            stats.mps = get_mps_stats(core_item, options);
        }
        if let Some(options) = outgoing_nps {
            stats.outgoing_nps = get_outgoing_nps_stats(core_item, options);
        }
        if let Some(options) = outgoing_cps {
            stats.outgoing_cps = get_outgoing_cps_stats(core_item, options);
        }
        if let Some(options) = outgoing_rps {
            stats.outgoing_rps = get_outgoing_rps_stats(core_item, options);
        }
        // Tank
        if *resists {
            stats.resists = StatResult::from_result_outer(core_item.get_stat_resists());
        }
        if *hp {
            stats.hp = StatResult::from_result_outer(core_item.get_stat_hp());
        }
        if let Some(options) = ehp {
            stats.ehp = get_ehp_stats(core_item, options);
        }
        if *wc_ehp {
            stats.wc_ehp = StatResult::from_result_outer(core_item.get_stat_wc_ehp());
        }
        if let Some(options) = rps {
            stats.rps = get_rps_stats(core_item, options);
        }
        if let Some(options) = erps {
            stats.erps = get_erps_stats(core_item, options);
        }
        if *breach_resist {
            stats.breach_resist = StatResult::from_result_outer(core_item.get_stat_breach_resist());
        }
        // Cap
        if *cap_amount {
            stats.cap_amount = StatResult::from_result_outer(core_item.get_stat_cap_amount());
        }
        if let Some(options) = cap_balance {
            stats.cap_balance = get_cap_balance_stats(core_item, options);
        }
        if let Some(options) = cap_sim {
            stats.cap_sim = get_cap_sim_stats(core_item, options);
        }
        if *neut_resist {
            stats.neut_resist = StatResult::from_result_outer(core_item.get_stat_neut_resist());
        }
        // Sensors
        if *locks {
            stats.locks = StatResult::from_result_outer(core_item.get_stat_locks());
        }
        if *lock_range {
            stats.lock_range = StatResult::from_result_outer(core_item.get_stat_lock_range());
        }
        if *scan_res {
            stats.scan_res = StatResult::from_result_outer(core_item.get_stat_scan_res());
        }
        if *sensors {
            stats.sensors = StatResult::from_result_outer(core_item.get_stat_sensors());
        }
        if *dscan_range {
            stats.dscan_range = StatResult::from_result_outer(core_item.get_stat_dscan_range());
        }
        if *probing_size {
            stats.probing_size = StatResult::from_result_outer(core_item.get_stat_probing_size());
        }
        if let Some(options) = incoming_jam {
            stats.incoming_jam = get_incoming_jam_stats(core_item, options);
        }
        // Mobility
        if *speed {
            stats.speed = StatResult::from_result_outer(core_item.get_stat_speed());
        }
        if *agility {
            stats.agility = StatResult::from_result_outer(core_item.get_stat_agility());
        }
        if *align_time {
            stats.align_time = StatResult::from_result_outer(core_item.get_stat_align_time());
        }
        if *sig_radius {
            stats.sig_radius = StatResult::from_result_outer(core_item.get_stat_sig_radius());
        }
        if let Some(options) = mass {
            stats.mass = get_mass_stats(core_item, options);
        }
        if *warp_speed {
            stats.warp_speed = StatResult::from_result_outer(core_item.get_stat_warp_speed());
        }
        if *max_warp_range {
            stats.max_warp_range = StatResult::from_result_outer(core_item.get_stat_max_warp_range());
        }
        if let Some(options) = jump {
            stats.jump = get_jump_stats(core_item, options);
        }
        // Misc
        if *drone_control_range {
            stats.drone_control_range = StatResult::from_result_outer(core_item.get_stat_drone_control_range());
        }
        if *can_warp {
            stats.can_warp = StatResult::from_result_outer(core_item.get_stat_can_warp());
        }
        if *can_jump_gate {
            stats.can_jump_gate = StatResult::from_result_outer(core_item.get_stat_can_jump_gate());
        }
        if *can_jump_wormhole {
            stats.can_jump_wormhole = StatResult::from_result_outer(core_item.get_stat_can_jump_wormhole());
        }
        if *can_jump_drive {
            stats.can_jump_drive = StatResult::from_result_outer(core_item.get_stat_can_jump_drive());
        }
        if *can_dock_station {
            stats.can_dock_station = StatResult::from_result_outer(core_item.get_stat_can_dock_station());
        }
        if *can_dock_citadel {
            stats.can_dock_citadel = StatResult::from_result_outer(core_item.get_stat_can_dock_citadel());
        }
        if *can_tether {
            stats.can_tether = StatResult::from_result_outer(core_item.get_stat_can_tether());
        }
        stats
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Output
////////////////////////////////////////////////////////////////////////////////////////////////////
fn get_dmg_stats(
    core_item: &mut rc::ItemMut,
    options: &[Result<StatOptionItemDmg, BrResolveError>],
) -> StatResult<StatDmg, StatItemAppliedError<!>, StatBrFallibleError<StatItemAppliedError<!>>> {
    collect_stats_err_both_br(options, |option| match option.projectee_item_id {
        Some(projectee_item_id) => core_item
            .get_stat_dmg_applied(
                option.time,
                option.crits,
                option.charges,
                option.state,
                &projectee_item_id,
            )
            .map(StatDmg::from_core_applied),
        None => core_item
            .get_stat_dmg(option.time, option.crits, option.charges, option.state)
            .map(StatDmg::from_core)
            .map_err(conv_err_item),
    })
}
fn get_mps_stats(
    core_item: &mut rc::ItemMut,
    options: &[StatOptionItemMining],
) -> StatResult<StatMining, StatItemError<!>, !> {
    collect_stats_err_inner(options, |option| {
        core_item.get_stat_mps(option.time, option.resource_kind, option.state)
    })
}
fn get_outgoing_nps_stats(
    core_item: &mut rc::ItemMut,
    options: &[Result<StatOptionItemOutNps, BrResolveError>],
) -> StatResult<PValue, StatItemAppliedError<!>, StatBrFallibleError<StatItemAppliedError<!>>> {
    collect_stats_err_both_br(options, |option| match option.projectee_item_id {
        Some(projectee_item_id) => {
            core_item.get_stat_outgoing_nps_applied(option.time, option.charges, option.state, &projectee_item_id)
        }
        None => core_item
            .get_stat_outgoing_nps(option.time, option.charges, option.state)
            .map_err(conv_err_item),
    })
}
fn get_outgoing_rps_stats(
    core_item: &mut rc::ItemMut,
    options: &[Result<StatOptionItemOutRps, BrResolveError>],
) -> StatResult<StatOutReps, StatItemAppliedError<!>, StatBrFallibleError<StatItemAppliedError<!>>> {
    collect_stats_err_both_br(options, |option| match option.projectee_item_id {
        Some(projectee_item_id) => {
            core_item.get_stat_outgoing_rps_applied(option.time, option.state, &projectee_item_id)
        }
        None => core_item
            .get_stat_outgoing_rps(option.time, option.state)
            .map_err(conv_err_item),
    })
}
fn get_outgoing_cps_stats(
    core_item: &mut rc::ItemMut,
    options: &[Result<StatOptionItemOutCps, BrResolveError>],
) -> StatResult<PValue, StatItemAppliedError<!>, StatBrFallibleError<StatItemAppliedError<!>>> {
    collect_stats_err_both_br(options, |option| match option.projectee_item_id {
        Some(projectee_item_id) => {
            core_item.get_stat_outgoing_cps_applied(option.time, option.state, &projectee_item_id)
        }
        None => core_item
            .get_stat_outgoing_cps(option.time, option.state)
            .map_err(conv_err_item),
    })
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Tank
////////////////////////////////////////////////////////////////////////////////////////////////////
fn get_ehp_stats(core_item: &mut rc::ItemMut, options: &[StatOptionEhp]) -> StatResult<StatEhp, StatItemError<!>, !> {
    collect_stats_err_inner(options, |option| core_item.get_stat_ehp(option.incoming_dps))
}
fn get_rps_stats(core_item: &mut rc::ItemMut, options: &[StatOptionRps]) -> StatResult<StatRps, StatItemError<!>, !> {
    collect_stats_err_inner(options, |option| {
        core_item.get_stat_rps(option.time, option.shield_perc)
    })
}
fn get_erps_stats(
    core_item: &mut rc::ItemMut,
    options: &[StatOptionErps],
) -> StatResult<StatErps, StatItemError<!>, !> {
    collect_stats_err_inner(options, |option| {
        core_item.get_stat_erps(option.incoming_dps, option.time, option.shield_perc)
    })
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Cap
////////////////////////////////////////////////////////////////////////////////////////////////////
fn get_cap_balance_stats(
    core_item: &mut rc::ItemMut,
    options: &[Result<StatOptionCapBlc, BrResolveError>],
) -> StatResult<Value, StatItemAppliedError<!>, StatBrFallibleError<StatItemAppliedError<!>>> {
    collect_stats_err_both_br(options, |option| {
        core_item.get_stat_cap_balance(option.src_kinds, option.time)
    })
}
fn get_cap_sim_stats(
    core_item: &mut rc::ItemMut,
    options: &[Result<StatOptionCapSim, BrResolveError>],
) -> StatResult<StatCapSim, StatItemAppliedError<!>, StatBrFallibleError<StatItemAppliedError<!>>> {
    collect_stats_err_both_br(options, |option| {
        core_item.get_stat_cap_sim(
            option.cap_perc,
            option.optional_reloads,
            &option.stagger,
            option.nosf_projectee_item_id.as_ref(),
        )
    })
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Sensors
////////////////////////////////////////////////////////////////////////////////////////////////////
fn get_incoming_jam_stats(
    core_item: &mut rc::ItemMut,
    options: &[StatOptionIncomingJam],
) -> StatResult<StatInJam, StatItemError<!>, !> {
    collect_stats_err_inner(options, |option| core_item.get_stat_incoming_jam(option.time))
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution getters - mobility
////////////////////////////////////////////////////////////////////////////////////////////////////
fn get_mass_stats(core_item: &mut rc::ItemMut, options: &[StatOptionMass]) -> StatResult<PValue, StatItemError<!>, !> {
    collect_stats_err_inner(options, |option| core_item.get_stat_mass(option.affectors))
}
fn get_jump_stats(
    core_item: &mut rc::ItemMut,
    options: &[StatOptionJump],
) -> StatResult<StatJump, StatItemError<StatJumpError>, !> {
    collect_stats_err_inner(options, |option| {
        core_item.get_stat_jump(option.range, &option.passenger_fit_ids, option.passenger_fuel_affectors)
    })
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Helpers
////////////////////////////////////////////////////////////////////////////////////////////////////
fn conv_err_item<SS>(err: StatItemError<SS>) -> StatItemAppliedError<SS>
where
    SS: std::error::Error,
{
    match err {
        StatItemError::ItemNotLoaded(err) => StatItemAppliedError::ItemNotLoaded(err),
        StatItemError::UnsupportedStat(err) => StatItemAppliedError::UnsupportedStat(err),
        StatItemError::StatSpecific(err) => StatItemAppliedError::StatSpecific(err),
    }
}
