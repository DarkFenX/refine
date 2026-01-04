use crate::{
    def::{DefCount, Idx, ItemId},
    rd::RAttrId,
    svc::{
        SvcCtx,
        calc::Calc,
        vast::{VastFitData, shared::get_attr_as_count},
    },
    ud::{UFit, UItemId, UItemVec},
    util::{RMap, RSet},
};

pub struct ValSlotCountFail {
    /// How many slots are taken by all the relevant items.
    pub used: DefCount,
    /// How many slots available.
    pub max: Option<DefCount>,
    /// IDs of items which break the validation limits. For unordered containers - all items, for
    /// ordered containers - only those which go past limit.
    pub users: Vec<ItemId>,
}

impl VastFitData {
    // Fast validations
    pub(in crate::svc::vast) fn validate_high_slot_count_fast(
        &self,
        kfs: &RSet<UItemId>,
        ctx: SvcCtx,
        calc: &mut Calc,
        fit: &UFit,
    ) -> bool {
        validate_fast_ordered(kfs, ctx, calc, fit.ship, ctx.ac().hi_slots, &fit.mods_high)
    }
    pub(in crate::svc::vast) fn validate_mid_slot_count_fast(
        &self,
        kfs: &RSet<UItemId>,
        ctx: SvcCtx,
        calc: &mut Calc,
        fit: &UFit,
    ) -> bool {
        validate_fast_ordered(kfs, ctx, calc, fit.ship, ctx.ac().med_slots, &fit.mods_mid)
    }
    pub(in crate::svc::vast) fn validate_low_slot_count_fast(
        &self,
        kfs: &RSet<UItemId>,
        ctx: SvcCtx,
        calc: &mut Calc,
        fit: &UFit,
    ) -> bool {
        validate_fast_ordered(kfs, ctx, calc, fit.ship, ctx.ac().low_slots, &fit.mods_low)
    }
    pub(in crate::svc::vast) fn validate_turret_slot_count_fast(
        &self,
        kfs: &RSet<UItemId>,
        ctx: SvcCtx,
        calc: &mut Calc,
        fit: &UFit,
    ) -> bool {
        validate_fast_unordered_set(kfs, ctx, calc, fit.ship, ctx.ac().turret_slots_left, &self.mods_turret)
    }
    pub(in crate::svc::vast) fn validate_launcher_slot_count_fast(
        &self,
        kfs: &RSet<UItemId>,
        ctx: SvcCtx,
        calc: &mut Calc,
        fit: &UFit,
    ) -> bool {
        validate_fast_unordered_set(
            kfs,
            ctx,
            calc,
            fit.ship,
            ctx.ac().launcher_slots_left,
            &self.mods_launcher,
        )
    }
    pub(in crate::svc::vast) fn validate_rig_slot_count_fast(
        &self,
        kfs: &RSet<UItemId>,
        ctx: SvcCtx,
        calc: &mut Calc,
        fit: &UFit,
    ) -> bool {
        validate_fast_unordered_set(kfs, ctx, calc, fit.ship, ctx.ac().upgrade_slots_left, &fit.rigs)
    }
    pub(in crate::svc::vast) fn validate_service_slot_count_fast(
        &self,
        kfs: &RSet<UItemId>,
        ctx: SvcCtx,
        calc: &mut Calc,
        fit: &UFit,
    ) -> bool {
        validate_fast_unordered_set(kfs, ctx, calc, fit.ship, ctx.ac().service_slots, &fit.services)
    }
    pub(in crate::svc::vast) fn validate_subsystem_slot_count_fast(
        &self,
        kfs: &RSet<UItemId>,
        ctx: SvcCtx,
        calc: &mut Calc,
        fit: &UFit,
    ) -> bool {
        validate_fast_unordered_set(kfs, ctx, calc, fit.ship, ctx.ac().max_subsystems, &fit.subsystems)
    }
    pub(in crate::svc::vast) fn validate_launched_drone_count_fast(
        &self,
        kfs: &RSet<UItemId>,
        ctx: SvcCtx,
        calc: &mut Calc,
        fit: &UFit,
    ) -> bool {
        validate_fast_unordered_map(
            kfs,
            ctx,
            calc,
            fit.character,
            ctx.ac().max_active_drones,
            &self.drones_online_bandwidth,
        )
    }
    pub(in crate::svc::vast) fn validate_launched_fighter_count_fast(
        &self,
        kfs: &RSet<UItemId>,
        ctx: SvcCtx,
        calc: &mut Calc,
        fit: &UFit,
    ) -> bool {
        validate_fast_unordered_set(kfs, ctx, calc, fit.ship, ctx.ac().ftr_tubes, &self.fighters_online)
    }
    pub(in crate::svc::vast) fn validate_launched_light_fighter_count_fast(
        &self,
        kfs: &RSet<UItemId>,
        ctx: SvcCtx,
        calc: &mut Calc,
        fit: &UFit,
    ) -> bool {
        validate_fast_unordered_set(
            kfs,
            ctx,
            calc,
            fit.ship,
            ctx.ac().ftr_light_slots,
            &self.light_fighters_online,
        )
    }
    pub(in crate::svc::vast) fn validate_launched_heavy_fighter_count_fast(
        &self,
        kfs: &RSet<UItemId>,
        ctx: SvcCtx,
        calc: &mut Calc,
        fit: &UFit,
    ) -> bool {
        validate_fast_unordered_set(
            kfs,
            ctx,
            calc,
            fit.ship,
            ctx.ac().ftr_heavy_slots,
            &self.heavy_fighters_online,
        )
    }
    pub(in crate::svc::vast) fn validate_launched_support_fighter_count_fast(
        &self,
        kfs: &RSet<UItemId>,
        ctx: SvcCtx,
        calc: &mut Calc,
        fit: &UFit,
    ) -> bool {
        validate_fast_unordered_set(
            kfs,
            ctx,
            calc,
            fit.ship,
            ctx.ac().ftr_support_slots,
            &self.support_fighters_online,
        )
    }
    pub(in crate::svc::vast) fn validate_launched_st_light_fighter_count_fast(
        &self,
        kfs: &RSet<UItemId>,
        ctx: SvcCtx,
        calc: &mut Calc,
        fit: &UFit,
    ) -> bool {
        validate_fast_unordered_set(
            kfs,
            ctx,
            calc,
            fit.ship,
            ctx.ac().ftr_st_light_slots,
            &self.st_light_fighters_online,
        )
    }
    pub(in crate::svc::vast) fn validate_launched_st_heavy_fighter_count_fast(
        &self,
        kfs: &RSet<UItemId>,
        ctx: SvcCtx,
        calc: &mut Calc,
        fit: &UFit,
    ) -> bool {
        validate_fast_unordered_set(
            kfs,
            ctx,
            calc,
            fit.ship,
            ctx.ac().ftr_st_heavy_slots,
            &self.st_heavy_fighters_online,
        )
    }
    pub(in crate::svc::vast) fn validate_launched_st_support_fighter_count_fast(
        &self,
        kfs: &RSet<UItemId>,
        ctx: SvcCtx,
        calc: &mut Calc,
        fit: &UFit,
    ) -> bool {
        validate_fast_unordered_set(
            kfs,
            ctx,
            calc,
            fit.ship,
            ctx.ac().ftr_st_support_slots,
            &self.st_support_fighters_online,
        )
    }
    // Verbose validations
    pub(in crate::svc::vast) fn validate_high_slot_count_verbose(
        &self,
        kfs: &RSet<UItemId>,
        ctx: SvcCtx,
        calc: &mut Calc,
        fit: &UFit,
    ) -> Option<ValSlotCountFail> {
        validate_verbose_ordered(kfs, ctx, calc, fit.ship, ctx.ac().hi_slots, &fit.mods_high)
    }
    pub(in crate::svc::vast) fn validate_mid_slot_count_verbose(
        &self,
        kfs: &RSet<UItemId>,
        ctx: SvcCtx,
        calc: &mut Calc,
        fit: &UFit,
    ) -> Option<ValSlotCountFail> {
        validate_verbose_ordered(kfs, ctx, calc, fit.ship, ctx.ac().med_slots, &fit.mods_mid)
    }
    pub(in crate::svc::vast) fn validate_low_slot_count_verbose(
        &self,
        kfs: &RSet<UItemId>,
        ctx: SvcCtx,
        calc: &mut Calc,
        fit: &UFit,
    ) -> Option<ValSlotCountFail> {
        validate_verbose_ordered(kfs, ctx, calc, fit.ship, ctx.ac().low_slots, &fit.mods_low)
    }
    pub(in crate::svc::vast) fn validate_turret_slot_count_verbose(
        &self,
        kfs: &RSet<UItemId>,
        ctx: SvcCtx,
        calc: &mut Calc,
        fit: &UFit,
    ) -> Option<ValSlotCountFail> {
        validate_verbose_unordered_set(kfs, ctx, calc, fit.ship, ctx.ac().turret_slots_left, &self.mods_turret)
    }
    pub(in crate::svc::vast) fn validate_launcher_slot_count_verbose(
        &self,
        kfs: &RSet<UItemId>,
        ctx: SvcCtx,
        calc: &mut Calc,
        fit: &UFit,
    ) -> Option<ValSlotCountFail> {
        validate_verbose_unordered_set(
            kfs,
            ctx,
            calc,
            fit.ship,
            ctx.ac().launcher_slots_left,
            &self.mods_launcher,
        )
    }
    pub(in crate::svc::vast) fn validate_rig_slot_count_verbose(
        &self,
        kfs: &RSet<UItemId>,
        ctx: SvcCtx,
        calc: &mut Calc,
        fit: &UFit,
    ) -> Option<ValSlotCountFail> {
        validate_verbose_unordered_set(kfs, ctx, calc, fit.ship, ctx.ac().upgrade_slots_left, &fit.rigs)
    }
    pub(in crate::svc::vast) fn validate_service_slot_count_verbose(
        &self,
        kfs: &RSet<UItemId>,
        ctx: SvcCtx,
        calc: &mut Calc,
        fit: &UFit,
    ) -> Option<ValSlotCountFail> {
        validate_verbose_unordered_set(kfs, ctx, calc, fit.ship, ctx.ac().service_slots, &fit.services)
    }
    pub(in crate::svc::vast) fn validate_subsystem_slot_count_verbose(
        &self,
        kfs: &RSet<UItemId>,
        ctx: SvcCtx,
        calc: &mut Calc,
        fit: &UFit,
    ) -> Option<ValSlotCountFail> {
        validate_verbose_unordered_set(kfs, ctx, calc, fit.ship, ctx.ac().max_subsystems, &fit.subsystems)
    }
    pub(in crate::svc::vast) fn validate_launched_drone_count_verbose(
        &self,
        kfs: &RSet<UItemId>,
        ctx: SvcCtx,
        calc: &mut Calc,
        fit: &UFit,
    ) -> Option<ValSlotCountFail> {
        validate_verbose_unordered_map(
            kfs,
            ctx,
            calc,
            fit.character,
            ctx.ac().max_active_drones,
            &self.drones_online_bandwidth,
        )
    }
    pub(in crate::svc::vast) fn validate_launched_fighter_count_verbose(
        &self,
        kfs: &RSet<UItemId>,
        ctx: SvcCtx,
        calc: &mut Calc,
        fit: &UFit,
    ) -> Option<ValSlotCountFail> {
        validate_verbose_unordered_set(kfs, ctx, calc, fit.ship, ctx.ac().ftr_tubes, &self.fighters_online)
    }
    pub(in crate::svc::vast) fn validate_launched_light_fighter_count_verbose(
        &self,
        kfs: &RSet<UItemId>,
        ctx: SvcCtx,
        calc: &mut Calc,
        fit: &UFit,
    ) -> Option<ValSlotCountFail> {
        validate_verbose_unordered_set(
            kfs,
            ctx,
            calc,
            fit.ship,
            ctx.ac().ftr_light_slots,
            &self.light_fighters_online,
        )
    }
    pub(in crate::svc::vast) fn validate_launched_heavy_fighter_count_verbose(
        &self,
        kfs: &RSet<UItemId>,
        ctx: SvcCtx,
        calc: &mut Calc,
        fit: &UFit,
    ) -> Option<ValSlotCountFail> {
        validate_verbose_unordered_set(
            kfs,
            ctx,
            calc,
            fit.ship,
            ctx.ac().ftr_heavy_slots,
            &self.heavy_fighters_online,
        )
    }
    pub(in crate::svc::vast) fn validate_launched_support_fighter_count_verbose(
        &self,
        kfs: &RSet<UItemId>,
        ctx: SvcCtx,
        calc: &mut Calc,
        fit: &UFit,
    ) -> Option<ValSlotCountFail> {
        validate_verbose_unordered_set(
            kfs,
            ctx,
            calc,
            fit.ship,
            ctx.ac().ftr_support_slots,
            &self.support_fighters_online,
        )
    }
    pub(in crate::svc::vast) fn validate_launched_st_light_fighter_count_verbose(
        &self,
        kfs: &RSet<UItemId>,
        ctx: SvcCtx,
        calc: &mut Calc,
        fit: &UFit,
    ) -> Option<ValSlotCountFail> {
        validate_verbose_unordered_set(
            kfs,
            ctx,
            calc,
            fit.ship,
            ctx.ac().ftr_st_light_slots,
            &self.st_light_fighters_online,
        )
    }
    pub(in crate::svc::vast) fn validate_launched_st_heavy_fighter_count_verbose(
        &self,
        kfs: &RSet<UItemId>,
        ctx: SvcCtx,
        calc: &mut Calc,
        fit: &UFit,
    ) -> Option<ValSlotCountFail> {
        validate_verbose_unordered_set(
            kfs,
            ctx,
            calc,
            fit.ship,
            ctx.ac().ftr_st_heavy_slots,
            &self.st_heavy_fighters_online,
        )
    }
    pub(in crate::svc::vast) fn validate_launched_st_support_fighter_count_verbose(
        &self,
        kfs: &RSet<UItemId>,
        ctx: SvcCtx,
        calc: &mut Calc,
        fit: &UFit,
    ) -> Option<ValSlotCountFail> {
        validate_verbose_unordered_set(
            kfs,
            ctx,
            calc,
            fit.ship,
            ctx.ac().ftr_st_support_slots,
            &self.st_support_fighters_online,
        )
    }
}

fn validate_fast_unordered_set(
    kfs: &RSet<UItemId>,
    ctx: SvcCtx,
    calc: &mut Calc,
    max_item_key: Option<UItemId>,
    max_attr_key: Option<RAttrId>,
    users: &RSet<UItemId>,
) -> bool {
    let used = users.len() as DefCount;
    let max = get_attr_as_count(ctx, calc, max_item_key, max_attr_key).unwrap_or(0);
    used <= max || users.is_subset(kfs)
}
fn validate_fast_unordered_map<T>(
    kfs: &RSet<UItemId>,
    ctx: SvcCtx,
    calc: &mut Calc,
    max_item_key: Option<UItemId>,
    max_attr_key: Option<RAttrId>,
    users: &RMap<UItemId, T>,
) -> bool {
    let used = users.len() as DefCount;
    let max = get_attr_as_count(ctx, calc, max_item_key, max_attr_key).unwrap_or(0);
    used <= max || users.is_subset(kfs)
}
fn validate_fast_ordered(
    kfs: &RSet<UItemId>,
    ctx: SvcCtx,
    calc: &mut Calc,
    max_item_key: Option<UItemId>,
    max_attr_key: Option<RAttrId>,
    users: &UItemVec,
) -> bool {
    let used = users.len() as DefCount;
    let max = get_attr_as_count(ctx, calc, max_item_key, max_attr_key).unwrap_or(0);
    match kfs.is_empty() {
        true => used <= max,
        false => match used <= max {
            true => true,
            false => users.iter_keys_from(max as Idx).all(|v| kfs.contains(v)),
        },
    }
}

fn validate_verbose_unordered_set(
    kfs: &RSet<UItemId>,
    ctx: SvcCtx,
    calc: &mut Calc,
    max_item_key: Option<UItemId>,
    max_attr_key: Option<RAttrId>,
    users: &RSet<UItemId>,
) -> Option<ValSlotCountFail> {
    let used = users.len() as DefCount;
    let max = get_attr_as_count(ctx, calc, max_item_key, max_attr_key);
    if used <= max.unwrap_or(0) {
        return None;
    }
    let users: Vec<_> = users
        .difference(kfs)
        .map(|item_key| ctx.u_data.items.eid_by_iid(*item_key))
        .collect();
    match users.is_empty() {
        true => None,
        false => Some(ValSlotCountFail { used, max, users }),
    }
}
fn validate_verbose_unordered_map<T>(
    kfs: &RSet<UItemId>,
    ctx: SvcCtx,
    calc: &mut Calc,
    max_item_key: Option<UItemId>,
    max_attr_key: Option<RAttrId>,
    users: &RMap<UItemId, T>,
) -> Option<ValSlotCountFail> {
    let used = users.len() as DefCount;
    let max = get_attr_as_count(ctx, calc, max_item_key, max_attr_key);
    if used <= max.unwrap_or(0) {
        return None;
    }
    let users: Vec<_> = users
        .difference(kfs)
        .map(|(item_key, _)| ctx.u_data.items.eid_by_iid(*item_key))
        .collect();
    match users.is_empty() {
        true => None,
        false => Some(ValSlotCountFail { used, max, users }),
    }
}
fn validate_verbose_ordered(
    kfs: &RSet<UItemId>,
    ctx: SvcCtx,
    calc: &mut Calc,
    max_item_key: Option<UItemId>,
    max_attr_key: Option<RAttrId>,
    users: &UItemVec,
) -> Option<ValSlotCountFail> {
    let used = users.len() as DefCount;
    let max = get_attr_as_count(ctx, calc, max_item_key, max_attr_key);
    let effective_max = max.unwrap_or(0);
    if used <= effective_max {
        return None;
    }
    let users: Vec<_> = users
        .iter_keys_from(effective_max as Idx)
        .filter(|item_key| !kfs.contains(item_key))
        .map(|item_key| ctx.u_data.items.eid_by_iid(*item_key))
        .collect();
    match users.is_empty() {
        true => None,
        false => Some(ValSlotCountFail { used, max, users }),
    }
}
