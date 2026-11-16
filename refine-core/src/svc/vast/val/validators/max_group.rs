use crate::{
    Count, ItemGrpId, ItemId,
    ad::AItemGrpId,
    rd::RAttrId,
    svc::{Calc, SvcCtx, vast::VastFitData},
    ud::UItemId,
    util::{RMapRMap, RSet},
    val::{ValItemStatus, ValResult},
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
pub struct ValMaxGroupFail {
    /// Map between group IDs which had failed items, and detailed group info.
    #[cfg_attr(feature = "serde", serde_as(as = "serde_with::KeyValueMap<_>"))]
    pub groups: Vec<ValMaxGroupGroupInfo>,
}

#[cfg_attr(
    feature = "serde",
    cfg_eval,
    serde_with::serde_as,
    derive(serde_tuple::Serialize_tuple)
)]
#[derive(Clone)]
pub struct ValMaxGroupGroupInfo {
    /// Group which has failed items.
    pub group_id: ItemGrpId,
    /// Items of the group, which are in high enough state to count for validation purposes.
    pub group_item_ids: Vec<ItemId>,
    /// Offending items and their group limits.
    #[cfg_attr(feature = "serde", serde_as(as = "serde_with::KeyValueMap<_>"))]
    pub items: Vec<ValMaxGroupItemInfo>,
}

#[cfg_attr(feature = "serde", derive(serde_tuple::Serialize_tuple))]
#[derive(Copy, Clone)]
pub struct ValMaxGroupItemInfo {
    /// Item which failed validation.
    pub item_id: ItemId,
    pub status: ValItemStatus,
    /// Max count of items in the group for this item not to fail the validation.
    pub limit: Count,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Validators
////////////////////////////////////////////////////////////////////////////////////////////////////
impl VastFitData {
    // Fast validations
    pub(in crate::svc::vast::val) fn validate_max_group_fitted_fast(
        &self,
        kfs: &RSet<UItemId>,
        ctx: &SvcCtx,
        calc: &mut Calc,
    ) -> bool {
        validate_fast(
            kfs,
            ctx,
            calc,
            &self.mods_svcs_rigs_max_group_fitted,
            ctx.ac().max_group_fitted,
        )
    }
    pub(in crate::svc::vast::val) fn validate_max_group_online_fast(
        &self,
        kfs: &RSet<UItemId>,
        ctx: &SvcCtx,
        calc: &mut Calc,
    ) -> bool {
        validate_fast(
            kfs,
            ctx,
            calc,
            &self.mods_svcs_max_group_online,
            ctx.ac().max_group_online,
        )
    }
    pub(in crate::svc::vast::val) fn validate_max_group_active_fast(
        &self,
        kfs: &RSet<UItemId>,
        ctx: &SvcCtx,
        calc: &mut Calc,
    ) -> bool {
        validate_fast(kfs, ctx, calc, &self.mods_max_group_active, ctx.ac().max_group_active)
    }
    // Verbose validations
    pub(in crate::svc::vast::val) fn validate_max_group_fitted_verbose(
        &self,
        kfs: &RSet<UItemId>,
        ctx: &SvcCtx,
        calc: &mut Calc,
    ) -> ValResult<ValMaxGroupFail> {
        validate_verbose(
            kfs,
            ctx,
            calc,
            &self.mods_svcs_rigs_max_group_fitted,
            ctx.ac().max_group_fitted,
        )
    }
    pub(in crate::svc::vast::val) fn validate_max_group_online_verbose(
        &self,
        kfs: &RSet<UItemId>,
        ctx: &SvcCtx,
        calc: &mut Calc,
    ) -> ValResult<ValMaxGroupFail> {
        validate_verbose(
            kfs,
            ctx,
            calc,
            &self.mods_svcs_max_group_online,
            ctx.ac().max_group_online,
        )
    }
    pub(in crate::svc::vast::val) fn validate_max_group_active_verbose(
        &self,
        kfs: &RSet<UItemId>,
        ctx: &SvcCtx,
        calc: &mut Calc,
    ) -> ValResult<ValMaxGroupFail> {
        validate_verbose(kfs, ctx, calc, &self.mods_max_group_active, ctx.ac().max_group_active)
    }
}

fn validate_fast(
    kfs: &RSet<UItemId>,
    ctx: &SvcCtx,
    calc: &mut Calc,
    max_group: &RMapRMap<AItemGrpId, UItemId, bool>,
    attr_rid: Option<RAttrId>,
) -> bool {
    let Some(attr_rid) = attr_rid else {
        return true;
    };
    for group_items in max_group.values() {
        let actual = Count::from_usize(group_items.len());
        for (&item_uid, &limited) in group_items.iter() {
            if !limited {
                continue;
            }
            let allowed = get_max_allowed_item_count(ctx, calc, item_uid, attr_rid);
            if actual > allowed && !kfs.contains(&item_uid) {
                return false;
            }
        }
    }
    true
}

fn validate_verbose(
    kfs: &RSet<UItemId>,
    ctx: &SvcCtx,
    calc: &mut Calc,
    max_group: &RMapRMap<AItemGrpId, UItemId, bool>,
    attr_rid: Option<RAttrId>,
) -> ValResult<ValMaxGroupFail> {
    let Some(attr_rid) = attr_rid else {
        return ValResult::Passed;
    };
    let mut groups = Vec::new();
    for (&item_grp_aid, group_items) in max_group.iter() {
        let actual = Count::from_usize(group_items.len());
        let mut items = Vec::new();
        for (&item_uid, &limited) in group_items.iter() {
            if !limited {
                continue;
            }
            let item_limit = get_max_allowed_item_count(ctx, calc, item_uid, attr_rid);
            if actual > item_limit {
                items.push(ValMaxGroupItemInfo {
                    item_id: ctx.u_data.items.ext_id_by_int_id(item_uid),
                    status: ValItemStatus::from_kfs(kfs, item_uid),
                    limit: item_limit,
                });
            }
        }
        if !items.is_empty() {
            groups.push(ValMaxGroupGroupInfo {
                group_id: ItemGrpId::from_aid(item_grp_aid),
                group_item_ids: group_items
                    .keys()
                    .map(|&group_item_uid| ctx.u_data.items.ext_id_by_int_id(group_item_uid))
                    .collect(),
                items,
            });
        }
    }
    match groups.is_empty() {
        true => ValResult::Passed,
        false => ValResult::from_fail(
            groups.iter().flat_map(|v| v.items.iter()).all(|v| v.status.is_kf()),
            ValMaxGroupFail { groups },
        ),
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Helpers
////////////////////////////////////////////////////////////////////////////////////////////////////
fn get_max_allowed_item_count(ctx: &SvcCtx, calc: &mut Calc, item_uid: UItemId, attr_rid: RAttrId) -> Count {
    Count::from_value_rounded(calc.get_item_attr_oextra(ctx, item_uid, attr_rid).unwrap())
}
