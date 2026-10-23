use crate::{
    err::BrResolveError,
    stats::{
        StatOptionCapBlc, StatOptionCapSim, StatOptionEhp, StatOptionErps, StatOptionExt, StatOptionIncomingJam,
        StatOptionInt, StatOptionItemDmg, StatOptionItemMining, StatOptionItemOutCps, StatOptionItemOutNps,
        StatOptionItemOutRps, StatOptionJump, StatOptionMass, StatOptionRps,
    },
};

pub(in crate::stats) struct ItemStatsOptionsResolved {
    // Output
    pub(super) dmg: Option<Vec<Result<StatOptionItemDmg, BrResolveError>>>,
    pub(super) mps: Option<Vec<StatOptionItemMining>>,
    pub(super) outgoing_nps: Option<Vec<Result<StatOptionItemOutNps, BrResolveError>>>,
    pub(super) outgoing_rps: Option<Vec<Result<StatOptionItemOutRps, BrResolveError>>>,
    pub(super) outgoing_cps: Option<Vec<Result<StatOptionItemOutCps, BrResolveError>>>,
    // Tank
    pub(super) resists: bool,
    pub(super) hp: bool,
    pub(super) ehp: Option<Vec<StatOptionEhp>>,
    pub(super) wc_ehp: bool,
    pub(super) rps: Option<Vec<StatOptionRps>>,
    pub(super) erps: Option<Vec<StatOptionErps>>,
    pub(super) breach_resist: bool,
    // Cap
    pub(super) cap_amount: bool,
    pub(super) cap_balance: Option<Vec<Result<StatOptionCapBlc, BrResolveError>>>,
    pub(super) cap_sim: Option<Vec<Result<StatOptionCapSim, BrResolveError>>>,
    pub(super) neut_resist: bool,
    // Sensors
    pub(super) locks: bool,
    pub(super) lock_range: bool,
    pub(super) scan_res: bool,
    pub(super) sensors: bool,
    pub(super) dscan_range: bool,
    pub(super) probing_size: bool,
    pub(super) incoming_jam: Option<Vec<StatOptionIncomingJam>>,
    // Mobility
    pub(super) speed: bool,
    pub(super) agility: bool,
    pub(super) align_time: bool,
    pub(super) sig_radius: bool,
    pub(super) mass: Option<Vec<StatOptionMass>>,
    pub(super) warp_speed: bool,
    pub(super) max_warp_range: bool,
    pub(super) jump: Option<Vec<StatOptionJump>>,
    // Misc
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
impl ItemStatsOptionsResolved {
    pub(super) fn blank_from_default(default: bool) -> Self {
        Self {
            // Output
            dmg: StatOptionInt::blank_from_default(default),
            mps: StatOptionExt::blank_from_default(default),
            outgoing_nps: StatOptionInt::blank_from_default(default),
            outgoing_rps: StatOptionInt::blank_from_default(default),
            outgoing_cps: StatOptionInt::blank_from_default(default),
            // Tank
            resists: default,
            hp: default,
            ehp: StatOptionExt::blank_from_default(default),
            wc_ehp: default,
            rps: StatOptionExt::blank_from_default(default),
            erps: StatOptionExt::blank_from_default(default),
            breach_resist: default,
            // Cap
            cap_amount: default,
            cap_balance: StatOptionInt::blank_from_default(default),
            cap_sim: StatOptionInt::blank_from_default(default),
            neut_resist: default,
            // Sensors
            locks: default,
            lock_range: default,
            scan_res: default,
            sensors: default,
            dscan_range: default,
            probing_size: default,
            incoming_jam: StatOptionExt::blank_from_default(default),
            // Mobility
            speed: default,
            agility: default,
            align_time: default,
            sig_radius: default,
            mass: StatOptionExt::blank_from_default(default),
            warp_speed: default,
            max_warp_range: default,
            jump: StatOptionExt::blank_from_default(default),
            // Misc
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
impl ItemStatsOptionsResolved {
    pub(in crate::stats) fn is_any_stat_requested(&self) -> bool {
        let Self {
            dmg,
            mps,
            outgoing_nps,
            outgoing_rps,
            outgoing_cps,
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
        // Output
        dmg.is_some()
            || mps.is_some()
            || outgoing_nps.is_some()
            || outgoing_rps.is_some()
            || outgoing_cps.is_some()
            // Tank
            || *resists
            || *hp
            || ehp.is_some()
            || *wc_ehp
            || rps.is_some()
            || erps.is_some()
            || *breach_resist
            // Cap
            || *cap_amount
            || cap_balance.is_some()
            || cap_sim.is_some()
            || *neut_resist
            // Sensors
            || *locks
            || *lock_range
            || *scan_res
            || *sensors
            || *dscan_range
            || *probing_size
            || incoming_jam.is_some()
            // Mobility
            || *speed
            || *agility
            || *align_time
            || *sig_radius
            || mass.is_some()
            || *warp_speed
            || *max_warp_range
            || jump.is_some()
            // Misc
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
