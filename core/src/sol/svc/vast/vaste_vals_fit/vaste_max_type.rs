use crate::{
    defs::{Count, EItemGrpId, SolItemId},
    sol::svc::vast::SolVastFitData,
    util::{StMap, StSet},
};

pub struct SolValMaxTypeFail {
    pub type_id: EItemGrpId,
    pub count: Count,
    pub items: Vec<SolValMaxTypeItemInfo>,
}

pub struct SolValMaxTypeItemInfo {
    pub item_id: SolItemId,
    pub max_allowed_count: Count,
}

impl SolVastFitData {
    // Fast validations
    pub(in crate::sol::svc::vast) fn validate_max_type_fitted_fast(&self, kfs: &StSet<SolItemId>) -> bool {
        for item_type_data in self.mods_svcs_max_type_fitted.values() {
            let fitted = item_type_data.len() as Count;
            for (item_id, &allowed) in item_type_data.iter() {
                if fitted > allowed && !kfs.contains(item_id) {
                    return false;
                }
            }
        }
        true
    }
    // Verbose validations
    pub(in crate::sol::svc::vast) fn validate_max_type_fitted_verbose(
        &self,
        kfs: &StSet<SolItemId>,
    ) -> Vec<SolValMaxTypeFail> {
        let mut items_by_type = StMap::new();
        for (type_id, item_type_data) in self.mods_svcs_max_type_fitted.iter() {
            let fitted = item_type_data.len() as Count;
            for (item_id, &allowed) in item_type_data.iter() {
                if fitted > allowed && !kfs.contains(item_id) {
                    items_by_type
                        .entry(*type_id)
                        .or_insert_with(Vec::new)
                        .push(SolValMaxTypeItemInfo {
                            item_id: *item_id,
                            max_allowed_count: allowed,
                        });
                }
            }
        }
        items_by_type
            .into_iter()
            .map(|(k, v)| SolValMaxTypeFail {
                type_id: k,
                count: self.mods_svcs_max_type_fitted.get_l1(&k).unwrap().len() as Count,
                items: v,
            })
            .collect()
    }
}
