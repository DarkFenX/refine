use crate::{
    num::Count,
    svc::{SvcCtx, calc::Calc, funcs::is_oattr_flag_set, vast::VastFitData},
    ud::{ItemId, UFit, UItemId},
    util::RSet,
};

pub struct ValCloakingBlockedFail {
    /// Item IDs of cloaks which are active, but their activation is blocked by something.
    pub module_ids: Vec<ItemId>,
}

impl VastFitData {
    // Fast validations
    pub(in crate::svc::vast) fn validate_cloaking_blocked_fast(
        &self,
        kfs: &RSet<UItemId>,
        ctx: SvcCtx,
        calc: &mut Calc,
        fit: &UFit,
    ) -> bool {
        if !self.mods_active_cloaks.is_empty() && !can_fit_activate_cloaks(ctx, calc, fit.ship, self.mods_fitted_cloaks)
        {
            for espec in self.mods_active_cloaks.iter() {
                if !kfs.contains(&espec.item_uid) {
                    return false;
                }
            }
        }
        true
    }
    // Verbose validations
    pub(in crate::svc::vast) fn validate_cloaking_blocked_verbose(
        &self,
        kfs: &RSet<UItemId>,
        ctx: SvcCtx,
        calc: &mut Calc,
        fit: &UFit,
    ) -> Option<ValCloakingBlockedFail> {
        let mut module_ids = RSet::new();
        if !self.mods_active_cloaks.is_empty() && !can_fit_activate_cloaks(ctx, calc, fit.ship, self.mods_fitted_cloaks)
        {
            for espec in self.mods_active_cloaks.iter() {
                if !kfs.contains(&espec.item_uid) {
                    module_ids.insert(ctx.u_data.items.xid_by_iid(espec.item_uid));
                }
            }
        }
        match module_ids.is_empty() {
            true => None,
            false => Some(ValCloakingBlockedFail {
                module_ids: module_ids.into_iter().collect(),
            }),
        }
    }
}

fn can_fit_activate_cloaks(ctx: SvcCtx, calc: &mut Calc, ship_uid: Option<UItemId>, fitted_cloaks: Count) -> bool {
    if fitted_cloaks > Count::ONE {
        return false;
    }
    if let Some(ship_uid) = ship_uid
        && !can_ship_cloak(ctx, calc, ship_uid)
    {
        return false;
    }
    true
}

fn can_ship_cloak(ctx: SvcCtx, calc: &mut Calc, ship_uid: UItemId) -> bool {
    is_oattr_flag_set(ctx, calc, ship_uid, ctx.ac().can_cloak).unwrap_or(true)
        && !is_oattr_flag_set(ctx, calc, ship_uid, ctx.ac().disallow_cloaking).unwrap_or(false)
}
