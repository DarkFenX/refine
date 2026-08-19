use rc::ItemMutCommon;

use crate::{
    PValue, Value,
    stats::{
        ItemStats, StatCapSim, StatDmg, StatEhp, StatErps, StatInJam, StatJump, StatMining, StatOptionCapBlc,
        StatOptionCapSim, StatOptionEhp, StatOptionErps, StatOptionExt, StatOptionIncomingJam, StatOptionItemDmg,
        StatOptionItemMining, StatOptionItemOutCps, StatOptionItemOutNps, StatOptionItemOutRps, StatOptionJump,
        StatOptionMass, StatOptionRps, StatOutReps, StatResult, StatRps,
        err::{StatItemAppliedError, StatItemError, StatJumpError},
        fatal::StatErrorFatality,
        option_support::{StatDefOption, StatDefOptionExt},
    },
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Construction
////////////////////////////////////////////////////////////////////////////////////////////////////
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Default)]
pub struct GetItemStatsCmd {
    #[cfg_attr(feature = "serde", serde(default = "custom_serde::stat_default"))]
    default: bool = true,
    // Output
    #[cfg_attr(feature = "serde", serde(default))]
    dmg: StatDefOptionExt<StatOptionItemDmg> = StatDefOptionExt::Default,
    #[cfg_attr(feature = "serde", serde(default))]
    mps: StatDefOptionExt<StatOptionItemMining> = StatDefOptionExt::Default,
    #[cfg_attr(feature = "serde", serde(default))]
    outgoing_nps: StatDefOptionExt<StatOptionItemOutNps> = StatDefOptionExt::Default,
    #[cfg_attr(feature = "serde", serde(default))]
    outgoing_rps: StatDefOptionExt<StatOptionItemOutRps> = StatDefOptionExt::Default,
    #[cfg_attr(feature = "serde", serde(default))]
    outgoing_cps: StatDefOptionExt<StatOptionItemOutCps> = StatDefOptionExt::Default,
    // Tank
    #[cfg_attr(feature = "serde", serde(default))]
    resists: StatDefOption = StatDefOption::Default,
    #[cfg_attr(feature = "serde", serde(default))]
    hp: StatDefOption = StatDefOption::Default,
    #[cfg_attr(feature = "serde", serde(default))]
    ehp: StatDefOptionExt<StatOptionEhp> = StatDefOptionExt::Default,
    #[cfg_attr(feature = "serde", serde(default))]
    wc_ehp: StatDefOption = StatDefOption::Default,
    #[cfg_attr(feature = "serde", serde(default))]
    rps: StatDefOptionExt<StatOptionRps> = StatDefOptionExt::Default,
    #[cfg_attr(feature = "serde", serde(default))]
    erps: StatDefOptionExt<StatOptionErps> = StatDefOptionExt::Default,
    #[cfg_attr(feature = "serde", serde(default))]
    breach_resist: StatDefOption = StatDefOption::Default,
    // Cap
    #[cfg_attr(feature = "serde", serde(default))]
    cap_amount: StatDefOption = StatDefOption::Default,
    #[cfg_attr(feature = "serde", serde(default))]
    cap_balance: StatDefOptionExt<StatOptionCapBlc> = StatDefOptionExt::Default,
    #[cfg_attr(feature = "serde", serde(default))]
    cap_sim: StatDefOptionExt<StatOptionCapSim> = StatDefOptionExt::Default,
    #[cfg_attr(feature = "serde", serde(default))]
    neut_resist: StatDefOption = StatDefOption::Default,
    // Sensors
    #[cfg_attr(feature = "serde", serde(default))]
    locks: StatDefOption = StatDefOption::Default,
    #[cfg_attr(feature = "serde", serde(default))]
    lock_range: StatDefOption = StatDefOption::Default,
    #[cfg_attr(feature = "serde", serde(default))]
    scan_res: StatDefOption = StatDefOption::Default,
    #[cfg_attr(feature = "serde", serde(default))]
    sensors: StatDefOption = StatDefOption::Default,
    #[cfg_attr(feature = "serde", serde(default))]
    dscan_range: StatDefOption = StatDefOption::Default,
    #[cfg_attr(feature = "serde", serde(default))]
    probing_size: StatDefOption = StatDefOption::Default,
    #[cfg_attr(feature = "serde", serde(default))]
    incoming_jam: StatDefOptionExt<StatOptionIncomingJam> = StatDefOptionExt::Default,
    // Mobility
    #[cfg_attr(feature = "serde", serde(default))]
    speed: StatDefOption = StatDefOption::Default,
    #[cfg_attr(feature = "serde", serde(default))]
    agility: StatDefOption = StatDefOption::Default,
    #[cfg_attr(feature = "serde", serde(default))]
    align_time: StatDefOption = StatDefOption::Default,
    #[cfg_attr(feature = "serde", serde(default))]
    sig_radius: StatDefOption = StatDefOption::Default,
    #[cfg_attr(feature = "serde", serde(default))]
    mass: StatDefOptionExt<StatOptionMass> = StatDefOptionExt::Default,
    #[cfg_attr(feature = "serde", serde(default))]
    warp_speed: StatDefOption = StatDefOption::Default,
    #[cfg_attr(feature = "serde", serde(default))]
    max_warp_range: StatDefOption = StatDefOption::Default,
    #[cfg_attr(feature = "serde", serde(default))]
    jump: StatDefOptionExt<StatOptionJump> = StatDefOptionExt::Default,
    // Misc
    #[cfg_attr(feature = "serde", serde(default))]
    drone_control_range: StatDefOption = StatDefOption::Default,
    #[cfg_attr(feature = "serde", serde(default))]
    can_warp: StatDefOption = StatDefOption::Default,
    #[cfg_attr(feature = "serde", serde(default))]
    can_jump_gate: StatDefOption = StatDefOption::Default,
    #[cfg_attr(feature = "serde", serde(default))]
    can_jump_wormhole: StatDefOption = StatDefOption::Default,
    #[cfg_attr(feature = "serde", serde(default))]
    can_jump_drive: StatDefOption = StatDefOption::Default,
    #[cfg_attr(feature = "serde", serde(default))]
    can_dock_station: StatDefOption = StatDefOption::Default,
    #[cfg_attr(feature = "serde", serde(default))]
    can_dock_citadel: StatDefOption = StatDefOption::Default,
    #[cfg_attr(feature = "serde", serde(default))]
    can_tether: StatDefOption = StatDefOption::Default,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Construction
////////////////////////////////////////////////////////////////////////////////////////////////////
impl GetItemStatsCmd {
    /// True to have all supported stats enabled by default, false to have them disabled.
    pub fn new(default: bool) -> Self {
        Self { default, .. }
    }
    // Output
    pub fn with_dmg(mut self, option: StatOptionExt<StatOptionItemDmg>) -> Self {
        self.dmg = option.into();
        self
    }
    pub fn with_mps(mut self, option: StatOptionExt<StatOptionItemMining>) -> Self {
        self.mps = option.into();
        self
    }
    pub fn with_outgoing_nps(mut self, option: StatOptionExt<StatOptionItemOutNps>) -> Self {
        self.outgoing_nps = option.into();
        self
    }
    pub fn with_outgoing_rps(mut self, option: StatOptionExt<StatOptionItemOutRps>) -> Self {
        self.outgoing_rps = option.into();
        self
    }
    pub fn with_outgoing_cps(mut self, option: StatOptionExt<StatOptionItemOutCps>) -> Self {
        self.outgoing_cps = option.into();
        self
    }
    // Tank
    pub fn with_resists(mut self, enabled: bool) -> Self {
        self.resists = enabled.into();
        self
    }
    pub fn with_hp(mut self, enabled: bool) -> Self {
        self.hp = enabled.into();
        self
    }
    pub fn with_ehp(mut self, option: StatOptionExt<StatOptionEhp>) -> Self {
        self.ehp = option.into();
        self
    }
    pub fn with_wc_ehp(mut self, enabled: bool) -> Self {
        self.wc_ehp = enabled.into();
        self
    }
    pub fn with_rps(mut self, option: StatOptionExt<StatOptionRps>) -> Self {
        self.rps = option.into();
        self
    }
    pub fn with_erps(mut self, option: StatOptionExt<StatOptionErps>) -> Self {
        self.erps = option.into();
        self
    }
    pub fn with_breach_resist(mut self, enabled: bool) -> Self {
        self.breach_resist = enabled.into();
        self
    }
    // Cap
    pub fn with_cap_amount(mut self, enabled: bool) -> Self {
        self.cap_amount = enabled.into();
        self
    }
    pub fn with_cap_balance(mut self, option: StatOptionExt<StatOptionCapBlc>) -> Self {
        self.cap_balance = option.into();
        self
    }
    pub fn with_cap_sim(mut self, option: StatOptionExt<StatOptionCapSim>) -> Self {
        self.cap_sim = option.into();
        self
    }
    pub fn with_neut_resist(mut self, enabled: bool) -> Self {
        self.neut_resist = enabled.into();
        self
    }
    // Sensors
    pub fn with_locks(mut self, enabled: bool) -> Self {
        self.locks = enabled.into();
        self
    }
    pub fn with_lock_range(mut self, enabled: bool) -> Self {
        self.lock_range = enabled.into();
        self
    }
    pub fn with_scan_res(mut self, enabled: bool) -> Self {
        self.scan_res = enabled.into();
        self
    }
    pub fn with_sensors(mut self, enabled: bool) -> Self {
        self.sensors = enabled.into();
        self
    }
    pub fn with_dscan_range(mut self, enabled: bool) -> Self {
        self.dscan_range = enabled.into();
        self
    }
    pub fn with_probing_size(mut self, enabled: bool) -> Self {
        self.probing_size = enabled.into();
        self
    }
    pub fn with_incoming_jam(mut self, option: StatOptionExt<StatOptionIncomingJam>) -> Self {
        self.incoming_jam = option.into();
        self
    }
    // Mobility
    pub fn with_speed(mut self, enabled: bool) -> Self {
        self.speed = enabled.into();
        self
    }
    pub fn with_agility(mut self, enabled: bool) -> Self {
        self.agility = enabled.into();
        self
    }
    pub fn with_align_time(mut self, enabled: bool) -> Self {
        self.align_time = enabled.into();
        self
    }
    pub fn with_sig_radius(mut self, enabled: bool) -> Self {
        self.sig_radius = enabled.into();
        self
    }
    pub fn with_mass(mut self, option: StatOptionExt<StatOptionMass>) -> Self {
        self.mass = option.into();
        self
    }
    pub fn with_warp_speed(mut self, enabled: bool) -> Self {
        self.warp_speed = enabled.into();
        self
    }
    pub fn with_max_warp_range(mut self, enabled: bool) -> Self {
        self.max_warp_range = enabled.into();
        self
    }
    pub fn with_jump(mut self, option: StatOptionExt<StatOptionJump>) -> Self {
        self.jump = option.into();
        self
    }
    // Misc
    pub fn with_drone_control_range(mut self, enabled: bool) -> Self {
        self.drone_control_range = enabled.into();
        self
    }
    pub fn with_can_warp(mut self, enabled: bool) -> Self {
        self.can_warp = enabled.into();
        self
    }
    pub fn with_can_jump_gate(mut self, enabled: bool) -> Self {
        self.can_jump_gate = enabled.into();
        self
    }
    pub fn with_can_jump_wormhole(mut self, enabled: bool) -> Self {
        self.can_jump_wormhole = enabled.into();
        self
    }
    pub fn with_can_jump_drive(mut self, enabled: bool) -> Self {
        self.can_jump_drive = enabled.into();
        self
    }
    pub fn with_can_dock_station(mut self, enabled: bool) -> Self {
        self.can_dock_station = enabled.into();
        self
    }
    pub fn with_can_dock_citadel(mut self, enabled: bool) -> Self {
        self.can_dock_citadel = enabled.into();
        self
    }
    pub fn with_can_tether(mut self, enabled: bool) -> Self {
        self.can_tether = enabled.into();
        self
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl GetItemStatsCmd {
    pub(crate) fn execute(self, core_item: &mut rc::ItemMut) -> ItemStats {
        let mut stats = ItemStats { .. };
        // Output
        if let Some(options) = self.dmg.into_enabled(self.default) {
            stats.dmg = get_dmg_stats(core_item, options);
        }
        if let Some(options) = self.mps.into_enabled(self.default) {
            stats.mps = get_mps_stats(core_item, options);
        }
        if let Some(options) = self.outgoing_nps.into_enabled(self.default) {
            stats.outgoing_nps = get_outgoing_nps_stats(core_item, options);
        }
        if let Some(options) = self.outgoing_cps.into_enabled(self.default) {
            stats.outgoing_cps = get_outgoing_cps_stats(core_item, options);
        }
        if let Some(options) = self.outgoing_rps.into_enabled(self.default) {
            stats.outgoing_rps = get_outgoing_rps_stats(core_item, options);
        }
        // Tank
        if self.resists.into_enabled(self.default) {
            stats.resists = StatResult::from_result_outer(core_item.get_stat_resists());
        }
        if self.hp.into_enabled(self.default) {
            stats.hp = StatResult::from_result_outer(core_item.get_stat_hp());
        }
        if let Some(options) = self.ehp.into_enabled(self.default) {
            stats.ehp = get_ehp_stats(core_item, options);
        }
        if self.wc_ehp.into_enabled(self.default) {
            stats.wc_ehp = StatResult::from_result_outer(core_item.get_stat_wc_ehp());
        }
        if let Some(options) = self.rps.into_enabled(self.default) {
            stats.rps = get_rps_stats(core_item, options);
        }
        if let Some(options) = self.erps.into_enabled(self.default) {
            stats.erps = get_erps_stats(core_item, options);
        }
        if self.breach_resist.into_enabled(self.default) {
            stats.breach_resist = StatResult::from_result_outer(core_item.get_stat_breach_resist());
        }
        // Cap
        if self.cap_amount.into_enabled(self.default) {
            stats.cap_amount = StatResult::from_result_outer(core_item.get_stat_cap_amount());
        }
        if let Some(options) = self.cap_balance.into_enabled(self.default) {
            stats.cap_balance = get_cap_balance_stats(core_item, options);
        }
        if let Some(options) = self.cap_sim.into_enabled(self.default) {
            stats.cap_sim = get_cap_sim_stats(core_item, options);
        }
        if self.neut_resist.into_enabled(self.default) {
            stats.neut_resist = StatResult::from_result_outer(core_item.get_stat_neut_resist());
        }
        // Sensors
        if self.locks.into_enabled(self.default) {
            stats.locks = StatResult::from_result_outer(core_item.get_stat_locks());
        }
        if self.lock_range.into_enabled(self.default) {
            stats.lock_range = StatResult::from_result_outer(core_item.get_stat_lock_range());
        }
        if self.scan_res.into_enabled(self.default) {
            stats.scan_res = StatResult::from_result_outer(core_item.get_stat_scan_res());
        }
        if self.sensors.into_enabled(self.default) {
            stats.sensors = StatResult::from_result_outer(core_item.get_stat_sensors());
        }
        if self.dscan_range.into_enabled(self.default) {
            stats.dscan_range = StatResult::from_result_outer(core_item.get_stat_dscan_range());
        }
        if self.probing_size.into_enabled(self.default) {
            stats.probing_size = StatResult::from_result_outer(core_item.get_stat_probing_size());
        }
        if let Some(options) = self.incoming_jam.into_enabled(self.default) {
            stats.incoming_jam = get_incoming_jam_stats(core_item, options);
        }
        // Mobility
        if self.speed.into_enabled(self.default) {
            stats.speed = StatResult::from_result_outer(core_item.get_stat_speed());
        }
        if self.agility.into_enabled(self.default) {
            stats.agility = StatResult::from_result_outer(core_item.get_stat_agility());
        }
        if self.align_time.into_enabled(self.default) {
            stats.align_time = StatResult::from_result_outer(core_item.get_stat_align_time());
        }
        if self.sig_radius.into_enabled(self.default) {
            stats.sig_radius = StatResult::from_result_outer(core_item.get_stat_sig_radius());
        }
        if let Some(options) = self.mass.into_enabled(self.default) {
            stats.mass = get_mass_stats(core_item, options);
        }
        if self.warp_speed.into_enabled(self.default) {
            stats.warp_speed = StatResult::from_result_outer(core_item.get_stat_warp_speed());
        }
        if self.max_warp_range.into_enabled(self.default) {
            stats.max_warp_range = StatResult::from_result_outer(core_item.get_stat_max_warp_range());
        }
        if let Some(options) = self.jump.into_enabled(self.default) {
            stats.jump = get_jump_stats(core_item, options);
        }
        // Misc
        if self.drone_control_range.into_enabled(self.default) {
            stats.drone_control_range = StatResult::from_result_outer(core_item.get_stat_drone_control_range());
        }
        if self.can_warp.into_enabled(self.default) {
            stats.can_warp = StatResult::from_result_outer(core_item.get_stat_can_warp());
        }
        if self.can_jump_gate.into_enabled(self.default) {
            stats.can_jump_gate = StatResult::from_result_outer(core_item.get_stat_can_jump_gate());
        }
        if self.can_jump_wormhole.into_enabled(self.default) {
            stats.can_jump_wormhole = StatResult::from_result_outer(core_item.get_stat_can_jump_wormhole());
        }
        if self.can_jump_drive.into_enabled(self.default) {
            stats.can_jump_drive = StatResult::from_result_outer(core_item.get_stat_can_jump_drive());
        }
        if self.can_dock_station.into_enabled(self.default) {
            stats.can_dock_station = StatResult::from_result_outer(core_item.get_stat_can_dock_station());
        }
        if self.can_dock_citadel.into_enabled(self.default) {
            stats.can_dock_citadel = StatResult::from_result_outer(core_item.get_stat_can_dock_citadel());
        }
        if self.can_tether.into_enabled(self.default) {
            stats.can_tether = StatResult::from_result_outer(core_item.get_stat_can_tether());
        }
        stats
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution getters - output
////////////////////////////////////////////////////////////////////////////////////////////////////
fn get_dmg_stats(
    core_item: &mut rc::ItemMut,
    options: Vec<StatOptionItemDmg>,
) -> StatResult<StatDmg, StatItemAppliedError<!>, StatItemAppliedError<!>> {
    let mut stats = Vec::with_capacity(options.len());
    for option in options.into_iter() {
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
    options: Vec<StatOptionItemMining>,
) -> StatResult<StatMining, StatItemError<!>, !> {
    let mut stats = Vec::with_capacity(options.len());
    for option in options.into_iter() {
        match core_item.get_stat_mps(option.time, option.resource_kind, option.state) {
            Ok(stat) => stats.push(Ok(stat)),
            Err(err) => return StatResult::Error(err),
        }
    }
    StatResult::Result(stats)
}
fn get_outgoing_nps_stats(
    core_item: &mut rc::ItemMut,
    options: Vec<StatOptionItemOutNps>,
) -> StatResult<PValue, StatItemAppliedError<!>, StatItemAppliedError<!>> {
    let mut stats = Vec::with_capacity(options.len());
    for option in options.into_iter() {
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
    options: Vec<StatOptionItemOutRps>,
) -> StatResult<StatOutReps, StatItemAppliedError<!>, StatItemAppliedError<!>> {
    let mut stats = Vec::with_capacity(options.len());
    for option in options.into_iter() {
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
    options: Vec<StatOptionItemOutCps>,
) -> StatResult<PValue, StatItemAppliedError<!>, StatItemAppliedError<!>> {
    let mut stats = Vec::with_capacity(options.len());
    for option in options.into_iter() {
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
// Execution getters - tank
////////////////////////////////////////////////////////////////////////////////////////////////////
fn get_ehp_stats(core_item: &mut rc::ItemMut, options: Vec<StatOptionEhp>) -> StatResult<StatEhp, StatItemError<!>, !> {
    let mut stats = Vec::with_capacity(options.len());
    for option in options.into_iter() {
        match core_item.get_stat_ehp(option.incoming_dps) {
            Ok(stat) => stats.push(Ok(stat)),
            Err(err) => return StatResult::Error(err),
        }
    }
    StatResult::Result(stats)
}
fn get_rps_stats(core_item: &mut rc::ItemMut, options: Vec<StatOptionRps>) -> StatResult<StatRps, StatItemError<!>, !> {
    let mut stats = Vec::with_capacity(options.len());
    for option in options.into_iter() {
        match core_item.get_stat_rps(option.time, option.shield_perc) {
            Ok(stat) => stats.push(Ok(stat)),
            Err(err) => return StatResult::Error(err),
        }
    }
    StatResult::Result(stats)
}
fn get_erps_stats(
    core_item: &mut rc::ItemMut,
    options: Vec<StatOptionErps>,
) -> StatResult<StatErps, StatItemError<!>, !> {
    let mut stats = Vec::with_capacity(options.len());
    for option in options.into_iter() {
        match core_item.get_stat_erps(option.incoming_dps, option.time, option.shield_perc) {
            Ok(stat) => stats.push(Ok(stat)),
            Err(err) => return StatResult::Error(err),
        }
    }
    StatResult::Result(stats)
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution getters - cap
////////////////////////////////////////////////////////////////////////////////////////////////////
fn get_cap_balance_stats(
    core_item: &mut rc::ItemMut,
    options: Vec<StatOptionCapBlc>,
) -> StatResult<Value, StatItemAppliedError<!>, StatItemAppliedError<!>> {
    let mut stats = Vec::with_capacity(options.len());
    for option in options.into_iter() {
        match core_item.get_stat_cap_balance(&option.src_kinds, option.time) {
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
    options: Vec<StatOptionCapSim>,
) -> StatResult<StatCapSim, StatItemAppliedError<!>, StatItemAppliedError<!>> {
    let mut stats = Vec::with_capacity(options.len());
    for option in options.into_iter() {
        match core_item.get_stat_cap_sim(
            option.cap_perc,
            option.optional_reloads,
            option.stagger,
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
// Execution getters - sensors
////////////////////////////////////////////////////////////////////////////////////////////////////
fn get_incoming_jam_stats(
    core_item: &mut rc::ItemMut,
    options: Vec<StatOptionIncomingJam>,
) -> StatResult<StatInJam, StatItemError<!>, !> {
    let mut stats = Vec::with_capacity(options.len());
    for option in options.into_iter() {
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
fn get_mass_stats(
    core_item: &mut rc::ItemMut,
    options: Vec<StatOptionMass>,
) -> StatResult<PValue, StatItemError<!>, !> {
    let mut stats = Vec::with_capacity(options.len());
    for option in options.into_iter() {
        match core_item.get_stat_mass(option.affectors) {
            Ok(stat) => stats.push(Ok(stat)),
            Err(err) => return StatResult::Error(err),
        }
    }
    StatResult::Result(stats)
}
fn get_jump_stats(
    core_item: &mut rc::ItemMut,
    options: Vec<StatOptionJump>,
) -> StatResult<StatJump, StatItemError<StatJumpError>, !> {
    let mut stats = Vec::with_capacity(options.len());
    for option in options.into_iter() {
        match core_item.get_stat_jump(option.range, &option.passenger_fit_ids, option.passenger_fuel_affectors) {
            Ok(stat) => stats.push(Ok(stat)),
            Err(err) => return StatResult::Error(err),
        }
    }
    StatResult::Result(stats)
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution getters - helpers
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

////////////////////////////////////////////////////////////////////////////////////////////////////
// Custom de/serialization
////////////////////////////////////////////////////////////////////////////////////////////////////
#[cfg(feature = "serde")]
mod custom_serde {
    pub(super) fn stat_default() -> bool {
        true
    }
}
