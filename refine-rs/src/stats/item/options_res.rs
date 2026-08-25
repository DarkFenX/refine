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
    pub(super) dmg: Vec<Result<StatOptionItemDmg, BrResolveError>>,
    pub(super) mps: Vec<StatOptionItemMining>,
    pub(super) outgoing_nps: Vec<Result<StatOptionItemOutNps, BrResolveError>>,
    pub(super) outgoing_rps: Vec<Result<StatOptionItemOutRps, BrResolveError>>,
    pub(super) outgoing_cps: Vec<Result<StatOptionItemOutCps, BrResolveError>>,
    // Tank
    pub(super) resists: bool,
    pub(super) hp: bool,
    pub(super) ehp: Vec<StatOptionEhp>,
    pub(super) wc_ehp: bool,
    pub(super) rps: Vec<StatOptionRps>,
    pub(super) erps: Vec<StatOptionErps>,
    pub(super) breach_resist: bool,
    // Cap
    pub(super) cap_amount: bool,
    pub(super) cap_balance: Vec<Result<StatOptionCapBlc, BrResolveError>>,
    pub(super) cap_sim: Vec<Result<StatOptionCapSim, BrResolveError>>,
    pub(super) neut_resist: bool,
    // Sensors
    pub(super) locks: bool,
    pub(super) lock_range: bool,
    pub(super) scan_res: bool,
    pub(super) sensors: bool,
    pub(super) dscan_range: bool,
    pub(super) probing_size: bool,
    pub(super) incoming_jam: Vec<StatOptionIncomingJam>,
    // Mobility
    pub(super) speed: bool,
    pub(super) agility: bool,
    pub(super) align_time: bool,
    pub(super) sig_radius: bool,
    pub(super) mass: Vec<StatOptionMass>,
    pub(super) warp_speed: bool,
    pub(super) max_warp_range: bool,
    pub(super) jump: Vec<StatOptionJump>,
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
impl ItemStatsOptionsResolved {
    pub(super) fn from_default(default: bool) -> Self {
        Self {
            // Output
            dmg: StatOptionInt::from_default(default),
            mps: StatOptionExt::from_default(default),
            outgoing_nps: StatOptionInt::from_default(default),
            outgoing_rps: StatOptionInt::from_default(default),
            outgoing_cps: StatOptionInt::from_default(default),
            // Tank
            resists: default,
            hp: default,
            ehp: StatOptionExt::from_default(default),
            wc_ehp: default,
            rps: StatOptionExt::from_default(default),
            erps: StatOptionExt::from_default(default),
            breach_resist: default,
            // Cap
            cap_amount: default,
            cap_balance: StatOptionInt::from_default(default),
            cap_sim: StatOptionInt::from_default(default),
            neut_resist: default,
            // Sensors
            locks: default,
            lock_range: default,
            scan_res: default,
            sensors: default,
            dscan_range: default,
            probing_size: default,
            incoming_jam: StatOptionExt::from_default(default),
            // Mobility
            speed: default,
            agility: default,
            align_time: default,
            sig_radius: default,
            mass: StatOptionExt::from_default(default),
            warp_speed: default,
            max_warp_range: default,
            jump: StatOptionExt::from_default(default),
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
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Anything-requested check
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ItemStatsOptionsResolved {
    pub(in crate::stats) fn is_any_stat_requested(&self) -> bool {
        // Output
        !self.dmg.is_empty()
            || !self.mps.is_empty()
            || !self.outgoing_nps.is_empty()
            || !self.outgoing_rps.is_empty()
            || !self.outgoing_cps.is_empty()
            // Tank
            || self.resists
            || self.hp
            || !self.ehp.is_empty()
            || self.wc_ehp
            || !self.rps.is_empty()
            || !self.erps.is_empty()
            || self.breach_resist
            // Cap
            || self.cap_amount
            || !self.cap_balance.is_empty()
            || !self.cap_sim.is_empty()
            || self.neut_resist
            // Sensors
            || self.locks
            || self.lock_range
            || self.scan_res
            || self.sensors
            || self.dscan_range
            || self.probing_size
            || !self.incoming_jam.is_empty()
            // Mobility
            || self.speed
            || self.agility
            || self.align_time
            || self.sig_radius
            || !self.mass.is_empty()
            || self.warp_speed
            || self.max_warp_range
            || !self.jump.is_empty()
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
