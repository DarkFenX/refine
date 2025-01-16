use crate::{
    defs::{Amount, SolItemId},
    sol::{
        svc::{calc::SolCalc, vast::SolVastFitData},
        uad::{fit::SolFit, SolUad},
    },
};

pub struct SolSlotValFail {
    pub used: Amount,
    pub total: Option<Amount>,
    pub users: Vec<SolItemId>,
}
impl SolSlotValFail {
    fn new(used: Amount, total: Option<Amount>, users: Vec<SolItemId>) -> Self {
        Self { used, total, users }
    }
}

impl SolVastFitData {
    // Fast validations
    pub(in crate::sol::svc::vast) fn validate_rig_slots_fast(
        &self,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
    ) -> bool {
        let stats = self.get_stats_rig_slots(uad, calc, fit);
        stats.used <= stats.total.unwrap_or(0)
    }
    pub(in crate::sol::svc::vast) fn validate_subsystem_slots_fast(
        &self,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
    ) -> bool {
        let stats = self.get_stats_subsystem_slots(uad, calc, fit);
        stats.used <= stats.total.unwrap_or(0)
    }
    // Verbose validations
    pub(in crate::sol::svc::vast) fn validate_rig_slots_verbose(
        &self,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
    ) -> Option<SolSlotValFail> {
        let stats = self.get_stats_rig_slots(uad, calc, fit);
        if stats.used <= stats.total.unwrap_or(0) {
            return None;
        }
        let users = fit.rigs.iter().map(|v| *v).collect();
        Some(SolSlotValFail::new(stats.used, stats.total, users))
    }
    pub(in crate::sol::svc::vast) fn validate_subsystem_slots_verbose(
        &self,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
    ) -> Option<SolSlotValFail> {
        let stats = self.get_stats_subsystem_slots(uad, calc, fit);
        if stats.used <= stats.total.unwrap_or(0) {
            return None;
        }
        let users = fit.subsystems.iter().map(|v| *v).collect();
        Some(SolSlotValFail::new(stats.used, stats.total, users))
    }
    // Private methods
}
