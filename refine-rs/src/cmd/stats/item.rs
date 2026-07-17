use rc::ItemMutCommon;

use crate::{
    PValue, Value,
    stats::{
        ItemStats, StatCapSim, StatDmg, StatEhp, StatErps, StatInJam, StatJump, StatMining, StatOption,
        StatOptionCapBlc, StatOptionCapSim, StatOptionEhp, StatOptionErps, StatOptionExt, StatOptionIncomingJam,
        StatOptionItemDmg, StatOptionItemMining, StatOptionItemOutCps, StatOptionItemOutNps, StatOptionItemOutRps,
        StatOptionJump, StatOptionMass, StatOptionRps, StatOutReps, StatRps,
    },
};

#[derive(Default)]
pub struct GetItemStatsCmd {
    pub default: bool = true,
    // Output
    pub dmg: StatOptionExt<StatOptionItemDmg> = StatOptionExt::Default,
    pub mps: StatOptionExt<StatOptionItemMining> = StatOptionExt::Default,
    pub outgoing_nps: StatOptionExt<StatOptionItemOutNps> = StatOptionExt::Default,
    pub outgoing_rps: StatOptionExt<StatOptionItemOutRps> = StatOptionExt::Default,
    pub outgoing_cps: StatOptionExt<StatOptionItemOutCps> = StatOptionExt::Default,
    // Tank
    pub resists: StatOption = StatOption::Default,
    pub hp: StatOption = StatOption::Default,
    pub ehp: StatOptionExt<StatOptionEhp> = StatOptionExt::Default,
    pub wc_ehp: StatOption = StatOption::Default,
    pub rps: StatOptionExt<StatOptionRps> = StatOptionExt::Default,
    pub erps: StatOptionExt<StatOptionErps> = StatOptionExt::Default,
    pub breach_resist: StatOption = StatOption::Default,
    // Cap
    pub cap_amount: StatOption = StatOption::Default,
    pub cap_balance: StatOptionExt<StatOptionCapBlc> = StatOptionExt::Default,
    pub cap_sim: StatOptionExt<StatOptionCapSim> = StatOptionExt::Default,
    pub neut_resist: StatOption = StatOption::Default,
    // Sensors
    pub locks: StatOption = StatOption::Default,
    pub lock_range: StatOption = StatOption::Default,
    pub scan_res: StatOption = StatOption::Default,
    pub sensors: StatOption = StatOption::Default,
    pub dscan_range: StatOption = StatOption::Default,
    pub probing_size: StatOption = StatOption::Default,
    pub incoming_jam: StatOptionExt<StatOptionIncomingJam> = StatOptionExt::Default,
    // Mobility
    pub speed: StatOption = StatOption::Default,
    pub agility: StatOption = StatOption::Default,
    pub align_time: StatOption = StatOption::Default,
    pub sig_radius: StatOption = StatOption::Default,
    pub mass: StatOptionExt<StatOptionMass> = StatOptionExt::Default,
    pub warp_speed: StatOption = StatOption::Default,
    pub max_warp_range: StatOption = StatOption::Default,
    pub jump: StatOptionExt<StatOptionJump> = StatOptionExt::Default,
    // Misc
    pub drone_control_range: StatOption = StatOption::Default,
    pub can_warp: StatOption = StatOption::Default,
    pub can_jump_gate: StatOption = StatOption::Default,
    pub can_jump_wormhole: StatOption = StatOption::Default,
    pub can_jump_drive: StatOption = StatOption::Default,
    pub can_dock_station: StatOption = StatOption::Default,
    pub can_dock_citadel: StatOption = StatOption::Default,
    pub can_tether: StatOption = StatOption::Default,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl GetItemStatsCmd {
    pub(crate) fn execute(self, core_item: &mut rc::ItemMut) -> ItemStats {
        let mut stats = ItemStats { .. };
        ////////////////////////////////////////////////////////////////////////////////////////////
        // Output
        ////////////////////////////////////////////////////////////////////////////////////////////
        if let Some(options) = self.dmg.into_enabled(self.default) {
            stats.dmg = get_dmg_stats(core_item, options).into();
        }
        if let Some(options) = self.mps.into_enabled(self.default) {
            stats.mps = get_mps_stats(core_item, options).into();
        }
        if let Some(options) = self.outgoing_nps.into_enabled(self.default) {
            stats.outgoing_nps = get_outgoing_nps_stats(core_item, options).into();
        }
        if let Some(options) = self.outgoing_cps.into_enabled(self.default) {
            stats.outgoing_cps = get_outgoing_cps_stats(core_item, options).into();
        }
        if let Some(options) = self.outgoing_rps.into_enabled(self.default) {
            stats.outgoing_rps = get_outgoing_rps_stats(core_item, options).into();
        }
        ////////////////////////////////////////////////////////////////////////////////////////////
        // Tank
        ////////////////////////////////////////////////////////////////////////////////////////////
        if self.resists.into_enabled(self.default) {
            stats.resists = core_item.get_stat_resists().into();
        }
        if self.hp.into_enabled(self.default) {
            stats.hp = core_item.get_stat_hp().into();
        }
        if let Some(options) = self.ehp.into_enabled(self.default) {
            stats.ehp = get_ehp_stats(core_item, options).into()
        }
        if self.wc_ehp.into_enabled(self.default) {
            stats.wc_ehp = core_item.get_stat_wc_ehp().into();
        }
        if let Some(options) = self.rps.into_enabled(self.default) {
            stats.rps = get_rps_stats(core_item, options).into();
        }
        if let Some(options) = self.erps.into_enabled(self.default) {
            stats.erps = get_erps_stats(core_item, options).into();
        }
        if self.breach_resist.into_enabled(self.default) {
            stats.breach_resist = core_item.get_stat_breach_resist().into();
        }
        ////////////////////////////////////////////////////////////////////////////////////////////
        // Cap
        ////////////////////////////////////////////////////////////////////////////////////////////
        if self.cap_amount.into_enabled(self.default) {
            stats.cap_amount = core_item.get_stat_cap_amount().into();
        }
        if let Some(options) = self.cap_balance.into_enabled(self.default) {
            stats.cap_balance = get_cap_balance_stats(core_item, options).into();
        }
        if let Some(options) = self.cap_sim.into_enabled(self.default) {
            stats.cap_sim = get_cap_sim_stats(core_item, options).into();
        }
        if self.neut_resist.into_enabled(self.default) {
            stats.neut_resist = core_item.get_stat_neut_resist().into();
        }
        ////////////////////////////////////////////////////////////////////////////////////////////
        // Sensors
        ////////////////////////////////////////////////////////////////////////////////////////////
        if self.locks.into_enabled(self.default) {
            stats.locks = core_item.get_stat_locks().into();
        }
        if self.lock_range.into_enabled(self.default) {
            stats.lock_range = core_item.get_stat_lock_range().into();
        }
        if self.scan_res.into_enabled(self.default) {
            stats.scan_res = core_item.get_stat_scan_res().into();
        }
        if self.sensors.into_enabled(self.default) {
            stats.sensors = core_item.get_stat_sensors().into();
        }
        if self.dscan_range.into_enabled(self.default) {
            stats.dscan_range = core_item.get_stat_dscan_range().into();
        }
        if self.probing_size.into_enabled(self.default) {
            stats.probing_size = core_item.get_stat_probing_size().unwrap_or_default().into();
        }
        if let Some(options) = self.incoming_jam.into_enabled(self.default) {
            stats.incoming_jam = get_incoming_jam_stats(core_item, options).into();
        }
        ////////////////////////////////////////////////////////////////////////////////////////////
        // Mobility
        ////////////////////////////////////////////////////////////////////////////////////////////
        if self.speed.into_enabled(self.default) {
            stats.speed = core_item.get_stat_speed().into();
        }
        if self.agility.into_enabled(self.default) {
            stats.agility = core_item.get_stat_agility().unwrap_or_default().into();
        }
        if self.align_time.into_enabled(self.default) {
            stats.align_time = core_item.get_stat_align_time().unwrap_or_default().into();
        }
        if self.sig_radius.into_enabled(self.default) {
            stats.sig_radius = core_item.get_stat_sig_radius().into();
        }
        if let Some(options) = self.mass.into_enabled(self.default) {
            stats.mass = get_mass_stats(core_item, options).into();
        }
        if self.warp_speed.into_enabled(self.default) {
            stats.warp_speed = core_item.get_stat_warp_speed().unwrap_or_default().into();
        }
        if self.max_warp_range.into_enabled(self.default) {
            stats.max_warp_range = core_item.get_stat_max_warp_range().unwrap_or_default().into();
        }
        if let Some(options) = self.jump.into_enabled(self.default) {
            stats.jump = get_jump_stats(core_item, options).into();
        }
        ////////////////////////////////////////////////////////////////////////////////////////////
        // Misc
        ////////////////////////////////////////////////////////////////////////////////////////////
        if self.drone_control_range.into_enabled(self.default) {
            stats.drone_control_range = core_item.get_stat_drone_control_range().into();
        }
        if self.can_warp.into_enabled(self.default) {
            stats.can_warp = core_item.get_stat_can_warp().into();
        }
        if self.can_jump_gate.into_enabled(self.default) {
            stats.can_jump_gate = core_item.get_stat_can_jump_gate().into();
        }
        if self.can_jump_wormhole.into_enabled(self.default) {
            stats.can_jump_wormhole = core_item.get_stat_can_jump_wormhole().into();
        }
        if self.can_jump_drive.into_enabled(self.default) {
            stats.can_jump_drive = core_item.get_stat_can_jump_drive().into();
        }
        if self.can_dock_station.into_enabled(self.default) {
            stats.can_dock_station = core_item.get_stat_can_dock_station().into();
        }
        if self.can_dock_citadel.into_enabled(self.default) {
            stats.can_dock_citadel = core_item.get_stat_can_dock_citadel().into();
        }
        if self.can_tether.into_enabled(self.default) {
            stats.can_tether = core_item.get_stat_can_tether().into();
        }
        stats
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Output
////////////////////////////////////////////////////////////////////////////////////////////////////
fn get_dmg_stats(core_item: &mut rc::ItemMut, options: Vec<StatOptionItemDmg>) -> Option<Vec<Option<StatDmg>>> {
    let mut stats = Vec::with_capacity(options.len());
    for option in options.into_iter() {
        match option.projectee_item_id {
            Some(projectee_item_id) => {
                match core_item.get_stat_dmg_applied(
                    option.time_options,
                    option.include_charges,
                    option.ignore_state,
                    &projectee_item_id,
                ) {
                    Ok(core_stat) => stats.push(Some(StatDmg::from_core_applied(core_stat))),
                    Err(core_err) => match is_fatal_app(core_err) {
                        true => return None,
                        false => stats.push(None),
                    },
                };
            }
            None => {
                match core_item.get_stat_dmg(option.time_options, option.include_charges, option.ignore_state) {
                    Ok(core_stat) => stats.push(Some(StatDmg::from_core(core_stat))),
                    Err(core_err) => match is_fatal(core_err) {
                        true => return None,
                        false => stats.push(None),
                    },
                };
            }
        }
    }
    Some(stats)
}
fn get_mps_stats(core_item: &mut rc::ItemMut, options: Vec<StatOptionItemMining>) -> Option<Vec<StatMining>> {
    let mut stats = Vec::with_capacity(options.len());
    for option in options.into_iter() {
        match core_item.get_stat_mps(option.time_options, option.mission, option.ignore_state) {
            Ok(stat) => stats.push(stat),
            Err(_) => return None,
        }
    }
    Some(stats)
}
fn get_outgoing_rps_stats(
    core_item: &mut rc::ItemMut,
    options: Vec<StatOptionItemOutRps>,
) -> Option<Vec<Option<StatOutReps>>> {
    let mut stats = Vec::with_capacity(options.len());
    for option in options.into_iter() {
        match option.projectee_item_id {
            Some(projectee_item_id) => {
                match core_item.get_stat_outgoing_rps_applied(
                    option.time_options,
                    option.ignore_state,
                    &projectee_item_id,
                ) {
                    Ok(stat) => stats.push(Some(stat)),
                    Err(core_err) => match is_fatal_app(core_err) {
                        true => return None,
                        false => stats.push(None),
                    },
                }
            }
            None => match core_item.get_stat_outgoing_rps(option.time_options, option.ignore_state) {
                Ok(stat) => stats.push(Some(stat)),
                Err(core_err) => match is_fatal(core_err) {
                    true => return None,
                    false => stats.push(None),
                },
            },
        }
    }
    Some(stats)
}
fn get_outgoing_nps_stats(
    core_item: &mut rc::ItemMut,
    options: Vec<StatOptionItemOutNps>,
) -> Option<Vec<Option<PValue>>> {
    let mut stats = Vec::with_capacity(options.len());
    for option in options.into_iter() {
        match option.projectee_item_id {
            Some(projectee_item_id) => {
                match core_item.get_stat_outgoing_nps_applied(
                    option.time_options,
                    option.include_charges,
                    option.ignore_state,
                    &projectee_item_id,
                ) {
                    Ok(stat) => stats.push(Some(stat)),
                    Err(core_err) => match is_fatal_app(core_err) {
                        true => return None,
                        false => stats.push(None),
                    },
                }
            }
            None => {
                match core_item.get_stat_outgoing_nps(option.time_options, option.include_charges, option.ignore_state)
                {
                    Ok(stat) => stats.push(Some(stat)),
                    Err(_) => return None,
                }
            }
        }
    }
    Some(stats)
}
fn get_outgoing_cps_stats(
    core_item: &mut rc::ItemMut,
    options: Vec<StatOptionItemOutCps>,
) -> Option<Vec<Option<PValue>>> {
    let mut stats = Vec::with_capacity(options.len());
    for option in options.into_iter() {
        match option.projectee_item_id {
            Some(projectee_item_id) => {
                match core_item.get_stat_outgoing_cps_applied(
                    option.time_options,
                    option.ignore_state,
                    &projectee_item_id,
                ) {
                    Ok(stat) => stats.push(Some(stat)),
                    Err(core_err) => match is_fatal_app(core_err) {
                        true => return None,
                        false => stats.push(None),
                    },
                }
            }
            None => match core_item.get_stat_outgoing_cps(option.time_options, option.ignore_state) {
                Ok(stat) => stats.push(Some(stat)),
                Err(_) => return None,
            },
        }
    }
    Some(stats)
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Tank
////////////////////////////////////////////////////////////////////////////////////////////////////
fn get_ehp_stats(core_item: &mut rc::ItemMut, options: Vec<StatOptionEhp>) -> Option<Vec<StatEhp>> {
    let mut stats = Vec::with_capacity(options.len());
    for option in options.into_iter() {
        match core_item.get_stat_ehp(option.incoming_dps) {
            Ok(stat) => stats.push(stat),
            Err(_) => return None,
        }
    }
    Some(stats)
}
fn get_rps_stats(core_item: &mut rc::ItemMut, options: Vec<StatOptionRps>) -> Option<Vec<StatRps>> {
    let mut stats = Vec::with_capacity(options.len());
    for option in options.into_iter() {
        match core_item.get_stat_rps(option.time_options, option.shield_perc) {
            Ok(stat) => stats.push(stat),
            Err(_) => return None,
        }
    }
    Some(stats)
}
fn get_erps_stats(core_item: &mut rc::ItemMut, options: Vec<StatOptionErps>) -> Option<Vec<StatErps>> {
    let mut stats = Vec::with_capacity(options.len());
    for option in options.into_iter() {
        match core_item.get_stat_erps(option.incoming_dps, option.time_options, option.shield_perc) {
            Ok(stat) => stats.push(stat),
            Err(_) => return None,
        }
    }
    Some(stats)
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Cap
////////////////////////////////////////////////////////////////////////////////////////////////////
fn get_cap_balance_stats(core_item: &mut rc::ItemMut, options: Vec<StatOptionCapBlc>) -> Option<Vec<Option<Value>>> {
    let mut stats = Vec::with_capacity(options.len());
    for option in options.into_iter() {
        match core_item.get_stat_cap_balance(&option.src_kinds, option.time_options) {
            Ok(stat) => stats.push(Some(stat)),
            Err(core_err) => match is_fatal_app(core_err) {
                true => return None,
                false => stats.push(None),
            },
        }
    }
    Some(stats)
}
fn get_cap_sim_stats(core_item: &mut rc::ItemMut, options: Vec<StatOptionCapSim>) -> Option<Vec<Option<StatCapSim>>> {
    let mut stats = Vec::with_capacity(options.len());
    for option in options.into_iter() {
        match core_item.get_stat_cap_sim(
            option.cap_perc,
            option.optional_reloads,
            option.stagger,
            option.nosf_projectee_item_id.as_ref(),
        ) {
            Ok(stat) => stats.push(Some(stat)),
            Err(core_err) => match is_fatal_app(core_err) {
                true => return None,
                false => stats.push(None),
            },
        }
    }
    Some(stats)
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Sensors
////////////////////////////////////////////////////////////////////////////////////////////////////
fn get_incoming_jam_stats(core_item: &mut rc::ItemMut, options: Vec<StatOptionIncomingJam>) -> Option<Vec<StatInJam>> {
    let mut stats = Vec::with_capacity(options.len());
    for option in options.into_iter() {
        match core_item.get_stat_incoming_jam(option.time_options) {
            Ok(stat) => stats.push(stat),
            Err(_) => return None,
        }
    }
    Some(stats)
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Mobility
////////////////////////////////////////////////////////////////////////////////////////////////////
fn get_mass_stats(core_item: &mut rc::ItemMut, options: Vec<StatOptionMass>) -> Option<Vec<PValue>> {
    let mut stats = Vec::with_capacity(options.len());
    for option in options.into_iter() {
        match core_item.get_stat_mass(option.affectors) {
            Ok(stat) => stats.push(stat),
            _ => return None,
        }
    }
    Some(stats)
}
fn get_jump_stats(core_item: &mut rc::ItemMut, options: Vec<StatOptionJump>) -> Option<Vec<StatJump>> {
    let mut stats = Vec::with_capacity(options.len());
    for option in options.into_iter() {
        match core_item.get_stat_jump(option.range, &option.passenger_fit_ids, option.passenger_fuel_affectors) {
            Ok(Some(stat)) => stats.push(stat),
            _ => return None,
        }
    }
    Some(stats)
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Helpers
////////////////////////////////////////////////////////////////////////////////////////////////////
fn is_fatal(core_err: rc::err::ItemStatError) -> bool {
    match core_err {
        rc::err::ItemStatError::ItemNotLoaded(_) | rc::err::ItemStatError::UnsupportedStat(_) => true,
    }
}

fn is_fatal_app(core_err: rc::err::ItemAppliedStatError) -> bool {
    match core_err {
        rc::err::ItemAppliedStatError::ItemNotLoaded(_) | rc::err::ItemAppliedStatError::UnsupportedStat(_) => true,
        rc::err::ItemAppliedStatError::ProjecteeNotFound(_)
        | rc::err::ItemAppliedStatError::ProjecteeCantTakeProjs(_) => false,
    }
}
