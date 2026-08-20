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
    pub(in crate::stats) dmg: O::Ext<StatOptionFitDmg<I>>,
    pub(in crate::stats) mps: O::Ext<StatOptionFitMining>,
    pub(in crate::stats) outgoing_nps: O::Ext<StatOptionFitOutNps<I>>,
    pub(in crate::stats) outgoing_rps: O::Ext<StatOptionFitOutRps<I>>,
    pub(in crate::stats) outgoing_cps: O::Ext<StatOptionFitOutCps<I>>,
    // Fit resources
    pub(in crate::stats) cpu: O::Reg,
    pub(in crate::stats) powergrid: O::Reg,
    pub(in crate::stats) calibration: O::Reg,
    pub(in crate::stats) drone_bay_volume: O::Reg,
    pub(in crate::stats) drone_bandwidth: O::Reg,
    pub(in crate::stats) fighter_bay_volume: O::Reg,
    // Fit slots
    pub(in crate::stats) high_slots: O::Reg,
    pub(in crate::stats) mid_slots: O::Reg,
    pub(in crate::stats) low_slots: O::Reg,
    pub(in crate::stats) turret_slots: O::Reg,
    pub(in crate::stats) launcher_slots: O::Reg,
    pub(in crate::stats) rig_slots: O::Reg,
    pub(in crate::stats) service_slots: O::Reg,
    pub(in crate::stats) subsystem_slots: O::Reg,
    pub(in crate::stats) launched_drones: O::Reg,
    pub(in crate::stats) launched_fighters: O::Reg,
    pub(in crate::stats) launched_light_fighters: O::Reg,
    pub(in crate::stats) launched_heavy_fighters: O::Reg,
    pub(in crate::stats) launched_support_fighters: O::Reg,
    pub(in crate::stats) launched_st_light_fighters: O::Reg,
    pub(in crate::stats) launched_st_heavy_fighters: O::Reg,
    pub(in crate::stats) launched_st_support_fighters: O::Reg,
    // Ship tank
    pub(in crate::stats) resists: O::Reg,
    pub(in crate::stats) hp: O::Reg,
    pub(in crate::stats) ehp: O::Ext<StatOptionEhp>,
    pub(in crate::stats) wc_ehp: O::Reg,
    pub(in crate::stats) rps: O::Ext<StatOptionRps>,
    pub(in crate::stats) erps: O::Ext<StatOptionErps>,
    pub(in crate::stats) breach_resist: O::Reg,
    // Ship cap
    pub(in crate::stats) cap_amount: O::Reg,
    pub(in crate::stats) cap_balance: O::Ext<StatOptionCapBlc<I>>,
    pub(in crate::stats) cap_sim: O::Ext<StatOptionCapSim<I>>,
    pub(in crate::stats) neut_resist: O::Reg,
    // Ship sensors
    pub(in crate::stats) locks: O::Reg,
    pub(in crate::stats) lock_range: O::Reg,
    pub(in crate::stats) scan_res: O::Reg,
    pub(in crate::stats) sensors: O::Reg,
    pub(in crate::stats) dscan_range: O::Reg,
    pub(in crate::stats) probing_size: O::Reg,
    pub(in crate::stats) incoming_jam: O::Ext<StatOptionIncomingJam>,
    // Ship mobility
    pub(in crate::stats) speed: O::Reg,
    pub(in crate::stats) agility: O::Reg,
    pub(in crate::stats) align_time: O::Reg,
    pub(in crate::stats) sig_radius: O::Reg,
    pub(in crate::stats) mass: O::Ext<StatOptionMass>,
    pub(in crate::stats) warp_speed: O::Reg,
    pub(in crate::stats) max_warp_range: O::Reg,
    pub(in crate::stats) jump: O::Ext<StatOptionJump<F>>,
    // Ship misc stats
    pub(in crate::stats) drone_control_range: O::Reg,
    pub(in crate::stats) can_warp: O::Reg,
    pub(in crate::stats) can_jump_gate: O::Reg,
    pub(in crate::stats) can_jump_wormhole: O::Reg,
    pub(in crate::stats) can_jump_drive: O::Reg,
    pub(in crate::stats) can_dock_station: O::Reg,
    pub(in crate::stats) can_dock_citadel: O::Reg,
    pub(in crate::stats) can_tether: O::Reg,
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
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<F, I> FitStatsOptionsInt<StatOptionRaw, F, I>
where
    F: Clone,
    I: Clone,
{
    pub(in crate::stats) fn resolve(self, default: bool) -> FitStatsOptionsInt<StatOptionResolved, F, I> {
        FitStatsOptionsInt {
            // Fit output stats
            dmg: self.dmg.into_enabled(default),
            mps: self.mps.into_enabled(default),
            outgoing_nps: self.outgoing_nps.into_enabled(default),
            outgoing_rps: self.outgoing_rps.into_enabled(default),
            outgoing_cps: self.outgoing_cps.into_enabled(default),
            // Fit resources
            cpu: self.cpu.into_enabled(default),
            powergrid: self.powergrid.into_enabled(default),
            calibration: self.calibration.into_enabled(default),
            drone_bay_volume: self.drone_bay_volume.into_enabled(default),
            drone_bandwidth: self.drone_bandwidth.into_enabled(default),
            fighter_bay_volume: self.fighter_bay_volume.into_enabled(default),
            // Fit slots
            high_slots: self.high_slots.into_enabled(default),
            mid_slots: self.mid_slots.into_enabled(default),
            low_slots: self.low_slots.into_enabled(default),
            turret_slots: self.turret_slots.into_enabled(default),
            launcher_slots: self.launcher_slots.into_enabled(default),
            rig_slots: self.rig_slots.into_enabled(default),
            service_slots: self.service_slots.into_enabled(default),
            subsystem_slots: self.subsystem_slots.into_enabled(default),
            launched_drones: self.launched_drones.into_enabled(default),
            launched_fighters: self.launched_fighters.into_enabled(default),
            launched_light_fighters: self.launched_light_fighters.into_enabled(default),
            launched_heavy_fighters: self.launched_heavy_fighters.into_enabled(default),
            launched_support_fighters: self.launched_support_fighters.into_enabled(default),
            launched_st_light_fighters: self.launched_st_light_fighters.into_enabled(default),
            launched_st_heavy_fighters: self.launched_st_heavy_fighters.into_enabled(default),
            launched_st_support_fighters: self.launched_st_support_fighters.into_enabled(default),
            // Ship tank
            resists: self.resists.into_enabled(default),
            hp: self.hp.into_enabled(default),
            ehp: self.ehp.into_enabled(default),
            wc_ehp: self.wc_ehp.into_enabled(default),
            rps: self.rps.into_enabled(default),
            erps: self.erps.into_enabled(default),
            breach_resist: self.breach_resist.into_enabled(default),
            // Ship cap
            cap_amount: self.cap_amount.into_enabled(default),
            cap_balance: self.cap_balance.into_enabled(default),
            cap_sim: self.cap_sim.into_enabled(default),
            neut_resist: self.neut_resist.into_enabled(default),
            // Ship sensors
            locks: self.locks.into_enabled(default),
            lock_range: self.lock_range.into_enabled(default),
            scan_res: self.scan_res.into_enabled(default),
            sensors: self.sensors.into_enabled(default),
            dscan_range: self.dscan_range.into_enabled(default),
            probing_size: self.probing_size.into_enabled(default),
            incoming_jam: self.incoming_jam.into_enabled(default),
            // Ship mobility
            speed: self.speed.into_enabled(default),
            agility: self.agility.into_enabled(default),
            align_time: self.align_time.into_enabled(default),
            sig_radius: self.sig_radius.into_enabled(default),
            mass: self.mass.into_enabled(default),
            warp_speed: self.warp_speed.into_enabled(default),
            max_warp_range: self.max_warp_range.into_enabled(default),
            jump: self.jump.into_enabled(default),
            // Ship misc stats
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
