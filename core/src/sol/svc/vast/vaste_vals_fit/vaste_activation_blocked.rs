use crate::{
    ac,
    sol::{
        ItemId,
        svc::{calc::Calc, vast::VastFitData},
        uad::Uad,
    },
    util::HSet,
};

use super::shared::is_flag_set;

pub struct ValActivationBlockedFail {
    pub item_id: ItemId,
}

impl VastFitData {
    // Fast validations
    pub(in crate::sol::svc::vast) fn validate_activation_blocked_fast(
        &self,
        kfs: &HSet<ItemId>,
        uad: &Uad,
        calc: &mut Calc,
    ) -> bool {
        self.mods_active
            .difference(kfs)
            .all(|v| !is_flag_set(uad, calc, v, &ac::attrs::ACTIVATION_BLOCKED))
    }
    // Verbose validations
    pub(in crate::sol::svc::vast) fn validate_activation_blocked_verbose(
        &self,
        kfs: &HSet<ItemId>,
        uad: &Uad,
        calc: &mut Calc,
    ) -> Vec<ValActivationBlockedFail> {
        self.mods_active
            .difference(kfs)
            .filter(|v| is_flag_set(uad, calc, v, &ac::attrs::ACTIVATION_BLOCKED))
            .map(|v| ValActivationBlockedFail { item_id: *v })
            .collect()
    }
}
