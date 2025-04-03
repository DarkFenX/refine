use crate::{
    sol::{Count, ItemId, ItemTypeId, svc::vast::VastFitData},
    util::{HMap, HSet},
};

pub struct ValMaxTypeFail {
    pub type_id: ItemTypeId,
    pub count: Count,
    pub items: Vec<ValMaxTypeItemInfo>,
}

pub struct ValMaxTypeItemInfo {
    pub item_id: ItemId,
    pub max_allowed_count: Count,
}

impl VastFitData {
    // Fast validations
    pub(in crate::sol::svc::vast) fn validate_max_type_fitted_fast(&self, kfs: &HSet<ItemId>) -> bool {
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
        kfs: &HSet<ItemId>,
    ) -> Vec<ValMaxTypeFail> {
        let mut items_by_type = HMap::new();
        for (a_item_id, item_type_data) in self.mods_svcs_max_type_fitted.iter() {
            let fitted = item_type_data.len() as Count;
            for (item_id, &allowed) in item_type_data.iter() {
                if fitted > allowed && !kfs.contains(item_id) {
                    items_by_type
                        .entry(*a_item_id)
                        .or_insert_with(Vec::new)
                        .push(ValMaxTypeItemInfo {
                            item_id: *item_id,
                            max_allowed_count: allowed,
                        });
                }
            }
        }
        items_by_type
            .into_iter()
            .map(|(k, v)| ValMaxTypeFail {
                type_id: k,
                count: self.mods_svcs_max_type_fitted.get_l1(&k).unwrap().len() as Count,
                items: v,
            })
            .collect()
    }
}
