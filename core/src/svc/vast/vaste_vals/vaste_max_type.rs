use std::collections::HashMap;

use crate::{
    def::{DefCount, ItemId, ItemTypeId},
    svc::{SvcCtx, vast::VastFitData},
    ud::UItemId,
    util::RSet,
};

pub struct ValMaxTypeFail {
    /// Map between item type IDs and per item-type info about failures.
    pub item_types: HashMap<ItemTypeId, ValMaxTypeTypeInfo>,
}
pub struct ValMaxTypeTypeInfo {
    /// How many items of this type is fit.
    pub item_type_count: DefCount,
    /// Items which break the limit, and what the limit is.
    pub items: HashMap<ItemId, DefCount>,
}

impl VastFitData {
    // Fast validations
    pub(in crate::svc::vast) fn validate_max_type_fitted_fast(&self, kfs: &RSet<UItemId>) -> bool {
        for item_type_data in self.mods_svcs_max_type_fitted.values() {
            let fitted = item_type_data.len() as DefCount;
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
        kfs: &RSet<UItemId>,
        ctx: SvcCtx,
    ) -> Option<ValMaxTypeFail> {
        let mut item_types = HashMap::new();
        for (item_aid, item_type_data) in self.mods_svcs_max_type_fitted.iter() {
            let fitted = item_type_data.len() as DefCount;
            for (item_key, &allowed) in item_type_data.iter() {
                if fitted > allowed && !kfs.contains(item_key) {
                    item_types
                        .entry(*item_aid)
                        .or_insert_with(|| ValMaxTypeTypeInfo {
                            item_type_count: fitted,
                            items: HashMap::new(),
                        })
                        .items
                        .insert(ctx.u_data.items.xid_by_iid(*item_key), allowed);
                }
            }
        }
        match item_types.is_empty() {
            true => None,
            false => Some(ValMaxTypeFail { item_types }),
        }
    }
}
