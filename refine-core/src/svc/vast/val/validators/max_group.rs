use crate::{
    Count, ItemId,
    ad::AItemGrpId,
    api::ItemGrpId,
    rd::RAttrId,
    svc::{Calc, SvcCtx, vast::VastFitData},
    ud::UItemId,
    util::{RMap, RMapRSet, RSet},
};

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
    /// How many items are in the group, in high enough state to count for validation purposes.
    pub group_item_count: Count,
    /// Offending items and their group limits.
    #[cfg_attr(feature = "serde", serde_as(as = "refine_serde::VecAsMap"))]
    pub items: Vec<ValMaxGroupItemInfo>,
}

#[cfg_attr(feature = "serde", derive(refine_serde::VecAsMapEntry))]
#[derive(Copy, Clone)]
pub struct ValMaxGroupItemInfo {
    /// Item which failed validation.
    #[cfg_attr(feature = "serde", vec_map(key))]
    pub item_id: ItemId,
    /// Max count of items in the group for this item not to fail the validation.
    #[cfg_attr(feature = "serde", vec_map(value))]
    pub limit: Count,
}

impl VastFitData {
    // Fast validations
    pub(in crate::svc::vast) fn validate_max_group_fitted_fast(
        &self,
        kfs: &RSet<UItemId>,
        ctx: SvcCtx,
        calc: &mut Calc,
    ) -> bool {
        validate_fast(
            kfs,
            ctx,
            calc,
            &self.mods_svcs_rigs_max_group_fitted_all,
            &self.mods_svcs_rigs_max_group_fitted_limited,
            ctx.ac().max_group_fitted,
        )
    }
    pub(in crate::svc::vast) fn validate_max_group_online_fast(
        &self,
        kfs: &RSet<UItemId>,
        ctx: SvcCtx,
        calc: &mut Calc,
    ) -> bool {
        validate_fast(
            kfs,
            ctx,
            calc,
            &self.mods_svcs_max_group_online_all,
            &self.mods_svcs_max_group_online_limited,
            ctx.ac().max_group_online,
        )
    }
    pub(in crate::svc::vast) fn validate_max_group_active_fast(
        &self,
        kfs: &RSet<UItemId>,
        ctx: SvcCtx,
        calc: &mut Calc,
    ) -> bool {
        validate_fast(
            kfs,
            ctx,
            calc,
            &self.mods_max_group_active_all,
            &self.mods_max_group_active_limited,
            ctx.ac().max_group_active,
        )
    }
    // Verbose validations
    pub(in crate::svc::vast) fn validate_max_group_fitted_verbose(
        &self,
        kfs: &RSet<UItemId>,
        ctx: SvcCtx,
        calc: &mut Calc,
    ) -> Option<ValMaxGroupFail> {
        validate_verbose(
            kfs,
            ctx,
            calc,
            &self.mods_svcs_rigs_max_group_fitted_all,
            &self.mods_svcs_rigs_max_group_fitted_limited,
            ctx.ac().max_group_fitted,
        )
    }
    pub(in crate::svc::vast) fn validate_max_group_online_verbose(
        &self,
        kfs: &RSet<UItemId>,
        ctx: SvcCtx,
        calc: &mut Calc,
    ) -> Option<ValMaxGroupFail> {
        validate_verbose(
            kfs,
            ctx,
            calc,
            &self.mods_svcs_max_group_online_all,
            &self.mods_svcs_max_group_online_limited,
            ctx.ac().max_group_online,
        )
    }
    pub(in crate::svc::vast) fn validate_max_group_active_verbose(
        &self,
        kfs: &RSet<UItemId>,
        ctx: SvcCtx,
        calc: &mut Calc,
    ) -> Option<ValMaxGroupFail> {
        validate_verbose(
            kfs,
            ctx,
            calc,
            &self.mods_max_group_active_all,
            &self.mods_max_group_active_limited,
            ctx.ac().max_group_active,
        )
    }
}

fn validate_fast(
    kfs: &RSet<UItemId>,
    ctx: SvcCtx,
    calc: &mut Calc,
    max_group_all: &RMapRSet<AItemGrpId, UItemId>,
    max_group_limited: &RMap<UItemId, AItemGrpId>,
    attr_rid: Option<RAttrId>,
) -> bool {
    let Some(attr_rid) = attr_rid else {
        return true;
    };
    for (&item_uid, item_grp_aid) in max_group_limited.iter() {
        let allowed = get_max_allowed_item_count(ctx, calc, item_uid, attr_rid);
        let actual = get_actual_item_count(max_group_all, item_grp_aid);
        if actual > allowed && !kfs.contains(&item_uid) {
            return false;
        }
    }
    true
}

fn validate_verbose(
    kfs: &RSet<UItemId>,
    ctx: SvcCtx,
    calc: &mut Calc,
    max_group_all: &RMapRSet<AItemGrpId, UItemId>,
    max_group_limited: &RMap<UItemId, AItemGrpId>,
    attr_rid: Option<RAttrId>,
) -> Option<ValMaxGroupFail> {
    let attr_rid = attr_rid?;
    let mut groups = RMap::new();
    for (&item_uid, &item_grp_aid) in max_group_limited.iter() {
        let item_limit = get_max_allowed_item_count(ctx, calc, item_uid, attr_rid);
        let actual = get_actual_item_count(max_group_all, &item_grp_aid);
        if actual > item_limit && !kfs.contains(&item_uid) {
            groups
                .entry(item_grp_aid)
                .or_insert_with(|| ValMaxGroupGroupInfo {
                    group_id: ItemGrpId::from_aid(item_grp_aid),
                    group_item_count: actual,
                    items: Vec::new(),
                })
                .items
                .push(ValMaxGroupItemInfo {
                    item_id: ctx.u_data.items.ext_id_by_int_id(item_uid),
                    limit: item_limit,
                });
        }
    }
    match groups.is_empty() {
        true => None,
        false => Some(ValMaxGroupFail {
            groups: groups.into_values().collect(),
        }),
    }
}

fn get_max_allowed_item_count(ctx: SvcCtx, calc: &mut Calc, item_uid: UItemId, attr_rid: RAttrId) -> Count {
    Count::from_value_rounded(calc.get_item_attr_oextra(ctx, item_uid, attr_rid).unwrap())
}
fn get_actual_item_count(max_group_all: &RMapRSet<AItemGrpId, UItemId>, item_grp_aid: &AItemGrpId) -> Count {
    Count::from_usize(max_group_all.get(item_grp_aid).len())
}
