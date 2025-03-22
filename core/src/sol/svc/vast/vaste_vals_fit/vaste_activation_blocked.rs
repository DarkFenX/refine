use crate::{
    consts,
    defs::SolItemId,
    sol::{
        svc::{calc::SolCalc, vast::SolVastFitData},
        uad::SolUad,
    },
    util::StSet,
};

use super::shared::is_flag_set;

pub struct SolValActivationBlockedFail {
    pub item_id: SolItemId,
}

impl SolVastFitData {
    // Fast validations
    pub(in crate::sol::svc::vast) fn validate_activation_blocked_fast(
        &self,
        kfs: &StSet<SolItemId>,
        uad: &SolUad,
        calc: &mut SolCalc,
    ) -> bool {
        self.mods_active
            .difference(kfs)
            .all(|v| !is_flag_set(uad, calc, v, &consts::attrs::ACTIVATION_BLOCKED))
    }
    // Verbose validations
    pub(in crate::sol::svc::vast) fn validate_activation_blocked_verbose(
        &self,
        kfs: &StSet<SolItemId>,
        uad: &SolUad,
        calc: &mut SolCalc,
    ) -> Vec<SolValActivationBlockedFail> {
        self.mods_active
            .difference(kfs)
            .filter(|v| is_flag_set(uad, calc, v, &consts::attrs::ACTIVATION_BLOCKED))
            .map(|v| SolValActivationBlockedFail { item_id: *v })
            .collect()
    }
}
