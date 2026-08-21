#[cfg(feature = "serde")]
use crate::stats::option::DeStatOptionKind;
use crate::{
    CmdResps, FitId, FitIdBr, ItemId, ItemIdBr,
    err::BrResolveError,
    stats::{
        StatOptionCapBlc, StatOptionCapSim, StatOptionEhp, StatOptionErps, StatOptionFitDmg, StatOptionFitMining,
        StatOptionFitOutCps, StatOptionFitOutNps, StatOptionFitOutRps, StatOptionIncomingJam, StatOptionJump,
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
pub(in crate::stats) struct FitStatsOptionsInt<O, F, I>
where
    O: StatOptionKind,
{
    // Fit output stats
    pub(super) dmg: StatOptionExtended<O, StatOptionFitDmg<I>>,
    pub(super) mps: StatOptionExtended<O, StatOptionFitMining>,
    pub(super) outgoing_nps: StatOptionExtended<O, StatOptionFitOutNps<I>>,
    pub(super) outgoing_rps: StatOptionExtended<O, StatOptionFitOutRps<I>>,
    pub(super) outgoing_cps: StatOptionExtended<O, StatOptionFitOutCps<I>>,
    // Fit resources
    pub(super) cpu: StatOptionRegular<O>,
    pub(super) powergrid: StatOptionRegular<O>,
    pub(super) calibration: StatOptionRegular<O>,
    pub(super) drone_bay_volume: StatOptionRegular<O>,
    pub(super) drone_bandwidth: StatOptionRegular<O>,
    pub(super) fighter_bay_volume: StatOptionRegular<O>,
    // Fit slots
    pub(super) high_slots: StatOptionRegular<O>,
    pub(super) mid_slots: StatOptionRegular<O>,
    pub(super) low_slots: StatOptionRegular<O>,
    pub(super) turret_slots: StatOptionRegular<O>,
    pub(super) launcher_slots: StatOptionRegular<O>,
    pub(super) rig_slots: StatOptionRegular<O>,
    pub(super) service_slots: StatOptionRegular<O>,
    pub(super) subsystem_slots: StatOptionRegular<O>,
    pub(super) launched_drones: StatOptionRegular<O>,
    pub(super) launched_fighters: StatOptionRegular<O>,
    pub(super) launched_light_fighters: StatOptionRegular<O>,
    pub(super) launched_heavy_fighters: StatOptionRegular<O>,
    pub(super) launched_support_fighters: StatOptionRegular<O>,
    pub(super) launched_st_light_fighters: StatOptionRegular<O>,
    pub(super) launched_st_heavy_fighters: StatOptionRegular<O>,
    pub(super) launched_st_support_fighters: StatOptionRegular<O>,
    // Ship tank
    pub(super) resists: StatOptionRegular<O>,
    pub(super) hp: StatOptionRegular<O>,
    pub(super) ehp: StatOptionExtended<O, StatOptionEhp>,
    pub(super) wc_ehp: StatOptionRegular<O>,
    pub(super) rps: StatOptionExtended<O, StatOptionRps>,
    pub(super) erps: StatOptionExtended<O, StatOptionErps>,
    pub(super) breach_resist: StatOptionRegular<O>,
    // Ship cap
    pub(super) cap_amount: StatOptionRegular<O>,
    pub(super) cap_balance: StatOptionExtended<O, StatOptionCapBlc<I>>,
    pub(super) cap_sim: StatOptionExtended<O, StatOptionCapSim<I>>,
    pub(super) neut_resist: StatOptionRegular<O>,
    // Ship sensors
    pub(super) locks: StatOptionRegular<O>,
    pub(super) lock_range: StatOptionRegular<O>,
    pub(super) scan_res: StatOptionRegular<O>,
    pub(super) sensors: StatOptionRegular<O>,
    pub(super) dscan_range: StatOptionRegular<O>,
    pub(super) probing_size: StatOptionRegular<O>,
    pub(super) incoming_jam: StatOptionExtended<O, StatOptionIncomingJam>,
    // Ship mobility
    pub(super) speed: StatOptionRegular<O>,
    pub(super) agility: StatOptionRegular<O>,
    pub(super) align_time: StatOptionRegular<O>,
    pub(super) sig_radius: StatOptionRegular<O>,
    pub(super) mass: StatOptionExtended<O, StatOptionMass>,
    pub(super) warp_speed: StatOptionRegular<O>,
    pub(super) max_warp_range: StatOptionRegular<O>,
    pub(super) jump: StatOptionExtended<O, StatOptionJump<F>>,
    // Ship misc stats
    pub(super) drone_control_range: StatOptionRegular<O>,
    pub(super) can_warp: StatOptionRegular<O>,
    pub(super) can_jump_gate: StatOptionRegular<O>,
    pub(super) can_jump_wormhole: StatOptionRegular<O>,
    pub(super) can_jump_drive: StatOptionRegular<O>,
    pub(super) can_dock_station: StatOptionRegular<O>,
    pub(super) can_dock_citadel: StatOptionRegular<O>,
    pub(super) can_tether: StatOptionRegular<O>,
}
impl<O, F, I> Default for FitStatsOptionsInt<O, F, I>
where
    O: StatOptionKind,
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
// Backref resolution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl FitStatsOptionsInt<StatOptionRaw, FitIdBr, ItemIdBr> {
    pub(super) fn br_resolve(
        self,
        resps: &CmdResps,
    ) -> Result<FitStatsOptionsInt<StatOptionRaw, FitId, ItemId>, BrResolveError> {
        Ok(FitStatsOptionsInt {
            // Fit output stats
            dmg: self.dmg.br_resolve(resps)?,
            mps: self.mps,
            outgoing_nps: self.outgoing_nps.br_resolve(resps)?,
            outgoing_rps: self.outgoing_rps.br_resolve(resps)?,
            outgoing_cps: self.outgoing_cps.br_resolve(resps)?,
            // Fit resources
            cpu: self.cpu,
            powergrid: self.powergrid,
            calibration: self.calibration,
            drone_bay_volume: self.drone_bay_volume,
            drone_bandwidth: self.drone_bandwidth,
            fighter_bay_volume: self.fighter_bay_volume,
            // Fit slots
            high_slots: self.high_slots,
            mid_slots: self.mid_slots,
            low_slots: self.low_slots,
            turret_slots: self.turret_slots,
            launcher_slots: self.launcher_slots,
            rig_slots: self.rig_slots,
            service_slots: self.service_slots,
            subsystem_slots: self.subsystem_slots,
            launched_drones: self.launched_drones,
            launched_fighters: self.launched_fighters,
            launched_light_fighters: self.launched_light_fighters,
            launched_heavy_fighters: self.launched_heavy_fighters,
            launched_support_fighters: self.launched_support_fighters,
            launched_st_light_fighters: self.launched_st_light_fighters,
            launched_st_heavy_fighters: self.launched_st_heavy_fighters,
            launched_st_support_fighters: self.launched_st_support_fighters,
            // Ship tank
            resists: self.resists,
            hp: self.hp,
            ehp: self.ehp,
            wc_ehp: self.wc_ehp,
            rps: self.rps,
            erps: self.erps,
            breach_resist: self.breach_resist,
            // Ship cap
            cap_amount: self.cap_amount,
            cap_balance: self.cap_balance.br_resolve(resps)?,
            cap_sim: self.cap_sim.br_resolve(resps)?,
            neut_resist: self.neut_resist,
            // Ship sensors
            locks: self.locks,
            lock_range: self.lock_range,
            scan_res: self.scan_res,
            sensors: self.sensors,
            dscan_range: self.dscan_range,
            probing_size: self.probing_size,
            incoming_jam: self.incoming_jam,
            // Ship mobility
            speed: self.speed,
            agility: self.agility,
            align_time: self.align_time,
            sig_radius: self.sig_radius,
            mass: self.mass,
            warp_speed: self.warp_speed,
            max_warp_range: self.max_warp_range,
            jump: self.jump.br_resolve(resps)?,
            // Ship misc stats
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
impl<F, I> FitStatsOptionsInt<StatOptionRaw, F, I> {
    pub(super) fn stat_resolve(self, default: bool) -> FitStatsOptionsInt<StatOptionResolved, F, I> {
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
