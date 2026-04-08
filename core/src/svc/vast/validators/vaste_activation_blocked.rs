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
        if let Some(block_attr_rid) = ctx.ac().activation_blocked {
            for item_uid in self.mods_active.iter() {
                if is_attr_flag_set(ctx, calc, *item_uid, block_attr_rid).unwrap_or(false) && !kfs.contains(item_uid) {
                    return false;
                }
            }
        }
        true
    }
    // Verbose validations
    pub(in crate::svc::vast) fn validate_activation_blocked_verbose(
        &self,
        kfs: &RSet<UItemId>,
        ctx: SvcCtx,
        calc: &mut Calc,
    ) -> Option<ValActivationBlockedFail> {
        let mut module_ids = Vec::new();
        if let Some(block_attr_rid) = ctx.ac().activation_blocked {
            for item_uid in self.mods_active.iter() {
                if is_attr_flag_set(ctx, calc, *item_uid, block_attr_rid).unwrap_or(false) && !kfs.contains(item_uid) {
                    module_ids.push(ctx.u_data.items.xid_by_iid(*item_uid));
                }
            }
        }
        match module_ids.is_empty() {
            true => None,
            false => Some(ValActivationBlockedFail { module_ids }),
        }
    }
}
