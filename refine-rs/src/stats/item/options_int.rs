#[cfg(feature = "serde")]
use crate::stats::option::DeStatOptionKind;
use crate::{
    CmdResps, FitId, FitIdBr, ItemId, ItemIdBr,
    err::BrResolveError,
    stats::{
        StatOptionCapBlc, StatOptionCapSim, StatOptionEhp, StatOptionErps, StatOptionIncomingJam, StatOptionItemDmg,
        StatOptionItemMining, StatOptionItemOutCps, StatOptionItemOutNps, StatOptionItemOutRps, StatOptionJump,
        StatOptionMass, StatOptionRps,
        option::{StatOptionExtended, StatOptionKind, StatOptionRaw, StatOptionRegular, StatOptionResolved},
    },
};

#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize),
    serde(
        default,
        bound(deserialize = "O: DeStatOptionKind, F: serde::Deserialize<'de>, I: serde::Deserialize<'de>")
    )
)]
#[derive(Clone)]
pub(in crate::stats) struct ItemStatsOptionsInt<O, F, I>
where
    O: StatOptionKind,
{
    // Output
    pub(super) dmg: StatOptionExtended<O, StatOptionItemDmg<I>>,
    pub(super) mps: StatOptionExtended<O, StatOptionItemMining>,
    pub(super) outgoing_nps: StatOptionExtended<O, StatOptionItemOutNps<I>>,
    pub(super) outgoing_rps: StatOptionExtended<O, StatOptionItemOutRps<I>>,
    pub(super) outgoing_cps: StatOptionExtended<O, StatOptionItemOutCps<I>>,
    // Tank
    pub(super) resists: StatOptionRegular<O>,
    pub(super) hp: StatOptionRegular<O>,
    pub(super) ehp: StatOptionExtended<O, StatOptionEhp>,
    pub(super) wc_ehp: StatOptionRegular<O>,
    pub(super) rps: StatOptionExtended<O, StatOptionRps>,
    pub(super) erps: StatOptionExtended<O, StatOptionErps>,
    pub(super) breach_resist: StatOptionRegular<O>,
    // Cap
    pub(super) cap_amount: StatOptionRegular<O>,
    pub(super) cap_balance: StatOptionExtended<O, StatOptionCapBlc<I>>,
    pub(super) cap_sim: StatOptionExtended<O, StatOptionCapSim<I>>,
    pub(super) neut_resist: StatOptionRegular<O>,
    // Sensors
    pub(super) locks: StatOptionRegular<O>,
    pub(super) lock_range: StatOptionRegular<O>,
    pub(super) scan_res: StatOptionRegular<O>,
    pub(super) sensors: StatOptionRegular<O>,
    pub(super) dscan_range: StatOptionRegular<O>,
    pub(super) probing_size: StatOptionRegular<O>,
    pub(super) incoming_jam: StatOptionExtended<O, StatOptionIncomingJam>,
    // Mobility
    pub(super) speed: StatOptionRegular<O>,
    pub(super) agility: StatOptionRegular<O>,
    pub(super) align_time: StatOptionRegular<O>,
    pub(super) sig_radius: StatOptionRegular<O>,
    pub(super) mass: StatOptionExtended<O, StatOptionMass>,
    pub(super) warp_speed: StatOptionRegular<O>,
    pub(super) max_warp_range: StatOptionRegular<O>,
    pub(super) jump: StatOptionExtended<O, StatOptionJump<F>>,
    // Misc
    pub(super) drone_control_range: StatOptionRegular<O>,
    pub(super) can_warp: StatOptionRegular<O>,
    pub(super) can_jump_gate: StatOptionRegular<O>,
    pub(super) can_jump_wormhole: StatOptionRegular<O>,
    pub(super) can_jump_drive: StatOptionRegular<O>,
    pub(super) can_dock_station: StatOptionRegular<O>,
    pub(super) can_dock_citadel: StatOptionRegular<O>,
    pub(super) can_tether: StatOptionRegular<O>,
}
impl<O, F, I> Default for ItemStatsOptionsInt<O, F, I>
where
    O: StatOptionKind,
{
    fn default() -> Self {
        Self {
            // Output
            dmg: Default::default(),
            mps: Default::default(),
            outgoing_nps: Default::default(),
            outgoing_rps: Default::default(),
            outgoing_cps: Default::default(),
            // Tank
            resists: Default::default(),
            hp: Default::default(),
            ehp: Default::default(),
            wc_ehp: Default::default(),
            rps: Default::default(),
            erps: Default::default(),
            breach_resist: Default::default(),
            // Cap
            cap_amount: Default::default(),
            cap_balance: Default::default(),
            cap_sim: Default::default(),
            neut_resist: Default::default(),
            // Sensors
            locks: Default::default(),
            lock_range: Default::default(),
            scan_res: Default::default(),
            sensors: Default::default(),
            dscan_range: Default::default(),
            probing_size: Default::default(),
            incoming_jam: Default::default(),
            // Mobility
            speed: Default::default(),
            agility: Default::default(),
            align_time: Default::default(),
            sig_radius: Default::default(),
            mass: Default::default(),
            warp_speed: Default::default(),
            max_warp_range: Default::default(),
            jump: Default::default(),
            // Misc
            drone_control_range: Default::default(),
            can_warp: Default::default(),
            can_jump_gate: Default::default(),
            can_jump_wormhole: Default::default(),
            can_jump_drive: Default::default(),
            can_dock_station: Default::default(),
            can_dock_citadel: Default::default(),
            can_tether: Default::default(),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Backref resolution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ItemStatsOptionsInt<StatOptionRaw, FitIdBr, ItemIdBr> {
    pub(super) fn br_resolve(
        self,
        resps: &CmdResps,
    ) -> Result<ItemStatsOptionsInt<StatOptionRaw, FitId, ItemId>, BrResolveError> {
        Ok(ItemStatsOptionsInt {
            // Output
            dmg: self.dmg.br_resolve(resps)?,
            mps: self.mps,
            outgoing_nps: self.outgoing_nps.br_resolve(resps)?,
            outgoing_rps: self.outgoing_rps.br_resolve(resps)?,
            outgoing_cps: self.outgoing_cps.br_resolve(resps)?,
            // Tank
            resists: self.resists,
            hp: self.hp,
            ehp: self.ehp,
            wc_ehp: self.wc_ehp,
            rps: self.rps,
            erps: self.erps,
            breach_resist: self.breach_resist,
            // Cap
            cap_amount: self.cap_amount,
            cap_balance: self.cap_balance.br_resolve(resps)?,
            cap_sim: self.cap_sim.br_resolve(resps)?,
            neut_resist: self.neut_resist,
            // Sensors
            locks: self.locks,
            lock_range: self.lock_range,
            scan_res: self.scan_res,
            sensors: self.sensors,
            dscan_range: self.dscan_range,
            probing_size: self.probing_size,
            incoming_jam: self.incoming_jam,
            // Mobility
            speed: self.speed,
            agility: self.agility,
            align_time: self.align_time,
            sig_radius: self.sig_radius,
            mass: self.mass,
            warp_speed: self.warp_speed,
            max_warp_range: self.max_warp_range,
            jump: self.jump.br_resolve(resps)?,
            // Misc
            drone_control_range: self.drone_control_range,
            can_warp: self.can_warp,
            can_jump_gate: self.can_jump_gate,
            can_jump_wormhole: self.can_jump_wormhole,
            can_jump_drive: self.can_jump_drive,
            can_dock_station: self.can_dock_station,
            can_dock_citadel: self.can_dock_citadel,
            can_tether: self.can_tether,
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Default + stat resolution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ItemStatsOptionsInt<StatOptionRaw, FitId, ItemId> {
    pub(super) fn stat_resolve(self, default: bool) -> ItemStatsOptionsInt<StatOptionResolved, FitId, ItemId> {
        ItemStatsOptionsInt {
            // Output
            dmg: self.dmg.stat_resolve(default),
            mps: self.mps.stat_resolve(default),
            outgoing_nps: self.outgoing_nps.stat_resolve(default),
            outgoing_rps: self.outgoing_rps.stat_resolve(default),
            outgoing_cps: self.outgoing_cps.stat_resolve(default),
            // Tank
            resists: self.resists.stat_resolve(default),
            hp: self.hp.stat_resolve(default),
            ehp: self.ehp.stat_resolve(default),
            wc_ehp: self.wc_ehp.stat_resolve(default),
            rps: self.rps.stat_resolve(default),
            erps: self.erps.stat_resolve(default),
            breach_resist: self.breach_resist.stat_resolve(default),
            // Cap
            cap_amount: self.cap_amount.stat_resolve(default),
            cap_balance: self.cap_balance.stat_resolve(default),
            cap_sim: self.cap_sim.stat_resolve(default),
            neut_resist: self.neut_resist.stat_resolve(default),
            // Sensors
            locks: self.locks.stat_resolve(default),
            lock_range: self.lock_range.stat_resolve(default),
            scan_res: self.scan_res.stat_resolve(default),
            sensors: self.sensors.stat_resolve(default),
            dscan_range: self.dscan_range.stat_resolve(default),
            probing_size: self.probing_size.stat_resolve(default),
            incoming_jam: self.incoming_jam.stat_resolve(default),
            // Mobility
            speed: self.speed.stat_resolve(default),
            agility: self.agility.stat_resolve(default),
            align_time: self.align_time.stat_resolve(default),
            sig_radius: self.sig_radius.stat_resolve(default),
            mass: self.mass.stat_resolve(default),
            warp_speed: self.warp_speed.stat_resolve(default),
            max_warp_range: self.max_warp_range.stat_resolve(default),
            jump: self.jump.stat_resolve(default),
            // Misc
            drone_control_range: self.drone_control_range.stat_resolve(default),
            can_warp: self.can_warp.stat_resolve(default),
            can_jump_gate: self.can_jump_gate.stat_resolve(default),
            can_jump_wormhole: self.can_jump_wormhole.stat_resolve(default),
            can_jump_drive: self.can_jump_drive.stat_resolve(default),
            can_dock_station: self.can_dock_station.stat_resolve(default),
            can_dock_citadel: self.can_dock_citadel.stat_resolve(default),
            can_tether: self.can_tether.stat_resolve(default),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Anything-requested check
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ItemStatsOptionsInt<StatOptionResolved, FitId, ItemId> {
    pub(in crate::stats) fn is_any_stat_requested(&self) -> bool {
        self.dmg.is_enabled() ||
        self.mps.is_enabled() ||
        self.outgoing_nps.is_enabled() ||
        self.outgoing_rps.is_enabled() ||
        self.outgoing_cps.is_enabled() ||
        // Tank
        self.resists.is_enabled() ||
        self.hp.is_enabled() ||
        self.ehp.is_enabled() ||
        self.wc_ehp.is_enabled() ||
        self.rps.is_enabled() ||
        self.erps.is_enabled() ||
        self.breach_resist.is_enabled() ||
        // Cap
        self.cap_amount.is_enabled() ||
        self.cap_balance.is_enabled() ||
        self.cap_sim.is_enabled() ||
        self.neut_resist.is_enabled() ||
        // Sensors
        self.locks.is_enabled() ||
        self.lock_range.is_enabled() ||
        self.scan_res.is_enabled() ||
        self.sensors.is_enabled() ||
        self.dscan_range.is_enabled() ||
        self.probing_size.is_enabled() ||
        self.incoming_jam.is_enabled() ||
        // Mobility
        self.speed.is_enabled() ||
        self.agility.is_enabled() ||
        self.align_time.is_enabled() ||
        self.sig_radius.is_enabled() ||
        self.mass.is_enabled() ||
        self.warp_speed.is_enabled() ||
        self.max_warp_range.is_enabled() ||
        self.jump.is_enabled() ||
        // Misc
        self.drone_control_range.is_enabled() ||
        self.can_warp.is_enabled() ||
        self.can_jump_gate.is_enabled() ||
        self.can_jump_wormhole.is_enabled() ||
        self.can_jump_drive.is_enabled() ||
        self.can_dock_station.is_enabled() ||
        self.can_dock_citadel.is_enabled() ||
        self.can_tether.is_enabled()
    }
}
