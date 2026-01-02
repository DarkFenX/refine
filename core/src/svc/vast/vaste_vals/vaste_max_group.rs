use std::collections::HashMap;

use crate::{
    ad::AItemGrpId,
    def::{Count, ItemGrpId, ItemId},
    rd::RAttrId,
    svc::{SvcCtx, calc::Calc, vast::VastFitData},
    ud::UItemId,
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
    attr_key: Option<RAttrId>,
) -> bool {
    let attr_key = match attr_key {
        Some(attr_key) => attr_key,
        None => return true,
    };
    for (&item_key, a_item_grp_id) in max_group_limited.iter() {
        let allowed = get_max_allowed_item_count(ctx, calc, item_key, attr_key);
        let actual = get_actual_item_count(max_group_all, a_item_grp_id);
        if actual > allowed && !kfs.contains(&item_key) {
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
    attr_key: Option<RAttrId>,
) -> Option<ValMaxGroupFail> {
    let attr_key = attr_key?;
    let mut groups = HashMap::new();
    for (&item_key, a_item_grp_id) in max_group_limited.iter() {
        let allowed = get_max_allowed_item_count(ctx, calc, item_key, attr_key);
        let actual = get_actual_item_count(max_group_all, a_item_grp_id);
        if actual > allowed && !kfs.contains(&item_key) {
            groups
                .entry(*a_item_grp_id)
                .or_insert_with(|| ValMaxGroupGroupInfo {
                    group_item_count: actual,
                    items: HashMap::new(),
                })
                .items
                .insert(ctx.u_data.items.eid_by_iid(item_key), allowed);
        }
    }
    match groups.is_empty() {
        true => None,
        false => Some(ValMaxGroupFail { groups }),
    }
}

fn get_max_allowed_item_count(ctx: SvcCtx, calc: &mut Calc, item_key: UItemId, attr_key: RAttrId) -> Count {
    calc.get_item_attr_oextra(ctx, item_key, attr_key).unwrap().round() as Count
}
fn get_actual_item_count(max_group_all: &RMapRSet<AItemGrpId, UItemId>, a_item_grp_id: &AItemGrpId) -> Count {
    max_group_all.get(a_item_grp_id).len() as Count
}
