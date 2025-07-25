use std::collections::HashMap;

use crate::{
    def::{Count, ItemId, ItemTypeId},
    svc::{SvcCtx, vast::VastFitData},
    ud::UItemKey,
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
    pub(in crate::svc::vast) fn validate_max_type_fitted_fast(&self, kfs: &RSet<UItemKey>) -> bool {
        for item_type_data in self.mods_svcs_max_type_fitted.values() {
            let fitted = item_type_data.len() as Count;
            for (item_key, &allowed) in item_type_data.iter() {
                if fitted > allowed && !kfs.contains(item_key) {
                    return false;
                }
            }
        }
        true
    }
    // Verbose validations
    pub(in crate::svc::vast) fn validate_max_type_fitted_verbose(
        &self,
        kfs: &RSet<UItemKey>,
        ctx: SvcCtx,
    ) -> Option<ValMaxTypeFail> {
        let mut item_types = HashMap::new();
        for (a_item_id, item_type_data) in self.mods_svcs_max_type_fitted.iter() {
            let fitted = item_type_data.len() as Count;
            for (item_key, &allowed) in item_type_data.iter() {
                if fitted > allowed && !kfs.contains(item_key) {
                    item_types
                        .entry(*a_item_id)
                        .or_insert_with(|| ValMaxTypeTypeInfo {
                            item_type_count: fitted,
                            items: HashMap::new(),
                        })
                        .items
                        .insert(ctx.u_data.items.id_by_key(*item_key), allowed);
                }
            }
        }
        match item_types.is_empty() {
            true => None,
            false => Some(ValMaxTypeFail { item_types }),
        }
    }
}
