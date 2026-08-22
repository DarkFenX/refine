use rc::ItemMutCommon;

use crate::{
    PValue, Value,
    stats::{
        ItemStats, StatCapSim, StatDmg, StatEhp, StatErps, StatInJam, StatJump, StatMining, StatOptionCapBlc,
        StatOptionCapSim, StatOptionEhp, StatOptionErps, StatOptionIncomingJam, StatOptionItemDmg,
        StatOptionItemMining, StatOptionItemOutCps, StatOptionItemOutNps, StatOptionItemOutRps, StatOptionJump,
        StatOptionMass, StatOptionRps, StatOutReps, StatResult, StatRps,
        err::{StatItemAppliedError, StatItemError, StatJumpError},
        fatal::StatErrorFatality,
        item::ItemStatsOptionsResolved,
    },
};

impl ItemStatsOptionsResolved {
    pub(in crate::stats) fn execute(&self, core_item: &mut rc::ItemMut) -> ItemStats {
        let mut stats = ItemStats { .. };
        // Output
        if !self.dmg.is_empty() {
            stats.dmg = get_dmg_stats(core_item, &self.dmg);
        }
        if !self.mps.is_empty() {
            stats.mps = get_mps_stats(core_item, &self.mps);
        }
        if !self.outgoing_nps.is_empty() {
            stats.outgoing_nps = get_outgoing_nps_stats(core_item, &self.outgoing_nps);
        }
        if !self.outgoing_cps.is_empty() {
            stats.outgoing_cps = get_outgoing_cps_stats(core_item, &self.outgoing_cps);
        }
        if !self.outgoing_rps.is_empty() {
            stats.outgoing_rps = get_outgoing_rps_stats(core_item, &self.outgoing_rps);
        }
        // Tank
        if self.resists {
            stats.resists = StatResult::from_result_outer(core_item.get_stat_resists());
        }
        if self.hp {
            stats.hp = StatResult::from_result_outer(core_item.get_stat_hp());
        }
        if !self.ehp.is_empty() {
            stats.ehp = get_ehp_stats(core_item, &self.ehp);
        }
        if self.wc_ehp {
            stats.wc_ehp = StatResult::from_result_outer(core_item.get_stat_wc_ehp());
        }
        if !self.rps.is_empty() {
            stats.rps = get_rps_stats(core_item, &self.rps);
        }
        if !self.erps.is_empty() {
            stats.erps = get_erps_stats(core_item, &self.erps);
        }
        if self.breach_resist {
            stats.breach_resist = StatResult::from_result_outer(core_item.get_stat_breach_resist());
        }
        // Cap
        if self.cap_amount {
            stats.cap_amount = StatResult::from_result_outer(core_item.get_stat_cap_amount());
        }
        if !self.cap_balance.is_empty() {
            stats.cap_balance = get_cap_balance_stats(core_item, &self.cap_balance);
        }
        if !self.cap_sim.is_empty() {
            stats.cap_sim = get_cap_sim_stats(core_item, &self.cap_sim);
        }
        if self.neut_resist {
            stats.neut_resist = StatResult::from_result_outer(core_item.get_stat_neut_resist());
        }
        // Sensors
        if self.locks {
            stats.locks = StatResult::from_result_outer(core_item.get_stat_locks());
        }
        if self.lock_range {
            stats.lock_range = StatResult::from_result_outer(core_item.get_stat_lock_range());
        }
        if self.scan_res {
            stats.scan_res = StatResult::from_result_outer(core_item.get_stat_scan_res());
        }
        if self.sensors {
            stats.sensors = StatResult::from_result_outer(core_item.get_stat_sensors());
        }
        if self.dscan_range {
            stats.dscan_range = StatResult::from_result_outer(core_item.get_stat_dscan_range());
        }
        if self.probing_size {
            stats.probing_size = StatResult::from_result_outer(core_item.get_stat_probing_size());
        }
        if !self.incoming_jam.is_empty() {
            stats.incoming_jam = get_incoming_jam_stats(core_item, &self.incoming_jam);
        }
        // Mobility
        if self.speed {
            stats.speed = StatResult::from_result_outer(core_item.get_stat_speed());
        }
        if self.agility {
            stats.agility = StatResult::from_result_outer(core_item.get_stat_agility());
        }
        if self.align_time {
            stats.align_time = StatResult::from_result_outer(core_item.get_stat_align_time());
        }
        if self.sig_radius {
            stats.sig_radius = StatResult::from_result_outer(core_item.get_stat_sig_radius());
        }
        if !self.mass.is_empty() {
            stats.mass = get_mass_stats(core_item, &self.mass);
        }
        if self.warp_speed {
            stats.warp_speed = StatResult::from_result_outer(core_item.get_stat_warp_speed());
        }
        if self.max_warp_range {
            stats.max_warp_range = StatResult::from_result_outer(core_item.get_stat_max_warp_range());
        }
        if !self.jump.is_empty() {
            stats.jump = get_jump_stats(core_item, &self.jump);
        }
        // Misc
        if self.drone_control_range {
            stats.drone_control_range = StatResult::from_result_outer(core_item.get_stat_drone_control_range());
        }
        if self.can_warp {
            stats.can_warp = StatResult::from_result_outer(core_item.get_stat_can_warp());
        }
        if self.can_jump_gate {
            stats.can_jump_gate = StatResult::from_result_outer(core_item.get_stat_can_jump_gate());
        }
        if self.can_jump_wormhole {
            stats.can_jump_wormhole = StatResult::from_result_outer(core_item.get_stat_can_jump_wormhole());
        }
        if self.can_jump_drive {
            stats.can_jump_drive = StatResult::from_result_outer(core_item.get_stat_can_jump_drive());
        }
        if self.can_dock_station {
            stats.can_dock_station = StatResult::from_result_outer(core_item.get_stat_can_dock_station());
        }
        if self.can_dock_citadel {
            stats.can_dock_citadel = StatResult::from_result_outer(core_item.get_stat_can_dock_citadel());
        }
        if self.can_tether {
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
    options: &[StatOptionItemDmg],
) -> StatResult<StatDmg, StatItemAppliedError<!>, StatItemAppliedError<!>> {
    let mut stats = Vec::with_capacity(options.len());
    for option in options.iter() {
        match option.projectee_item_id {
            Some(projectee_item_id) => {
                match core_item.get_stat_dmg_applied(
                    option.time,
                    option.crits,
                    option.charges,
                    option.state,
                    &projectee_item_id,
                ) {
                    Ok(stat) => stats.push(Ok(StatDmg::from_core_applied(stat))),
                    Err(err) => match err.is_fatal() {
                        true => return StatResult::Error(err),
                        false => stats.push(Err(err)),
                    },
                };
            }
            None => {
                match core_item.get_stat_dmg(option.time, option.crits, option.charges, option.state) {
                    Ok(stat) => stats.push(Ok(StatDmg::from_core(stat))),
                    Err(err) => {
                        let err = conv_err_item(err);
                        match err.is_fatal() {
                            true => return StatResult::Error(err),
                            false => stats.push(Err(err)),
                        }
                    }
                };
            }
        }
    }
    StatResult::Result(stats)
}
fn get_mps_stats(
    core_item: &mut rc::ItemMut,
    options: &[StatOptionItemMining],
) -> StatResult<StatMining, StatItemError<!>, !> {
    let mut stats = Vec::with_capacity(options.len());
    for option in options.iter() {
        match core_item.get_stat_mps(option.time, option.resource_kind, option.state) {
            Ok(stat) => stats.push(Ok(stat)),
            Err(err) => return StatResult::Error(err),
        }
    }
    StatResult::Result(stats)
}
fn get_outgoing_nps_stats(
    core_item: &mut rc::ItemMut,
    options: &[StatOptionItemOutNps],
) -> StatResult<PValue, StatItemAppliedError<!>, StatItemAppliedError<!>> {
    let mut stats = Vec::with_capacity(options.len());
    for option in options.iter() {
        match option.projectee_item_id {
            Some(projectee_item_id) => {
                match core_item.get_stat_outgoing_nps_applied(
                    option.time,
                    option.charges,
                    option.state,
                    &projectee_item_id,
                ) {
                    Ok(stat) => stats.push(Ok(stat)),
                    Err(err) => match err.is_fatal() {
                        true => return StatResult::Error(err),
                        false => stats.push(Err(err)),
                    },
                }
            }
            None => match core_item.get_stat_outgoing_nps(option.time, option.charges, option.state) {
                Ok(stat) => stats.push(Ok(stat)),
                Err(err) => {
                    let err = conv_err_item(err);
                    match err.is_fatal() {
                        true => return StatResult::Error(err),
                        false => stats.push(Err(err)),
                    }
                }
            },
        }
    }
    StatResult::Result(stats)
}
fn get_outgoing_rps_stats(
    core_item: &mut rc::ItemMut,
    options: &[StatOptionItemOutRps],
) -> StatResult<StatOutReps, StatItemAppliedError<!>, StatItemAppliedError<!>> {
    let mut stats = Vec::with_capacity(options.len());
    for option in options.iter() {
        match option.projectee_item_id {
            Some(projectee_item_id) => {
                match core_item.get_stat_outgoing_rps_applied(option.time, option.state, &projectee_item_id) {
                    Ok(stat) => stats.push(Ok(stat)),
                    Err(err) => match err.is_fatal() {
                        true => return StatResult::Error(err),
                        false => stats.push(Err(err)),
                    },
                }
            }
            None => match core_item.get_stat_outgoing_rps(option.time, option.state) {
                Ok(stat) => stats.push(Ok(stat)),
                Err(err) => {
                    let err = conv_err_item(err);
                    match err.is_fatal() {
                        true => return StatResult::Error(err),
                        false => stats.push(Err(err)),
                    }
                }
            },
        }
    }
    StatResult::Result(stats)
}
fn get_outgoing_cps_stats(
    core_item: &mut rc::ItemMut,
    options: &[StatOptionItemOutCps],
) -> StatResult<PValue, StatItemAppliedError<!>, StatItemAppliedError<!>> {
    let mut stats = Vec::with_capacity(options.len());
    for option in options.iter() {
        match option.projectee_item_id {
            Some(projectee_item_id) => {
                match core_item.get_stat_outgoing_cps_applied(option.time, option.state, &projectee_item_id) {
                    Ok(stat) => stats.push(Ok(stat)),
                    Err(err) => match err.is_fatal() {
                        true => return StatResult::Error(err),
                        false => stats.push(Err(err)),
                    },
                }
            }
            None => match core_item.get_stat_outgoing_cps(option.time, option.state) {
                Ok(stat) => stats.push(Ok(stat)),
                Err(err) => {
                    let err = conv_err_item(err);
                    match err.is_fatal() {
                        true => return StatResult::Error(err),
                        false => stats.push(Err(err)),
                    }
                }
            },
        }
    }
    StatResult::Result(stats)
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Tank
////////////////////////////////////////////////////////////////////////////////////////////////////
fn get_ehp_stats(core_item: &mut rc::ItemMut, options: &[StatOptionEhp]) -> StatResult<StatEhp, StatItemError<!>, !> {
    let mut stats = Vec::with_capacity(options.len());
    for option in options.iter() {
        match core_item.get_stat_ehp(option.incoming_dps) {
            Ok(stat) => stats.push(Ok(stat)),
            Err(err) => return StatResult::Error(err),
        }
    }
    StatResult::Result(stats)
}
fn get_rps_stats(core_item: &mut rc::ItemMut, options: &[StatOptionRps]) -> StatResult<StatRps, StatItemError<!>, !> {
    let mut stats = Vec::with_capacity(options.len());
    for option in options.iter() {
        match core_item.get_stat_rps(option.time, option.shield_perc) {
            Ok(stat) => stats.push(Ok(stat)),
            Err(err) => return StatResult::Error(err),
        }
    }
    StatResult::Result(stats)
}
fn get_erps_stats(
    core_item: &mut rc::ItemMut,
    options: &[StatOptionErps],
) -> StatResult<StatErps, StatItemError<!>, !> {
    let mut stats = Vec::with_capacity(options.len());
    for option in options.iter() {
        match core_item.get_stat_erps(option.incoming_dps, option.time, option.shield_perc) {
            Ok(stat) => stats.push(Ok(stat)),
            Err(err) => return StatResult::Error(err),
        }
    }
    StatResult::Result(stats)
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Cap
////////////////////////////////////////////////////////////////////////////////////////////////////
fn get_cap_balance_stats(
    core_item: &mut rc::ItemMut,
    options: &[StatOptionCapBlc],
) -> StatResult<Value, StatItemAppliedError<!>, StatItemAppliedError<!>> {
    let mut stats = Vec::with_capacity(options.len());
    for option in options.iter() {
        match core_item.get_stat_cap_balance(option.src_kinds, option.time) {
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
    core_item: &mut rc::ItemMut,
    options: &[StatOptionCapSim],
) -> StatResult<StatCapSim, StatItemAppliedError<!>, StatItemAppliedError<!>> {
    let mut stats = Vec::with_capacity(options.len());
    for option in options.iter() {
        match core_item.get_stat_cap_sim(
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
// Sensors
////////////////////////////////////////////////////////////////////////////////////////////////////
fn get_incoming_jam_stats(
    core_item: &mut rc::ItemMut,
    options: &[StatOptionIncomingJam],
) -> StatResult<StatInJam, StatItemError<!>, !> {
    let mut stats = Vec::with_capacity(options.len());
    for option in options.iter() {
        match core_item.get_stat_incoming_jam(option.time) {
            Ok(stat) => stats.push(Ok(stat)),
            Err(err) => return StatResult::Error(err),
        }
    }
    StatResult::Result(stats)
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution getters - mobility
////////////////////////////////////////////////////////////////////////////////////////////////////
fn get_mass_stats(core_item: &mut rc::ItemMut, options: &[StatOptionMass]) -> StatResult<PValue, StatItemError<!>, !> {
    let mut stats = Vec::with_capacity(options.len());
    for option in options.iter() {
        match core_item.get_stat_mass(option.affectors) {
            Ok(stat) => stats.push(Ok(stat)),
            Err(err) => return StatResult::Error(err),
        }
    }
    StatResult::Result(stats)
}
fn get_jump_stats(
    core_item: &mut rc::ItemMut,
    options: &[StatOptionJump],
) -> StatResult<StatJump, StatItemError<StatJumpError>, !> {
    let mut stats = Vec::with_capacity(options.len());
    for option in options.iter() {
        match core_item.get_stat_jump(option.range, &option.passenger_fit_ids, option.passenger_fuel_affectors) {
            Ok(stat) => stats.push(Ok(stat)),
            Err(err) => return StatResult::Error(err),
        }
    }
    StatResult::Result(stats)
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
