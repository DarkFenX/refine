use itertools::Itertools;

use crate::{
    Count,
    rd::RAttrId,
    svc::{
        Calc, SvcCtx,
        vast::{VastFitData, shared::get_attr_as_count},
    },
    ud::{UFit, UItemId, UItemVec},
    util::{RMap, RSet},
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
pub struct ValSlotCountFail {
    /// How many slots are taken by all the relevant items.
    pub used: Count,
    /// How many slots available.
    pub max: Option<Count>,
    /// Items which fail the validation. For unordered containers - all items, for ordered
    /// containers - only those which go past the slot limit.
    #[cfg_attr(feature = "serde", serde_as(as = "refine_serde::VecAsMap"))]
    pub users: Vec<ValItemInfo>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Validators
////////////////////////////////////////////////////////////////////////////////////////////////////
impl VastFitData {
    // Fast validations
    pub(in crate::svc::vast::val) fn validate_high_slot_count_fast(
        &self,
        kfs: &RSet<UItemId>,
        ctx: &SvcCtx,
        calc: &mut Calc,
        fit: &UFit,
    ) -> bool {
        validate_fast_ordered(kfs, ctx, calc, fit.ship, ctx.ac().hi_slots, &fit.mods_high)
    }
    pub(in crate::svc::vast::val) fn validate_mid_slot_count_fast(
        &self,
        kfs: &RSet<UItemId>,
        ctx: &SvcCtx,
        calc: &mut Calc,
        fit: &UFit,
    ) -> bool {
        validate_fast_ordered(kfs, ctx, calc, fit.ship, ctx.ac().med_slots, &fit.mods_mid)
    }
    pub(in crate::svc::vast::val) fn validate_low_slot_count_fast(
        &self,
        kfs: &RSet<UItemId>,
        ctx: &SvcCtx,
        calc: &mut Calc,
        fit: &UFit,
    ) -> bool {
        validate_fast_ordered(kfs, ctx, calc, fit.ship, ctx.ac().low_slots, &fit.mods_low)
    }
    pub(in crate::svc::vast::val) fn validate_turret_slot_count_fast(
        &self,
        kfs: &RSet<UItemId>,
        ctx: &SvcCtx,
        calc: &mut Calc,
        fit: &UFit,
    ) -> bool {
        validate_fast_unordered_set(kfs, ctx, calc, fit.ship, ctx.ac().turret_slots_left, &self.mods_turret)
    }
    pub(in crate::svc::vast::val) fn validate_launcher_slot_count_fast(
        &self,
        kfs: &RSet<UItemId>,
        ctx: &SvcCtx,
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
    pub(in crate::svc::vast::val) fn validate_rig_slot_count_fast(
        &self,
        kfs: &RSet<UItemId>,
        ctx: &SvcCtx,
        calc: &mut Calc,
        fit: &UFit,
    ) -> bool {
        validate_fast_unordered_set(kfs, ctx, calc, fit.ship, ctx.ac().upgrade_slots_left, &fit.rigs)
    }
    pub(in crate::svc::vast::val) fn validate_service_slot_count_fast(
        &self,
        kfs: &RSet<UItemId>,
        ctx: &SvcCtx,
        calc: &mut Calc,
        fit: &UFit,
    ) -> bool {
        validate_fast_unordered_set(kfs, ctx, calc, fit.ship, ctx.ac().service_slots, &fit.services)
    }
    pub(in crate::svc::vast::val) fn validate_subsystem_slot_count_fast(
        &self,
        kfs: &RSet<UItemId>,
        ctx: &SvcCtx,
        calc: &mut Calc,
        fit: &UFit,
    ) -> bool {
        validate_fast_unordered_set(kfs, ctx, calc, fit.ship, ctx.ac().max_subsystems, &fit.subsystems)
    }
    pub(in crate::svc::vast::val) fn validate_launched_drone_count_fast(
        &self,
        kfs: &RSet<UItemId>,
        ctx: &SvcCtx,
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
    pub(in crate::svc::vast::val) fn validate_launched_fighter_count_fast(
        &self,
        kfs: &RSet<UItemId>,
        ctx: &SvcCtx,
        calc: &mut Calc,
        fit: &UFit,
    ) -> bool {
        validate_fast_unordered_set(kfs, ctx, calc, fit.ship, ctx.ac().ftr_tubes, &self.fighters_online)
    }
    pub(in crate::svc::vast::val) fn validate_launched_light_fighter_count_fast(
        &self,
        kfs: &RSet<UItemId>,
        ctx: &SvcCtx,
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
    pub(in crate::svc::vast::val) fn validate_launched_heavy_fighter_count_fast(
        &self,
        kfs: &RSet<UItemId>,
        ctx: &SvcCtx,
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
    pub(in crate::svc::vast::val) fn validate_launched_support_fighter_count_fast(
        &self,
        kfs: &RSet<UItemId>,
        ctx: &SvcCtx,
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
    pub(in crate::svc::vast::val) fn validate_launched_st_light_fighter_count_fast(
        &self,
        kfs: &RSet<UItemId>,
        ctx: &SvcCtx,
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
    pub(in crate::svc::vast::val) fn validate_launched_st_heavy_fighter_count_fast(
        &self,
        kfs: &RSet<UItemId>,
        ctx: &SvcCtx,
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
    pub(in crate::svc::vast::val) fn validate_launched_st_support_fighter_count_fast(
        &self,
        kfs: &RSet<UItemId>,
        ctx: &SvcCtx,
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
    pub(in crate::svc::vast::val) fn validate_high_slot_count_verbose(
        &self,
        kfs: &RSet<UItemId>,
        ctx: &SvcCtx,
        calc: &mut Calc,
        fit: &UFit,
    ) -> ValResult<ValSlotCountFail> {
        validate_verbose_ordered(kfs, ctx, calc, fit.ship, ctx.ac().hi_slots, &fit.mods_high)
    }
    pub(in crate::svc::vast::val) fn validate_mid_slot_count_verbose(
        &self,
        kfs: &RSet<UItemId>,
        ctx: &SvcCtx,
        calc: &mut Calc,
        fit: &UFit,
    ) -> ValResult<ValSlotCountFail> {
        validate_verbose_ordered(kfs, ctx, calc, fit.ship, ctx.ac().med_slots, &fit.mods_mid)
    }
    pub(in crate::svc::vast::val) fn validate_low_slot_count_verbose(
        &self,
        kfs: &RSet<UItemId>,
        ctx: &SvcCtx,
        calc: &mut Calc,
        fit: &UFit,
    ) -> ValResult<ValSlotCountFail> {
        validate_verbose_ordered(kfs, ctx, calc, fit.ship, ctx.ac().low_slots, &fit.mods_low)
    }
    pub(in crate::svc::vast::val) fn validate_turret_slot_count_verbose(
        &self,
        kfs: &RSet<UItemId>,
        ctx: &SvcCtx,
        calc: &mut Calc,
        fit: &UFit,
    ) -> ValResult<ValSlotCountFail> {
        validate_verbose_unordered_set(kfs, ctx, calc, fit.ship, ctx.ac().turret_slots_left, &self.mods_turret)
    }
    pub(in crate::svc::vast::val) fn validate_launcher_slot_count_verbose(
        &self,
        kfs: &RSet<UItemId>,
        ctx: &SvcCtx,
        calc: &mut Calc,
        fit: &UFit,
    ) -> ValResult<ValSlotCountFail> {
        validate_verbose_unordered_set(
            kfs,
            ctx,
            calc,
            fit.ship,
            ctx.ac().launcher_slots_left,
            &self.mods_launcher,
        )
    }
    pub(in crate::svc::vast::val) fn validate_rig_slot_count_verbose(
        &self,
        kfs: &RSet<UItemId>,
        ctx: &SvcCtx,
        calc: &mut Calc,
        fit: &UFit,
    ) -> ValResult<ValSlotCountFail> {
        validate_verbose_unordered_set(kfs, ctx, calc, fit.ship, ctx.ac().upgrade_slots_left, &fit.rigs)
    }
    pub(in crate::svc::vast::val) fn validate_service_slot_count_verbose(
        &self,
        kfs: &RSet<UItemId>,
        ctx: &SvcCtx,
        calc: &mut Calc,
        fit: &UFit,
    ) -> ValResult<ValSlotCountFail> {
        validate_verbose_unordered_set(kfs, ctx, calc, fit.ship, ctx.ac().service_slots, &fit.services)
    }
    pub(in crate::svc::vast::val) fn validate_subsystem_slot_count_verbose(
        &self,
        kfs: &RSet<UItemId>,
        ctx: &SvcCtx,
        calc: &mut Calc,
        fit: &UFit,
    ) -> ValResult<ValSlotCountFail> {
        validate_verbose_unordered_set(kfs, ctx, calc, fit.ship, ctx.ac().max_subsystems, &fit.subsystems)
    }
    pub(in crate::svc::vast::val) fn validate_launched_drone_count_verbose(
        &self,
        kfs: &RSet<UItemId>,
        ctx: &SvcCtx,
        calc: &mut Calc,
        fit: &UFit,
    ) -> ValResult<ValSlotCountFail> {
        validate_verbose_unordered_map(
            kfs,
            ctx,
            calc,
            fit.character,
            ctx.ac().max_active_drones,
            &self.drones_online_bandwidth,
        )
    }
    pub(in crate::svc::vast::val) fn validate_launched_fighter_count_verbose(
        &self,
        kfs: &RSet<UItemId>,
        ctx: &SvcCtx,
        calc: &mut Calc,
        fit: &UFit,
    ) -> ValResult<ValSlotCountFail> {
        validate_verbose_unordered_set(kfs, ctx, calc, fit.ship, ctx.ac().ftr_tubes, &self.fighters_online)
    }
    pub(in crate::svc::vast::val) fn validate_launched_light_fighter_count_verbose(
        &self,
        kfs: &RSet<UItemId>,
        ctx: &SvcCtx,
        calc: &mut Calc,
        fit: &UFit,
    ) -> ValResult<ValSlotCountFail> {
        validate_verbose_unordered_set(
            kfs,
            ctx,
            calc,
            fit.ship,
            ctx.ac().ftr_light_slots,
            &self.light_fighters_online,
        )
    }
    pub(in crate::svc::vast::val) fn validate_launched_heavy_fighter_count_verbose(
        &self,
        kfs: &RSet<UItemId>,
        ctx: &SvcCtx,
        calc: &mut Calc,
        fit: &UFit,
    ) -> ValResult<ValSlotCountFail> {
        validate_verbose_unordered_set(
            kfs,
            ctx,
            calc,
            fit.ship,
            ctx.ac().ftr_heavy_slots,
            &self.heavy_fighters_online,
        )
    }
    pub(in crate::svc::vast::val) fn validate_launched_support_fighter_count_verbose(
        &self,
        kfs: &RSet<UItemId>,
        ctx: &SvcCtx,
        calc: &mut Calc,
        fit: &UFit,
    ) -> ValResult<ValSlotCountFail> {
        validate_verbose_unordered_set(
            kfs,
            ctx,
            calc,
            fit.ship,
            ctx.ac().ftr_support_slots,
            &self.support_fighters_online,
        )
    }
    pub(in crate::svc::vast::val) fn validate_launched_st_light_fighter_count_verbose(
        &self,
        kfs: &RSet<UItemId>,
        ctx: &SvcCtx,
        calc: &mut Calc,
        fit: &UFit,
    ) -> ValResult<ValSlotCountFail> {
        validate_verbose_unordered_set(
            kfs,
            ctx,
            calc,
            fit.ship,
            ctx.ac().ftr_st_light_slots,
            &self.st_light_fighters_online,
        )
    }
    pub(in crate::svc::vast::val) fn validate_launched_st_heavy_fighter_count_verbose(
        &self,
        kfs: &RSet<UItemId>,
        ctx: &SvcCtx,
        calc: &mut Calc,
        fit: &UFit,
    ) -> ValResult<ValSlotCountFail> {
        validate_verbose_unordered_set(
            kfs,
            ctx,
            calc,
            fit.ship,
            ctx.ac().ftr_st_heavy_slots,
            &self.st_heavy_fighters_online,
        )
    }
    pub(in crate::svc::vast::val) fn validate_launched_st_support_fighter_count_verbose(
        &self,
        kfs: &RSet<UItemId>,
        ctx: &SvcCtx,
        calc: &mut Calc,
        fit: &UFit,
    ) -> ValResult<ValSlotCountFail> {
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
    ctx: &SvcCtx,
    calc: &mut Calc,
    max_item_uid: Option<UItemId>,
    max_attr_rid: Option<RAttrId>,
    users: &RSet<UItemId>,
) -> bool {
    if users.is_empty() {
        return true;
    }
    let used = Count::from_usize(users.len());
    let max = get_attr_as_count(ctx, calc, max_item_uid, max_attr_rid).unwrap_or(Count::ZERO);
    used <= max || users.is_subset(kfs)
}
fn validate_fast_unordered_map<T>(
    kfs: &RSet<UItemId>,
    ctx: &SvcCtx,
    calc: &mut Calc,
    max_item_uid: Option<UItemId>,
    max_attr_rid: Option<RAttrId>,
    users: &RMap<UItemId, T>,
) -> bool {
    if users.is_empty() {
        return true;
    }
    let used = Count::from_usize(users.len());
    let max = get_attr_as_count(ctx, calc, max_item_uid, max_attr_rid).unwrap_or(Count::ZERO);
    used <= max || users.is_subset(kfs)
}
fn validate_fast_ordered(
    kfs: &RSet<UItemId>,
    ctx: &SvcCtx,
    calc: &mut Calc,
    max_item_uid: Option<UItemId>,
    max_attr_rid: Option<RAttrId>,
    users: &UItemVec,
) -> bool {
    let used = Count::from_usize(users.slot_count());
    if used == Count::ZERO {
        return true;
    }
    let max = get_attr_as_count(ctx, calc, max_item_uid, max_attr_rid).unwrap_or(Count::ZERO);
    match kfs.is_empty() {
        true => used <= max,
        false => match used <= max {
            true => true,
            false => users.iter_uids_from_pos(max.into_rack_pos()).all(|v| kfs.contains(&v)),
        },
    }
}

fn validate_verbose_unordered_set(
    kfs: &RSet<UItemId>,
    ctx: &SvcCtx,
    calc: &mut Calc,
    max_item_uid: Option<UItemId>,
    max_attr_rid: Option<RAttrId>,
    users: &RSet<UItemId>,
) -> ValResult<ValSlotCountFail> {
    let used = Count::from_usize(users.len());
    let max = get_attr_as_count(ctx, calc, max_item_uid, max_attr_rid);
    if used <= max.unwrap_or(Count::ZERO) {
        return ValResult::Passed;
    }
    let users = users
        .iter()
        .map(|&item_uid| ValItemInfo::from_uid(kfs, ctx, item_uid))
        .collect_vec();
    ValResult::from_fail(
        users.iter().all(|v| v.status.is_kf()),
        ValSlotCountFail { used, max, users },
    )
}
fn validate_verbose_unordered_map<T>(
    kfs: &RSet<UItemId>,
    ctx: &SvcCtx,
    calc: &mut Calc,
    max_item_uid: Option<UItemId>,
    max_attr_rid: Option<RAttrId>,
    users: &RMap<UItemId, T>,
) -> ValResult<ValSlotCountFail> {
    let used = Count::from_usize(users.len());
    let max = get_attr_as_count(ctx, calc, max_item_uid, max_attr_rid);
    if used <= max.unwrap_or(Count::ZERO) {
        return ValResult::Passed;
    }
    let users = users
        .iter()
        .map(|(&item_uid, _)| ValItemInfo::from_uid(kfs, ctx, item_uid))
        .collect_vec();
    ValResult::from_fail(
        users.iter().all(|v| v.status.is_kf()),
        ValSlotCountFail { used, max, users },
    )
}
fn validate_verbose_ordered(
    kfs: &RSet<UItemId>,
    ctx: &SvcCtx,
    calc: &mut Calc,
    max_item_uid: Option<UItemId>,
    max_attr_rid: Option<RAttrId>,
    users: &UItemVec,
) -> ValResult<ValSlotCountFail> {
    let used = Count::from_usize(users.slot_count());
    let max = get_attr_as_count(ctx, calc, max_item_uid, max_attr_rid);
    let effective_max = max.unwrap_or(Count::ZERO);
    if used <= effective_max {
        return ValResult::Passed;
    }
    let users = users
        .iter_uids_from_pos(effective_max.into_rack_pos())
        .map(|item_uid| ValItemInfo::from_uid(kfs, ctx, item_uid))
        .collect_vec();
    match users.is_empty() {
        true => ValResult::Passed,
        false => ValResult::from_fail(
            users.iter().all(|v| v.status.is_kf()),
            ValSlotCountFail { used, max, users },
        ),
    }
}
