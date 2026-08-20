use crate::stats::{
    StatOptionCapBlc, StatOptionCapSim, StatOptionEhp, StatOptionErps, StatOptionFitDmg, StatOptionFitMining,
    StatOptionFitOutCps, StatOptionFitOutNps, StatOptionFitOutRps, StatOptionIncomingJam, StatOptionJump,
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
        O::Ext<StatOptionFitDmg<I>>: Default + serde::Deserialize<'de>,
        O::Ext<StatOptionFitMining>: Default + serde::Deserialize<'de>,
        O::Ext<StatOptionFitOutNps<I>>: Default + serde::Deserialize<'de>,
        O::Ext<StatOptionFitOutRps<I>>: Default + serde::Deserialize<'de>,
        O::Ext<StatOptionFitOutCps<I>>: Default + serde::Deserialize<'de>,
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
pub(in crate::stats) struct FitStatsOptionsInt<O, F, I>
where
    O: StatOptionKind,
    F: Clone,
    I: Clone,
{
    // Fit output stats
    pub(super) dmg: O::Ext<StatOptionFitDmg<I>>,
    pub(super) mps: O::Ext<StatOptionFitMining>,
    pub(super) outgoing_nps: O::Ext<StatOptionFitOutNps<I>>,
    pub(super) outgoing_rps: O::Ext<StatOptionFitOutRps<I>>,
    pub(super) outgoing_cps: O::Ext<StatOptionFitOutCps<I>>,
    // Fit resources
    pub(super) cpu: O::Reg,
    pub(super) powergrid: O::Reg,
    pub(super) calibration: O::Reg,
    pub(super) drone_bay_volume: O::Reg,
    pub(super) drone_bandwidth: O::Reg,
    pub(super) fighter_bay_volume: O::Reg,
    // Fit slots
    pub(super) high_slots: O::Reg,
    pub(super) mid_slots: O::Reg,
    pub(super) low_slots: O::Reg,
    pub(super) turret_slots: O::Reg,
    pub(super) launcher_slots: O::Reg,
    pub(super) rig_slots: O::Reg,
    pub(super) service_slots: O::Reg,
    pub(super) subsystem_slots: O::Reg,
    pub(super) launched_drones: O::Reg,
    pub(super) launched_fighters: O::Reg,
    pub(super) launched_light_fighters: O::Reg,
    pub(super) launched_heavy_fighters: O::Reg,
    pub(super) launched_support_fighters: O::Reg,
    pub(super) launched_st_light_fighters: O::Reg,
    pub(super) launched_st_heavy_fighters: O::Reg,
    pub(super) launched_st_support_fighters: O::Reg,
    // Ship tank
    pub(super) resists: O::Reg,
    pub(super) hp: O::Reg,
    pub(super) ehp: O::Ext<StatOptionEhp>,
    pub(super) wc_ehp: O::Reg,
    pub(super) rps: O::Ext<StatOptionRps>,
    pub(super) erps: O::Ext<StatOptionErps>,
    pub(super) breach_resist: O::Reg,
    // Ship cap
    pub(super) cap_amount: O::Reg,
    pub(super) cap_balance: O::Ext<StatOptionCapBlc<I>>,
    pub(super) cap_sim: O::Ext<StatOptionCapSim<I>>,
    pub(super) neut_resist: O::Reg,
    // Ship sensors
    pub(super) locks: O::Reg,
    pub(super) lock_range: O::Reg,
    pub(super) scan_res: O::Reg,
    pub(super) sensors: O::Reg,
    pub(super) dscan_range: O::Reg,
    pub(super) probing_size: O::Reg,
    pub(super) incoming_jam: O::Ext<StatOptionIncomingJam>,
    // Ship mobility
    pub(super) speed: O::Reg,
    pub(super) agility: O::Reg,
    pub(super) align_time: O::Reg,
    pub(super) sig_radius: O::Reg,
    pub(super) mass: O::Ext<StatOptionMass>,
    pub(super) warp_speed: O::Reg,
    pub(super) max_warp_range: O::Reg,
    pub(super) jump: O::Ext<StatOptionJump<F>>,
    // Ship misc stats
    pub(super) drone_control_range: O::Reg,
    pub(super) can_warp: O::Reg,
    pub(super) can_jump_gate: O::Reg,
    pub(super) can_jump_wormhole: O::Reg,
    pub(super) can_jump_drive: O::Reg,
    pub(super) can_dock_station: O::Reg,
    pub(super) can_dock_citadel: O::Reg,
    pub(super) can_tether: O::Reg,
}
impl<O, F, I> Default for FitStatsOptionsInt<O, F, I>
where
    O: StatOptionKind,
    F: Clone,
    I: Clone,
    O::Ext<StatOptionFitDmg<I>>: Default,
    O::Ext<StatOptionFitMining>: Default,
    O::Ext<StatOptionFitOutNps<I>>: Default,
    O::Ext<StatOptionFitOutRps<I>>: Default,
    O::Ext<StatOptionFitOutCps<I>>: Default,
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
            // Fit output stats
            dmg: Default::default(),
            mps: Default::default(),
            outgoing_nps: Default::default(),
            outgoing_rps: Default::default(),
            outgoing_cps: Default::default(),
            // Fit resources
            cpu: Default::default(),
            powergrid: Default::default(),
            calibration: Default::default(),
            drone_bay_volume: Default::default(),
            drone_bandwidth: Default::default(),
            fighter_bay_volume: Default::default(),
            // Fit slots
            high_slots: Default::default(),
            mid_slots: Default::default(),
            low_slots: Default::default(),
            turret_slots: Default::default(),
            launcher_slots: Default::default(),
            rig_slots: Default::default(),
            service_slots: Default::default(),
            subsystem_slots: Default::default(),
            launched_drones: Default::default(),
            launched_fighters: Default::default(),
            launched_light_fighters: Default::default(),
            launched_heavy_fighters: Default::default(),
            launched_support_fighters: Default::default(),
            launched_st_light_fighters: Default::default(),
            launched_st_heavy_fighters: Default::default(),
            launched_st_support_fighters: Default::default(),
            // Ship tank
            resists: Default::default(),
            hp: Default::default(),
            ehp: Default::default(),
            wc_ehp: Default::default(),
            rps: Default::default(),
            erps: Default::default(),
            breach_resist: Default::default(),
            // Ship cap
            cap_amount: Default::default(),
            cap_balance: Default::default(),
            cap_sim: Default::default(),
            neut_resist: Default::default(),
            // Ship sensors
            locks: Default::default(),
            lock_range: Default::default(),
            scan_res: Default::default(),
            sensors: Default::default(),
            dscan_range: Default::default(),
            probing_size: Default::default(),
            incoming_jam: Default::default(),
            // Ship mobility
            speed: Default::default(),
            agility: Default::default(),
            align_time: Default::default(),
            sig_radius: Default::default(),
            mass: Default::default(),
            warp_speed: Default::default(),
            max_warp_range: Default::default(),
            jump: Default::default(),
            // Ship misc stats
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
// Default + stat resolution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<F, I> FitStatsOptionsInt<StatOptionRaw, F, I>
where
    F: Clone,
    I: Clone,
{
    pub(in crate::stats) fn stat_resolve(self, default: bool) -> FitStatsOptionsInt<StatOptionResolved, F, I> {
        FitStatsOptionsInt {
            // Fit output stats
            dmg: self.dmg.stat_resolve(default),
            mps: self.mps.stat_resolve(default),
            outgoing_nps: self.outgoing_nps.stat_resolve(default),
            outgoing_rps: self.outgoing_rps.stat_resolve(default),
            outgoing_cps: self.outgoing_cps.stat_resolve(default),
            // Fit resources
            cpu: self.cpu.stat_resolve(default),
            powergrid: self.powergrid.stat_resolve(default),
            calibration: self.calibration.stat_resolve(default),
            drone_bay_volume: self.drone_bay_volume.stat_resolve(default),
            drone_bandwidth: self.drone_bandwidth.stat_resolve(default),
            fighter_bay_volume: self.fighter_bay_volume.stat_resolve(default),
            // Fit slots
            high_slots: self.high_slots.stat_resolve(default),
            mid_slots: self.mid_slots.stat_resolve(default),
            low_slots: self.low_slots.stat_resolve(default),
            turret_slots: self.turret_slots.stat_resolve(default),
            launcher_slots: self.launcher_slots.stat_resolve(default),
            rig_slots: self.rig_slots.stat_resolve(default),
            service_slots: self.service_slots.stat_resolve(default),
            subsystem_slots: self.subsystem_slots.stat_resolve(default),
            launched_drones: self.launched_drones.stat_resolve(default),
            launched_fighters: self.launched_fighters.stat_resolve(default),
            launched_light_fighters: self.launched_light_fighters.stat_resolve(default),
            launched_heavy_fighters: self.launched_heavy_fighters.stat_resolve(default),
            launched_support_fighters: self.launched_support_fighters.stat_resolve(default),
            launched_st_light_fighters: self.launched_st_light_fighters.stat_resolve(default),
            launched_st_heavy_fighters: self.launched_st_heavy_fighters.stat_resolve(default),
            launched_st_support_fighters: self.launched_st_support_fighters.stat_resolve(default),
            // Ship tank
            resists: self.resists.stat_resolve(default),
            hp: self.hp.stat_resolve(default),
            ehp: self.ehp.stat_resolve(default),
            wc_ehp: self.wc_ehp.stat_resolve(default),
            rps: self.rps.stat_resolve(default),
            erps: self.erps.stat_resolve(default),
            breach_resist: self.breach_resist.stat_resolve(default),
            // Ship cap
            cap_amount: self.cap_amount.stat_resolve(default),
            cap_balance: self.cap_balance.stat_resolve(default),
            cap_sim: self.cap_sim.stat_resolve(default),
            neut_resist: self.neut_resist.stat_resolve(default),
            // Ship sensors
            locks: self.locks.stat_resolve(default),
            lock_range: self.lock_range.stat_resolve(default),
            scan_res: self.scan_res.stat_resolve(default),
            sensors: self.sensors.stat_resolve(default),
            dscan_range: self.dscan_range.stat_resolve(default),
            probing_size: self.probing_size.stat_resolve(default),
            incoming_jam: self.incoming_jam.stat_resolve(default),
            // Ship mobility
            speed: self.speed.stat_resolve(default),
            agility: self.agility.stat_resolve(default),
            align_time: self.align_time.stat_resolve(default),
            sig_radius: self.sig_radius.stat_resolve(default),
            mass: self.mass.stat_resolve(default),
            warp_speed: self.warp_speed.stat_resolve(default),
            max_warp_range: self.max_warp_range.stat_resolve(default),
            jump: self.jump.stat_resolve(default),
            // Ship misc stats
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
