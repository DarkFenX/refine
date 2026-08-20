use crate::{
    FitId, FitIdBr, ItemId, ItemIdBr,
    stats::{
        FitStats, StatOptionCapBlc, StatOptionCapSim, StatOptionEhp, StatOptionErps, StatOptionExt, StatOptionFitDmg,
        StatOptionFitMining, StatOptionFitOutCps, StatOptionFitOutNps, StatOptionFitOutRps, StatOptionIncomingJam,
        StatOptionJump, StatOptionMass, StatOptionRps, fit::FitStatsOptionsInt, option::StatOptionRaw,
    },
};

#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize),
    serde(
        default,
        bound(deserialize = "FitStatsOptionsInt<StatOptionRaw, F, I>: Default + serde::Deserialize<'de>")
    )
)]
#[derive(Clone)]
pub struct FitStatsOptions<F = FitId, I = ItemId>
where
    F: Clone,
    I: Clone,
{
    #[cfg_attr(feature = "serde", serde(default))]
    default: bool = false,
    #[cfg_attr(feature = "serde", serde(flatten))]
    options: FitStatsOptionsInt<StatOptionRaw, F, I>,
}
impl<F, I> Default for FitStatsOptions<F, I>
where
    F: Clone,
    I: Clone,
    FitStatsOptionsInt<StatOptionRaw, F, I>: Default,
{
    fn default() -> Self {
        Self {
            options: Default::default(),
            ..
        }
    }
}

pub type FitStatsOptionsBr = FitStatsOptions<FitIdBr, ItemIdBr>;

////////////////////////////////////////////////////////////////////////////////////////////////////
// Construction
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<F, I> FitStatsOptions<F, I>
where
    F: Clone,
    I: Clone,
{
    /// True to have all supported stats enabled by default, false to have them disabled.
    pub fn new(default: bool) -> Self {
        Self {
            default,
            options: FitStatsOptionsInt::default(),
        }
    }
    // Fit output stats
    pub fn with_dmg(mut self, option: StatOptionExt<StatOptionFitDmg<I>>) -> Self {
        self.options.dmg = option.into();
        self
    }
    pub fn with_mps(mut self, option: StatOptionExt<StatOptionFitMining>) -> Self {
        self.options.mps = option.into();
        self
    }
    pub fn with_outgoing_nps(mut self, option: StatOptionExt<StatOptionFitOutNps<I>>) -> Self {
        self.options.outgoing_nps = option.into();
        self
    }
    pub fn with_outgoing_rps(mut self, option: StatOptionExt<StatOptionFitOutRps<I>>) -> Self {
        self.options.outgoing_rps = option.into();
        self
    }
    pub fn with_outgoing_cps(mut self, option: StatOptionExt<StatOptionFitOutCps<I>>) -> Self {
        self.options.outgoing_cps = option.into();
        self
    }
    // Fit resources
    pub fn with_cpu(mut self, enabled: bool) -> Self {
        self.options.cpu = enabled.into();
        self
    }
    pub fn with_powergrid(mut self, enabled: bool) -> Self {
        self.options.powergrid = enabled.into();
        self
    }
    pub fn with_calibration(mut self, enabled: bool) -> Self {
        self.options.calibration = enabled.into();
        self
    }
    pub fn with_drone_bay_volume(mut self, enabled: bool) -> Self {
        self.options.drone_bay_volume = enabled.into();
        self
    }
    pub fn with_drone_bandwidth(mut self, enabled: bool) -> Self {
        self.options.drone_bandwidth = enabled.into();
        self
    }
    pub fn with_fighter_bay_volume(mut self, enabled: bool) -> Self {
        self.options.fighter_bay_volume = enabled.into();
        self
    }
    // Fit slots
    pub fn with_high_slots(mut self, enabled: bool) -> Self {
        self.options.high_slots = enabled.into();
        self
    }
    pub fn with_mid_slots(mut self, enabled: bool) -> Self {
        self.options.mid_slots = enabled.into();
        self
    }
    pub fn with_low_slots(mut self, enabled: bool) -> Self {
        self.options.low_slots = enabled.into();
        self
    }
    pub fn with_turret_slots(mut self, enabled: bool) -> Self {
        self.options.turret_slots = enabled.into();
        self
    }
    pub fn with_launcher_slots(mut self, enabled: bool) -> Self {
        self.options.launcher_slots = enabled.into();
        self
    }
    pub fn with_rig_slots(mut self, enabled: bool) -> Self {
        self.options.rig_slots = enabled.into();
        self
    }
    pub fn with_service_slots(mut self, enabled: bool) -> Self {
        self.options.service_slots = enabled.into();
        self
    }
    pub fn with_subsystem_slots(mut self, enabled: bool) -> Self {
        self.options.subsystem_slots = enabled.into();
        self
    }
    pub fn with_launched_drones(mut self, enabled: bool) -> Self {
        self.options.launched_drones = enabled.into();
        self
    }
    pub fn with_launched_fighters(mut self, enabled: bool) -> Self {
        self.options.launched_fighters = enabled.into();
        self
    }
    pub fn with_launched_light_fighters(mut self, enabled: bool) -> Self {
        self.options.launched_light_fighters = enabled.into();
        self
    }
    pub fn with_launched_heavy_fighters(mut self, enabled: bool) -> Self {
        self.options.launched_heavy_fighters = enabled.into();
        self
    }
    pub fn with_launched_support_fighters(mut self, enabled: bool) -> Self {
        self.options.launched_support_fighters = enabled.into();
        self
    }
    pub fn with_launched_st_light_fighters(mut self, enabled: bool) -> Self {
        self.options.launched_st_light_fighters = enabled.into();
        self
    }
    pub fn with_launched_st_heavy_fighters(mut self, enabled: bool) -> Self {
        self.options.launched_st_heavy_fighters = enabled.into();
        self
    }
    pub fn with_launched_st_support_fighters(mut self, enabled: bool) -> Self {
        self.options.launched_st_support_fighters = enabled.into();
        self
    }
    // Ship tank
    pub fn with_resists(mut self, enabled: bool) -> Self {
        self.options.resists = enabled.into();
        self
    }
    pub fn with_hp(mut self, enabled: bool) -> Self {
        self.options.hp = enabled.into();
        self
    }
    pub fn with_ehp(mut self, option: StatOptionExt<StatOptionEhp>) -> Self {
        self.options.ehp = option.into();
        self
    }
    pub fn with_wc_ehp(mut self, enabled: bool) -> Self {
        self.options.wc_ehp = enabled.into();
        self
    }
    pub fn with_rps(mut self, option: StatOptionExt<StatOptionRps>) -> Self {
        self.options.rps = option.into();
        self
    }
    pub fn with_erps(mut self, option: StatOptionExt<StatOptionErps>) -> Self {
        self.options.erps = option.into();
        self
    }
    pub fn with_breach_resist(mut self, enabled: bool) -> Self {
        self.options.breach_resist = enabled.into();
        self
    }
    // Ship cap
    pub fn with_cap_amount(mut self, enabled: bool) -> Self {
        self.options.cap_amount = enabled.into();
        self
    }
    pub fn with_cap_balance(mut self, option: StatOptionExt<StatOptionCapBlc<I>>) -> Self {
        self.options.cap_balance = option.into();
        self
    }
    pub fn with_cap_sim(mut self, option: StatOptionExt<StatOptionCapSim<I>>) -> Self {
        self.options.cap_sim = option.into();
        self
    }
    pub fn with_neut_resist(mut self, enabled: bool) -> Self {
        self.options.neut_resist = enabled.into();
        self
    }
    // Ship sensors
    pub fn with_locks(mut self, enabled: bool) -> Self {
        self.options.locks = enabled.into();
        self
    }
    pub fn with_lock_range(mut self, enabled: bool) -> Self {
        self.options.lock_range = enabled.into();
        self
    }
    pub fn with_scan_res(mut self, enabled: bool) -> Self {
        self.options.scan_res = enabled.into();
        self
    }
    pub fn with_sensors(mut self, enabled: bool) -> Self {
        self.options.sensors = enabled.into();
        self
    }
    pub fn with_dscan_range(mut self, enabled: bool) -> Self {
        self.options.dscan_range = enabled.into();
        self
    }
    pub fn with_probing_size(mut self, enabled: bool) -> Self {
        self.options.probing_size = enabled.into();
        self
    }
    pub fn with_incoming_jam(mut self, option: StatOptionExt<StatOptionIncomingJam>) -> Self {
        self.options.incoming_jam = option.into();
        self
    }
    // Ship mobility
    pub fn with_speed(mut self, enabled: bool) -> Self {
        self.options.speed = enabled.into();
        self
    }
    pub fn with_agility(mut self, enabled: bool) -> Self {
        self.options.agility = enabled.into();
        self
    }
    pub fn with_align_time(mut self, enabled: bool) -> Self {
        self.options.align_time = enabled.into();
        self
    }
    pub fn with_sig_radius(mut self, enabled: bool) -> Self {
        self.options.sig_radius = enabled.into();
        self
    }
    pub fn with_mass(mut self, option: StatOptionExt<StatOptionMass>) -> Self {
        self.options.mass = option.into();
        self
    }
    pub fn with_warp_speed(mut self, enabled: bool) -> Self {
        self.options.warp_speed = enabled.into();
        self
    }
    pub fn with_max_warp_range(mut self, enabled: bool) -> Self {
        self.options.max_warp_range = enabled.into();
        self
    }
    pub fn with_jump(mut self, option: StatOptionExt<StatOptionJump<F>>) -> Self {
        self.options.jump = option.into();
        self
    }
    // Ship misc stats
    pub fn with_drone_control_range(mut self, enabled: bool) -> Self {
        self.options.drone_control_range = enabled.into();
        self
    }
    pub fn with_can_warp(mut self, enabled: bool) -> Self {
        self.options.can_warp = enabled.into();
        self
    }
    pub fn with_can_jump_gate(mut self, enabled: bool) -> Self {
        self.options.can_jump_gate = enabled.into();
        self
    }
    pub fn with_can_jump_wormhole(mut self, enabled: bool) -> Self {
        self.options.can_jump_wormhole = enabled.into();
        self
    }
    pub fn with_can_jump_drive(mut self, enabled: bool) -> Self {
        self.options.can_jump_drive = enabled.into();
        self
    }
    pub fn with_can_dock_station(mut self, enabled: bool) -> Self {
        self.options.can_dock_station = enabled.into();
        self
    }
    pub fn with_can_dock_citadel(mut self, enabled: bool) -> Self {
        self.options.can_dock_citadel = enabled.into();
        self
    }
    pub fn with_can_tether(mut self, enabled: bool) -> Self {
        self.options.can_tether = enabled.into();
        self
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl FitStatsOptions {
    pub(crate) fn execute(self, core_fit: &mut rc::FitMut) -> FitStats {
        self.options.stat_resolve(self.default).execute(core_fit)
    }
}
