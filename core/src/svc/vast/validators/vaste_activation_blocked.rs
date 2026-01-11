use super::shared::is_attr_flag_set;
use crate::{
    svc::{SvcCtx, calc::Calc, vast::VastFitData},
    ud::{ItemId, UItemId},
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
        let attr_rid = match ctx.ac().activation_blocked {
            Some(attr_rid) => attr_rid,
            None => return true,
        };
        self.mods_active
            .difference(kfs)
            .all(|item_uid| !is_attr_flag_set(ctx, calc, *item_uid, attr_rid))
    }
    // Verbose validations
    pub(in crate::svc::vast) fn validate_activation_blocked_verbose(
        &self,
        kfs: &RSet<UItemId>,
        ctx: SvcCtx,
        calc: &mut Calc,
    ) -> Option<ValActivationBlockedFail> {
        let attr_rid = ctx.ac().activation_blocked?;
        let module_ids: Vec<_> = self
            .mods_active
            .difference(kfs)
            .filter(|item_uid| is_attr_flag_set(ctx, calc, **item_uid, attr_rid))
            .map(|item_uid| ctx.u_data.items.xid_by_iid(*item_uid))
            .collect();
        match module_ids.is_empty() {
            true => None,
            false => Some(ValActivationBlockedFail { module_ids }),
        }
    }
}
