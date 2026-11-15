use crate::{
    Count,
    svc::{Calc, SvcCtx, funcs::is_oattr_flag_set, vast::VastFitData},
    ud::{UFit, UItemId},
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
pub struct ValCloakingBlockedFail {
    /// Cloaks which are active, but their activation is blocked by something.
    #[cfg_attr(feature = "serde", serde_as(as = "refine_serde::VecAsMap"))]
    pub modules: Vec<ValItemInfo>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Validators
////////////////////////////////////////////////////////////////////////////////////////////////////
impl VastFitData {
    // Fast validations
    pub(in crate::svc::vast::val) fn validate_cloaking_blocked_fast(
        &self,
        kfs: &RSet<UItemId>,
        ctx: &SvcCtx,
        calc: &mut Calc,
        fit: &UFit,
    ) -> bool {
        if !self.mods_active_cloaks.is_empty() && !self.can_fit_activate_cloaks(ctx, calc, fit.ship) {
            for espec in self.mods_active_cloaks.iter() {
                if !kfs.contains(&espec.item_uid) {
                    return false;
                }
            }
        }
        true
    }
    // Verbose validations
    pub(in crate::svc::vast::val) fn validate_cloaking_blocked_verbose(
        &self,
        kfs: &RSet<UItemId>,
        ctx: &SvcCtx,
        calc: &mut Calc,
        fit: &UFit,
    ) -> ValResult<ValCloakingBlockedFail> {
        let mut module_uids = RSet::new();
        if !self.mods_active_cloaks.is_empty() && !self.can_fit_activate_cloaks(ctx, calc, fit.ship) {
            for espec in self.mods_active_cloaks.iter() {
                module_uids.insert(espec.item_uid);
            }
        }
        let modules: Vec<_> = module_uids
            .into_iter()
            .map(|module_uid| ValItemInfo::from_uid(kfs, ctx, module_uid))
            .collect();
        match modules.is_empty() {
            true => ValResult::Passed,
            false => ValResult::from_fail(
                modules.iter().all(|v| v.status.is_kf()),
                ValCloakingBlockedFail { modules },
            ),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Helpers
////////////////////////////////////////////////////////////////////////////////////////////////////
impl VastFitData {
    fn can_fit_activate_cloaks(&self, ctx: &SvcCtx, calc: &mut Calc, ship_uid: Option<UItemId>) -> bool {
        if self.mods_fitted_cloaks > Count::ONE {
            return false;
        }
        if !self.mod_effects_disallow_cloak.is_empty() {
            return false;
        }
        if !self.mods_active_block_cloak.is_empty() {
            return false;
        }
        if let Some(ship_uid) = ship_uid
            && !can_ship_cloak(ctx, calc, ship_uid)
        {
            return false;
        }
        true
    }
}

fn can_ship_cloak(ctx: &SvcCtx, calc: &mut Calc, ship_uid: UItemId) -> bool {
    is_oattr_flag_set(ctx, calc, ship_uid, ctx.ac().can_cloak).unwrap_or(true)
        && !is_oattr_flag_set(ctx, calc, ship_uid, ctx.ac().disallow_cloaking).unwrap_or(false)
}
