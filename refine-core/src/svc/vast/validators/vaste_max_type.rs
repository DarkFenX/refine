use crate::{
    api::ItemTypeId,
    num::Count,
    svc::{SvcCtx, vast::VastFitData},
    ud::{ItemId, UItemId},
    util::{RMap, RSet},
};

#[cfg_attr(
    feature = "serde",
    cfg_eval,
    serde_with::serde_as,
    derive(serde::Serialize),
    serde(transparent)
)]
pub struct ValMaxTypeFail {
    /// Items and details about failures.
    #[cfg_attr(feature = "serde", serde_as(as = "serde_with::Map<_, _>"))]
    pub item_types: Vec<(ItemTypeId, ValMaxTypeTypeInfo)>,
}

#[cfg_attr(
    feature = "serde",
    cfg_eval,
    serde_with::serde_as,
    derive(serde_tuple::Serialize_tuple)
)]
pub struct ValMaxTypeTypeInfo {
    /// How many items of this type is fit.
    pub item_type_count: Count,
    /// Items which break the limit, and what the limit is.
    #[cfg_attr(feature = "serde", serde_as(as = "&serde_with::Map<_, _>"))]
    pub items: Vec<(ItemId, Count)>,
}

impl VastFitData {
    // Fast validations
    pub(in crate::svc::vast) fn validate_max_type_fitted_fast(&self, kfs: &RSet<UItemId>) -> bool {
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
    pub(in crate::svc::vast) fn validate_max_type_fitted_verbose(
        &self,
        kfs: &RSet<UItemId>,
        ctx: SvcCtx,
    ) -> Option<ValMaxTypeFail> {
        let mut item_types = RMap::new();
        for (item_aid, item_type_data) in self.mods_svcs_max_type_fitted.iter() {
            let fitted = Count::from_usize(item_type_data.len());
            for (item_uid, &allowed) in item_type_data.iter() {
                if fitted > allowed && !kfs.contains(item_uid) {
                    item_types
                        .entry(ItemTypeId::from_aid(*item_aid))
                        .or_insert_with(|| ValMaxTypeTypeInfo {
                            item_type_count: fitted,
                            items: Vec::new(),
                        })
                        .items
                        .push((ctx.u_data.items.ext_id_by_int_id(*item_uid), allowed));
                }
            }
        }
        match item_types.is_empty() {
            true => None,
            false => Some(ValMaxTypeFail {
                item_types: item_types.into_iter().collect(),
            }),
        }
    }
}
