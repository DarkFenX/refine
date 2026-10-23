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
        let Self {
            dmg,
            mps,
            outgoing_nps,
            outgoing_rps,
            outgoing_cps,
            ehp,
            rps,
            erps,
            cap_balance,
            cap_sim,
            incoming_jam,
            mass,
            jump,
            // Flags have no extended options to complete
            cpu: _,
            powergrid: _,
            calibration: _,
            drone_bay_volume: _,
            drone_bandwidth: _,
            fighter_bay_volume: _,
            high_slots: _,
            mid_slots: _,
            low_slots: _,
            turret_slots: _,
            launcher_slots: _,
            rig_slots: _,
            service_slots: _,
            subsystem_slots: _,
            launched_drones: _,
            launched_fighters: _,
            launched_light_fighters: _,
            launched_heavy_fighters: _,
            launched_support_fighters: _,
            launched_st_light_fighters: _,
            launched_st_heavy_fighters: _,
            launched_st_support_fighters: _,
            resists: _,
            hp: _,
            wc_ehp: _,
            breach_resist: _,
            cap_amount: _,
            neut_resist: _,
            locks: _,
            lock_range: _,
            scan_res: _,
            sensors: _,
            dscan_range: _,
            probing_size: _,
            speed: _,
            agility: _,
            align_time: _,
            sig_radius: _,
            warp_speed: _,
            max_warp_range: _,
            drone_control_range: _,
            can_warp: _,
            can_jump_gate: _,
            can_jump_wormhole: _,
            can_jump_drive: _,
            can_dock_station: _,
            can_dock_citadel: _,
            can_tether: _,
        } = self;
        StatOptionInt::complete_blank_with_default(dmg);
        StatOptionExt::complete_blank_with_default(mps);
        StatOptionInt::complete_blank_with_default(outgoing_nps);
        StatOptionInt::complete_blank_with_default(outgoing_rps);
        StatOptionInt::complete_blank_with_default(outgoing_cps);
        StatOptionExt::complete_blank_with_default(ehp);
        StatOptionExt::complete_blank_with_default(rps);
        StatOptionExt::complete_blank_with_default(erps);
        StatOptionInt::complete_blank_with_default(cap_balance);
        StatOptionInt::complete_blank_with_default(cap_sim);
        StatOptionExt::complete_blank_with_default(incoming_jam);
        StatOptionExt::complete_blank_with_default(mass);
        StatOptionExt::complete_blank_with_default(jump);
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Anything-requested check
////////////////////////////////////////////////////////////////////////////////////////////////////
impl FitStatsOptionsResolved {
    pub(in crate::stats) fn is_any_stat_requested(&self) -> bool {
        let Self {
            dmg,
            mps,
            outgoing_nps,
            outgoing_rps,
            outgoing_cps,
            cpu,
            powergrid,
            calibration,
            drone_bay_volume,
            drone_bandwidth,
            fighter_bay_volume,
            high_slots,
            mid_slots,
            low_slots,
            turret_slots,
            launcher_slots,
            rig_slots,
            service_slots,
            subsystem_slots,
            launched_drones,
            launched_fighters,
            launched_light_fighters,
            launched_heavy_fighters,
            launched_support_fighters,
            launched_st_light_fighters,
            launched_st_heavy_fighters,
            launched_st_support_fighters,
            resists,
            hp,
            ehp,
            wc_ehp,
            rps,
            erps,
            breach_resist,
            cap_amount,
            cap_balance,
            cap_sim,
            neut_resist,
            locks,
            lock_range,
            scan_res,
            sensors,
            dscan_range,
            probing_size,
            incoming_jam,
            speed,
            agility,
            align_time,
            sig_radius,
            mass,
            warp_speed,
            max_warp_range,
            jump,
            drone_control_range,
            can_warp,
            can_jump_gate,
            can_jump_wormhole,
            can_jump_drive,
            can_dock_station,
            can_dock_citadel,
            can_tether,
        } = self;
        // Fit output stats
        dmg.is_some()
            || mps.is_some()
            || outgoing_nps.is_some()
            || outgoing_rps.is_some()
            || outgoing_cps.is_some()
            // Fit resources
            || *cpu
            || *powergrid
            || *calibration
            || *drone_bay_volume
            || *drone_bandwidth
            || *fighter_bay_volume
            // Fit slots
            || *high_slots
            || *mid_slots
            || *low_slots
            || *turret_slots
            || *launcher_slots
            || *rig_slots
            || *service_slots
            || *subsystem_slots
            || *launched_drones
            || *launched_fighters
            || *launched_light_fighters
            || *launched_heavy_fighters
            || *launched_support_fighters
            || *launched_st_light_fighters
            || *launched_st_heavy_fighters
            || *launched_st_support_fighters
            // Ship tank
            || *resists
            || *hp
            || ehp.is_some()
            || *wc_ehp
            || rps.is_some()
            || erps.is_some()
            || *breach_resist
            // Ship cap
            || *cap_amount
            || cap_balance.is_some()
            || cap_sim.is_some()
            || *neut_resist
            // Ship sensors
            || *locks
            || *lock_range
            || *scan_res
            || *sensors
            || *dscan_range
            || *probing_size
            || incoming_jam.is_some()
            // Ship mobility
            || *speed
            || *agility
            || *align_time
            || *sig_radius
            || mass.is_some()
            || *warp_speed
            || *max_warp_range
            || jump.is_some()
            // Ship misc stats
            || *drone_control_range
            || *can_warp
            || *can_jump_gate
            || *can_jump_wormhole
            || *can_jump_drive
            || *can_dock_station
            || *can_dock_citadel
            || *can_tether
    }
}
