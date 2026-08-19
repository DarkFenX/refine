use crate::stats::{
    StatOptionCapBlc, StatOptionCapSim, StatOptionEhp, StatOptionErps, StatOptionIncomingJam, StatOptionItemDmg,
    StatOptionItemMining, StatOptionItemOutCps, StatOptionItemOutNps, StatOptionItemOutRps, StatOptionJump,
    StatOptionMass, StatOptionRps,
    option::{StatOptionKind, StatOptionRaw, StatOptionResolved},
};

#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize),
    serde(
        default,
        bound(deserialize = "
        O::Reg: Default + serde::Deserialize<'de>,
        O::Ext<StatOptionItemDmg>: Default + serde::Deserialize<'de>,
        O::Ext<StatOptionItemMining>: Default + serde::Deserialize<'de>,
        O::Ext<StatOptionItemOutNps>: Default + serde::Deserialize<'de>,
        O::Ext<StatOptionItemOutRps>: Default + serde::Deserialize<'de>,
        O::Ext<StatOptionItemOutCps>: Default + serde::Deserialize<'de>,
        O::Ext<StatOptionEhp>: Default + serde::Deserialize<'de>,
        O::Ext<StatOptionRps>: Default + serde::Deserialize<'de>,
        O::Ext<StatOptionErps>: Default + serde::Deserialize<'de>,
        O::Ext<StatOptionCapBlc>: Default + serde::Deserialize<'de>,
        O::Ext<StatOptionCapSim>: Default + serde::Deserialize<'de>,
        O::Ext<StatOptionIncomingJam>: Default + serde::Deserialize<'de>,
        O::Ext<StatOptionMass>: Default + serde::Deserialize<'de>,
        O::Ext<StatOptionJump>: Default + serde::Deserialize<'de>")
    )
)]
#[derive(Clone)]
pub(in crate::stats) struct StatItemOptionsInt<O: StatOptionKind> {
    // Output
    pub(in crate::stats) dmg: O::Ext<StatOptionItemDmg>,
    pub(in crate::stats) mps: O::Ext<StatOptionItemMining>,
    pub(in crate::stats) outgoing_nps: O::Ext<StatOptionItemOutNps>,
    pub(in crate::stats) outgoing_rps: O::Ext<StatOptionItemOutRps>,
    pub(in crate::stats) outgoing_cps: O::Ext<StatOptionItemOutCps>,
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
    pub(in crate::stats) cap_balance: O::Ext<StatOptionCapBlc>,
    pub(in crate::stats) cap_sim: O::Ext<StatOptionCapSim>,
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
    pub(in crate::stats) jump: O::Ext<StatOptionJump>,
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
impl<O> Default for StatItemOptionsInt<O>
where
    O: StatOptionKind,
    O::Ext<StatOptionItemDmg>: Default,
    O::Ext<StatOptionItemMining>: Default,
    O::Ext<StatOptionItemOutNps>: Default,
    O::Ext<StatOptionItemOutRps>: Default,
    O::Ext<StatOptionItemOutCps>: Default,
    O::Ext<StatOptionEhp>: Default,
    O::Ext<StatOptionRps>: Default,
    O::Ext<StatOptionErps>: Default,
    O::Ext<StatOptionCapBlc>: Default,
    O::Ext<StatOptionCapSim>: Default,
    O::Ext<StatOptionIncomingJam>: Default,
    O::Ext<StatOptionMass>: Default,
    O::Ext<StatOptionJump>: Default,
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
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl StatItemOptionsInt<StatOptionRaw> {
    pub(in crate::stats) fn resolve(self, default: bool) -> StatItemOptionsInt<StatOptionResolved> {
        StatItemOptionsInt {
            // Output
            dmg: self.dmg.into_enabled(default),
            mps: self.mps.into_enabled(default),
            outgoing_nps: self.outgoing_nps.into_enabled(default),
            outgoing_rps: self.outgoing_rps.into_enabled(default),
            outgoing_cps: self.outgoing_cps.into_enabled(default),
            // Tank
            resists: self.resists.into_enabled(default),
            hp: self.hp.into_enabled(default),
            ehp: self.ehp.into_enabled(default),
            wc_ehp: self.wc_ehp.into_enabled(default),
            rps: self.rps.into_enabled(default),
            erps: self.erps.into_enabled(default),
            breach_resist: self.breach_resist.into_enabled(default),
            // Cap
            cap_amount: self.cap_amount.into_enabled(default),
            cap_balance: self.cap_balance.into_enabled(default),
            cap_sim: self.cap_sim.into_enabled(default),
            neut_resist: self.neut_resist.into_enabled(default),
            // Sensors
            locks: self.locks.into_enabled(default),
            lock_range: self.lock_range.into_enabled(default),
            scan_res: self.scan_res.into_enabled(default),
            sensors: self.sensors.into_enabled(default),
            dscan_range: self.dscan_range.into_enabled(default),
            probing_size: self.probing_size.into_enabled(default),
            incoming_jam: self.incoming_jam.into_enabled(default),
            // Mobility
            speed: self.speed.into_enabled(default),
            agility: self.agility.into_enabled(default),
            align_time: self.align_time.into_enabled(default),
            sig_radius: self.sig_radius.into_enabled(default),
            mass: self.mass.into_enabled(default),
            warp_speed: self.warp_speed.into_enabled(default),
            max_warp_range: self.max_warp_range.into_enabled(default),
            jump: self.jump.into_enabled(default),
            // Misc
            drone_control_range: self.drone_control_range.into_enabled(default),
            can_warp: self.can_warp.into_enabled(default),
            can_jump_gate: self.can_jump_gate.into_enabled(default),
            can_jump_wormhole: self.can_jump_wormhole.into_enabled(default),
            can_jump_drive: self.can_jump_drive.into_enabled(default),
            can_dock_station: self.can_dock_station.into_enabled(default),
            can_dock_citadel: self.can_dock_citadel.into_enabled(default),
            can_tether: self.can_tether.into_enabled(default),
        }
    }
}
