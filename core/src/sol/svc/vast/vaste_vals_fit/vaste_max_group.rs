use crate::{
    defs::{Count, EAttrId, EItemGrpId, SolItemId},
    ec,
    sol::{
        svc::{calc::SolCalc, vast::SolVastFitData},
        uad::SolUad,
    },
    util::{StMap, StMapSetL1},
};

pub struct SolValMaxGroupFail {
    pub group_id: EItemGrpId,
    pub count: Count,
    pub items: Vec<SolValMaxGroupItemInfo>,
}
impl SolValMaxGroupFail {
    fn new(group_id: EItemGrpId, count: Count, items: Vec<SolValMaxGroupItemInfo>) -> Self {
        Self { group_id, count, items }
    }
}

pub struct SolValMaxGroupItemInfo {
    pub item_id: SolItemId,
    pub max_allowed_count: Count,
}
impl SolValMaxGroupItemInfo {
    pub(in crate::sol::svc::vast) fn new(item_id: SolItemId, max_allowed_count: Count) -> Self {
        Self {
            item_id,
            max_allowed_count,
        }
    }
}

impl SolVastFitData {
    // Fast validations
    pub(in crate::sol::svc::vast) fn validate_max_group_fitted_fast(&self, uad: &SolUad, calc: &mut SolCalc) -> bool {
        validate_fast(
            uad,
            calc,
            &self.mods_rigs_max_group_fitted_all,
            &self.mods_rigs_max_group_fitted_limited,
            ec::attrs::MAX_GROUP_FITTED,
        )
    }
    pub(in crate::sol::svc::vast) fn validate_max_group_online_fast(&self, uad: &SolUad, calc: &mut SolCalc) -> bool {
        validate_fast(
            uad,
            calc,
            &self.mods_max_group_online_all,
            &self.mods_max_group_online_limited,
            ec::attrs::MAX_GROUP_ONLINE,
        )
    }
    pub(in crate::sol::svc::vast) fn validate_max_group_active_fast(&self, uad: &SolUad, calc: &mut SolCalc) -> bool {
        validate_fast(
            uad,
            calc,
            &self.mods_max_group_active_all,
            &self.mods_max_group_active_limited,
            ec::attrs::MAX_GROUP_ACTIVE,
        )
    }
    // Verbose validations
    pub(in crate::sol::svc::vast) fn validate_max_group_fitted_verbose(
        &self,
        uad: &SolUad,
        calc: &mut SolCalc,
    ) -> Vec<SolValMaxGroupFail> {
        validate_verbose(
            uad,
            calc,
            &self.mods_rigs_max_group_fitted_all,
            &self.mods_rigs_max_group_fitted_limited,
            ec::attrs::MAX_GROUP_FITTED,
        )
    }
    pub(in crate::sol::svc::vast) fn validate_max_group_online_verbose(
        &self,
        uad: &SolUad,
        calc: &mut SolCalc,
    ) -> Vec<SolValMaxGroupFail> {
        validate_verbose(
            uad,
            calc,
            &self.mods_max_group_online_all,
            &self.mods_max_group_online_limited,
            ec::attrs::MAX_GROUP_ONLINE,
        )
    }
    pub(in crate::sol::svc::vast) fn validate_max_group_active_verbose(
        &self,
        uad: &SolUad,
        calc: &mut SolCalc,
    ) -> Vec<SolValMaxGroupFail> {
        validate_verbose(
            uad,
            calc,
            &self.mods_max_group_active_all,
            &self.mods_max_group_active_limited,
            ec::attrs::MAX_GROUP_ACTIVE,
        )
    }
}

fn validate_fast(
    uad: &SolUad,
    calc: &mut SolCalc,
    max_group_all: &StMapSetL1<EItemGrpId, SolItemId>,
    max_group_limited: &StMap<SolItemId, EItemGrpId>,
    attr_id: EAttrId,
) -> bool {
    for (item_id, grp_id) in max_group_limited.iter() {
        let allowed = match calc.get_item_attr_val(uad, item_id, &attr_id) {
            Ok(value) => value.extra.round() as Count,
            // Limited items are guaranteed to have some unmodified limit value
            Err(_) => uad
                .items
                .get_item(item_id)
                .unwrap()
                .get_attrs()
                .unwrap()
                .get(&attr_id)
                .unwrap()
                .round() as Count,
        };
        let fitted = max_group_all.get(grp_id).len() as Count;
        if fitted > allowed {
            return false;
        }
    }
    true
}

fn validate_verbose(
    uad: &SolUad,
    calc: &mut SolCalc,
    max_group_all: &StMapSetL1<EItemGrpId, SolItemId>,
    max_group_limited: &StMap<SolItemId, EItemGrpId>,
    attr_id: EAttrId,
) -> Vec<SolValMaxGroupFail> {
    let mut items_by_grp = StMap::new();
    for (item_id, grp_id) in max_group_limited.iter() {
        let allowed = match calc.get_item_attr_val(uad, item_id, &attr_id) {
            Ok(value) => value.extra.round() as Count,
            // Limited items are guaranteed to have some unmodified limit value
            Err(_) => uad
                .items
                .get_item(item_id)
                .unwrap()
                .get_attrs()
                .unwrap()
                .get(&attr_id)
                .unwrap()
                .round() as Count,
        };
        let fitted = max_group_all.get(grp_id).len() as Count;
        if fitted > allowed {
            items_by_grp
                .entry(*grp_id)
                .or_insert_with(Vec::new)
                .push(SolValMaxGroupItemInfo::new(*item_id, allowed));
        }
    }
    items_by_grp
        .into_iter()
        .map(|(k, v)| SolValMaxGroupFail::new(k, max_group_all.get(&k).len() as Count, v))
        .collect()
}
