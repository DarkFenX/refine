use crate::{
    CmdResps, FitId, FitIdBr, ItemId, ItemIdBr,
    err::BrResolveError,
    stats::{
        StatOptionCapBlc, StatOptionCapSim, StatOptionEhp, StatOptionErps, StatOptionIncomingJam, StatOptionItemDmg,
        StatOptionItemMining, StatOptionItemOutCps, StatOptionItemOutNps, StatOptionItemOutRps, StatOptionJump,
        StatOptionMass, StatOptionRps,
        option::{StatOptionKind, StatOptionRaw, StatOptionResolved},
    },
};

#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize),
    serde(
        default,
        bound(deserialize = "
        O::Reg: Default + serde::Deserialize<'de>,
        O::Ext<StatOptionItemDmg<I>>: Default + serde::Deserialize<'de>,
        O::Ext<StatOptionItemMining>: Default + serde::Deserialize<'de>,
        O::Ext<StatOptionItemOutNps<I>>: Default + serde::Deserialize<'de>,
        O::Ext<StatOptionItemOutRps<I>>: Default + serde::Deserialize<'de>,
        O::Ext<StatOptionItemOutCps<I>>: Default + serde::Deserialize<'de>,
        O::Ext<StatOptionEhp>: Default + serde::Deserialize<'de>,
        O::Ext<StatOptionRps>: Default + serde::Deserialize<'de>,
        O::Ext<StatOptionErps>: Default + serde::Deserialize<'de>,
        O::Ext<StatOptionCapBlc<I>>: Default + serde::Deserialize<'de>,
        O::Ext<StatOptionCapSim<I>>: Default + serde::Deserialize<'de>,
        O::Ext<StatOptionIncomingJam>: Default + serde::Deserialize<'de>,
        O::Ext<StatOptionMass>: Default + serde::Deserialize<'de>,
        O::Ext<StatOptionJump<F>>: Default + serde::Deserialize<'de>")
    )
)]
#[derive(Clone)]
pub(in crate::stats) struct ItemStatsOptionsInt<O, F, I>
where
    O: StatOptionKind,
    F: Clone,
    I: Clone,
{
    // Output
    pub(in crate::stats) dmg: O::Ext<StatOptionItemDmg<I>>,
    pub(in crate::stats) mps: O::Ext<StatOptionItemMining>,
    pub(in crate::stats) outgoing_nps: O::Ext<StatOptionItemOutNps<I>>,
    pub(in crate::stats) outgoing_rps: O::Ext<StatOptionItemOutRps<I>>,
    pub(in crate::stats) outgoing_cps: O::Ext<StatOptionItemOutCps<I>>,
    // Tank
    pub(in crate::stats) resists: O::Reg,
    pub(in crate::stats) hp: O::Reg,
    pub(in crate::stats) ehp: O::Ext<StatOptionEhp>,
    pub(in crate::stats) wc_ehp: O::Reg,
    pub(in crate::stats) rps: O::Ext<StatOptionRps>,
    pub(in crate::stats) erps: O::Ext<StatOptionErps>,
    pub(in crate::stats) breach_resist: O::Reg,
    // Cap
    pub(in crate::stats) cap_amount: O::Reg,
    pub(in crate::stats) cap_balance: O::Ext<StatOptionCapBlc<I>>,
    pub(in crate::stats) cap_sim: O::Ext<StatOptionCapSim<I>>,
    pub(in crate::stats) neut_resist: O::Reg,
    // Sensors
    pub(in crate::stats) locks: O::Reg,
    pub(in crate::stats) lock_range: O::Reg,
    pub(in crate::stats) scan_res: O::Reg,
    pub(in crate::stats) sensors: O::Reg,
    pub(in crate::stats) dscan_range: O::Reg,
    pub(in crate::stats) probing_size: O::Reg,
    pub(in crate::stats) incoming_jam: O::Ext<StatOptionIncomingJam>,
    // Mobility
    pub(in crate::stats) speed: O::Reg,
    pub(in crate::stats) agility: O::Reg,
    pub(in crate::stats) align_time: O::Reg,
    pub(in crate::stats) sig_radius: O::Reg,
    pub(in crate::stats) mass: O::Ext<StatOptionMass>,
    pub(in crate::stats) warp_speed: O::Reg,
    pub(in crate::stats) max_warp_range: O::Reg,
    pub(in crate::stats) jump: O::Ext<StatOptionJump<F>>,
    // Misc
    pub(in crate::stats) drone_control_range: O::Reg,
    pub(in crate::stats) can_warp: O::Reg,
    pub(in crate::stats) can_jump_gate: O::Reg,
    pub(in crate::stats) can_jump_wormhole: O::Reg,
    pub(in crate::stats) can_jump_drive: O::Reg,
    pub(in crate::stats) can_dock_station: O::Reg,
    pub(in crate::stats) can_dock_citadel: O::Reg,
    pub(in crate::stats) can_tether: O::Reg,
}
impl<O, F, I> Default for ItemStatsOptionsInt<O, F, I>
where
    O: StatOptionKind,
    F: Clone,
    I: Clone,
    O::Ext<StatOptionItemDmg<I>>: Default,
    O::Ext<StatOptionItemMining>: Default,
    O::Ext<StatOptionItemOutNps<I>>: Default,
    O::Ext<StatOptionItemOutRps<I>>: Default,
    O::Ext<StatOptionItemOutCps<I>>: Default,
    O::Ext<StatOptionEhp>: Default,
    O::Ext<StatOptionRps>: Default,
    O::Ext<StatOptionErps>: Default,
    O::Ext<StatOptionCapBlc<I>>: Default,
    O::Ext<StatOptionCapSim<I>>: Default,
    O::Ext<StatOptionIncomingJam>: Default,
    O::Ext<StatOptionMass>: Default,
    O::Ext<StatOptionJump<F>>: Default,
    O::Reg: Default,
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
    pub(in crate::stats) fn br_resolve(
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
    pub(in crate::stats) fn stat_resolve(
        self,
        default: bool,
    ) -> ItemStatsOptionsInt<StatOptionResolved, FitId, ItemId> {
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
