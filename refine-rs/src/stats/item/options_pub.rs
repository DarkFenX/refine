use super::options_int::ItemStatsOptionsInt;
use crate::{
    CmdResps, FitId, FitIdBr, ItemId, ItemIdBr,
    err::BrResolveError,
    shared::BrResolvable,
    stats::{
        StatOptionCapBlc, StatOptionCapSim, StatOptionEhp, StatOptionErps, StatOptionExt, StatOptionIncomingJam,
        StatOptionItemDmg, StatOptionItemMining, StatOptionItemOutCps, StatOptionItemOutNps, StatOptionItemOutRps,
        StatOptionJump, StatOptionMass, StatOptionRps,
        option::{StatOptionRaw, StatOptionResolved},
    },
};

#[cfg_attr(feature = "serde", derive(serde::Deserialize), serde(default))]
#[derive(Clone)]
pub struct ItemStatsOptions<F = FitId, I = ItemId>
{
    #[cfg_attr(feature = "serde", serde(default))]
    default: bool = false,
    #[cfg_attr(feature = "serde", serde(flatten))]
    options: ItemStatsOptionsInt<StatOptionRaw, F, I>,
}
impl<F, I> Default for ItemStatsOptions<F, I> {
    fn default() -> Self {
        Self {
            options: Default::default(),
            ..
        }
    }
}

pub type ItemStatsOptionsBr = ItemStatsOptions<FitIdBr, ItemIdBr>;

////////////////////////////////////////////////////////////////////////////////////////////////////
// Construction
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<F, I> ItemStatsOptions<F, I> {
    /// True to have all supported stats enabled by default, false to have them disabled.
    pub fn new(default: bool) -> Self {
        Self {
            default,
            options: ItemStatsOptionsInt::default(),
        }
    }
    // Output
    pub fn with_dmg(mut self, option: StatOptionExt<StatOptionItemDmg<I>>) -> Self {
        self.options.dmg = option.into();
        self
    }
    pub fn with_mps(mut self, option: StatOptionExt<StatOptionItemMining>) -> Self {
        self.options.mps = option.into();
        self
    }
    pub fn with_outgoing_nps(mut self, option: StatOptionExt<StatOptionItemOutNps<I>>) -> Self {
        self.options.outgoing_nps = option.into();
        self
    }
    pub fn with_outgoing_rps(mut self, option: StatOptionExt<StatOptionItemOutRps<I>>) -> Self {
        self.options.outgoing_rps = option.into();
        self
    }
    pub fn with_outgoing_cps(mut self, option: StatOptionExt<StatOptionItemOutCps<I>>) -> Self {
        self.options.outgoing_cps = option.into();
        self
    }
    // Tank
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
    // Cap
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
    // Sensors
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
    // Mobility
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
    // Misc
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
// Backref resolution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl BrResolvable for ItemStatsOptionsBr {
    type Target = ItemStatsOptions;
    fn br_resolve(self, resps: &CmdResps) -> Result<Self::Target, BrResolveError> {
        Ok(Self::Target {
            default: self.default,
            options: self.options.br_resolve(resps)?,
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Default + stat resolution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ItemStatsOptions {
    pub(super) fn stat_resolve(self) -> ItemStatsOptionsInt<StatOptionResolved, FitId, ItemId> {
        self.options.stat_resolve(self.default)
    }
}

impl From<ItemStatsOptions<FitId, ItemId>> for ItemStatsOptionsInt<StatOptionResolved, FitId, ItemId> {
    fn from(value: ItemStatsOptions<FitId, ItemId>) -> Self {
        value.options.stat_resolve(value.default)
    }
}
