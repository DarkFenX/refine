use super::shared::is_attr_flag_set;
use crate::{
    def::ItemId,
    svc::{SvcCtx, calc::Calc, vast::VastFitData},
    ud::UItemId,
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
        kfs: &RSet<UItemId>,
        ctx: SvcCtx,
        calc: &mut Calc,
    ) -> bool {
        let attr_key = match ctx.ac().activation_blocked {
            Some(attr_key) => attr_key,
            None => return true,
        };
        self.mods_active
            .difference(kfs)
            .all(|item_key| !is_attr_flag_set(ctx, calc, *item_key, attr_key))
    }
    // Verbose validations
    pub(in crate::svc::vast) fn validate_activation_blocked_verbose(
        &self,
        kfs: &RSet<UItemId>,
        ctx: SvcCtx,
        calc: &mut Calc,
    ) -> Option<ValActivationBlockedFail> {
        let attr_key = ctx.ac().activation_blocked?;
        let module_ids: Vec<_> = self
            .mods_active
            .difference(kfs)
            .filter(|item_key| is_attr_flag_set(ctx, calc, **item_key, attr_key))
            .map(|item_key| ctx.u_data.items.eid_by_iid(*item_key))
            .collect();
        match module_ids.is_empty() {
            true => None,
            false => Some(ValActivationBlockedFail { module_ids }),
        }
    }
}
