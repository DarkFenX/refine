use crate::{
    ac,
    sol::{
        ItemId,
        svc::{calc::Calc, vast::VastFitData},
        uad::Uad,
    },
    util::RSet,
};

use super::shared::is_flag_set;

pub struct ValActivationBlockedFail {
    pub item_ids: std::collections::HashSet<ItemId>,
}

impl VastFitData {
    // Fast validations
    pub(in crate::sol::svc::vast) fn validate_activation_blocked_fast(
        &self,
        kfs: &RSet<ItemId>,
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
        kfs: &RSet<ItemId>,
        uad: &Uad,
        calc: &mut Calc,
    ) -> Option<ValActivationBlockedFail> {
        let item_ids = self
            .mods_active
            .difference(kfs)
            .filter(|v| is_flag_set(uad, calc, v, &ac::attrs::ACTIVATION_BLOCKED))
            .copied()
            .collect::<std::collections::HashSet<_>>();
        if item_ids.is_empty() {
            return None;
        }
        Some(ValActivationBlockedFail { item_ids })
    }
}
