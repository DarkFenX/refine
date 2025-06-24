use super::shared::is_flag_set;
use crate::{
    ac,
    sol::{
        ItemId, ItemKey,
        svc::{calc::Calc, eprojs::EProjs, vast::VastFitData},
        uad::Uad,
    },
    util::RSet,
};

pub struct ValActivationBlockedFail {
    /// Item IDs of modules which are active, but their activation is blocked by something.
    pub module_ids: Vec<ItemId>,
}

impl VastFitData {
    // Fast validations
    pub(in crate::sol::svc::vast) fn validate_activation_blocked_fast(
        &self,
        kfs: &RSet<ItemKey>,
        uad: &Uad,
        eprojs: &EProjs,
        calc: &mut Calc,
    ) -> bool {
        self.mods_active
            .difference(kfs)
            .all(|item_key| !is_flag_set(uad, eprojs, calc, *item_key, &ac::attrs::ACTIVATION_BLOCKED))
    }
    // Verbose validations
    pub(in crate::sol::svc::vast) fn validate_activation_blocked_verbose(
        &self,
        kfs: &RSet<ItemKey>,
        uad: &Uad,
        eprojs: &EProjs,
        calc: &mut Calc,
    ) -> Option<ValActivationBlockedFail> {
        let module_ids: Vec<_> = self
            .mods_active
            .difference(kfs)
            .filter(|item_key| is_flag_set(uad, eprojs, calc, **item_key, &ac::attrs::ACTIVATION_BLOCKED))
            .map(|item_key| uad.items.id_by_key(*item_key))
            .collect();
        match module_ids.is_empty() {
            true => None,
            false => Some(ValActivationBlockedFail { module_ids }),
        }
    }
}
