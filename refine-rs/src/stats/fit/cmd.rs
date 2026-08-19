use crate::{
    PValue, Value,
    stats::{
        FitStats, StatCapSim, StatDefOption, StatDefOptionExt, StatDmg, StatEhp, StatErps, StatErrorFatality,
        StatInJam, StatJump, StatMining, StatOptionCapBlc, StatOptionCapSim, StatOptionEhp, StatOptionErps,
        StatOptionExt, StatOptionFitDmg, StatOptionFitMining, StatOptionFitOutCps, StatOptionFitOutNps,
        StatOptionFitOutRps, StatOptionIncomingJam, StatOptionJump, StatOptionMass, StatOptionRps, StatOutReps,
        StatResult, StatRps,
        err::{StatFitAppliedError, StatFitShipAppliedError, StatFitShipError, StatJumpError},
    },
};

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Default)]
pub struct GetFitStatsCmd {
    #[cfg_attr(feature = "serde", serde(default))]
    default: bool = true,
    // Fit output stats
    #[cfg_attr(feature = "serde", serde(default))]
    dmg: StatDefOptionExt<StatOptionFitDmg> = StatDefOptionExt::Default,
    #[cfg_attr(feature = "serde", serde(default))]
    mps: StatDefOptionExt<StatOptionFitMining> = StatDefOptionExt::Default,
    #[cfg_attr(feature = "serde", serde(default))]
    outgoing_nps: StatDefOptionExt<StatOptionFitOutNps> = StatDefOptionExt::Default,
    #[cfg_attr(feature = "serde", serde(default))]
    outgoing_rps: StatDefOptionExt<StatOptionFitOutRps> = StatDefOptionExt::Default,
    #[cfg_attr(feature = "serde", serde(default))]
    outgoing_cps: StatDefOptionExt<StatOptionFitOutCps> = StatDefOptionExt::Default,
    // Fit resources
    #[cfg_attr(feature = "serde", serde(default))]
    cpu: StatDefOption = StatDefOption::Default,
    #[cfg_attr(feature = "serde", serde(default))]
    powergrid: StatDefOption = StatDefOption::Default,
    #[cfg_attr(feature = "serde", serde(default))]
    calibration: StatDefOption = StatDefOption::Default,
    #[cfg_attr(feature = "serde", serde(default))]
    drone_bay_volume: StatDefOption = StatDefOption::Default,
    #[cfg_attr(feature = "serde", serde(default))]
    drone_bandwidth: StatDefOption = StatDefOption::Default,
    #[cfg_attr(feature = "serde", serde(default))]
    fighter_bay_volume: StatDefOption = StatDefOption::Default,
    // Fit slots
    #[cfg_attr(feature = "serde", serde(default))]
    high_slots: StatDefOption = StatDefOption::Default,
    #[cfg_attr(feature = "serde", serde(default))]
    mid_slots: StatDefOption = StatDefOption::Default,
    #[cfg_attr(feature = "serde", serde(default))]
    low_slots: StatDefOption = StatDefOption::Default,
    #[cfg_attr(feature = "serde", serde(default))]
    turret_slots: StatDefOption = StatDefOption::Default,
    #[cfg_attr(feature = "serde", serde(default))]
    launcher_slots: StatDefOption = StatDefOption::Default,
    #[cfg_attr(feature = "serde", serde(default))]
    rig_slots: StatDefOption = StatDefOption::Default,
    #[cfg_attr(feature = "serde", serde(default))]
    service_slots: StatDefOption = StatDefOption::Default,
    #[cfg_attr(feature = "serde", serde(default))]
    subsystem_slots: StatDefOption = StatDefOption::Default,
    #[cfg_attr(feature = "serde", serde(default))]
    launched_drones: StatDefOption = StatDefOption::Default,
    #[cfg_attr(feature = "serde", serde(default))]
    launched_fighters: StatDefOption = StatDefOption::Default,
    #[cfg_attr(feature = "serde", serde(default))]
    launched_light_fighters: StatDefOption = StatDefOption::Default,
    #[cfg_attr(feature = "serde", serde(default))]
    launched_heavy_fighters: StatDefOption = StatDefOption::Default,
    #[cfg_attr(feature = "serde", serde(default))]
    launched_support_fighters: StatDefOption = StatDefOption::Default,
    #[cfg_attr(feature = "serde", serde(default))]
    launched_st_light_fighters: StatDefOption = StatDefOption::Default,
    #[cfg_attr(feature = "serde", serde(default))]
    launched_st_heavy_fighters: StatDefOption = StatDefOption::Default,
    #[cfg_attr(feature = "serde", serde(default))]
    launched_st_support_fighters: StatDefOption = StatDefOption::Default,
    // Ship tank
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
    // Ship cap
    #[cfg_attr(feature = "serde", serde(default))]
    cap_amount: StatDefOption = StatDefOption::Default,
    #[cfg_attr(feature = "serde", serde(default))]
    cap_balance: StatDefOptionExt<StatOptionCapBlc> = StatDefOptionExt::Default,
    #[cfg_attr(feature = "serde", serde(default))]
    cap_sim: StatDefOptionExt<StatOptionCapSim> = StatDefOptionExt::Default,
    #[cfg_attr(feature = "serde", serde(default))]
    neut_resist: StatDefOption = StatDefOption::Default,
    // Ship sensors
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
    // Ship mobility
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
    // Ship misc stats
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
impl GetFitStatsCmd {
    /// True to have all supported stats enabled by default, false to have them disabled.
    pub fn new(default: bool) -> Self {
        Self { default, .. }
    }
    // Fit output stats
    pub fn with_dmg(mut self, option: StatOptionExt<StatOptionFitDmg>) -> Self {
        self.dmg = option.into();
        self
    }
    pub fn with_mps(mut self, option: StatOptionExt<StatOptionFitMining>) -> Self {
        self.mps = option.into();
        self
    }
    pub fn with_outgoing_nps(mut self, option: StatOptionExt<StatOptionFitOutNps>) -> Self {
        self.outgoing_nps = option.into();
        self
    }
    pub fn with_outgoing_rps(mut self, option: StatOptionExt<StatOptionFitOutRps>) -> Self {
        self.outgoing_rps = option.into();
        self
    }
    pub fn with_outgoing_cps(mut self, option: StatOptionExt<StatOptionFitOutCps>) -> Self {
        self.outgoing_cps = option.into();
        self
    }
    // Fit resources
    pub fn with_cpu(mut self, enabled: bool) -> Self {
        self.cpu = enabled.into();
        self
    }
    pub fn with_powergrid(mut self, enabled: bool) -> Self {
        self.powergrid = enabled.into();
        self
    }
    pub fn with_calibration(mut self, enabled: bool) -> Self {
        self.calibration = enabled.into();
        self
    }
    pub fn with_drone_bay_volume(mut self, enabled: bool) -> Self {
        self.drone_bay_volume = enabled.into();
        self
    }
    pub fn with_drone_bandwidth(mut self, enabled: bool) -> Self {
        self.drone_bandwidth = enabled.into();
        self
    }
    pub fn with_fighter_bay_volume(mut self, enabled: bool) -> Self {
        self.fighter_bay_volume = enabled.into();
        self
    }
    // Fit slots
    pub fn with_high_slots(mut self, enabled: bool) -> Self {
        self.high_slots = enabled.into();
        self
    }
    pub fn with_mid_slots(mut self, enabled: bool) -> Self {
        self.mid_slots = enabled.into();
        self
    }
    pub fn with_low_slots(mut self, enabled: bool) -> Self {
        self.low_slots = enabled.into();
        self
    }
    pub fn with_turret_slots(mut self, enabled: bool) -> Self {
        self.turret_slots = enabled.into();
        self
    }
    pub fn with_launcher_slots(mut self, enabled: bool) -> Self {
        self.launcher_slots = enabled.into();
        self
    }
    pub fn with_rig_slots(mut self, enabled: bool) -> Self {
        self.rig_slots = enabled.into();
        self
    }
    pub fn with_service_slots(mut self, enabled: bool) -> Self {
        self.service_slots = enabled.into();
        self
    }
    pub fn with_subsystem_slots(mut self, enabled: bool) -> Self {
        self.subsystem_slots = enabled.into();
        self
    }
    pub fn with_launched_drones(mut self, enabled: bool) -> Self {
        self.launched_drones = enabled.into();
        self
    }
    pub fn with_launched_fighters(mut self, enabled: bool) -> Self {
        self.launched_fighters = enabled.into();
        self
    }
    pub fn with_launched_light_fighters(mut self, enabled: bool) -> Self {
        self.launched_light_fighters = enabled.into();
        self
    }
    pub fn with_launched_heavy_fighters(mut self, enabled: bool) -> Self {
        self.launched_heavy_fighters = enabled.into();
        self
    }
    pub fn with_launched_support_fighters(mut self, enabled: bool) -> Self {
        self.launched_support_fighters = enabled.into();
        self
    }
    pub fn with_launched_st_light_fighters(mut self, enabled: bool) -> Self {
        self.launched_st_light_fighters = enabled.into();
        self
    }
    pub fn with_launched_st_heavy_fighters(mut self, enabled: bool) -> Self {
        self.launched_st_heavy_fighters = enabled.into();
        self
    }
    pub fn with_launched_st_support_fighters(mut self, enabled: bool) -> Self {
        self.launched_st_support_fighters = enabled.into();
        self
    }
    // Ship tank
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
    // Ship cap
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
    // Ship sensors
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
    // Ship mobility
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
    // Ship misc stats
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
impl GetFitStatsCmd {
    pub(crate) fn execute(self, core_fit: &mut rc::FitMut) -> FitStats {
        let mut stats = FitStats { .. };
        // Fit output stats
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
        // Fit resources
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
        // Fit slots
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
        // Ship tank
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
        // Ship cap
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
        // Ship sensors
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
        // Ship mobility
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
        // Ship misc stats
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
// Execution getters - fit output stats
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
// Execution getters - ship tank
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
// Execution getters - ship cap
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
// Execution getters - ship sensors
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
// Execution getters - ship mobility
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
