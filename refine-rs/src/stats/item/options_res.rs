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
impl ItemStatsOptionsResolved {
    pub(in crate::stats) fn is_any_stat_requested(&self) -> bool {
        // Output
        self.dmg.is_some()
            || self.mps.is_some()
            || self.outgoing_nps.is_some()
            || self.outgoing_rps.is_some()
            || self.outgoing_cps.is_some()
            // Tank
            || self.resists
            || self.hp
            || self.ehp.is_some()
            || self.wc_ehp
            || self.rps.is_some()
            || self.erps.is_some()
            || self.breach_resist
            // Cap
            || self.cap_amount
            || self.cap_balance.is_some()
            || self.cap_sim.is_some()
            || self.neut_resist
            // Sensors
            || self.locks
            || self.lock_range
            || self.scan_res
            || self.sensors
            || self.dscan_range
            || self.probing_size
            || self.incoming_jam.is_some()
            // Mobility
            || self.speed
            || self.agility
            || self.align_time
            || self.sig_radius
            || self.mass.is_some()
            || self.warp_speed
            || self.max_warp_range
            || self.jump.is_some()
            // Misc
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
