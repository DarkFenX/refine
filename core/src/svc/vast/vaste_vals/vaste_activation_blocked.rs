use super::shared::is_flag_set;
use crate::{
    ac,
    def::ItemId,
    svc::{SvcCtx, calc::Calc, vast::VastFitData},
    uad::UadItemKey,
    util::RSet,
};

pub struct ValActivationBlockedFail {
    /// Item IDs of modules which are active, but their activation is blocked by something.
    pub module_ids: Vec<ItemId>,
}

impl VastFitData {
    // Fast validations
    pub(in crate::svc::vast) fn validate_activation_blocked_fast(
        &self,
        kfs: &RSet<UadItemKey>,
        ctx: SvcCtx,
        calc: &mut Calc,
    ) -> bool {
        self.mods_active
            .difference(kfs)
            .all(|item_key| !is_flag_set(ctx, calc, *item_key, &ac::attrs::ACTIVATION_BLOCKED))
    }
    // Verbose validations
    pub(in crate::svc::vast) fn validate_activation_blocked_verbose(
        &self,
        kfs: &RSet<UadItemKey>,
        ctx: SvcCtx,
        calc: &mut Calc,
    ) -> Option<ValActivationBlockedFail> {
        let module_ids: Vec<_> = self
            .mods_active
            .difference(kfs)
            .filter(|item_key| is_flag_set(ctx, calc, **item_key, &ac::attrs::ACTIVATION_BLOCKED))
            .map(|item_key| ctx.uad.items.id_by_key(*item_key))
            .collect();
        match module_ids.is_empty() {
            true => None,
            false => Some(ValActivationBlockedFail { module_ids }),
        }
    }
}
