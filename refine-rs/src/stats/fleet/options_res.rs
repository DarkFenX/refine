use crate::{
    err::BrResolveError,
    stats::{
        StatOptionExt, StatOptionFitDmg, StatOptionFitMining, StatOptionFitOutCps, StatOptionFitOutNps,
        StatOptionFitOutRps, StatOptionInt, StatOptionMass,
    },
};

pub(in crate::stats) struct FleetStatsOptionsResolved {
    pub(super) dmg: Option<Vec<Result<StatOptionFitDmg, BrResolveError>>>,
    pub(super) mps: Option<Vec<StatOptionFitMining>>,
    pub(super) outgoing_nps: Option<Vec<Result<StatOptionFitOutNps, BrResolveError>>>,
    pub(super) outgoing_rps: Option<Vec<Result<StatOptionFitOutRps, BrResolveError>>>,
    pub(super) outgoing_cps: Option<Vec<Result<StatOptionFitOutCps, BrResolveError>>>,
    pub(super) mass: Option<Vec<StatOptionMass>>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Default + stat resolution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl FleetStatsOptionsResolved {
    pub(super) fn blank_from_default(default: bool) -> Self {
        Self {
            dmg: StatOptionInt::blank_from_default(default),
            mps: StatOptionExt::blank_from_default(default),
            outgoing_nps: StatOptionInt::blank_from_default(default),
            outgoing_rps: StatOptionInt::blank_from_default(default),
            outgoing_cps: StatOptionInt::blank_from_default(default),
            mass: StatOptionExt::blank_from_default(default),
        }
    }
    pub(super) fn complete_extended_defaults(&mut self) {
        StatOptionInt::complete_blank_with_default(&mut self.dmg);
        StatOptionExt::complete_blank_with_default(&mut self.mps);
        StatOptionInt::complete_blank_with_default(&mut self.outgoing_nps);
        StatOptionInt::complete_blank_with_default(&mut self.outgoing_rps);
        StatOptionInt::complete_blank_with_default(&mut self.outgoing_cps);
        StatOptionExt::complete_blank_with_default(&mut self.mass);
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Anything-requested check
////////////////////////////////////////////////////////////////////////////////////////////////////
impl FleetStatsOptionsResolved {
    pub(in crate::stats) fn is_any_stat_requested(&self) -> bool {
        self.dmg.is_some()
            || self.mps.is_some()
            || self.outgoing_nps.is_some()
            || self.outgoing_rps.is_some()
            || self.outgoing_cps.is_some()
            || self.mass.is_some()
    }
}
