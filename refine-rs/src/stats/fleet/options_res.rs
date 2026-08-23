use crate::stats::{
    StatOptionExt, StatOptionFitDmg, StatOptionFitMining, StatOptionFitOutCps, StatOptionFitOutNps,
    StatOptionFitOutRps, StatOptionMass,
};

pub(in crate::stats) struct FleetStatsOptionsResolved {
    pub(super) dmg: Vec<StatOptionFitDmg>,
    pub(super) mps: Vec<StatOptionFitMining>,
    pub(super) outgoing_nps: Vec<StatOptionFitOutNps>,
    pub(super) outgoing_rps: Vec<StatOptionFitOutRps>,
    pub(super) outgoing_cps: Vec<StatOptionFitOutCps>,
    pub(super) mass: Vec<StatOptionMass>,
}
impl FleetStatsOptionsResolved {
    pub(super) fn from_default(default: bool) -> Self {
        Self {
            dmg: StatOptionExt::from_default(default),
            mps: StatOptionExt::from_default(default),
            outgoing_nps: StatOptionExt::from_default(default),
            outgoing_rps: StatOptionExt::from_default(default),
            outgoing_cps: StatOptionExt::from_default(default),
            mass: StatOptionExt::from_default(default),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Anything-requested check
////////////////////////////////////////////////////////////////////////////////////////////////////
impl FleetStatsOptionsResolved {
    pub(in crate::stats) fn is_any_stat_requested(&self) -> bool {
        !self.dmg.is_empty()
            || !self.mps.is_empty()
            || !self.outgoing_nps.is_empty()
            || !self.outgoing_rps.is_empty()
            || !self.outgoing_cps.is_empty()
            || !self.mass.is_empty()
    }
}
