use crate::{
    ac, ad,
    sol::{
        Count, ItemGrpId, ItemId,
        svc::{calc::Calc, vast::VastFitData},
        uad::Uad,
    },
    util::{RMap, RMapRSet, RSet},
};

pub struct ValMaxGroupFail {
    pub group_id: ItemGrpId,
    pub count: Count,
    pub items: Vec<ValMaxGroupItemInfo>,
}

pub struct ValMaxGroupItemInfo {
    pub item_id: ItemId,
    pub max_allowed_count: Count,
}

impl VastFitData {
    // Fast validations
    pub(in crate::sol::svc::vast) fn validate_max_group_fitted_fast(
        &self,
        kfs: &RSet<ItemId>,
        uad: &Uad,
        calc: &mut Calc,
    ) -> bool {
        validate_fast(
            kfs,
            uad,
            calc,
            &self.mods_svcs_rigs_max_group_fitted_all,
            &self.mods_svcs_rigs_max_group_fitted_limited,
            &ac::attrs::MAX_GROUP_FITTED,
        )
    }
    pub(in crate::sol::svc::vast) fn validate_max_group_online_fast(
        &self,
        kfs: &RSet<ItemId>,
        uad: &Uad,
        calc: &mut Calc,
    ) -> bool {
        validate_fast(
            kfs,
            uad,
            calc,
            &self.mods_svcs_max_group_online_all,
            &self.mods_svcs_max_group_online_limited,
            &ac::attrs::MAX_GROUP_ONLINE,
        )
    }
    pub(in crate::sol::svc::vast) fn validate_max_group_active_fast(
        &self,
        kfs: &RSet<ItemId>,
        uad: &Uad,
        calc: &mut Calc,
    ) -> bool {
        validate_fast(
            kfs,
            uad,
            calc,
            &self.mods_max_group_active_all,
            &self.mods_max_group_active_limited,
            &ac::attrs::MAX_GROUP_ACTIVE,
        )
    }
    // Verbose validations
    pub(in crate::sol::svc::vast) fn validate_max_group_fitted_verbose(
        &self,
        kfs: &RSet<ItemId>,
        uad: &Uad,
        calc: &mut Calc,
    ) -> Vec<ValMaxGroupFail> {
        validate_verbose(
            kfs,
            uad,
            calc,
            &self.mods_svcs_rigs_max_group_fitted_all,
            &self.mods_svcs_rigs_max_group_fitted_limited,
            &ac::attrs::MAX_GROUP_FITTED,
        )
    }
    pub(in crate::sol::svc::vast) fn validate_max_group_online_verbose(
        &self,
        kfs: &RSet<ItemId>,
        uad: &Uad,
        calc: &mut Calc,
    ) -> Vec<ValMaxGroupFail> {
        validate_verbose(
            kfs,
            uad,
            calc,
            &self.mods_svcs_max_group_online_all,
            &self.mods_svcs_max_group_online_limited,
            &ac::attrs::MAX_GROUP_ONLINE,
        )
    }
    pub(in crate::sol::svc::vast) fn validate_max_group_active_verbose(
        &self,
        kfs: &RSet<ItemId>,
        uad: &Uad,
        calc: &mut Calc,
    ) -> Vec<ValMaxGroupFail> {
        validate_verbose(
            kfs,
            uad,
            calc,
            &self.mods_max_group_active_all,
            &self.mods_max_group_active_limited,
            &ac::attrs::MAX_GROUP_ACTIVE,
        )
    }
}

fn validate_fast(
    kfs: &RSet<ItemId>,
    uad: &Uad,
    calc: &mut Calc,
    max_group_all: &RMapRSet<ad::AItemGrpId, ItemId>,
    max_group_limited: &RMap<ItemId, ad::AItemGrpId>,
    a_attr_id: &ad::AAttrId,
) -> bool {
    for (item_id, a_item_grp_id) in max_group_limited.iter() {
        let allowed = get_max_allowed_item_count(uad, calc, item_id, a_attr_id);
        let actual = get_actual_item_count(max_group_all, a_item_grp_id);
        if actual > allowed && !kfs.contains(item_id) {
            return false;
        }
    }
    true
}

fn validate_verbose(
    kfs: &RSet<ItemId>,
    uad: &Uad,
    calc: &mut Calc,
    max_group_all: &RMapRSet<ad::AItemGrpId, ItemId>,
    max_group_limited: &RMap<ItemId, ad::AItemGrpId>,
    a_attr_id: &ad::AAttrId,
) -> Vec<ValMaxGroupFail> {
    let mut items_by_grp = RMap::new();
    for (item_id, a_item_grp_id) in max_group_limited.iter() {
        let allowed = get_max_allowed_item_count(uad, calc, item_id, a_attr_id);
        let actual = get_actual_item_count(max_group_all, a_item_grp_id);
        if actual > allowed && !kfs.contains(item_id) {
            items_by_grp
                .entry(*a_item_grp_id)
                .or_insert_with(Vec::new)
                .push(ValMaxGroupItemInfo {
                    item_id: *item_id,
                    max_allowed_count: allowed,
                });
        }
    }
    items_by_grp
        .into_iter()
        .map(|(k, v)| ValMaxGroupFail {
            group_id: k,
            count: get_actual_item_count(max_group_all, &k),
            items: v,
        })
        .collect()
}

fn get_max_allowed_item_count(uad: &Uad, calc: &mut Calc, item_id: &ItemId, a_attr_id: &ad::AAttrId) -> Count {
    match calc.get_item_attr_val_extra(uad, item_id, a_attr_id) {
        Some(value) => value.round() as Count,
        // Limited items are guaranteed to have some unmodified limit value
        None => uad
            .items
            .get_item(item_id)
            .unwrap()
            .get_a_attrs()
            .unwrap()
            .get(a_attr_id)
            .unwrap()
            .round() as Count,
    }
}
fn get_actual_item_count(max_group_all: &RMapRSet<ad::AItemGrpId, ItemId>, a_item_grp_id: &ad::AItemGrpId) -> Count {
    max_group_all.get(a_item_grp_id).len() as Count
}
