use std::collections::HashMap;

use crate::{
    ac, ad,
    sol::{
        Count, ItemGrpId, ItemId, ItemKey,
        svc::{calc::Calc, eprojs::EProjs, vast::VastFitData},
        uad::Uad,
    },
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
    pub(in crate::sol::svc::vast) fn validate_max_group_fitted_fast(
        &self,
        kfs: &RSet<ItemKey>,
        uad: &Uad,
        eprojs: &EProjs,
        calc: &mut Calc,
    ) -> bool {
        validate_fast(
            kfs,
            uad,
            eprojs,
            calc,
            &self.mods_svcs_rigs_max_group_fitted_all,
            &self.mods_svcs_rigs_max_group_fitted_limited,
            &ac::attrs::MAX_GROUP_FITTED,
        )
    }
    pub(in crate::sol::svc::vast) fn validate_max_group_online_fast(
        &self,
        kfs: &RSet<ItemKey>,
        uad: &Uad,
        eprojs: &EProjs,
        calc: &mut Calc,
    ) -> bool {
        validate_fast(
            kfs,
            uad,
            eprojs,
            calc,
            &self.mods_svcs_max_group_online_all,
            &self.mods_svcs_max_group_online_limited,
            &ac::attrs::MAX_GROUP_ONLINE,
        )
    }
    pub(in crate::sol::svc::vast) fn validate_max_group_active_fast(
        &self,
        kfs: &RSet<ItemKey>,
        uad: &Uad,
        eprojs: &EProjs,
        calc: &mut Calc,
    ) -> bool {
        validate_fast(
            kfs,
            uad,
            eprojs,
            calc,
            &self.mods_max_group_active_all,
            &self.mods_max_group_active_limited,
            &ac::attrs::MAX_GROUP_ACTIVE,
        )
    }
    // Verbose validations
    pub(in crate::sol::svc::vast) fn validate_max_group_fitted_verbose(
        &self,
        kfs: &RSet<ItemKey>,
        uad: &Uad,
        eprojs: &EProjs,
        calc: &mut Calc,
    ) -> Option<ValMaxGroupFail> {
        validate_verbose(
            kfs,
            uad,
            eprojs,
            calc,
            &self.mods_svcs_rigs_max_group_fitted_all,
            &self.mods_svcs_rigs_max_group_fitted_limited,
            &ac::attrs::MAX_GROUP_FITTED,
        )
    }
    pub(in crate::sol::svc::vast) fn validate_max_group_online_verbose(
        &self,
        kfs: &RSet<ItemKey>,
        uad: &Uad,
        eprojs: &EProjs,
        calc: &mut Calc,
    ) -> Option<ValMaxGroupFail> {
        validate_verbose(
            kfs,
            uad,
            eprojs,
            calc,
            &self.mods_svcs_max_group_online_all,
            &self.mods_svcs_max_group_online_limited,
            &ac::attrs::MAX_GROUP_ONLINE,
        )
    }
    pub(in crate::sol::svc::vast) fn validate_max_group_active_verbose(
        &self,
        kfs: &RSet<ItemKey>,
        uad: &Uad,
        eprojs: &EProjs,
        calc: &mut Calc,
    ) -> Option<ValMaxGroupFail> {
        validate_verbose(
            kfs,
            uad,
            eprojs,
            calc,
            &self.mods_max_group_active_all,
            &self.mods_max_group_active_limited,
            &ac::attrs::MAX_GROUP_ACTIVE,
        )
    }
}

fn validate_fast(
    kfs: &RSet<ItemKey>,
    uad: &Uad,
    eprojs: &EProjs,
    calc: &mut Calc,
    max_group_all: &RMapRSet<ad::AItemGrpId, ItemKey>,
    max_group_limited: &RMap<ItemKey, ad::AItemGrpId>,
    a_attr_id: &ad::AAttrId,
) -> bool {
    for (&item_key, a_item_grp_id) in max_group_limited.iter() {
        let allowed = get_max_allowed_item_count(uad, eprojs, calc, item_key, a_attr_id);
        let actual = get_actual_item_count(max_group_all, a_item_grp_id);
        if actual > allowed && !kfs.contains(&item_key) {
            return false;
        }
    }
    true
}

fn validate_verbose(
    kfs: &RSet<ItemKey>,
    uad: &Uad,
    eprojs: &EProjs,
    calc: &mut Calc,
    max_group_all: &RMapRSet<ad::AItemGrpId, ItemKey>,
    max_group_limited: &RMap<ItemKey, ad::AItemGrpId>,
    a_attr_id: &ad::AAttrId,
) -> Option<ValMaxGroupFail> {
    let mut groups = HashMap::new();
    for (&item_key, a_item_grp_id) in max_group_limited.iter() {
        let allowed = get_max_allowed_item_count(uad, eprojs, calc, item_key, a_attr_id);
        let actual = get_actual_item_count(max_group_all, a_item_grp_id);
        if actual > allowed && !kfs.contains(&item_key) {
            groups
                .entry(*a_item_grp_id)
                .or_insert_with(|| ValMaxGroupGroupInfo {
                    group_item_count: actual,
                    items: HashMap::new(),
                })
                .items
                .insert(uad.items.id_by_key(item_key), allowed);
        }
    }
    match groups.is_empty() {
        true => None,
        false => Some(ValMaxGroupFail { groups }),
    }
}

fn get_max_allowed_item_count(
    uad: &Uad,
    eprojs: &EProjs,
    calc: &mut Calc,
    item_key: ItemKey,
    a_attr_id: &ad::AAttrId,
) -> Count {
    calc.get_item_attr_val_extra(uad, eprojs, item_key, a_attr_id)
        .unwrap()
        .round() as Count
}
fn get_actual_item_count(max_group_all: &RMapRSet<ad::AItemGrpId, ItemKey>, a_item_grp_id: &ad::AItemGrpId) -> Count {
    max_group_all.get(a_item_grp_id).len() as Count
}
