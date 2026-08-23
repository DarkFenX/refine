use crate::{
    CmdResps, FitId, FitIdBr, ItemId, ItemIdBr,
    err::BrResolveError,
    shared::BrResolvable,
    stats::{
        StatOptionCapBlc, StatOptionCapSim, StatOptionEhp, StatOptionErps, StatOptionExt, StatOptionIncomingJam,
        StatOptionItemDmg, StatOptionItemMining, StatOptionItemOutCps, StatOptionItemOutNps, StatOptionItemOutRps,
        StatOptionJump, StatOptionMass, StatOptionRps, item::ItemStatsOptionsResolved,
    },
};


/// Which stats to fetch for an item.
///
/// By default, all stats are not fetched.
#[derive(Clone)]
pub struct ItemStatsOptions<F = FitId, I = ItemId> {
    default: bool = false,
    overrides: Vec<ItemStatOption<F, I>> = Vec::new(),
}
impl<F, I> Default for ItemStatsOptions<F, I> {
    fn default() -> Self {
        Self { .. }
    }
}

pub type ItemStatsOptionsBr = ItemStatsOptions<FitIdBr, ItemIdBr>;

#[derive(Clone)]
enum ItemStatOption<F, I> {
    // Output
    Dmg(StatOptionExt<StatOptionItemDmg<I>>),
    Mps(StatOptionExt<StatOptionItemMining>),
    OutgoingNps(StatOptionExt<StatOptionItemOutNps<I>>),
    OutgoingRps(StatOptionExt<StatOptionItemOutRps<I>>),
    OutgoingCps(StatOptionExt<StatOptionItemOutCps<I>>),
    // Tank
    Resists(bool),
    Hp(bool),
    Ehp(StatOptionExt<StatOptionEhp>),
    WcEhp(bool),
    Rps(StatOptionExt<StatOptionRps>),
    Erps(StatOptionExt<StatOptionErps>),
    BreachResist(bool),
    // Cap
    CapAmount(bool),
    CapBalance(StatOptionExt<StatOptionCapBlc<I>>),
    CapSim(StatOptionExt<StatOptionCapSim<I>>),
    NeutResist(bool),
    // Sensors
    Locks(bool),
    LockRange(bool),
    ScanRes(bool),
    Sensors(bool),
    DscanRange(bool),
    ProbingSize(bool),
    IncomingJam(StatOptionExt<StatOptionIncomingJam>),
    // Mobility
    Speed(bool),
    Agility(bool),
    AlignTime(bool),
    SigRadius(bool),
    Mass(StatOptionExt<StatOptionMass>),
    WarpSpeed(bool),
    MaxWarpRange(bool),
    Jump(StatOptionExt<StatOptionJump<F>>),
    // Misc
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
impl<F, I> ItemStatsOptions<F, I> {
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
    pub fn with_dmg(mut self, option: StatOptionExt<StatOptionItemDmg<I>>) -> Self {
        self.overrides.push(ItemStatOption::Dmg(option));
        self
    }
    pub fn with_mps(mut self, option: StatOptionExt<StatOptionItemMining>) -> Self {
        self.overrides.push(ItemStatOption::Mps(option));
        self
    }
    pub fn with_outgoing_nps(mut self, option: StatOptionExt<StatOptionItemOutNps<I>>) -> Self {
        self.overrides.push(ItemStatOption::OutgoingNps(option));
        self
    }
    pub fn with_outgoing_rps(mut self, option: StatOptionExt<StatOptionItemOutRps<I>>) -> Self {
        self.overrides.push(ItemStatOption::OutgoingRps(option));
        self
    }
    pub fn with_outgoing_cps(mut self, option: StatOptionExt<StatOptionItemOutCps<I>>) -> Self {
        self.overrides.push(ItemStatOption::OutgoingCps(option));
        self
    }
    pub fn with_resists(mut self, enabled: bool) -> Self {
        self.overrides.push(ItemStatOption::Resists(enabled));
        self
    }
    pub fn with_hp(mut self, enabled: bool) -> Self {
        self.overrides.push(ItemStatOption::Hp(enabled));
        self
    }
    pub fn with_ehp(mut self, option: StatOptionExt<StatOptionEhp>) -> Self {
        self.overrides.push(ItemStatOption::Ehp(option));
        self
    }
    pub fn with_wc_ehp(mut self, enabled: bool) -> Self {
        self.overrides.push(ItemStatOption::WcEhp(enabled));
        self
    }
    pub fn with_rps(mut self, option: StatOptionExt<StatOptionRps>) -> Self {
        self.overrides.push(ItemStatOption::Rps(option));
        self
    }
    pub fn with_erps(mut self, option: StatOptionExt<StatOptionErps>) -> Self {
        self.overrides.push(ItemStatOption::Erps(option));
        self
    }
    pub fn with_breach_resist(mut self, enabled: bool) -> Self {
        self.overrides.push(ItemStatOption::BreachResist(enabled));
        self
    }
    pub fn with_cap_amount(mut self, enabled: bool) -> Self {
        self.overrides.push(ItemStatOption::CapAmount(enabled));
        self
    }
    pub fn with_cap_balance(mut self, option: StatOptionExt<StatOptionCapBlc<I>>) -> Self {
        self.overrides.push(ItemStatOption::CapBalance(option));
        self
    }
    pub fn with_cap_sim(mut self, option: StatOptionExt<StatOptionCapSim<I>>) -> Self {
        self.overrides.push(ItemStatOption::CapSim(option));
        self
    }
    pub fn with_neut_resist(mut self, enabled: bool) -> Self {
        self.overrides.push(ItemStatOption::NeutResist(enabled));
        self
    }
    pub fn with_locks(mut self, enabled: bool) -> Self {
        self.overrides.push(ItemStatOption::Locks(enabled));
        self
    }
    pub fn with_lock_range(mut self, enabled: bool) -> Self {
        self.overrides.push(ItemStatOption::LockRange(enabled));
        self
    }
    pub fn with_scan_res(mut self, enabled: bool) -> Self {
        self.overrides.push(ItemStatOption::ScanRes(enabled));
        self
    }
    pub fn with_sensors(mut self, enabled: bool) -> Self {
        self.overrides.push(ItemStatOption::Sensors(enabled));
        self
    }
    pub fn with_dscan_range(mut self, enabled: bool) -> Self {
        self.overrides.push(ItemStatOption::DscanRange(enabled));
        self
    }
    pub fn with_probing_size(mut self, enabled: bool) -> Self {
        self.overrides.push(ItemStatOption::ProbingSize(enabled));
        self
    }
    pub fn with_incoming_jam(mut self, option: StatOptionExt<StatOptionIncomingJam>) -> Self {
        self.overrides.push(ItemStatOption::IncomingJam(option));
        self
    }
    pub fn with_speed(mut self, enabled: bool) -> Self {
        self.overrides.push(ItemStatOption::Speed(enabled));
        self
    }
    pub fn with_agility(mut self, enabled: bool) -> Self {
        self.overrides.push(ItemStatOption::Agility(enabled));
        self
    }
    pub fn with_align_time(mut self, enabled: bool) -> Self {
        self.overrides.push(ItemStatOption::AlignTime(enabled));
        self
    }
    pub fn with_sig_radius(mut self, enabled: bool) -> Self {
        self.overrides.push(ItemStatOption::SigRadius(enabled));
        self
    }
    pub fn with_mass(mut self, option: StatOptionExt<StatOptionMass>) -> Self {
        self.overrides.push(ItemStatOption::Mass(option));
        self
    }
    pub fn with_warp_speed(mut self, enabled: bool) -> Self {
        self.overrides.push(ItemStatOption::WarpSpeed(enabled));
        self
    }
    pub fn with_max_warp_range(mut self, enabled: bool) -> Self {
        self.overrides.push(ItemStatOption::MaxWarpRange(enabled));
        self
    }
    pub fn with_jump(mut self, option: StatOptionExt<StatOptionJump<F>>) -> Self {
        self.overrides.push(ItemStatOption::Jump(option));
        self
    }
    pub fn with_drone_control_range(mut self, enabled: bool) -> Self {
        self.overrides.push(ItemStatOption::DroneControlRange(enabled));
        self
    }
    pub fn with_can_warp(mut self, enabled: bool) -> Self {
        self.overrides.push(ItemStatOption::CanWarp(enabled));
        self
    }
    pub fn with_can_jump_gate(mut self, enabled: bool) -> Self {
        self.overrides.push(ItemStatOption::CanJumpGate(enabled));
        self
    }
    pub fn with_can_jump_wormhole(mut self, enabled: bool) -> Self {
        self.overrides.push(ItemStatOption::CanJumpWormhole(enabled));
        self
    }
    pub fn with_can_jump_drive(mut self, enabled: bool) -> Self {
        self.overrides.push(ItemStatOption::CanJumpDrive(enabled));
        self
    }
    pub fn with_can_dock_station(mut self, enabled: bool) -> Self {
        self.overrides.push(ItemStatOption::CanDockStation(enabled));
        self
    }
    pub fn with_can_dock_citadel(mut self, enabled: bool) -> Self {
        self.overrides.push(ItemStatOption::CanDockCitadel(enabled));
        self
    }
    pub fn with_can_tether(mut self, enabled: bool) -> Self {
        self.overrides.push(ItemStatOption::CanTether(enabled));
        self
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Backref resolution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl BrResolvable for ItemStatsOptionsBr {
    type Target = ItemStatsOptions;
    fn br_resolve(self, resps: &CmdResps) -> Result<Self::Target, BrResolveError> {
        let mut overrides = Vec::with_capacity(self.overrides.len());
        for option in self.overrides.into_iter() {
            overrides.push(match option {
                // Output
                ItemStatOption::Dmg(option) => ItemStatOption::Dmg(option.br_resolve(resps)?),
                ItemStatOption::Mps(option) => ItemStatOption::Mps(option),
                ItemStatOption::OutgoingNps(option) => ItemStatOption::OutgoingNps(option.br_resolve(resps)?),
                ItemStatOption::OutgoingRps(option) => ItemStatOption::OutgoingRps(option.br_resolve(resps)?),
                ItemStatOption::OutgoingCps(option) => ItemStatOption::OutgoingCps(option.br_resolve(resps)?),
                // Tank
                ItemStatOption::Resists(option) => ItemStatOption::Resists(option),
                ItemStatOption::Hp(option) => ItemStatOption::Hp(option),
                ItemStatOption::Ehp(option) => ItemStatOption::Ehp(option),
                ItemStatOption::WcEhp(option) => ItemStatOption::WcEhp(option),
                ItemStatOption::Rps(option) => ItemStatOption::Rps(option),
                ItemStatOption::Erps(option) => ItemStatOption::Erps(option),
                ItemStatOption::BreachResist(option) => ItemStatOption::BreachResist(option),
                // Cap
                ItemStatOption::CapAmount(option) => ItemStatOption::CapAmount(option),
                ItemStatOption::CapBalance(option) => ItemStatOption::CapBalance(option.br_resolve(resps)?),
                ItemStatOption::CapSim(option) => ItemStatOption::CapSim(option.br_resolve(resps)?),
                ItemStatOption::NeutResist(option) => ItemStatOption::NeutResist(option),
                // Sensors
                ItemStatOption::Locks(option) => ItemStatOption::Locks(option),
                ItemStatOption::LockRange(option) => ItemStatOption::LockRange(option),
                ItemStatOption::ScanRes(option) => ItemStatOption::ScanRes(option),
                ItemStatOption::Sensors(option) => ItemStatOption::Sensors(option),
                ItemStatOption::DscanRange(option) => ItemStatOption::DscanRange(option),
                ItemStatOption::ProbingSize(option) => ItemStatOption::ProbingSize(option),
                ItemStatOption::IncomingJam(option) => ItemStatOption::IncomingJam(option),
                // Mobility
                ItemStatOption::Speed(option) => ItemStatOption::Speed(option),
                ItemStatOption::Agility(option) => ItemStatOption::Agility(option),
                ItemStatOption::AlignTime(option) => ItemStatOption::AlignTime(option),
                ItemStatOption::SigRadius(option) => ItemStatOption::SigRadius(option),
                ItemStatOption::Mass(option) => ItemStatOption::Mass(option),
                ItemStatOption::WarpSpeed(option) => ItemStatOption::WarpSpeed(option),
                ItemStatOption::MaxWarpRange(option) => ItemStatOption::MaxWarpRange(option),
                ItemStatOption::Jump(option) => ItemStatOption::Jump(option.br_resolve(resps)?),
                // Misc
                ItemStatOption::DroneControlRange(option) => ItemStatOption::DroneControlRange(option),
                ItemStatOption::CanWarp(option) => ItemStatOption::CanWarp(option),
                ItemStatOption::CanJumpGate(option) => ItemStatOption::CanJumpGate(option),
                ItemStatOption::CanJumpWormhole(option) => ItemStatOption::CanJumpWormhole(option),
                ItemStatOption::CanJumpDrive(option) => ItemStatOption::CanJumpDrive(option),
                ItemStatOption::CanDockStation(option) => ItemStatOption::CanDockStation(option),
                ItemStatOption::CanDockCitadel(option) => ItemStatOption::CanDockCitadel(option),
                ItemStatOption::CanTether(option) => ItemStatOption::CanTether(option),
            });
        }
        Ok(Self::Target {
            default: self.default,
            overrides,
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Default + stat resolution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ItemStatsOptions {
    pub(super) fn stat_resolve(self) -> ItemStatsOptionsResolved {
        let mut resolved = ItemStatsOptionsResolved::from_default(self.default);
        for option in self.overrides.into_iter() {
            match option {
                // Output
                ItemStatOption::Dmg(option) => resolved.dmg = option.stat_resolve(),
                ItemStatOption::Mps(option) => resolved.mps = option.stat_resolve(),
                ItemStatOption::OutgoingNps(option) => resolved.outgoing_nps = option.stat_resolve(),
                ItemStatOption::OutgoingRps(option) => resolved.outgoing_rps = option.stat_resolve(),
                ItemStatOption::OutgoingCps(option) => resolved.outgoing_cps = option.stat_resolve(),
                // Tank
                ItemStatOption::Resists(option) => resolved.resists = option,
                ItemStatOption::Hp(option) => resolved.hp = option,
                ItemStatOption::Ehp(option) => resolved.ehp = option.stat_resolve(),
                ItemStatOption::WcEhp(option) => resolved.wc_ehp = option,
                ItemStatOption::Rps(option) => resolved.rps = option.stat_resolve(),
                ItemStatOption::Erps(option) => resolved.erps = option.stat_resolve(),
                ItemStatOption::BreachResist(option) => resolved.breach_resist = option,
                // Cap
                ItemStatOption::CapAmount(option) => resolved.cap_amount = option,
                ItemStatOption::CapBalance(option) => resolved.cap_balance = option.stat_resolve(),
                ItemStatOption::CapSim(option) => resolved.cap_sim = option.stat_resolve(),
                ItemStatOption::NeutResist(option) => resolved.neut_resist = option,
                // Sensors
                ItemStatOption::Locks(option) => resolved.locks = option,
                ItemStatOption::LockRange(option) => resolved.lock_range = option,
                ItemStatOption::ScanRes(option) => resolved.scan_res = option,
                ItemStatOption::Sensors(option) => resolved.sensors = option,
                ItemStatOption::DscanRange(option) => resolved.dscan_range = option,
                ItemStatOption::ProbingSize(option) => resolved.probing_size = option,
                ItemStatOption::IncomingJam(option) => resolved.incoming_jam = option.stat_resolve(),
                // Mobility
                ItemStatOption::Speed(option) => resolved.speed = option,
                ItemStatOption::Agility(option) => resolved.agility = option,
                ItemStatOption::AlignTime(option) => resolved.align_time = option,
                ItemStatOption::SigRadius(option) => resolved.sig_radius = option,
                ItemStatOption::Mass(option) => resolved.mass = option.stat_resolve(),
                ItemStatOption::WarpSpeed(option) => resolved.warp_speed = option,
                ItemStatOption::MaxWarpRange(option) => resolved.max_warp_range = option,
                ItemStatOption::Jump(option) => resolved.jump = option.stat_resolve(),
                // Misc
                ItemStatOption::DroneControlRange(option) => resolved.drone_control_range = option,
                ItemStatOption::CanWarp(option) => resolved.can_warp = option,
                ItemStatOption::CanJumpGate(option) => resolved.can_jump_gate = option,
                ItemStatOption::CanJumpWormhole(option) => resolved.can_jump_wormhole = option,
                ItemStatOption::CanJumpDrive(option) => resolved.can_jump_drive = option,
                ItemStatOption::CanDockStation(option) => resolved.can_dock_station = option,
                ItemStatOption::CanDockCitadel(option) => resolved.can_dock_citadel = option,
                ItemStatOption::CanTether(option) => resolved.can_tether = option,
            }
        }
        resolved
    }
}

impl From<ItemStatsOptions<FitId, ItemId>> for ItemStatsOptionsResolved {
    fn from(value: ItemStatsOptions) -> Self {
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

    impl<'de, F, I> Deserialize<'de> for ItemStatsOptions<F, I>
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
        // Output
        Dmg,
        Mps,
        OutgoingNps,
        OutgoingRps,
        OutgoingCps,
        // Tank
        Resists,
        Hp,
        Ehp,
        WcEhp,
        Rps,
        Erps,
        BreachResist,
        // Cap
        CapAmount,
        CapBalance,
        CapSim,
        NeutResist,
        // Sensors
        Locks,
        LockRange,
        ScanRes,
        Sensors,
        DscanRange,
        ProbingSize,
        IncomingJam,
        // Mobility
        Speed,
        Agility,
        AlignTime,
        SigRadius,
        Mass,
        WarpSpeed,
        MaxWarpRange,
        Jump,
        // Misc
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
        type Value = ItemStatsOptions<F, I>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("struct ItemStatsOptions")
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
                    // Output
                    Key::Dmg => {
                        overrides.extend(map.next_value::<Option<_>>()?.map(ItemStatOption::Dmg));
                    }
                    Key::Mps => {
                        overrides.extend(map.next_value::<Option<_>>()?.map(ItemStatOption::Mps));
                    }
                    Key::OutgoingNps => {
                        overrides.extend(map.next_value::<Option<_>>()?.map(ItemStatOption::OutgoingNps));
                    }
                    Key::OutgoingRps => {
                        overrides.extend(map.next_value::<Option<_>>()?.map(ItemStatOption::OutgoingRps));
                    }
                    Key::OutgoingCps => {
                        overrides.extend(map.next_value::<Option<_>>()?.map(ItemStatOption::OutgoingCps));
                    }
                    // Tank
                    Key::Resists => {
                        overrides.extend(map.next_value::<Option<_>>()?.map(ItemStatOption::Resists));
                    }
                    Key::Hp => {
                        overrides.extend(map.next_value::<Option<_>>()?.map(ItemStatOption::Hp));
                    }
                    Key::Ehp => {
                        overrides.extend(map.next_value::<Option<_>>()?.map(ItemStatOption::Ehp));
                    }
                    Key::WcEhp => {
                        overrides.extend(map.next_value::<Option<_>>()?.map(ItemStatOption::WcEhp));
                    }
                    Key::Rps => {
                        overrides.extend(map.next_value::<Option<_>>()?.map(ItemStatOption::Rps));
                    }
                    Key::Erps => {
                        overrides.extend(map.next_value::<Option<_>>()?.map(ItemStatOption::Erps));
                    }
                    Key::BreachResist => {
                        overrides.extend(map.next_value::<Option<_>>()?.map(ItemStatOption::BreachResist));
                    }
                    // Cap
                    Key::CapAmount => {
                        overrides.extend(map.next_value::<Option<_>>()?.map(ItemStatOption::CapAmount));
                    }
                    Key::CapBalance => {
                        overrides.extend(map.next_value::<Option<_>>()?.map(ItemStatOption::CapBalance));
                    }
                    Key::CapSim => {
                        overrides.extend(map.next_value::<Option<_>>()?.map(ItemStatOption::CapSim));
                    }
                    Key::NeutResist => {
                        overrides.extend(map.next_value::<Option<_>>()?.map(ItemStatOption::NeutResist));
                    }
                    // Sensors
                    Key::Locks => {
                        overrides.extend(map.next_value::<Option<_>>()?.map(ItemStatOption::Locks));
                    }
                    Key::LockRange => {
                        overrides.extend(map.next_value::<Option<_>>()?.map(ItemStatOption::LockRange));
                    }
                    Key::ScanRes => {
                        overrides.extend(map.next_value::<Option<_>>()?.map(ItemStatOption::ScanRes));
                    }
                    Key::Sensors => {
                        overrides.extend(map.next_value::<Option<_>>()?.map(ItemStatOption::Sensors));
                    }
                    Key::DscanRange => {
                        overrides.extend(map.next_value::<Option<_>>()?.map(ItemStatOption::DscanRange));
                    }
                    Key::ProbingSize => {
                        overrides.extend(map.next_value::<Option<_>>()?.map(ItemStatOption::ProbingSize));
                    }
                    Key::IncomingJam => {
                        overrides.extend(map.next_value::<Option<_>>()?.map(ItemStatOption::IncomingJam));
                    }
                    // Mobility
                    Key::Speed => {
                        overrides.extend(map.next_value::<Option<_>>()?.map(ItemStatOption::Speed));
                    }
                    Key::Agility => {
                        overrides.extend(map.next_value::<Option<_>>()?.map(ItemStatOption::Agility));
                    }
                    Key::AlignTime => {
                        overrides.extend(map.next_value::<Option<_>>()?.map(ItemStatOption::AlignTime));
                    }
                    Key::SigRadius => {
                        overrides.extend(map.next_value::<Option<_>>()?.map(ItemStatOption::SigRadius));
                    }
                    Key::Mass => {
                        overrides.extend(map.next_value::<Option<_>>()?.map(ItemStatOption::Mass));
                    }
                    Key::WarpSpeed => {
                        overrides.extend(map.next_value::<Option<_>>()?.map(ItemStatOption::WarpSpeed));
                    }
                    Key::MaxWarpRange => {
                        overrides.extend(map.next_value::<Option<_>>()?.map(ItemStatOption::MaxWarpRange));
                    }
                    Key::Jump => {
                        overrides.extend(map.next_value::<Option<_>>()?.map(ItemStatOption::Jump));
                    }
                    // Misc
                    Key::DroneControlRange => {
                        overrides.extend(map.next_value::<Option<_>>()?.map(ItemStatOption::DroneControlRange));
                    }
                    Key::CanWarp => {
                        overrides.extend(map.next_value::<Option<_>>()?.map(ItemStatOption::CanWarp));
                    }
                    Key::CanJumpGate => {
                        overrides.extend(map.next_value::<Option<_>>()?.map(ItemStatOption::CanJumpGate));
                    }
                    Key::CanJumpWormhole => {
                        overrides.extend(map.next_value::<Option<_>>()?.map(ItemStatOption::CanJumpWormhole));
                    }
                    Key::CanJumpDrive => {
                        overrides.extend(map.next_value::<Option<_>>()?.map(ItemStatOption::CanJumpDrive));
                    }
                    Key::CanDockStation => {
                        overrides.extend(map.next_value::<Option<_>>()?.map(ItemStatOption::CanDockStation));
                    }
                    Key::CanDockCitadel => {
                        overrides.extend(map.next_value::<Option<_>>()?.map(ItemStatOption::CanDockCitadel));
                    }
                    Key::CanTether => {
                        overrides.extend(map.next_value::<Option<_>>()?.map(ItemStatOption::CanTether));
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
