use crate::stats::{
    StatOptionCapBlc, StatOptionCapSim, StatOptionEhp, StatOptionErps, StatOptionExt, StatOptionFitDmg,
    StatOptionFitMining, StatOptionFitOutCps, StatOptionFitOutNps, StatOptionFitOutRps, StatOptionIncomingJam,
    StatOptionJump, StatOptionMass, StatOptionRps,
};

pub(in crate::stats) struct FitStatsOptionsResolved {
    // Fit output stats
    pub(super) dmg: Vec<StatOptionFitDmg>,
    pub(super) mps: Vec<StatOptionFitMining>,
    pub(super) outgoing_nps: Vec<StatOptionFitOutNps>,
    pub(super) outgoing_rps: Vec<StatOptionFitOutRps>,
    pub(super) outgoing_cps: Vec<StatOptionFitOutCps>,
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
    pub(super) ehp: Vec<StatOptionEhp>,
    pub(super) wc_ehp: bool,
    pub(super) rps: Vec<StatOptionRps>,
    pub(super) erps: Vec<StatOptionErps>,
    pub(super) breach_resist: bool,
    // Ship cap
    pub(super) cap_amount: bool,
    pub(super) cap_balance: Vec<StatOptionCapBlc>,
    pub(super) cap_sim: Vec<StatOptionCapSim>,
    pub(super) neut_resist: bool,
    // Ship sensors
    pub(super) locks: bool,
    pub(super) lock_range: bool,
    pub(super) scan_res: bool,
    pub(super) sensors: bool,
    pub(super) dscan_range: bool,
    pub(super) probing_size: bool,
    pub(super) incoming_jam: Vec<StatOptionIncomingJam>,
    // Ship mobility
    pub(super) speed: bool,
    pub(super) agility: bool,
    pub(super) align_time: bool,
    pub(super) sig_radius: bool,
    pub(super) mass: Vec<StatOptionMass>,
    pub(super) warp_speed: bool,
    pub(super) max_warp_range: bool,
    pub(super) jump: Vec<StatOptionJump>,
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
impl FitStatsOptionsResolved {
    pub(super) fn from_default(default: bool) -> Self {
        Self {
            // Fit output stats
            dmg: StatOptionExt::stat_default(default),
            mps: StatOptionExt::stat_default(default),
            outgoing_nps: StatOptionExt::stat_default(default),
            outgoing_rps: StatOptionExt::stat_default(default),
            outgoing_cps: StatOptionExt::stat_default(default),
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
            ehp: StatOptionExt::stat_default(default),
            wc_ehp: default,
            rps: StatOptionExt::stat_default(default),
            erps: StatOptionExt::stat_default(default),
            breach_resist: default,
            // Ship cap
            cap_amount: default,
            cap_balance: StatOptionExt::stat_default(default),
            cap_sim: StatOptionExt::stat_default(default),
            neut_resist: default,
            // Ship sensors
            locks: default,
            lock_range: default,
            scan_res: default,
            sensors: default,
            dscan_range: default,
            probing_size: default,
            incoming_jam: StatOptionExt::stat_default(default),
            // Ship mobility
            speed: default,
            agility: default,
            align_time: default,
            sig_radius: default,
            mass: StatOptionExt::stat_default(default),
            warp_speed: default,
            max_warp_range: default,
            jump: StatOptionExt::stat_default(default),
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
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Anything-requested check
////////////////////////////////////////////////////////////////////////////////////////////////////
impl FitStatsOptionsResolved {
    pub(in crate::stats) fn is_any_stat_requested(&self) -> bool {
        // Fit output stats
        !self.dmg.is_empty()
            || !self.mps.is_empty()
            || !self.outgoing_nps.is_empty()
            || !self.outgoing_rps.is_empty()
            || !self.outgoing_cps.is_empty()
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
            || !self.ehp.is_empty()
            || self.wc_ehp
            || !self.rps.is_empty()
            || !self.erps.is_empty()
            || self.breach_resist
            // Ship cap
            || self.cap_amount
            || !self.cap_balance.is_empty()
            || !self.cap_sim.is_empty()
            || self.neut_resist
            // Ship sensors
            || self.locks
            || self.lock_range
            || self.scan_res
            || self.sensors
            || self.dscan_range
            || self.probing_size
            || !self.incoming_jam.is_empty()
            // Ship mobility
            || self.speed
            || self.agility
            || self.align_time
            || self.sig_radius
            || !self.mass.is_empty()
            || self.warp_speed
            || self.max_warp_range
            || !self.jump.is_empty()
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
