use crate::{
    Count,
    rd::RAttrId,
    svc::{
        Calc, SvcCtx,
        vast::{VastFitData, shared::get_attr_as_count},
    },
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
    derive(serde_tuple::Serialize_tuple)
)]
#[derive(Clone)]
pub struct ValUnusableSlotFail {
    /// How many slots available (when this validation fails, it's either None or 0).
    pub max: Option<Count>,
    /// Items which would attempt to take those slots if you used them.
    #[cfg_attr(feature = "serde", serde_as(as = "refine_serde::VecAsMap"))]
    pub users: Vec<ValItemInfo>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Validators
////////////////////////////////////////////////////////////////////////////////////////////////////
impl VastFitData {
    // Fast validations
    pub(in crate::svc::vast::val) fn validate_unlaunchable_drone_slot_fast(
        &self,
        kfs: &RSet<UItemId>,
        ctx: &SvcCtx,
        calc: &mut Calc,
        fit: &UFit,
    ) -> bool {
        validate_fast(kfs, ctx, calc, fit.character, ctx.ac().max_active_drones, &fit.drones)
    }
    pub(in crate::svc::vast::val) fn validate_unlaunchable_fighter_fast(
        &self,
        kfs: &RSet<UItemId>,
        ctx: &SvcCtx,
        calc: &mut Calc,
        fit: &UFit,
    ) -> bool {
        validate_fast(kfs, ctx, calc, fit.ship, ctx.ac().ftr_tubes, &fit.fighters)
    }
    pub(in crate::svc::vast::val) fn validate_unlaunchable_light_fighter_fast(
        &self,
        kfs: &RSet<UItemId>,
        ctx: &SvcCtx,
        calc: &mut Calc,
        fit: &UFit,
    ) -> bool {
        validate_fast(kfs, ctx, calc, fit.ship, ctx.ac().ftr_light_slots, &self.light_fighters)
    }
    pub(in crate::svc::vast::val) fn validate_unlaunchable_heavy_fighter_fast(
        &self,
        kfs: &RSet<UItemId>,
        ctx: &SvcCtx,
        calc: &mut Calc,
        fit: &UFit,
    ) -> bool {
        validate_fast(kfs, ctx, calc, fit.ship, ctx.ac().ftr_heavy_slots, &self.heavy_fighters)
    }
    pub(in crate::svc::vast::val) fn validate_unlaunchable_support_fighter_fast(
        &self,
        kfs: &RSet<UItemId>,
        ctx: &SvcCtx,
        calc: &mut Calc,
        fit: &UFit,
    ) -> bool {
        validate_fast(
            kfs,
            ctx,
            calc,
            fit.ship,
            ctx.ac().ftr_support_slots,
            &self.support_fighters,
        )
    }
    pub(in crate::svc::vast::val) fn validate_unlaunchable_st_light_fighter_fast(
        &self,
        kfs: &RSet<UItemId>,
        ctx: &SvcCtx,
        calc: &mut Calc,
        fit: &UFit,
    ) -> bool {
        validate_fast(
            kfs,
            ctx,
            calc,
            fit.ship,
            ctx.ac().ftr_st_light_slots,
            &self.st_light_fighters,
        )
    }
    pub(in crate::svc::vast::val) fn validate_unlaunchable_st_heavy_fighter_fast(
        &self,
        kfs: &RSet<UItemId>,
        ctx: &SvcCtx,
        calc: &mut Calc,
        fit: &UFit,
    ) -> bool {
        validate_fast(
            kfs,
            ctx,
            calc,
            fit.ship,
            ctx.ac().ftr_st_heavy_slots,
            &self.st_heavy_fighters,
        )
    }
    pub(in crate::svc::vast::val) fn validate_unlaunchable_st_support_fighter_fast(
        &self,
        kfs: &RSet<UItemId>,
        ctx: &SvcCtx,
        calc: &mut Calc,
        fit: &UFit,
    ) -> bool {
        validate_fast(
            kfs,
            ctx,
            calc,
            fit.ship,
            ctx.ac().ftr_st_support_slots,
            &self.st_support_fighters,
        )
    }
    // Verbose validations
    pub(in crate::svc::vast::val) fn validate_unlaunchable_drone_slot_verbose(
        &self,
        kfs: &RSet<UItemId>,
        ctx: &SvcCtx,
        calc: &mut Calc,
        fit: &UFit,
    ) -> ValResult<ValUnusableSlotFail> {
        validate_verbose(kfs, ctx, calc, fit.character, ctx.ac().max_active_drones, &fit.drones)
    }
    pub(in crate::svc::vast::val) fn validate_unlaunchable_fighter_verbose(
        &self,
        kfs: &RSet<UItemId>,
        ctx: &SvcCtx,
        calc: &mut Calc,
        fit: &UFit,
    ) -> ValResult<ValUnusableSlotFail> {
        validate_verbose(kfs, ctx, calc, fit.ship, ctx.ac().ftr_tubes, &fit.fighters)
    }
    pub(in crate::svc::vast::val) fn validate_unlaunchable_light_fighter_verbose(
        &self,
        kfs: &RSet<UItemId>,
        ctx: &SvcCtx,
        calc: &mut Calc,
        fit: &UFit,
    ) -> ValResult<ValUnusableSlotFail> {
        validate_verbose(kfs, ctx, calc, fit.ship, ctx.ac().ftr_light_slots, &self.light_fighters)
    }
    pub(in crate::svc::vast::val) fn validate_unlaunchable_heavy_fighter_verbose(
        &self,
        kfs: &RSet<UItemId>,
        ctx: &SvcCtx,
        calc: &mut Calc,
        fit: &UFit,
    ) -> ValResult<ValUnusableSlotFail> {
        validate_verbose(kfs, ctx, calc, fit.ship, ctx.ac().ftr_heavy_slots, &self.heavy_fighters)
    }
    pub(in crate::svc::vast::val) fn validate_unlaunchable_support_fighter_verbose(
        &self,
        kfs: &RSet<UItemId>,
        ctx: &SvcCtx,
        calc: &mut Calc,
        fit: &UFit,
    ) -> ValResult<ValUnusableSlotFail> {
        validate_verbose(
            kfs,
            ctx,
            calc,
            fit.ship,
            ctx.ac().ftr_support_slots,
            &self.support_fighters,
        )
    }
    pub(in crate::svc::vast::val) fn validate_unlaunchable_st_light_fighter_verbose(
        &self,
        kfs: &RSet<UItemId>,
        ctx: &SvcCtx,
        calc: &mut Calc,
        fit: &UFit,
    ) -> ValResult<ValUnusableSlotFail> {
        validate_verbose(
            kfs,
            ctx,
            calc,
            fit.ship,
            ctx.ac().ftr_st_light_slots,
            &self.st_light_fighters,
        )
    }
    pub(in crate::svc::vast::val) fn validate_unlaunchable_st_heavy_fighter_verbose(
        &self,
        kfs: &RSet<UItemId>,
        ctx: &SvcCtx,
        calc: &mut Calc,
        fit: &UFit,
    ) -> ValResult<ValUnusableSlotFail> {
        validate_verbose(
            kfs,
            ctx,
            calc,
            fit.ship,
            ctx.ac().ftr_st_heavy_slots,
            &self.st_heavy_fighters,
        )
    }
    pub(in crate::svc::vast::val) fn validate_unlaunchable_st_support_fighter_verbose(
        &self,
        kfs: &RSet<UItemId>,
        ctx: &SvcCtx,
        calc: &mut Calc,
        fit: &UFit,
    ) -> ValResult<ValUnusableSlotFail> {
        validate_verbose(
            kfs,
            ctx,
            calc,
            fit.ship,
            ctx.ac().ftr_st_support_slots,
            &self.st_support_fighters,
        )
    }
}

fn validate_fast(
    kfs: &RSet<UItemId>,
    ctx: &SvcCtx,
    calc: &mut Calc,
    max_item_uid: Option<UItemId>,
    max_attr_rid: Option<RAttrId>,
    users: &RSet<UItemId>,
) -> bool {
    if users.is_empty() {
        return true;
    }
    let max = get_attr_as_count(ctx, calc, max_item_uid, max_attr_rid).unwrap_or(Count::ZERO);
    if max > Count::ZERO {
        return true;
    }
    users.is_subset(kfs)
}
fn validate_verbose(
    kfs: &RSet<UItemId>,
    ctx: &SvcCtx,
    calc: &mut Calc,
    max_item_uid: Option<UItemId>,
    max_attr_rid: Option<RAttrId>,
    users: &RSet<UItemId>,
) -> ValResult<ValUnusableSlotFail> {
    if users.is_empty() {
        return ValResult::Passed;
    }
    let max = get_attr_as_count(ctx, calc, max_item_uid, max_attr_rid);
    if max.unwrap_or(Count::ZERO) > Count::ZERO {
        return ValResult::Passed;
    }
    let users: Vec<_> = users
        .iter()
        .map(|&item_uid| ValItemInfo::from_uid(kfs, ctx, item_uid))
        .collect();
    ValResult::from_fail(
        users.iter().all(|v| v.status.is_kf()),
        ValUnusableSlotFail { max, users },
    )
}
