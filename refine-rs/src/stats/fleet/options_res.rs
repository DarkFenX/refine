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
        let Self {
            dmg,
            mps,
            outgoing_nps,
            outgoing_rps,
            outgoing_cps,
            mass,
        } = self;
        StatOptionInt::complete_blank_with_default(dmg);
        StatOptionExt::complete_blank_with_default(mps);
        StatOptionInt::complete_blank_with_default(outgoing_nps);
        StatOptionInt::complete_blank_with_default(outgoing_rps);
        StatOptionInt::complete_blank_with_default(outgoing_cps);
        StatOptionExt::complete_blank_with_default(mass);
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Anything-requested check
////////////////////////////////////////////////////////////////////////////////////////////////////
impl FleetStatsOptionsResolved {
    pub(in crate::stats) fn is_any_stat_requested(&self) -> bool {
        let Self {
            dmg,
            mps,
            outgoing_nps,
            outgoing_rps,
            outgoing_cps,
            mass,
        } = self;
        dmg.is_some()
            || mps.is_some()
            || outgoing_nps.is_some()
            || outgoing_rps.is_some()
            || outgoing_cps.is_some()
            || mass.is_some()
    }
}
