use crate::{
    misc::Count,
    rd::RAttrId,
    svc::{
        SvcCtx,
        calc::Calc,
        vast::{VastFitData, shared::get_attr_as_count},
    },
    ud::{ItemId, UFit, UItemId},
    util::RSet,
};

pub struct ValUnusableSlotFail {
    /// How many slots available (when this validation fails, it's either None or 0).
    pub max: Option<Count>,
    /// IDs of items which would attempt to take those slots if you used them.
    pub users: Vec<ItemId>,
}

impl VastFitData {
    // Fast validations
    pub(in crate::svc::vast) fn validate_unlaunchable_drone_slot_fast(
        &self,
        kfs: &RSet<UItemId>,
        ctx: SvcCtx,
        calc: &mut Calc,
        fit: &UFit,
    ) -> bool {
        validate_fast(kfs, ctx, calc, fit.character, ctx.ac().max_active_drones, &fit.drones)
    }
    pub(in crate::svc::vast) fn validate_unlaunchable_fighter_fast(
        &self,
        kfs: &RSet<UItemId>,
        ctx: SvcCtx,
        calc: &mut Calc,
        fit: &UFit,
    ) -> bool {
        validate_fast(kfs, ctx, calc, fit.ship, ctx.ac().ftr_tubes, &fit.fighters)
    }
    pub(in crate::svc::vast) fn validate_unlaunchable_light_fighter_fast(
        &self,
        kfs: &RSet<UItemId>,
        ctx: SvcCtx,
        calc: &mut Calc,
        fit: &UFit,
    ) -> bool {
        validate_fast(kfs, ctx, calc, fit.ship, ctx.ac().ftr_light_slots, &self.light_fighters)
    }
    pub(in crate::svc::vast) fn validate_unlaunchable_heavy_fighter_fast(
        &self,
        kfs: &RSet<UItemId>,
        ctx: SvcCtx,
        calc: &mut Calc,
        fit: &UFit,
    ) -> bool {
        validate_fast(kfs, ctx, calc, fit.ship, ctx.ac().ftr_heavy_slots, &self.heavy_fighters)
    }
    pub(in crate::svc::vast) fn validate_unlaunchable_support_fighter_fast(
        &self,
        kfs: &RSet<UItemId>,
        ctx: SvcCtx,
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
    pub(in crate::svc::vast) fn validate_unlaunchable_st_light_fighter_fast(
        &self,
        kfs: &RSet<UItemId>,
        ctx: SvcCtx,
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
    pub(in crate::svc::vast) fn validate_unlaunchable_st_heavy_fighter_fast(
        &self,
        kfs: &RSet<UItemId>,
        ctx: SvcCtx,
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
    pub(in crate::svc::vast) fn validate_unlaunchable_st_support_fighter_fast(
        &self,
        kfs: &RSet<UItemId>,
        ctx: SvcCtx,
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
    pub(in crate::svc::vast) fn validate_unlaunchable_drone_slot_verbose(
        &self,
        kfs: &RSet<UItemId>,
        ctx: SvcCtx,
        calc: &mut Calc,
        fit: &UFit,
    ) -> Option<ValUnusableSlotFail> {
        validate_verbose(kfs, ctx, calc, fit.character, ctx.ac().max_active_drones, &fit.drones)
    }
    pub(in crate::svc::vast) fn validate_unlaunchable_fighter_verbose(
        &self,
        kfs: &RSet<UItemId>,
        ctx: SvcCtx,
        calc: &mut Calc,
        fit: &UFit,
    ) -> Option<ValUnusableSlotFail> {
        validate_verbose(kfs, ctx, calc, fit.ship, ctx.ac().ftr_tubes, &fit.fighters)
    }
    pub(in crate::svc::vast) fn validate_unlaunchable_light_fighter_verbose(
        &self,
        kfs: &RSet<UItemId>,
        ctx: SvcCtx,
        calc: &mut Calc,
        fit: &UFit,
    ) -> Option<ValUnusableSlotFail> {
        validate_verbose(kfs, ctx, calc, fit.ship, ctx.ac().ftr_light_slots, &self.light_fighters)
    }
    pub(in crate::svc::vast) fn validate_unlaunchable_heavy_fighter_verbose(
        &self,
        kfs: &RSet<UItemId>,
        ctx: SvcCtx,
        calc: &mut Calc,
        fit: &UFit,
    ) -> Option<ValUnusableSlotFail> {
        validate_verbose(kfs, ctx, calc, fit.ship, ctx.ac().ftr_heavy_slots, &self.heavy_fighters)
    }
    pub(in crate::svc::vast) fn validate_unlaunchable_support_fighter_verbose(
        &self,
        kfs: &RSet<UItemId>,
        ctx: SvcCtx,
        calc: &mut Calc,
        fit: &UFit,
    ) -> Option<ValUnusableSlotFail> {
        validate_verbose(
            kfs,
            ctx,
            calc,
            fit.ship,
            ctx.ac().ftr_support_slots,
            &self.support_fighters,
        )
    }
    pub(in crate::svc::vast) fn validate_unlaunchable_st_light_fighter_verbose(
        &self,
        kfs: &RSet<UItemId>,
        ctx: SvcCtx,
        calc: &mut Calc,
        fit: &UFit,
    ) -> Option<ValUnusableSlotFail> {
        validate_verbose(
            kfs,
            ctx,
            calc,
            fit.ship,
            ctx.ac().ftr_st_light_slots,
            &self.st_light_fighters,
        )
    }
    pub(in crate::svc::vast) fn validate_unlaunchable_st_heavy_fighter_verbose(
        &self,
        kfs: &RSet<UItemId>,
        ctx: SvcCtx,
        calc: &mut Calc,
        fit: &UFit,
    ) -> Option<ValUnusableSlotFail> {
        validate_verbose(
            kfs,
            ctx,
            calc,
            fit.ship,
            ctx.ac().ftr_st_heavy_slots,
            &self.st_heavy_fighters,
        )
    }
    pub(in crate::svc::vast) fn validate_unlaunchable_st_support_fighter_verbose(
        &self,
        kfs: &RSet<UItemId>,
        ctx: SvcCtx,
        calc: &mut Calc,
        fit: &UFit,
    ) -> Option<ValUnusableSlotFail> {
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
    ctx: SvcCtx,
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
    ctx: SvcCtx,
    calc: &mut Calc,
    max_item_uid: Option<UItemId>,
    max_attr_rid: Option<RAttrId>,
    users: &RSet<UItemId>,
) -> Option<ValUnusableSlotFail> {
    if users.is_empty() {
        return None;
    }
    let max = get_attr_as_count(ctx, calc, max_item_uid, max_attr_rid);
    if max.unwrap_or(Count::ZERO) > Count::ZERO {
        return None;
    }
    let users: Vec<_> = users
        .difference(kfs)
        .map(|item_uid| ctx.u_data.items.xid_by_iid(*item_uid))
        .collect();
    match users.is_empty() {
        true => None,
        false => Some(ValUnusableSlotFail { max, users }),
    }
}
