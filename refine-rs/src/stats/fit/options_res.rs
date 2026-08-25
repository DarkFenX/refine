use crate::{
    err::BrResolveError,
    stats::{
        StatOptionCapBlc, StatOptionCapSim, StatOptionEhp, StatOptionErps, StatOptionExt, StatOptionFitDmg,
        StatOptionFitMining, StatOptionFitOutCps, StatOptionFitOutNps, StatOptionFitOutRps, StatOptionIncomingJam,
        StatOptionInt, StatOptionJump, StatOptionMass, StatOptionRps,
    },
};

pub(in crate::stats) struct FitStatsOptionsResolved {
    // Fit output stats
    pub(super) dmg: Option<Vec<Result<StatOptionFitDmg, BrResolveError>>>,
    pub(super) mps: Option<Vec<StatOptionFitMining>>,
    pub(super) outgoing_nps: Option<Vec<Result<StatOptionFitOutNps, BrResolveError>>>,
    pub(super) outgoing_rps: Option<Vec<Result<StatOptionFitOutRps, BrResolveError>>>,
    pub(super) outgoing_cps: Option<Vec<Result<StatOptionFitOutCps, BrResolveError>>>,
    // Fit resources
    pub(super) cpu: bool,
    pub(super) powergrid: bool,
    pub(super) calibration: bool,
    pub(super) drone_bay_volume: bool,
    pub(super) drone_bandwidth: bool,
    pub(super) fighter_bay_volume: bool,
    // Fit slots
    pub(super) high_slots: bool,
    pub(super) mid_slots: bool,
    pub(super) low_slots: bool,
    pub(super) turret_slots: bool,
    pub(super) launcher_slots: bool,
    pub(super) rig_slots: bool,
    pub(super) service_slots: bool,
    pub(super) subsystem_slots: bool,
    pub(super) launched_drones: bool,
    pub(super) launched_fighters: bool,
    pub(super) launched_light_fighters: bool,
    pub(super) launched_heavy_fighters: bool,
    pub(super) launched_support_fighters: bool,
    pub(super) launched_st_light_fighters: bool,
    pub(super) launched_st_heavy_fighters: bool,
    pub(super) launched_st_support_fighters: bool,
    // Ship tank
    pub(super) resists: bool,
    pub(super) hp: bool,
    pub(super) ehp: Option<Vec<StatOptionEhp>>,
    pub(super) wc_ehp: bool,
    pub(super) rps: Option<Vec<StatOptionRps>>,
    pub(super) erps: Option<Vec<StatOptionErps>>,
    pub(super) breach_resist: bool,
    // Ship cap
    pub(super) cap_amount: bool,
    pub(super) cap_balance: Option<Vec<Result<StatOptionCapBlc, BrResolveError>>>,
    pub(super) cap_sim: Option<Vec<Result<StatOptionCapSim, BrResolveError>>>,
    pub(super) neut_resist: bool,
    // Ship sensors
    pub(super) locks: bool,
    pub(super) lock_range: bool,
    pub(super) scan_res: bool,
    pub(super) sensors: bool,
    pub(super) dscan_range: bool,
    pub(super) probing_size: bool,
    pub(super) incoming_jam: Option<Vec<StatOptionIncomingJam>>,
    // Ship mobility
    pub(super) speed: bool,
    pub(super) agility: bool,
    pub(super) align_time: bool,
    pub(super) sig_radius: bool,
    pub(super) mass: Option<Vec<StatOptionMass>>,
    pub(super) warp_speed: bool,
    pub(super) max_warp_range: bool,
    pub(super) jump: Option<Vec<StatOptionJump>>,
    // Ship misc stats
    pub(super) drone_control_range: bool,
    pub(super) can_warp: bool,
    pub(super) can_jump_gate: bool,
    pub(super) can_jump_wormhole: bool,
    pub(super) can_jump_drive: bool,
    pub(super) can_dock_station: bool,
    pub(super) can_dock_citadel: bool,
    pub(super) can_tether: bool,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Default + stat resolution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl FitStatsOptionsResolved {
    pub(super) fn blank_from_default(default: bool) -> Self {
        Self {
            // Fit output stats
            dmg: StatOptionInt::blank_from_default(default),
            mps: StatOptionExt::blank_from_default(default),
            outgoing_nps: StatOptionInt::blank_from_default(default),
            outgoing_rps: StatOptionInt::blank_from_default(default),
            outgoing_cps: StatOptionInt::blank_from_default(default),
            // Fit resources
            cpu: default,
            powergrid: default,
            calibration: default,
            drone_bay_volume: default,
            drone_bandwidth: default,
            fighter_bay_volume: default,
            // Fit slots
            high_slots: default,
            mid_slots: default,
            low_slots: default,
            turret_slots: default,
            launcher_slots: default,
            rig_slots: default,
            service_slots: default,
            subsystem_slots: default,
            launched_drones: default,
            launched_fighters: default,
            launched_light_fighters: default,
            launched_heavy_fighters: default,
            launched_support_fighters: default,
            launched_st_light_fighters: default,
            launched_st_heavy_fighters: default,
            launched_st_support_fighters: default,
            // Ship tank
            resists: default,
            hp: default,
            ehp: StatOptionExt::blank_from_default(default),
            wc_ehp: default,
            rps: StatOptionExt::blank_from_default(default),
            erps: StatOptionExt::blank_from_default(default),
            breach_resist: default,
            // Ship cap
            cap_amount: default,
            cap_balance: StatOptionInt::blank_from_default(default),
            cap_sim: StatOptionInt::blank_from_default(default),
            neut_resist: default,
            // Ship sensors
            locks: default,
            lock_range: default,
            scan_res: default,
            sensors: default,
            dscan_range: default,
            probing_size: default,
            incoming_jam: StatOptionExt::blank_from_default(default),
            // Ship mobility
            speed: default,
            agility: default,
            align_time: default,
            sig_radius: default,
            mass: StatOptionExt::blank_from_default(default),
            warp_speed: default,
            max_warp_range: default,
            jump: StatOptionExt::blank_from_default(default),
            // Ship misc stats
            drone_control_range: default,
            can_warp: default,
            can_jump_gate: default,
            can_jump_wormhole: default,
            can_jump_drive: default,
            can_dock_station: default,
            can_dock_citadel: default,
            can_tether: default,
        }
    }
    pub(super) fn complete_extended_defaults(&mut self) {
        StatOptionInt::complete_blank_with_default(&mut self.dmg);
        StatOptionExt::complete_blank_with_default(&mut self.mps);
        StatOptionInt::complete_blank_with_default(&mut self.outgoing_nps);
        StatOptionInt::complete_blank_with_default(&mut self.outgoing_rps);
        StatOptionInt::complete_blank_with_default(&mut self.outgoing_cps);
        StatOptionExt::complete_blank_with_default(&mut self.ehp);
        StatOptionExt::complete_blank_with_default(&mut self.rps);
        StatOptionExt::complete_blank_with_default(&mut self.erps);
        StatOptionInt::complete_blank_with_default(&mut self.cap_balance);
        StatOptionInt::complete_blank_with_default(&mut self.cap_sim);
        StatOptionExt::complete_blank_with_default(&mut self.incoming_jam);
        StatOptionExt::complete_blank_with_default(&mut self.mass);
        StatOptionExt::complete_blank_with_default(&mut self.jump);
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Anything-requested check
////////////////////////////////////////////////////////////////////////////////////////////////////
impl FitStatsOptionsResolved {
    pub(in crate::stats) fn is_any_stat_requested(&self) -> bool {
        // Fit output stats
        self.dmg.is_some()
            || self.mps.is_some()
            || self.outgoing_nps.is_some()
            || self.outgoing_rps.is_some()
            || self.outgoing_cps.is_some()
            // Fit resources
            || self.cpu
            || self.powergrid
            || self.calibration
            || self.drone_bay_volume
            || self.drone_bandwidth
            || self.fighter_bay_volume
            // Fit slots
            || self.high_slots
            || self.mid_slots
            || self.low_slots
            || self.turret_slots
            || self.launcher_slots
            || self.rig_slots
            || self.service_slots
            || self.subsystem_slots
            || self.launched_drones
            || self.launched_fighters
            || self.launched_light_fighters
            || self.launched_heavy_fighters
            || self.launched_support_fighters
            || self.launched_st_light_fighters
            || self.launched_st_heavy_fighters
            || self.launched_st_support_fighters
            // Ship tank
            || self.resists
            || self.hp
            || self.ehp.is_some()
            || self.wc_ehp
            || self.rps.is_some()
            || self.erps.is_some()
            || self.breach_resist
            // Ship cap
            || self.cap_amount
            || self.cap_balance.is_some()
            || self.cap_sim.is_some()
            || self.neut_resist
            // Ship sensors
            || self.locks
            || self.lock_range
            || self.scan_res
            || self.sensors
            || self.dscan_range
            || self.probing_size
            || self.incoming_jam.is_some()
            // Ship mobility
            || self.speed
            || self.agility
            || self.align_time
            || self.sig_radius
            || self.mass.is_some()
            || self.warp_speed
            || self.max_warp_range
            || self.jump.is_some()
            // Ship misc stats
            || self.drone_control_range
            || self.can_warp
            || self.can_jump_gate
            || self.can_jump_wormhole
            || self.can_jump_drive
            || self.can_dock_station
            || self.can_dock_citadel
            || self.can_tether
    }
}
