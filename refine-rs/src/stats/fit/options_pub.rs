use crate::{
    CmdResps, FitId, FitIdBr, ItemId, ItemIdBr,
    shared::BrResolveInfallible,
    stats::{
        StatOptionCapBlcGen, StatOptionCapSimGen, StatOptionEhp, StatOptionErps, StatOptionExt, StatOptionFitDmgGen,
        StatOptionFitMining, StatOptionFitOutCpsGen, StatOptionFitOutNpsGen, StatOptionFitOutRpsGen,
        StatOptionIncomingJam, StatOptionInt, StatOptionJumpGen, StatOptionMass, StatOptionRps,
        fit::FitStatsOptionsResolved,
    },
};

/// Which stats to fetch for a fit.
///
/// By default, all stats are not fetched.
#[derive(Clone)]
pub struct FitStatsOptionsGen<F, I> {
    default: bool = false,
    overrides: Vec<FitStatOption<F, I>> = Vec::new(),
}
impl<F, I> Default for FitStatsOptionsGen<F, I> {
    fn default() -> Self {
        Self { .. }
    }
}

pub type FitStatsOptions = FitStatsOptionsGen<FitId, ItemId>;
pub type FitStatsOptionsBr = FitStatsOptionsGen<FitIdBr, ItemIdBr>;

#[derive(Clone)]
enum FitStatOption<F, I> {
    // Fit output stats
    Dmg(StatOptionInt<StatOptionFitDmgGen<I>>),
    Mps(StatOptionExt<StatOptionFitMining>),
    OutgoingNps(StatOptionInt<StatOptionFitOutNpsGen<I>>),
    OutgoingRps(StatOptionInt<StatOptionFitOutRpsGen<I>>),
    OutgoingCps(StatOptionInt<StatOptionFitOutCpsGen<I>>),
    // Fit resources
    Cpu(bool),
    Powergrid(bool),
    Calibration(bool),
    DroneBayVolume(bool),
    DroneBandwidth(bool),
    FighterBayVolume(bool),
    // Fit slots
    HighSlots(bool),
    MidSlots(bool),
    LowSlots(bool),
    TurretSlots(bool),
    LauncherSlots(bool),
    RigSlots(bool),
    ServiceSlots(bool),
    SubsystemSlots(bool),
    LaunchedDrones(bool),
    LaunchedFighters(bool),
    LaunchedLightFighters(bool),
    LaunchedHeavyFighters(bool),
    LaunchedSupportFighters(bool),
    LaunchedStLightFighters(bool),
    LaunchedStHeavyFighters(bool),
    LaunchedStSupportFighters(bool),
    // Ship tank
    Resists(bool),
    Hp(bool),
    Ehp(StatOptionExt<StatOptionEhp>),
    WcEhp(bool),
    Rps(StatOptionExt<StatOptionRps>),
    Erps(StatOptionExt<StatOptionErps>),
    BreachResist(bool),
    // Ship cap
    CapAmount(bool),
    CapBalance(StatOptionInt<StatOptionCapBlcGen<I>>),
    CapSim(StatOptionInt<StatOptionCapSimGen<I>>),
    NeutResist(bool),
    // Ship sensors
    Locks(bool),
    LockRange(bool),
    ScanRes(bool),
    Sensors(bool),
    DscanRange(bool),
    ProbingSize(bool),
    IncomingJam(StatOptionExt<StatOptionIncomingJam>),
    // Ship mobility
    Speed(bool),
    Agility(bool),
    AlignTime(bool),
    SigRadius(bool),
    Mass(StatOptionExt<StatOptionMass>),
    WarpSpeed(bool),
    MaxWarpRange(bool),
    Jump(StatOptionExt<StatOptionJumpGen<F>>),
    // Ship misc stats
    DroneControlRange(bool),
    CanWarp(bool),
    CanJumpGate(bool),
    CanJumpWormhole(bool),
    CanJumpDrive(bool),
    CanDockStation(bool),
    CanDockCitadel(bool),
    CanTether(bool),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Construction
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<F, I> FitStatsOptionsGen<F, I> {
    /// True to have all supported stats enabled by default, false to have them disabled.
    pub fn new(default: bool) -> Self {
        Self { default, .. }
    }
    /// True to have all supported stats enabled by default, false to have them disabled.
    pub fn with_override_capacity(default: bool, capacity: usize) -> Self {
        Self {
            default,
            overrides: Vec::with_capacity(capacity),
        }
    }
    // Fit output stats
    pub fn with_dmg(mut self, option: StatOptionExt<StatOptionFitDmgGen<I>>) -> Self {
        self.overrides.push(FitStatOption::Dmg(option.into_internal()));
        self
    }
    pub fn with_mps(mut self, option: StatOptionExt<StatOptionFitMining>) -> Self {
        self.overrides.push(FitStatOption::Mps(option));
        self
    }
    pub fn with_outgoing_nps(mut self, option: StatOptionExt<StatOptionFitOutNpsGen<I>>) -> Self {
        self.overrides.push(FitStatOption::OutgoingNps(option.into_internal()));
        self
    }
    pub fn with_outgoing_rps(mut self, option: StatOptionExt<StatOptionFitOutRpsGen<I>>) -> Self {
        self.overrides.push(FitStatOption::OutgoingRps(option.into_internal()));
        self
    }
    pub fn with_outgoing_cps(mut self, option: StatOptionExt<StatOptionFitOutCpsGen<I>>) -> Self {
        self.overrides.push(FitStatOption::OutgoingCps(option.into_internal()));
        self
    }
    // Fit resources
    pub fn with_cpu(mut self, enabled: bool) -> Self {
        self.overrides.push(FitStatOption::Cpu(enabled));
        self
    }
    pub fn with_powergrid(mut self, enabled: bool) -> Self {
        self.overrides.push(FitStatOption::Powergrid(enabled));
        self
    }
    pub fn with_calibration(mut self, enabled: bool) -> Self {
        self.overrides.push(FitStatOption::Calibration(enabled));
        self
    }
    pub fn with_drone_bay_volume(mut self, enabled: bool) -> Self {
        self.overrides.push(FitStatOption::DroneBayVolume(enabled));
        self
    }
    pub fn with_drone_bandwidth(mut self, enabled: bool) -> Self {
        self.overrides.push(FitStatOption::DroneBandwidth(enabled));
        self
    }
    pub fn with_fighter_bay_volume(mut self, enabled: bool) -> Self {
        self.overrides.push(FitStatOption::FighterBayVolume(enabled));
        self
    }
    // Fit slots
    pub fn with_high_slots(mut self, enabled: bool) -> Self {
        self.overrides.push(FitStatOption::HighSlots(enabled));
        self
    }
    pub fn with_mid_slots(mut self, enabled: bool) -> Self {
        self.overrides.push(FitStatOption::MidSlots(enabled));
        self
    }
    pub fn with_low_slots(mut self, enabled: bool) -> Self {
        self.overrides.push(FitStatOption::LowSlots(enabled));
        self
    }
    pub fn with_turret_slots(mut self, enabled: bool) -> Self {
        self.overrides.push(FitStatOption::TurretSlots(enabled));
        self
    }
    pub fn with_launcher_slots(mut self, enabled: bool) -> Self {
        self.overrides.push(FitStatOption::LauncherSlots(enabled));
        self
    }
    pub fn with_rig_slots(mut self, enabled: bool) -> Self {
        self.overrides.push(FitStatOption::RigSlots(enabled));
        self
    }
    pub fn with_service_slots(mut self, enabled: bool) -> Self {
        self.overrides.push(FitStatOption::ServiceSlots(enabled));
        self
    }
    pub fn with_subsystem_slots(mut self, enabled: bool) -> Self {
        self.overrides.push(FitStatOption::SubsystemSlots(enabled));
        self
    }
    pub fn with_launched_drones(mut self, enabled: bool) -> Self {
        self.overrides.push(FitStatOption::LaunchedDrones(enabled));
        self
    }
    pub fn with_launched_fighters(mut self, enabled: bool) -> Self {
        self.overrides.push(FitStatOption::LaunchedFighters(enabled));
        self
    }
    pub fn with_launched_light_fighters(mut self, enabled: bool) -> Self {
        self.overrides.push(FitStatOption::LaunchedLightFighters(enabled));
        self
    }
    pub fn with_launched_heavy_fighters(mut self, enabled: bool) -> Self {
        self.overrides.push(FitStatOption::LaunchedHeavyFighters(enabled));
        self
    }
    pub fn with_launched_support_fighters(mut self, enabled: bool) -> Self {
        self.overrides.push(FitStatOption::LaunchedSupportFighters(enabled));
        self
    }
    pub fn with_launched_st_light_fighters(mut self, enabled: bool) -> Self {
        self.overrides.push(FitStatOption::LaunchedStLightFighters(enabled));
        self
    }
    pub fn with_launched_st_heavy_fighters(mut self, enabled: bool) -> Self {
        self.overrides.push(FitStatOption::LaunchedStHeavyFighters(enabled));
        self
    }
    pub fn with_launched_st_support_fighters(mut self, enabled: bool) -> Self {
        self.overrides.push(FitStatOption::LaunchedStSupportFighters(enabled));
        self
    }
    // Ship tank
    pub fn with_resists(mut self, enabled: bool) -> Self {
        self.overrides.push(FitStatOption::Resists(enabled));
        self
    }
    pub fn with_hp(mut self, enabled: bool) -> Self {
        self.overrides.push(FitStatOption::Hp(enabled));
        self
    }
    pub fn with_ehp(mut self, option: StatOptionExt<StatOptionEhp>) -> Self {
        self.overrides.push(FitStatOption::Ehp(option));
        self
    }
    pub fn with_wc_ehp(mut self, enabled: bool) -> Self {
        self.overrides.push(FitStatOption::WcEhp(enabled));
        self
    }
    pub fn with_rps(mut self, option: StatOptionExt<StatOptionRps>) -> Self {
        self.overrides.push(FitStatOption::Rps(option));
        self
    }
    pub fn with_erps(mut self, option: StatOptionExt<StatOptionErps>) -> Self {
        self.overrides.push(FitStatOption::Erps(option));
        self
    }
    pub fn with_breach_resist(mut self, enabled: bool) -> Self {
        self.overrides.push(FitStatOption::BreachResist(enabled));
        self
    }
    // Ship cap
    pub fn with_cap_amount(mut self, enabled: bool) -> Self {
        self.overrides.push(FitStatOption::CapAmount(enabled));
        self
    }
    pub fn with_cap_balance(mut self, option: StatOptionExt<StatOptionCapBlcGen<I>>) -> Self {
        self.overrides.push(FitStatOption::CapBalance(option.into_internal()));
        self
    }
    pub fn with_cap_sim(mut self, option: StatOptionExt<StatOptionCapSimGen<I>>) -> Self {
        self.overrides.push(FitStatOption::CapSim(option.into_internal()));
        self
    }
    pub fn with_neut_resist(mut self, enabled: bool) -> Self {
        self.overrides.push(FitStatOption::NeutResist(enabled));
        self
    }
    // Ship sensors
    pub fn with_locks(mut self, enabled: bool) -> Self {
        self.overrides.push(FitStatOption::Locks(enabled));
        self
    }
    pub fn with_lock_range(mut self, enabled: bool) -> Self {
        self.overrides.push(FitStatOption::LockRange(enabled));
        self
    }
    pub fn with_scan_res(mut self, enabled: bool) -> Self {
        self.overrides.push(FitStatOption::ScanRes(enabled));
        self
    }
    pub fn with_sensors(mut self, enabled: bool) -> Self {
        self.overrides.push(FitStatOption::Sensors(enabled));
        self
    }
    pub fn with_dscan_range(mut self, enabled: bool) -> Self {
        self.overrides.push(FitStatOption::DscanRange(enabled));
        self
    }
    pub fn with_probing_size(mut self, enabled: bool) -> Self {
        self.overrides.push(FitStatOption::ProbingSize(enabled));
        self
    }
    pub fn with_incoming_jam(mut self, option: StatOptionExt<StatOptionIncomingJam>) -> Self {
        self.overrides.push(FitStatOption::IncomingJam(option));
        self
    }
    // Ship mobility
    pub fn with_speed(mut self, enabled: bool) -> Self {
        self.overrides.push(FitStatOption::Speed(enabled));
        self
    }
    pub fn with_agility(mut self, enabled: bool) -> Self {
        self.overrides.push(FitStatOption::Agility(enabled));
        self
    }
    pub fn with_align_time(mut self, enabled: bool) -> Self {
        self.overrides.push(FitStatOption::AlignTime(enabled));
        self
    }
    pub fn with_sig_radius(mut self, enabled: bool) -> Self {
        self.overrides.push(FitStatOption::SigRadius(enabled));
        self
    }
    pub fn with_mass(mut self, option: StatOptionExt<StatOptionMass>) -> Self {
        self.overrides.push(FitStatOption::Mass(option));
        self
    }
    pub fn with_warp_speed(mut self, enabled: bool) -> Self {
        self.overrides.push(FitStatOption::WarpSpeed(enabled));
        self
    }
    pub fn with_max_warp_range(mut self, enabled: bool) -> Self {
        self.overrides.push(FitStatOption::MaxWarpRange(enabled));
        self
    }
    pub fn with_jump(mut self, option: StatOptionExt<StatOptionJumpGen<F>>) -> Self {
        self.overrides.push(FitStatOption::Jump(option));
        self
    }
    // Ship misc stats
    pub fn with_drone_control_range(mut self, enabled: bool) -> Self {
        self.overrides.push(FitStatOption::DroneControlRange(enabled));
        self
    }
    pub fn with_can_warp(mut self, enabled: bool) -> Self {
        self.overrides.push(FitStatOption::CanWarp(enabled));
        self
    }
    pub fn with_can_jump_gate(mut self, enabled: bool) -> Self {
        self.overrides.push(FitStatOption::CanJumpGate(enabled));
        self
    }
    pub fn with_can_jump_wormhole(mut self, enabled: bool) -> Self {
        self.overrides.push(FitStatOption::CanJumpWormhole(enabled));
        self
    }
    pub fn with_can_jump_drive(mut self, enabled: bool) -> Self {
        self.overrides.push(FitStatOption::CanJumpDrive(enabled));
        self
    }
    pub fn with_can_dock_station(mut self, enabled: bool) -> Self {
        self.overrides.push(FitStatOption::CanDockStation(enabled));
        self
    }
    pub fn with_can_dock_citadel(mut self, enabled: bool) -> Self {
        self.overrides.push(FitStatOption::CanDockCitadel(enabled));
        self
    }
    pub fn with_can_tether(mut self, enabled: bool) -> Self {
        self.overrides.push(FitStatOption::CanTether(enabled));
        self
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Backref resolution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl BrResolveInfallible for FitStatsOptionsBr {
    type Target = FitStatsOptions;
    fn br_resolve_infallible(self, resps: &CmdResps) -> Self::Target {
        let mut overrides = Vec::with_capacity(self.overrides.len());
        for option in self.overrides.into_iter() {
            overrides.push(match option {
                // Fit output stats
                FitStatOption::Dmg(option) => FitStatOption::Dmg(option.br_resolve_stored(resps)),
                FitStatOption::Mps(option) => FitStatOption::Mps(option),
                FitStatOption::OutgoingNps(option) => FitStatOption::OutgoingNps(option.br_resolve_stored(resps)),
                FitStatOption::OutgoingRps(option) => FitStatOption::OutgoingRps(option.br_resolve_stored(resps)),
                FitStatOption::OutgoingCps(option) => FitStatOption::OutgoingCps(option.br_resolve_stored(resps)),
                // Fit resources
                FitStatOption::Cpu(option) => FitStatOption::Cpu(option),
                FitStatOption::Powergrid(option) => FitStatOption::Powergrid(option),
                FitStatOption::Calibration(option) => FitStatOption::Calibration(option),
                FitStatOption::DroneBayVolume(option) => FitStatOption::DroneBayVolume(option),
                FitStatOption::DroneBandwidth(option) => FitStatOption::DroneBandwidth(option),
                FitStatOption::FighterBayVolume(option) => FitStatOption::FighterBayVolume(option),
                // Fit slots
                FitStatOption::HighSlots(option) => FitStatOption::HighSlots(option),
                FitStatOption::MidSlots(option) => FitStatOption::MidSlots(option),
                FitStatOption::LowSlots(option) => FitStatOption::LowSlots(option),
                FitStatOption::TurretSlots(option) => FitStatOption::TurretSlots(option),
                FitStatOption::LauncherSlots(option) => FitStatOption::LauncherSlots(option),
                FitStatOption::RigSlots(option) => FitStatOption::RigSlots(option),
                FitStatOption::ServiceSlots(option) => FitStatOption::ServiceSlots(option),
                FitStatOption::SubsystemSlots(option) => FitStatOption::SubsystemSlots(option),
                FitStatOption::LaunchedDrones(option) => FitStatOption::LaunchedDrones(option),
                FitStatOption::LaunchedFighters(option) => FitStatOption::LaunchedFighters(option),
                FitStatOption::LaunchedLightFighters(option) => FitStatOption::LaunchedLightFighters(option),
                FitStatOption::LaunchedHeavyFighters(option) => FitStatOption::LaunchedHeavyFighters(option),
                FitStatOption::LaunchedSupportFighters(option) => FitStatOption::LaunchedSupportFighters(option),
                FitStatOption::LaunchedStLightFighters(option) => FitStatOption::LaunchedStLightFighters(option),
                FitStatOption::LaunchedStHeavyFighters(option) => FitStatOption::LaunchedStHeavyFighters(option),
                FitStatOption::LaunchedStSupportFighters(option) => FitStatOption::LaunchedStSupportFighters(option),
                // Ship tank
                FitStatOption::Resists(option) => FitStatOption::Resists(option),
                FitStatOption::Hp(option) => FitStatOption::Hp(option),
                FitStatOption::Ehp(option) => FitStatOption::Ehp(option),
                FitStatOption::WcEhp(option) => FitStatOption::WcEhp(option),
                FitStatOption::Rps(option) => FitStatOption::Rps(option),
                FitStatOption::Erps(option) => FitStatOption::Erps(option),
                FitStatOption::BreachResist(option) => FitStatOption::BreachResist(option),
                // Ship cap
                FitStatOption::CapAmount(option) => FitStatOption::CapAmount(option),
                FitStatOption::CapBalance(option) => FitStatOption::CapBalance(option.br_resolve_stored(resps)),
                FitStatOption::CapSim(option) => FitStatOption::CapSim(option.br_resolve_stored(resps)),
                FitStatOption::NeutResist(option) => FitStatOption::NeutResist(option),
                // Ship sensors
                FitStatOption::Locks(option) => FitStatOption::Locks(option),
                FitStatOption::LockRange(option) => FitStatOption::LockRange(option),
                FitStatOption::ScanRes(option) => FitStatOption::ScanRes(option),
                FitStatOption::Sensors(option) => FitStatOption::Sensors(option),
                FitStatOption::DscanRange(option) => FitStatOption::DscanRange(option),
                FitStatOption::ProbingSize(option) => FitStatOption::ProbingSize(option),
                // Ship mobility
                FitStatOption::IncomingJam(option) => FitStatOption::IncomingJam(option),
                FitStatOption::Speed(option) => FitStatOption::Speed(option),
                FitStatOption::Agility(option) => FitStatOption::Agility(option),
                FitStatOption::AlignTime(option) => FitStatOption::AlignTime(option),
                FitStatOption::SigRadius(option) => FitStatOption::SigRadius(option),
                FitStatOption::Mass(option) => FitStatOption::Mass(option),
                FitStatOption::WarpSpeed(option) => FitStatOption::WarpSpeed(option),
                FitStatOption::MaxWarpRange(option) => FitStatOption::MaxWarpRange(option),
                FitStatOption::Jump(option) => FitStatOption::Jump(option.br_resolve_infallible(resps)),
                // Ship misc stats
                FitStatOption::DroneControlRange(option) => FitStatOption::DroneControlRange(option),
                FitStatOption::CanWarp(option) => FitStatOption::CanWarp(option),
                FitStatOption::CanJumpGate(option) => FitStatOption::CanJumpGate(option),
                FitStatOption::CanJumpWormhole(option) => FitStatOption::CanJumpWormhole(option),
                FitStatOption::CanJumpDrive(option) => FitStatOption::CanJumpDrive(option),
                FitStatOption::CanDockStation(option) => FitStatOption::CanDockStation(option),
                FitStatOption::CanDockCitadel(option) => FitStatOption::CanDockCitadel(option),
                FitStatOption::CanTether(option) => FitStatOption::CanTether(option),
            });
        }
        Self::Target {
            default: self.default,
            overrides,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Default + stat resolution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl FitStatsOptions {
    pub(super) fn stat_resolve(self) -> FitStatsOptionsResolved {
        let mut resolved = FitStatsOptionsResolved::blank_from_default(self.default);
        for option in self.overrides.into_iter() {
            match option {
                // Fit output stats
                FitStatOption::Dmg(option) => resolved.dmg = option.into_resolved(),
                FitStatOption::Mps(option) => resolved.mps = option.into_resolved(),
                FitStatOption::OutgoingNps(option) => resolved.outgoing_nps = option.into_resolved(),
                FitStatOption::OutgoingRps(option) => resolved.outgoing_rps = option.into_resolved(),
                FitStatOption::OutgoingCps(option) => resolved.outgoing_cps = option.into_resolved(),
                // Fit resources
                FitStatOption::Cpu(option) => resolved.cpu = option,
                FitStatOption::Powergrid(option) => resolved.powergrid = option,
                FitStatOption::Calibration(option) => resolved.calibration = option,
                FitStatOption::DroneBayVolume(option) => resolved.drone_bay_volume = option,
                FitStatOption::DroneBandwidth(option) => resolved.drone_bandwidth = option,
                FitStatOption::FighterBayVolume(option) => resolved.fighter_bay_volume = option,
                // Fit slots
                FitStatOption::HighSlots(option) => resolved.high_slots = option,
                FitStatOption::MidSlots(option) => resolved.mid_slots = option,
                FitStatOption::LowSlots(option) => resolved.low_slots = option,
                FitStatOption::TurretSlots(option) => resolved.turret_slots = option,
                FitStatOption::LauncherSlots(option) => resolved.launcher_slots = option,
                FitStatOption::RigSlots(option) => resolved.rig_slots = option,
                FitStatOption::ServiceSlots(option) => resolved.service_slots = option,
                FitStatOption::SubsystemSlots(option) => resolved.subsystem_slots = option,
                FitStatOption::LaunchedDrones(option) => resolved.launched_drones = option,
                FitStatOption::LaunchedFighters(option) => resolved.launched_fighters = option,
                FitStatOption::LaunchedLightFighters(option) => resolved.launched_light_fighters = option,
                FitStatOption::LaunchedHeavyFighters(option) => resolved.launched_heavy_fighters = option,
                FitStatOption::LaunchedSupportFighters(option) => resolved.launched_support_fighters = option,
                FitStatOption::LaunchedStLightFighters(option) => resolved.launched_st_light_fighters = option,
                FitStatOption::LaunchedStHeavyFighters(option) => resolved.launched_st_heavy_fighters = option,
                FitStatOption::LaunchedStSupportFighters(option) => resolved.launched_st_support_fighters = option,
                // Ship tank
                FitStatOption::Resists(option) => resolved.resists = option,
                FitStatOption::Hp(option) => resolved.hp = option,
                FitStatOption::Ehp(option) => resolved.ehp = option.into_resolved(),
                FitStatOption::WcEhp(option) => resolved.wc_ehp = option,
                FitStatOption::Rps(option) => resolved.rps = option.into_resolved(),
                FitStatOption::Erps(option) => resolved.erps = option.into_resolved(),
                FitStatOption::BreachResist(option) => resolved.breach_resist = option,
                // Ship cap
                FitStatOption::CapAmount(option) => resolved.cap_amount = option,
                FitStatOption::CapBalance(option) => resolved.cap_balance = option.into_resolved(),
                FitStatOption::CapSim(option) => resolved.cap_sim = option.into_resolved(),
                FitStatOption::NeutResist(option) => resolved.neut_resist = option,
                // Ship sensors
                FitStatOption::Locks(option) => resolved.locks = option,
                FitStatOption::LockRange(option) => resolved.lock_range = option,
                FitStatOption::ScanRes(option) => resolved.scan_res = option,
                FitStatOption::Sensors(option) => resolved.sensors = option,
                FitStatOption::DscanRange(option) => resolved.dscan_range = option,
                FitStatOption::ProbingSize(option) => resolved.probing_size = option,
                FitStatOption::IncomingJam(option) => resolved.incoming_jam = option.into_resolved(),
                // Ship mobility
                FitStatOption::Speed(option) => resolved.speed = option,
                FitStatOption::Agility(option) => resolved.agility = option,
                FitStatOption::AlignTime(option) => resolved.align_time = option,
                FitStatOption::SigRadius(option) => resolved.sig_radius = option,
                FitStatOption::Mass(option) => resolved.mass = option.into_resolved(),
                FitStatOption::WarpSpeed(option) => resolved.warp_speed = option,
                FitStatOption::MaxWarpRange(option) => resolved.max_warp_range = option,
                FitStatOption::Jump(option) => resolved.jump = option.into_resolved(),
                // Ship misc stats
                FitStatOption::DroneControlRange(option) => resolved.drone_control_range = option,
                FitStatOption::CanWarp(option) => resolved.can_warp = option,
                FitStatOption::CanJumpGate(option) => resolved.can_jump_gate = option,
                FitStatOption::CanJumpWormhole(option) => resolved.can_jump_wormhole = option,
                FitStatOption::CanJumpDrive(option) => resolved.can_jump_drive = option,
                FitStatOption::CanDockStation(option) => resolved.can_dock_station = option,
                FitStatOption::CanDockCitadel(option) => resolved.can_dock_citadel = option,
                FitStatOption::CanTether(option) => resolved.can_tether = option,
            }
        }
        resolved.complete_extended_defaults();
        resolved
    }
}

impl From<FitStatsOptions> for FitStatsOptionsResolved {
    fn from(value: FitStatsOptions) -> Self {
        value.stat_resolve()
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Custom de/serialization
////////////////////////////////////////////////////////////////////////////////////////////////////
#[cfg(feature = "serde")]
mod custom_serde {
    use std::marker::PhantomData;

    use serde::de::{Deserialize, Deserializer, IgnoredAny, MapAccess, Visitor};

    use super::*;

    impl<'de, F, I> Deserialize<'de> for FitStatsOptionsGen<F, I>
    where
        F: Deserialize<'de>,
        I: Deserialize<'de>,
    {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            deserializer.deserialize_map(VisitorImpl(PhantomData))
        }
    }

    #[derive(serde::Deserialize)]
    #[serde(field_identifier, rename_all = "snake_case")]
    enum Key {
        Default,
        // Fit output stats
        Dmg,
        Mps,
        OutgoingNps,
        OutgoingRps,
        OutgoingCps,
        // Fit resources
        Cpu,
        Powergrid,
        Calibration,
        DroneBayVolume,
        DroneBandwidth,
        FighterBayVolume,
        // Fit slots
        HighSlots,
        MidSlots,
        LowSlots,
        TurretSlots,
        LauncherSlots,
        RigSlots,
        ServiceSlots,
        SubsystemSlots,
        LaunchedDrones,
        LaunchedFighters,
        LaunchedLightFighters,
        LaunchedHeavyFighters,
        LaunchedSupportFighters,
        LaunchedStLightFighters,
        LaunchedStHeavyFighters,
        LaunchedStSupportFighters,
        // Ship tank
        Resists,
        Hp,
        Ehp,
        WcEhp,
        Rps,
        Erps,
        BreachResist,
        // Ship cap
        CapAmount,
        CapBalance,
        CapSim,
        NeutResist,
        // Ship sensors
        Locks,
        LockRange,
        ScanRes,
        Sensors,
        DscanRange,
        ProbingSize,
        IncomingJam,
        // Ship mobility
        Speed,
        Agility,
        AlignTime,
        SigRadius,
        Mass,
        WarpSpeed,
        MaxWarpRange,
        Jump,
        // Ship misc stats
        DroneControlRange,
        CanWarp,
        CanJumpGate,
        CanJumpWormhole,
        CanJumpDrive,
        CanDockStation,
        CanDockCitadel,
        CanTether,
        #[serde(other)]
        Unknown,
    }

    struct VisitorImpl<F, I>(PhantomData<(F, I)>);
    impl<'de, F, I> Visitor<'de> for VisitorImpl<F, I>
    where
        F: Deserialize<'de>,
        I: Deserialize<'de>,
    {
        type Value = FitStatsOptionsGen<F, I>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("struct FitStatsOptions")
        }

        fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
        where
            A: MapAccess<'de>,
        {
            let mut options = Self::Value::default();
            let overrides = &mut options.overrides;
            while let Some(key) = map.next_key::<Key>()? {
                match key {
                    Key::Default => options.default = map.next_value()?,
                    // Fit output stats
                    Key::Dmg => {
                        overrides.extend(map.next_value::<Option<_>>()?.map(FitStatOption::Dmg));
                    }
                    Key::Mps => {
                        overrides.extend(map.next_value::<Option<_>>()?.map(FitStatOption::Mps));
                    }
                    Key::OutgoingNps => {
                        overrides.extend(map.next_value::<Option<_>>()?.map(FitStatOption::OutgoingNps));
                    }
                    Key::OutgoingRps => {
                        overrides.extend(map.next_value::<Option<_>>()?.map(FitStatOption::OutgoingRps));
                    }
                    Key::OutgoingCps => {
                        overrides.extend(map.next_value::<Option<_>>()?.map(FitStatOption::OutgoingCps));
                    }
                    // Fit resources
                    Key::Cpu => {
                        overrides.extend(map.next_value::<Option<_>>()?.map(FitStatOption::Cpu));
                    }
                    Key::Powergrid => {
                        overrides.extend(map.next_value::<Option<_>>()?.map(FitStatOption::Powergrid));
                    }
                    Key::Calibration => {
                        overrides.extend(map.next_value::<Option<_>>()?.map(FitStatOption::Calibration));
                    }
                    Key::DroneBayVolume => {
                        overrides.extend(map.next_value::<Option<_>>()?.map(FitStatOption::DroneBayVolume));
                    }
                    Key::DroneBandwidth => {
                        overrides.extend(map.next_value::<Option<_>>()?.map(FitStatOption::DroneBandwidth));
                    }
                    Key::FighterBayVolume => {
                        overrides.extend(map.next_value::<Option<_>>()?.map(FitStatOption::FighterBayVolume));
                    }
                    // Fit slots
                    Key::HighSlots => {
                        overrides.extend(map.next_value::<Option<_>>()?.map(FitStatOption::HighSlots));
                    }
                    Key::MidSlots => {
                        overrides.extend(map.next_value::<Option<_>>()?.map(FitStatOption::MidSlots));
                    }
                    Key::LowSlots => {
                        overrides.extend(map.next_value::<Option<_>>()?.map(FitStatOption::LowSlots));
                    }
                    Key::TurretSlots => {
                        overrides.extend(map.next_value::<Option<_>>()?.map(FitStatOption::TurretSlots));
                    }
                    Key::LauncherSlots => {
                        overrides.extend(map.next_value::<Option<_>>()?.map(FitStatOption::LauncherSlots));
                    }
                    Key::RigSlots => {
                        overrides.extend(map.next_value::<Option<_>>()?.map(FitStatOption::RigSlots));
                    }
                    Key::ServiceSlots => {
                        overrides.extend(map.next_value::<Option<_>>()?.map(FitStatOption::ServiceSlots));
                    }
                    Key::SubsystemSlots => {
                        overrides.extend(map.next_value::<Option<_>>()?.map(FitStatOption::SubsystemSlots));
                    }
                    Key::LaunchedDrones => {
                        overrides.extend(map.next_value::<Option<_>>()?.map(FitStatOption::LaunchedDrones));
                    }
                    Key::LaunchedFighters => {
                        overrides.extend(map.next_value::<Option<_>>()?.map(FitStatOption::LaunchedFighters));
                    }
                    Key::LaunchedLightFighters => {
                        overrides.extend(map.next_value::<Option<_>>()?.map(FitStatOption::LaunchedLightFighters));
                    }
                    Key::LaunchedHeavyFighters => {
                        overrides.extend(map.next_value::<Option<_>>()?.map(FitStatOption::LaunchedHeavyFighters));
                    }
                    Key::LaunchedSupportFighters => {
                        overrides.extend(
                            map.next_value::<Option<_>>()?
                                .map(FitStatOption::LaunchedSupportFighters),
                        );
                    }
                    Key::LaunchedStLightFighters => {
                        overrides.extend(
                            map.next_value::<Option<_>>()?
                                .map(FitStatOption::LaunchedStLightFighters),
                        );
                    }
                    Key::LaunchedStHeavyFighters => {
                        overrides.extend(
                            map.next_value::<Option<_>>()?
                                .map(FitStatOption::LaunchedStHeavyFighters),
                        );
                    }
                    Key::LaunchedStSupportFighters => {
                        overrides.extend(
                            map.next_value::<Option<_>>()?
                                .map(FitStatOption::LaunchedStSupportFighters),
                        );
                    }
                    // Ship tank
                    Key::Resists => {
                        overrides.extend(map.next_value::<Option<_>>()?.map(FitStatOption::Resists));
                    }
                    Key::Hp => {
                        overrides.extend(map.next_value::<Option<_>>()?.map(FitStatOption::Hp));
                    }
                    Key::Ehp => {
                        overrides.extend(map.next_value::<Option<_>>()?.map(FitStatOption::Ehp));
                    }
                    Key::WcEhp => {
                        overrides.extend(map.next_value::<Option<_>>()?.map(FitStatOption::WcEhp));
                    }
                    Key::Rps => {
                        overrides.extend(map.next_value::<Option<_>>()?.map(FitStatOption::Rps));
                    }
                    Key::Erps => {
                        overrides.extend(map.next_value::<Option<_>>()?.map(FitStatOption::Erps));
                    }
                    Key::BreachResist => {
                        overrides.extend(map.next_value::<Option<_>>()?.map(FitStatOption::BreachResist));
                    }
                    // Ship cap
                    Key::CapAmount => {
                        overrides.extend(map.next_value::<Option<_>>()?.map(FitStatOption::CapAmount));
                    }
                    Key::CapBalance => {
                        overrides.extend(map.next_value::<Option<_>>()?.map(FitStatOption::CapBalance));
                    }
                    Key::CapSim => {
                        overrides.extend(map.next_value::<Option<_>>()?.map(FitStatOption::CapSim));
                    }
                    Key::NeutResist => {
                        overrides.extend(map.next_value::<Option<_>>()?.map(FitStatOption::NeutResist));
                    }
                    // Ship sensors
                    Key::Locks => {
                        overrides.extend(map.next_value::<Option<_>>()?.map(FitStatOption::Locks));
                    }
                    Key::LockRange => {
                        overrides.extend(map.next_value::<Option<_>>()?.map(FitStatOption::LockRange));
                    }
                    Key::ScanRes => {
                        overrides.extend(map.next_value::<Option<_>>()?.map(FitStatOption::ScanRes));
                    }
                    Key::Sensors => {
                        overrides.extend(map.next_value::<Option<_>>()?.map(FitStatOption::Sensors));
                    }
                    Key::DscanRange => {
                        overrides.extend(map.next_value::<Option<_>>()?.map(FitStatOption::DscanRange));
                    }
                    Key::ProbingSize => {
                        overrides.extend(map.next_value::<Option<_>>()?.map(FitStatOption::ProbingSize));
                    }
                    Key::IncomingJam => {
                        overrides.extend(map.next_value::<Option<_>>()?.map(FitStatOption::IncomingJam));
                    }
                    // Ship mobility
                    Key::Speed => {
                        overrides.extend(map.next_value::<Option<_>>()?.map(FitStatOption::Speed));
                    }
                    Key::Agility => {
                        overrides.extend(map.next_value::<Option<_>>()?.map(FitStatOption::Agility));
                    }
                    Key::AlignTime => {
                        overrides.extend(map.next_value::<Option<_>>()?.map(FitStatOption::AlignTime));
                    }
                    Key::SigRadius => {
                        overrides.extend(map.next_value::<Option<_>>()?.map(FitStatOption::SigRadius));
                    }
                    Key::Mass => {
                        overrides.extend(map.next_value::<Option<_>>()?.map(FitStatOption::Mass));
                    }
                    Key::WarpSpeed => {
                        overrides.extend(map.next_value::<Option<_>>()?.map(FitStatOption::WarpSpeed));
                    }
                    Key::MaxWarpRange => {
                        overrides.extend(map.next_value::<Option<_>>()?.map(FitStatOption::MaxWarpRange));
                    }
                    Key::Jump => {
                        overrides.extend(map.next_value::<Option<_>>()?.map(FitStatOption::Jump));
                    }
                    // Ship misc stats
                    Key::DroneControlRange => {
                        overrides.extend(map.next_value::<Option<_>>()?.map(FitStatOption::DroneControlRange));
                    }
                    Key::CanWarp => {
                        overrides.extend(map.next_value::<Option<_>>()?.map(FitStatOption::CanWarp));
                    }
                    Key::CanJumpGate => {
                        overrides.extend(map.next_value::<Option<_>>()?.map(FitStatOption::CanJumpGate));
                    }
                    Key::CanJumpWormhole => {
                        overrides.extend(map.next_value::<Option<_>>()?.map(FitStatOption::CanJumpWormhole));
                    }
                    Key::CanJumpDrive => {
                        overrides.extend(map.next_value::<Option<_>>()?.map(FitStatOption::CanJumpDrive));
                    }
                    Key::CanDockStation => {
                        overrides.extend(map.next_value::<Option<_>>()?.map(FitStatOption::CanDockStation));
                    }
                    Key::CanDockCitadel => {
                        overrides.extend(map.next_value::<Option<_>>()?.map(FitStatOption::CanDockCitadel));
                    }
                    Key::CanTether => {
                        overrides.extend(map.next_value::<Option<_>>()?.map(FitStatOption::CanTether));
                    }
                    Key::Unknown => {
                        map.next_value::<IgnoredAny>()?;
                    }
                }
            }
            Ok(options)
        }
    }
}
