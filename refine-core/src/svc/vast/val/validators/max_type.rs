use crate::{
    Count, ItemId, ItemTypeId,
    svc::{SvcCtx, vast::VastFitData},
    ud::UItemId,
    util::{RMap, RSet},
};

#[cfg_attr(
    feature = "serde",
    cfg_eval,
    serde_with::serde_as,
    derive(serde::Serialize),
    serde(transparent)
)]
#[derive(Clone)]
pub struct ValMaxTypeFail {
    #[cfg_attr(feature = "serde", serde_as(as = "serde_with::KeyValueMap<_>"))]
    pub item_types: Vec<ValMaxTypeTypeInfo>,
}

#[cfg_attr(
    feature = "serde",
    cfg_eval,
    serde_with::serde_as,
    derive(serde_tuple::Serialize_tuple)
)]
#[derive(Clone)]
pub struct ValMaxTypeTypeInfo {
    /// Type ID of an item this fit has too many.
    pub item_type_id: ItemTypeId,
    /// How many items of this type is fit.
    pub item_type_count: Count,
    /// Items which break the limit, and what the limit is.
    #[cfg_attr(feature = "serde", serde_as(as = "refine_serde::VecAsMap"))]
    pub items: Vec<ValMaxTypeItemInfo>,
}

#[cfg_attr(feature = "serde", derive(refine_serde::VecAsMapEntry))]
#[derive(Copy, Clone)]
pub struct ValMaxTypeItemInfo {
    /// Item which failed validation.
    #[cfg_attr(feature = "serde", vec_map(key))]
    pub item_id: ItemId,
    /// Max count of items of the same type ID this item allows to have.
    #[cfg_attr(feature = "serde", vec_map(value))]
    pub limit: Count,
}

impl VastFitData {
    // Fast validations
    pub(in crate::svc::vast::val) fn validate_max_type_fitted_fast(&self, kfs: &RSet<UItemId>) -> bool {
        for item_type_data in self.mods_svcs_max_type_fitted.values() {
            let fitted = Count::from_usize(item_type_data.len());
            for (item_uid, &allowed) in item_type_data.iter() {
                if fitted > allowed && !kfs.contains(item_uid) {
                    return false;
                }
            }
        }
        true
    }
    // Verbose validations
    pub(in crate::svc::vast::val) fn validate_max_type_fitted_verbose(
        &self,
        kfs: &RSet<UItemId>,
        ctx: SvcCtx,
    ) -> Option<ValMaxTypeFail> {
        let mut item_types = RMap::new();
        for (&item_aid, item_type_data) in self.mods_svcs_max_type_fitted.iter() {
            let fitted = Count::from_usize(item_type_data.len());
            for (item_uid, &allowed) in item_type_data.iter() {
                if fitted > allowed && !kfs.contains(item_uid) {
                    item_types
                        .entry(item_aid)
                        .or_insert_with(|| ValMaxTypeTypeInfo {
                            item_type_id: ItemTypeId::from_aid(item_aid),
                            item_type_count: fitted,
                            items: Vec::new(),
                        })
                        .items
                        .push(ValMaxTypeItemInfo {
                            item_id: ctx.u_data.items.ext_id_by_int_id(*item_uid),
                            limit: allowed,
                        });
                }
            }
        }
        match item_types.is_empty() {
            true => None,
            false => Some(ValMaxTypeFail {
                item_types: item_types.into_values().collect(),
            }),
        }
    }
}
