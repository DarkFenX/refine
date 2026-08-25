use crate::{
    err::BrResolveError,
    stats::{
        StatOptionExt, StatOptionFitDmg, StatOptionFitMining, StatOptionFitOutCps, StatOptionFitOutNps,
        StatOptionFitOutRps, StatOptionInt, StatOptionMass,
    },
};

pub(in crate::stats) struct FleetStatsOptionsResolved {
    pub(super) dmg: Vec<Result<StatOptionFitDmg, BrResolveError>>,
    pub(super) mps: Vec<StatOptionFitMining>,
    pub(super) outgoing_nps: Vec<Result<StatOptionFitOutNps, BrResolveError>>,
    pub(super) outgoing_rps: Vec<Result<StatOptionFitOutRps, BrResolveError>>,
    pub(super) outgoing_cps: Vec<Result<StatOptionFitOutCps, BrResolveError>>,
    pub(super) mass: Vec<StatOptionMass>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Default + stat resolution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl FleetStatsOptionsResolved {
    pub(super) fn from_default(default: bool) -> Self {
        Self {
            dmg: StatOptionInt::from_default(default),
            mps: StatOptionExt::from_default(default),
            outgoing_nps: StatOptionInt::from_default(default),
            outgoing_rps: StatOptionInt::from_default(default),
            outgoing_cps: StatOptionInt::from_default(default),
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
