use std::collections::HashMap;

use crate::{
    sol::{Count, ItemId, ItemTypeId, svc::vast::VastFitData},
    util::RSet,
};

pub struct ValMaxTypeFail {
    /// Map between item type IDs and per item-type info about failures.
    pub item_types: HashMap<ItemTypeId, ValMaxTypeTypeInfo>,
}
pub struct ValMaxTypeTypeInfo {
    /// How many items of this type is fit.
    pub item_type_count: Count,
    /// Items which break the limit, and what the limit is.
    pub items: HashMap<ItemId, Count>,
}

impl VastFitData {
    // Fast validations
    pub(in crate::sol::svc::vast) fn validate_max_type_fitted_fast(&self, kfs: &RSet<ItemId>) -> bool {
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
        kfs: &RSet<ItemId>,
    ) -> Option<ValMaxTypeFail> {
        let mut item_types = HashMap::new();
        for (a_item_id, item_type_data) in self.mods_svcs_max_type_fitted.iter() {
            let fitted = item_type_data.len() as Count;
            for (item_id, &allowed) in item_type_data.iter() {
                if fitted > allowed && !kfs.contains(item_id) {
                    item_types
                        .entry(*a_item_id)
                        .or_insert_with(|| ValMaxTypeTypeInfo {
                            item_type_count: fitted,
                            items: HashMap::new(),
                        })
                        .items
                        .insert(*item_id, allowed);
                }
            }
        }
        match item_types.is_empty() {
            true => None,
            false => Some(ValMaxTypeFail { item_types }),
        }
    }
}
