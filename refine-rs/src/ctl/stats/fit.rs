use crate::{
    PValue, Value,
    stats::{
        FitStats, StatCapSim, StatDmg, StatEhp, StatErps, StatInJam, StatJump, StatMining, StatOption,
        StatOptionCapBlc, StatOptionCapSim, StatOptionEhp, StatOptionErps, StatOptionExt, StatOptionFitDmg,
        StatOptionFitMining, StatOptionFitOutCps, StatOptionFitOutNps, StatOptionFitOutRps, StatOptionIncomingJam,
        StatOptionJump, StatOptionMass, StatOptionRps, StatOutReps, StatResult, StatRps,
        err::{StatFitAppliedError, StatFitShipAppliedError, StatFitShipError, StatJumpError},
    },
    svc::StatErrorFatality,
};

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Default)]
pub struct GetFitStatsCmd {
    #[cfg_attr(feature = "serde", serde(default))]
    pub default: bool = true,
    // Fit output stats
    #[cfg_attr(feature = "serde", serde(default))]
    pub dmg: StatOptionExt<StatOptionFitDmg> = StatOptionExt::Default,
    #[cfg_attr(feature = "serde", serde(default))]
    pub mps: StatOptionExt<StatOptionFitMining> = StatOptionExt::Default,
    #[cfg_attr(feature = "serde", serde(default))]
    pub outgoing_nps: StatOptionExt<StatOptionFitOutNps> = StatOptionExt::Default,
    #[cfg_attr(feature = "serde", serde(default))]
    pub outgoing_rps: StatOptionExt<StatOptionFitOutRps> = StatOptionExt::Default,
    #[cfg_attr(feature = "serde", serde(default))]
    pub outgoing_cps: StatOptionExt<StatOptionFitOutCps> = StatOptionExt::Default,
    // Fit resources
    #[cfg_attr(feature = "serde", serde(default))]
    pub cpu: StatOption = StatOption::Default,
    #[cfg_attr(feature = "serde", serde(default))]
    pub powergrid: StatOption = StatOption::Default,
    #[cfg_attr(feature = "serde", serde(default))]
    pub calibration: StatOption = StatOption::Default,
    #[cfg_attr(feature = "serde", serde(default))]
    pub drone_bay_volume: StatOption = StatOption::Default,
    #[cfg_attr(feature = "serde", serde(default))]
    pub drone_bandwidth: StatOption = StatOption::Default,
    #[cfg_attr(feature = "serde", serde(default))]
    pub fighter_bay_volume: StatOption = StatOption::Default,
    // Fit slots
    #[cfg_attr(feature = "serde", serde(default))]
    pub high_slots: StatOption = StatOption::Default,
    #[cfg_attr(feature = "serde", serde(default))]
    pub mid_slots: StatOption = StatOption::Default,
    #[cfg_attr(feature = "serde", serde(default))]
    pub low_slots: StatOption = StatOption::Default,
    #[cfg_attr(feature = "serde", serde(default))]
    pub turret_slots: StatOption = StatOption::Default,
    #[cfg_attr(feature = "serde", serde(default))]
    pub launcher_slots: StatOption = StatOption::Default,
    #[cfg_attr(feature = "serde", serde(default))]
    pub rig_slots: StatOption = StatOption::Default,
    #[cfg_attr(feature = "serde", serde(default))]
    pub service_slots: StatOption = StatOption::Default,
    #[cfg_attr(feature = "serde", serde(default))]
    pub subsystem_slots: StatOption = StatOption::Default,
    #[cfg_attr(feature = "serde", serde(default))]
    pub launched_drones: StatOption = StatOption::Default,
    #[cfg_attr(feature = "serde", serde(default))]
    pub launched_fighters: StatOption = StatOption::Default,
    #[cfg_attr(feature = "serde", serde(default))]
    pub launched_light_fighters: StatOption = StatOption::Default,
    #[cfg_attr(feature = "serde", serde(default))]
    pub launched_heavy_fighters: StatOption = StatOption::Default,
    #[cfg_attr(feature = "serde", serde(default))]
    pub launched_support_fighters: StatOption = StatOption::Default,
    #[cfg_attr(feature = "serde", serde(default))]
    pub launched_st_light_fighters: StatOption = StatOption::Default,
    #[cfg_attr(feature = "serde", serde(default))]
    pub launched_st_heavy_fighters: StatOption = StatOption::Default,
    #[cfg_attr(feature = "serde", serde(default))]
    pub launched_st_support_fighters: StatOption = StatOption::Default,
    // Ship tank
    #[cfg_attr(feature = "serde", serde(default))]
    pub resists: StatOption = StatOption::Default,
    #[cfg_attr(feature = "serde", serde(default))]
    pub hp: StatOption = StatOption::Default,
    #[cfg_attr(feature = "serde", serde(default))]
    pub ehp: StatOptionExt<StatOptionEhp> = StatOptionExt::Default,
    #[cfg_attr(feature = "serde", serde(default))]
    pub wc_ehp: StatOption = StatOption::Default,
    #[cfg_attr(feature = "serde", serde(default))]
    pub rps: StatOptionExt<StatOptionRps> = StatOptionExt::Default,
    #[cfg_attr(feature = "serde", serde(default))]
    pub erps: StatOptionExt<StatOptionErps> = StatOptionExt::Default,
    #[cfg_attr(feature = "serde", serde(default))]
    pub breach_resist: StatOption = StatOption::Default,
    // Ship cap
    #[cfg_attr(feature = "serde", serde(default))]
    pub cap_amount: StatOption = StatOption::Default,
    #[cfg_attr(feature = "serde", serde(default))]
    pub cap_balance: StatOptionExt<StatOptionCapBlc> = StatOptionExt::Default,
    #[cfg_attr(feature = "serde", serde(default))]
    pub cap_sim: StatOptionExt<StatOptionCapSim> = StatOptionExt::Default,
    #[cfg_attr(feature = "serde", serde(default))]
    pub neut_resist: StatOption = StatOption::Default,
    // Ship sensors
    #[cfg_attr(feature = "serde", serde(default))]
    pub locks: StatOption = StatOption::Default,
    #[cfg_attr(feature = "serde", serde(default))]
    pub lock_range: StatOption = StatOption::Default,
    #[cfg_attr(feature = "serde", serde(default))]
    pub scan_res: StatOption = StatOption::Default,
    #[cfg_attr(feature = "serde", serde(default))]
    pub sensors: StatOption = StatOption::Default,
    #[cfg_attr(feature = "serde", serde(default))]
    pub dscan_range: StatOption = StatOption::Default,
    #[cfg_attr(feature = "serde", serde(default))]
    pub probing_size: StatOption = StatOption::Default,
    #[cfg_attr(feature = "serde", serde(default))]
    pub incoming_jam: StatOptionExt<StatOptionIncomingJam> = StatOptionExt::Default,
    // Ship mobility
    #[cfg_attr(feature = "serde", serde(default))]
    pub speed: StatOption = StatOption::Default,
    #[cfg_attr(feature = "serde", serde(default))]
    pub agility: StatOption = StatOption::Default,
    #[cfg_attr(feature = "serde", serde(default))]
    pub align_time: StatOption = StatOption::Default,
    #[cfg_attr(feature = "serde", serde(default))]
    pub sig_radius: StatOption = StatOption::Default,
    #[cfg_attr(feature = "serde", serde(default))]
    pub mass: StatOptionExt<StatOptionMass> = StatOptionExt::Default,
    #[cfg_attr(feature = "serde", serde(default))]
    pub warp_speed: StatOption = StatOption::Default,
    #[cfg_attr(feature = "serde", serde(default))]
    pub max_warp_range: StatOption = StatOption::Default,
    #[cfg_attr(feature = "serde", serde(default))]
    pub jump: StatOptionExt<StatOptionJump> = StatOptionExt::Default,
    // Ship misc stats
    #[cfg_attr(feature = "serde", serde(default))]
    pub drone_control_range: StatOption = StatOption::Default,
    #[cfg_attr(feature = "serde", serde(default))]
    pub can_warp: StatOption = StatOption::Default,
    #[cfg_attr(feature = "serde", serde(default))]
    pub can_jump_gate: StatOption = StatOption::Default,
    #[cfg_attr(feature = "serde", serde(default))]
    pub can_jump_wormhole: StatOption = StatOption::Default,
    #[cfg_attr(feature = "serde", serde(default))]
    pub can_jump_drive: StatOption = StatOption::Default,
    #[cfg_attr(feature = "serde", serde(default))]
    pub can_dock_station: StatOption = StatOption::Default,
    #[cfg_attr(feature = "serde", serde(default))]
    pub can_dock_citadel: StatOption = StatOption::Default,
    #[cfg_attr(feature = "serde", serde(default))]
    pub can_tether: StatOption = StatOption::Default,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl GetFitStatsCmd {
    pub(crate) fn execute(self, core_fit: &mut rc::FitMut) -> FitStats {
        let mut stats = FitStats { .. };
        ////////////////////////////////////////////////////////////////////////////////////////////
        // Fit output stats
        ////////////////////////////////////////////////////////////////////////////////////////////
        if let Some(options) = self.dmg.into_enabled(self.default) {
            stats.dmg = get_dmg_stats(core_fit, options);
        }
        if let Some(options) = self.mps.into_enabled(self.default) {
            stats.mps = get_mps_stats(core_fit, options);
        }
        if let Some(options) = self.outgoing_nps.into_enabled(self.default) {
            stats.outgoing_nps = get_outgoing_nps_stats(core_fit, options);
        }
        if let Some(options) = self.outgoing_cps.into_enabled(self.default) {
            stats.outgoing_cps = get_outgoing_cps_stats(core_fit, options);
        }
        if let Some(options) = self.outgoing_rps.into_enabled(self.default) {
            stats.outgoing_rps = get_outgoing_rps_stats(core_fit, options);
        }
        ////////////////////////////////////////////////////////////////////////////////////////////
        // Fit resources
        ////////////////////////////////////////////////////////////////////////////////////////////
        if self.cpu.into_enabled(self.default) {
            stats.cpu = StatResult::from_stat(core_fit.get_stat_cpu());
        }
        if self.powergrid.into_enabled(self.default) {
            stats.powergrid = StatResult::from_stat(core_fit.get_stat_powergrid());
        }
        if self.calibration.into_enabled(self.default) {
            stats.calibration = StatResult::from_stat(core_fit.get_stat_calibration());
        }
        if self.drone_bay_volume.into_enabled(self.default) {
            stats.drone_bay_volume = StatResult::from_stat(core_fit.get_stat_drone_bay_volume());
        }
        if self.drone_bandwidth.into_enabled(self.default) {
            stats.drone_bandwidth = StatResult::from_stat(core_fit.get_stat_drone_bandwidth());
        }
        if self.fighter_bay_volume.into_enabled(self.default) {
            stats.fighter_bay_volume = StatResult::from_stat(core_fit.get_stat_fighter_bay_volume());
        }
        ////////////////////////////////////////////////////////////////////////////////////////////
        // Fit slots
        ////////////////////////////////////////////////////////////////////////////////////////////
        if self.high_slots.into_enabled(self.default) {
            stats.high_slots = StatResult::from_stat(core_fit.get_stat_high_slots());
        }
        if self.mid_slots.into_enabled(self.default) {
            stats.mid_slots = StatResult::from_stat(core_fit.get_stat_mid_slots());
        }
        if self.low_slots.into_enabled(self.default) {
            stats.low_slots = StatResult::from_stat(core_fit.get_stat_low_slots());
        }
        if self.turret_slots.into_enabled(self.default) {
            stats.turret_slots = StatResult::from_stat(core_fit.get_stat_turret_slots());
        }
        if self.launcher_slots.into_enabled(self.default) {
            stats.launcher_slots = StatResult::from_stat(core_fit.get_stat_launcher_slots());
        }
        if self.rig_slots.into_enabled(self.default) {
            stats.rig_slots = StatResult::from_stat(core_fit.get_stat_rig_slots());
        }
        if self.service_slots.into_enabled(self.default) {
            stats.service_slots = StatResult::from_stat(core_fit.get_stat_service_slots());
        }
        if self.subsystem_slots.into_enabled(self.default) {
            stats.subsystem_slots = StatResult::from_stat(core_fit.get_stat_subsystem_slots());
        }
        if self.launched_drones.into_enabled(self.default) {
            stats.launched_drones = StatResult::from_stat(core_fit.get_stat_launched_drones());
        }
        if self.launched_fighters.into_enabled(self.default) {
            stats.launched_fighters = StatResult::from_stat(core_fit.get_stat_launched_fighters());
        }
        if self.launched_light_fighters.into_enabled(self.default) {
            stats.launched_light_fighters = StatResult::from_stat(core_fit.get_stat_launched_light_fighters());
        }
        if self.launched_heavy_fighters.into_enabled(self.default) {
            stats.launched_heavy_fighters = StatResult::from_stat(core_fit.get_stat_launched_heavy_fighters());
        }
        if self.launched_support_fighters.into_enabled(self.default) {
            stats.launched_support_fighters = StatResult::from_stat(core_fit.get_stat_launched_support_fighters());
        }
        if self.launched_st_light_fighters.into_enabled(self.default) {
            stats.launched_st_light_fighters = StatResult::from_stat(core_fit.get_stat_launched_st_light_fighters());
        }
        if self.launched_st_heavy_fighters.into_enabled(self.default) {
            stats.launched_st_heavy_fighters = StatResult::from_stat(core_fit.get_stat_launched_st_heavy_fighters());
        }
        if self.launched_st_support_fighters.into_enabled(self.default) {
            stats.launched_st_support_fighters =
                StatResult::from_stat(core_fit.get_stat_launched_st_support_fighters());
        }
        ////////////////////////////////////////////////////////////////////////////////////////////
        // Ship tank
        ////////////////////////////////////////////////////////////////////////////////////////////
        if self.resists.into_enabled(self.default) {
            stats.resists = StatResult::from_result_outer(core_fit.get_stat_resists());
        }
        if self.hp.into_enabled(self.default) {
            stats.hp = StatResult::from_result_outer(core_fit.get_stat_hp());
        }
        if let Some(options) = self.ehp.into_enabled(self.default) {
            stats.ehp = get_ehp_stats(core_fit, options);
        }
        if self.wc_ehp.into_enabled(self.default) {
            stats.wc_ehp = StatResult::from_result_outer(core_fit.get_stat_wc_ehp());
        }
        if let Some(options) = self.rps.into_enabled(self.default) {
            stats.rps = get_rps_stats(core_fit, options);
        }
        if let Some(options) = self.erps.into_enabled(self.default) {
            stats.erps = get_erps_stats(core_fit, options);
        }
        if self.breach_resist.into_enabled(self.default) {
            stats.breach_resist = StatResult::from_result_outer(core_fit.get_stat_breach_resist());
        }
        ////////////////////////////////////////////////////////////////////////////////////////////
        // Ship cap
        ////////////////////////////////////////////////////////////////////////////////////////////
        if self.cap_amount.into_enabled(self.default) {
            stats.cap_amount = StatResult::from_result_outer(core_fit.get_stat_cap_amount());
        }
        if let Some(options) = self.cap_balance.into_enabled(self.default) {
            stats.cap_balance = get_cap_balance_stats(core_fit, options);
        }
        if let Some(options) = self.cap_sim.into_enabled(self.default) {
            stats.cap_sim = get_cap_sim_stats(core_fit, options);
        }
        if self.neut_resist.into_enabled(self.default) {
            stats.neut_resist = StatResult::from_result_outer(core_fit.get_stat_neut_resist());
        }
        ////////////////////////////////////////////////////////////////////////////////////////////
        // Ship sensors
        ////////////////////////////////////////////////////////////////////////////////////////////
        if self.locks.into_enabled(self.default) {
            stats.locks = StatResult::from_result_outer(core_fit.get_stat_locks());
        }
        if self.lock_range.into_enabled(self.default) {
            stats.lock_range = StatResult::from_result_outer(core_fit.get_stat_lock_range());
        }
        if self.scan_res.into_enabled(self.default) {
            stats.scan_res = StatResult::from_result_outer(core_fit.get_stat_scan_res());
        }
        if self.sensors.into_enabled(self.default) {
            stats.sensors = StatResult::from_result_outer(core_fit.get_stat_sensors());
        }
        if self.dscan_range.into_enabled(self.default) {
            stats.dscan_range = StatResult::from_result_outer(core_fit.get_stat_dscan_range());
        }
        if self.probing_size.into_enabled(self.default) {
            stats.probing_size = StatResult::from_result_outer(core_fit.get_stat_probing_size());
        }
        if let Some(options) = self.incoming_jam.into_enabled(self.default) {
            stats.incoming_jam = get_incoming_jam_stats(core_fit, options);
        }
        ////////////////////////////////////////////////////////////////////////////////////////////
        // Ship mobility
        ////////////////////////////////////////////////////////////////////////////////////////////
        if self.speed.into_enabled(self.default) {
            stats.speed = StatResult::from_result_outer(core_fit.get_stat_speed());
        }
        if self.agility.into_enabled(self.default) {
            stats.agility = StatResult::from_result_outer(core_fit.get_stat_agility());
        }
        if self.align_time.into_enabled(self.default) {
            stats.align_time = StatResult::from_result_outer(core_fit.get_stat_align_time());
        }
        if self.sig_radius.into_enabled(self.default) {
            stats.sig_radius = StatResult::from_result_outer(core_fit.get_stat_sig_radius());
        }
        if let Some(options) = self.mass.into_enabled(self.default) {
            stats.mass = get_mass_stats(core_fit, options);
        }
        if self.warp_speed.into_enabled(self.default) {
            stats.warp_speed = StatResult::from_result_outer(core_fit.get_stat_warp_speed());
        }
        if self.max_warp_range.into_enabled(self.default) {
            stats.max_warp_range = StatResult::from_result_outer(core_fit.get_stat_max_warp_range());
        }
        if let Some(options) = self.jump.into_enabled(self.default) {
            stats.jump = get_jump_stats(core_fit, options);
        }
        ////////////////////////////////////////////////////////////////////////////////////////////
        // Ship misc stats
        ////////////////////////////////////////////////////////////////////////////////////////////
        if self.drone_control_range.into_enabled(self.default) {
            stats.drone_control_range = StatResult::from_result_outer(core_fit.get_stat_drone_control_range());
        }
        if self.can_warp.into_enabled(self.default) {
            stats.can_warp = StatResult::from_result_outer(core_fit.get_stat_can_warp());
        }
        if self.can_jump_gate.into_enabled(self.default) {
            stats.can_jump_gate = StatResult::from_result_outer(core_fit.get_stat_can_jump_gate());
        }
        if self.can_jump_wormhole.into_enabled(self.default) {
            stats.can_jump_wormhole = StatResult::from_result_outer(core_fit.get_stat_can_jump_wormhole());
        }
        if self.can_jump_drive.into_enabled(self.default) {
            stats.can_jump_drive = StatResult::from_result_outer(core_fit.get_stat_can_jump_drive());
        }
        if self.can_dock_station.into_enabled(self.default) {
            stats.can_dock_station = StatResult::from_result_outer(core_fit.get_stat_can_dock_station());
        }
        if self.can_dock_citadel.into_enabled(self.default) {
            stats.can_dock_citadel = StatResult::from_result_outer(core_fit.get_stat_can_dock_citadel());
        }
        if self.can_tether.into_enabled(self.default) {
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
    options: Vec<StatOptionFitDmg>,
) -> StatResult<StatDmg, !, StatFitAppliedError> {
    let mut stats = Vec::with_capacity(options.len());
    for option in options.into_iter() {
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
fn get_mps_stats(core_fit: &mut rc::FitMut, options: Vec<StatOptionFitMining>) -> StatResult<StatMining, !, !> {
    let mut stats = Vec::with_capacity(options.len());
    for option in options.into_iter() {
        let stat = core_fit.get_stat_mps(option.item_kinds, option.time, option.resource_kind);
        stats.push(Ok(stat));
    }
    StatResult::Result(stats)
}
fn get_outgoing_nps_stats(
    core_fit: &mut rc::FitMut,
    options: Vec<StatOptionFitOutNps>,
) -> StatResult<PValue, !, StatFitAppliedError> {
    let mut stats = Vec::with_capacity(options.len());
    for option in options.into_iter() {
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
    options: Vec<StatOptionFitOutRps>,
) -> StatResult<StatOutReps, !, StatFitAppliedError> {
    let mut stats = Vec::with_capacity(options.len());
    for option in options.into_iter() {
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
    options: Vec<StatOptionFitOutCps>,
) -> StatResult<PValue, !, StatFitAppliedError> {
    let mut stats = Vec::with_capacity(options.len());
    for option in options.into_iter() {
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
fn get_ehp_stats(
    core_fit: &mut rc::FitMut,
    options: Vec<StatOptionEhp>,
) -> StatResult<StatEhp, StatFitShipError<!>, !> {
    let mut stats = Vec::with_capacity(options.len());
    for option in options.into_iter() {
        match core_fit.get_stat_ehp(option.incoming_dps) {
            Ok(stat) => stats.push(Ok(stat)),
            Err(err) => return StatResult::Error(err),
        }
    }
    StatResult::Result(stats)
}
fn get_rps_stats(
    core_fit: &mut rc::FitMut,
    options: Vec<StatOptionRps>,
) -> StatResult<StatRps, StatFitShipError<!>, !> {
    let mut stats = Vec::with_capacity(options.len());
    for option in options.into_iter() {
        match core_fit.get_stat_rps(option.time, option.shield_perc) {
            Ok(stat) => stats.push(Ok(stat)),
            Err(err) => return StatResult::Error(err),
        }
    }
    StatResult::Result(stats)
}
fn get_erps_stats(
    core_fit: &mut rc::FitMut,
    options: Vec<StatOptionErps>,
) -> StatResult<StatErps, StatFitShipError<!>, !> {
    let mut stats = Vec::with_capacity(options.len());
    for option in options.into_iter() {
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
    options: Vec<StatOptionCapBlc>,
) -> StatResult<Value, StatFitShipAppliedError<!>, StatFitShipAppliedError<!>> {
    let mut stats = Vec::with_capacity(options.len());
    for option in options.into_iter() {
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
    options: Vec<StatOptionCapSim>,
) -> StatResult<StatCapSim, StatFitShipAppliedError<!>, StatFitShipAppliedError<!>> {
    let mut stats = Vec::with_capacity(options.len());
    for option in options.into_iter() {
        match core_fit.get_stat_cap_sim(
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
// Ship sensors
////////////////////////////////////////////////////////////////////////////////////////////////////
fn get_incoming_jam_stats(
    core_fit: &mut rc::FitMut,
    options: Vec<StatOptionIncomingJam>,
) -> StatResult<StatInJam, StatFitShipError<!>, !> {
    let mut stats = Vec::with_capacity(options.len());
    for option in options.into_iter() {
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
fn get_mass_stats(
    core_fit: &mut rc::FitMut,
    options: Vec<StatOptionMass>,
) -> StatResult<PValue, StatFitShipError<!>, !> {
    let mut stats = Vec::with_capacity(options.len());
    for option in options.into_iter() {
        match core_fit.get_stat_mass(option.affectors) {
            Ok(stat) => stats.push(Ok(stat)),
            Err(err) => return StatResult::Error(err),
        }
    }
    StatResult::Result(stats)
}
fn get_jump_stats(
    core_fit: &mut rc::FitMut,
    options: Vec<StatOptionJump>,
) -> StatResult<StatJump, StatFitShipError<StatJumpError>, !> {
    let mut stats = Vec::with_capacity(options.len());
    for option in options.into_iter() {
        match core_fit.get_stat_jump(option.range, &option.passenger_fit_ids, option.passenger_fuel_affectors) {
            Ok(stat) => stats.push(Ok(stat)),
            Err(err) => return StatResult::Error(err),
        }
    }
    StatResult::Result(stats)
}
