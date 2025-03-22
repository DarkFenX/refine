use crate::{
    consts,
    defs::{Count, EAttrId, EItemGrpId, SolItemId},
    sol::{
        svc::{calc::SolCalc, vast::SolVastFitData},
        uad::SolUad,
    },
    util::{StMap, StMapSetL1, StSet},
};

pub struct SolValMaxGroupFail {
    pub group_id: EItemGrpId,
    pub count: Count,
    pub items: Vec<SolValMaxGroupItemInfo>,
}

pub struct SolValMaxGroupItemInfo {
    pub item_id: SolItemId,
    pub max_allowed_count: Count,
}

impl SolVastFitData {
    // Fast validations
    pub(in crate::sol::svc::vast) fn validate_max_group_fitted_fast(
        &self,
        kfs: &StSet<SolItemId>,
        uad: &SolUad,
        calc: &mut SolCalc,
    ) -> bool {
        validate_fast(
            kfs,
            uad,
            calc,
            &self.mods_svcs_rigs_max_group_fitted_all,
            &self.mods_svcs_rigs_max_group_fitted_limited,
            &consts::attrs::MAX_GROUP_FITTED,
        )
    }
    pub(in crate::sol::svc::vast) fn validate_max_group_online_fast(
        &self,
        kfs: &StSet<SolItemId>,
        uad: &SolUad,
        calc: &mut SolCalc,
    ) -> bool {
        validate_fast(
            kfs,
            uad,
            calc,
            &self.mods_svcs_max_group_online_all,
            &self.mods_svcs_max_group_online_limited,
            &consts::attrs::MAX_GROUP_ONLINE,
        )
    }
    pub(in crate::sol::svc::vast) fn validate_max_group_active_fast(
        &self,
        kfs: &StSet<SolItemId>,
        uad: &SolUad,
        calc: &mut SolCalc,
    ) -> bool {
        validate_fast(
            kfs,
            uad,
            calc,
            &self.mods_max_group_active_all,
            &self.mods_max_group_active_limited,
            &consts::attrs::MAX_GROUP_ACTIVE,
        )
    }
    // Verbose validations
    pub(in crate::sol::svc::vast) fn validate_max_group_fitted_verbose(
        &self,
        kfs: &StSet<SolItemId>,
        uad: &SolUad,
        calc: &mut SolCalc,
    ) -> Vec<SolValMaxGroupFail> {
        validate_verbose(
            kfs,
            uad,
            calc,
            &self.mods_svcs_rigs_max_group_fitted_all,
            &self.mods_svcs_rigs_max_group_fitted_limited,
            &consts::attrs::MAX_GROUP_FITTED,
        )
    }
    pub(in crate::sol::svc::vast) fn validate_max_group_online_verbose(
        &self,
        kfs: &StSet<SolItemId>,
        uad: &SolUad,
        calc: &mut SolCalc,
    ) -> Vec<SolValMaxGroupFail> {
        validate_verbose(
            kfs,
            uad,
            calc,
            &self.mods_svcs_max_group_online_all,
            &self.mods_svcs_max_group_online_limited,
            &consts::attrs::MAX_GROUP_ONLINE,
        )
    }
    pub(in crate::sol::svc::vast) fn validate_max_group_active_verbose(
        &self,
        kfs: &StSet<SolItemId>,
        uad: &SolUad,
        calc: &mut SolCalc,
    ) -> Vec<SolValMaxGroupFail> {
        validate_verbose(
            kfs,
            uad,
            calc,
            &self.mods_max_group_active_all,
            &self.mods_max_group_active_limited,
            &consts::attrs::MAX_GROUP_ACTIVE,
        )
    }
}

fn validate_fast(
    kfs: &StSet<SolItemId>,
    uad: &SolUad,
    calc: &mut SolCalc,
    max_group_all: &StMapSetL1<EItemGrpId, SolItemId>,
    max_group_limited: &StMap<SolItemId, EItemGrpId>,
    attr_id: &EAttrId,
) -> bool {
    for (item_id, grp_id) in max_group_limited.iter() {
        let allowed = get_max_allowed_item_count(uad, calc, item_id, attr_id);
        let actual = get_actual_item_count(max_group_all, grp_id);
        if actual > allowed && !kfs.contains(item_id) {
            return false;
        }
    }
    true
}

fn validate_verbose(
    kfs: &StSet<SolItemId>,
    uad: &SolUad,
    calc: &mut SolCalc,
    max_group_all: &StMapSetL1<EItemGrpId, SolItemId>,
    max_group_limited: &StMap<SolItemId, EItemGrpId>,
    attr_id: &EAttrId,
) -> Vec<SolValMaxGroupFail> {
    let mut items_by_grp = StMap::new();
    for (item_id, grp_id) in max_group_limited.iter() {
        let allowed = get_max_allowed_item_count(uad, calc, item_id, attr_id);
        let actual = get_actual_item_count(max_group_all, grp_id);
        if actual > allowed && !kfs.contains(item_id) {
            items_by_grp
                .entry(*grp_id)
                .or_insert_with(Vec::new)
                .push(SolValMaxGroupItemInfo {
                    item_id: *item_id,
                    max_allowed_count: allowed,
                });
        }
    }
    items_by_grp
        .into_iter()
        .map(|(k, v)| SolValMaxGroupFail {
            group_id: k,
            count: get_actual_item_count(max_group_all, &k),
            items: v,
        })
        .collect()
}

fn get_max_allowed_item_count(uad: &SolUad, calc: &mut SolCalc, item_id: &SolItemId, attr_id: &EAttrId) -> Count {
    match calc.get_item_attr_val_simple(uad, item_id, attr_id) {
        Some(value) => value.round() as Count,
        // Limited items are guaranteed to have some unmodified limit value
        None => uad
            .items
            .get_item(item_id)
            .unwrap()
            .get_attrs()
            .unwrap()
            .get(attr_id)
            .unwrap()
            .round() as Count,
    }
}
fn get_actual_item_count(max_group_all: &StMapSetL1<EItemGrpId, SolItemId>, grp_id: &EItemGrpId) -> Count {
    max_group_all.get(grp_id).len() as Count
}
