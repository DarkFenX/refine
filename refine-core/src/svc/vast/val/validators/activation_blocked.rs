use crate::{
    svc::{Calc, SvcCtx, funcs::is_attr_flag_set, vast::VastFitData},
    ud::UItemId,
    util::RSet,
    val::{ValItemInfo, ValResult},
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Result
////////////////////////////////////////////////////////////////////////////////////////////////////
#[cfg_attr(
    feature = "serde",
    cfg_eval,
    serde_with::serde_as,
    derive(serde::Serialize),
    serde(transparent)
)]
#[derive(Clone)]
pub struct ValActivationBlockedFail {
    /// Modules which are active, but their activation is blocked by something.
    #[cfg_attr(feature = "serde", serde_as(as = "refine_serde::VecAsMap"))]
    pub modules: Vec<ValItemInfo>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Validators
////////////////////////////////////////////////////////////////////////////////////////////////////
impl VastFitData {
    // Fast validations
    pub(in crate::svc::vast::val) fn validate_activation_blocked_fast(
        &self,
        kfs: &RSet<UItemId>,
        ctx: &SvcCtx,
        calc: &mut Calc,
    ) -> bool {
        if let Some(block_attr_rid) = ctx.ac().activation_blocked {
            for &item_uid in self.mods_active.iter() {
                if is_attr_flag_set(ctx, calc, item_uid, block_attr_rid).unwrap_or(false) && !kfs.contains(&item_uid) {
                    return false;
                }
            }
        }
        true
    }
    // Verbose validations
    pub(in crate::svc::vast::val) fn validate_activation_blocked_verbose(
        &self,
        kfs: &RSet<UItemId>,
        ctx: &SvcCtx,
        calc: &mut Calc,
    ) -> ValResult<ValActivationBlockedFail> {
        let mut modules = Vec::new();
        if let Some(block_attr_rid) = ctx.ac().activation_blocked {
            for &item_uid in self.mods_active.iter() {
                if is_attr_flag_set(ctx, calc, item_uid, block_attr_rid).unwrap_or(false) {
                    modules.push(ValItemInfo::from_uid(kfs, ctx, item_uid));
                }
            }
        }
        match modules.is_empty() {
            true => ValResult::Passed,
            false => ValResult::from_fail(
                modules.iter().all(|v| v.status.is_kf()),
                ValActivationBlockedFail { modules },
            ),
        }
    }
}
