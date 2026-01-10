use std::collections::HashMap;

use crate::{
    ad::AItemGrpId,
    api::ItemGrpId,
    num::Count,
    rd::RAttrId,
    svc::{SvcCtx, calc::Calc, vast::VastFitData},
    ud::{ItemId, UItemId},
    util::{RMap, RMapRSet, RSet},
};

pub struct ValMaxGroupFail {
    /// Map between group IDs which had failed items, and detailed group info.
    pub groups: HashMap<ItemGrpId, ValMaxGroupGroupInfo>,
}

pub struct ValMaxGroupGroupInfo {
    /// How many items from that group are in an appropriate state.
    pub group_item_count: Count,
    /// Map between offending item IDs and per-item group count limits.
    pub items: HashMap<ItemId, Count>,
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
    let attr_rid = match attr_rid {
        Some(attr_rid) => attr_rid,
        None => return true,
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
    let mut groups = HashMap::new();
    for (&item_uid, item_grp_aid) in max_group_limited.iter() {
        let allowed = get_max_allowed_item_count(ctx, calc, item_uid, attr_rid);
        let actual = get_actual_item_count(max_group_all, item_grp_aid);
        if actual > allowed && !kfs.contains(&item_uid) {
            groups
                .entry(ItemGrpId::from_aid(*item_grp_aid))
                .or_insert_with(|| ValMaxGroupGroupInfo {
                    group_item_count: actual,
                    items: HashMap::new(),
                })
                .items
                .insert(ctx.u_data.items.xid_by_iid(item_uid), allowed);
        }
    }
    match groups.is_empty() {
        true => None,
        false => Some(ValMaxGroupFail { groups }),
    }
}

fn get_max_allowed_item_count(ctx: SvcCtx, calc: &mut Calc, item_uid: UItemId, attr_rid: RAttrId) -> Count {
    Count::from_value_rounded(calc.get_item_attr_oextra(ctx, item_uid, attr_rid).unwrap())
}
fn get_actual_item_count(max_group_all: &RMapRSet<AItemGrpId, UItemId>, item_grp_aid: &AItemGrpId) -> Count {
    Count::from_usize(max_group_all.get(item_grp_aid).len())
}
