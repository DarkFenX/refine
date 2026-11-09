use crate::{
    ad::{AAttrId, AEffectId, AItem, AItemId, AItemListId},
    rd::{RAttrConsts, RAttrId, REffect, REffectConsts, REffectId, RItemBase, RItemFlexData, RItemListId},
    util::{PDense, RMap},
};

/// Represents an item (or item type, according to EVE terminology).
///
/// An item carries alot of info needed to calculate fit attributes, for example base attribute
/// values.
pub(crate) struct RItem {
    pub(crate) base: RItemBase,
    pub(crate) flex_data: RItemFlexData,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl RItem {
    pub(in crate::rd::data) fn from_a_item(a_item: &AItem) -> Self {
        Self {
            base: RItemBase::from_a_item(a_item),
            flex_data: RItemFlexData::default(),
        }
    }
    pub(in crate::rd::data) fn fill_runtime(
        &mut self,
        a_items: &RMap<AItemId, AItem>,
        item_list_aid_rid_map: &RMap<AItemListId, RItemListId>,
        attr_aid_rid_map: &RMap<AAttrId, RAttrId>,
        effect_aid_rid_map: &RMap<AEffectId, REffectId>,
        attr_consts: &RAttrConsts,
        effect_consts: &REffectConsts,
        r_effects: &PDense<REffectId, REffect>,
    ) {
        self.base
            .fill_runtime_basic(a_items, item_list_aid_rid_map, effect_aid_rid_map, r_effects);
        self.flex_data
            .fill_runtime(&self.base, a_items, attr_aid_rid_map, attr_consts);
        self.base.fill_runtime_extended(
            a_items,
            item_list_aid_rid_map,
            attr_aid_rid_map,
            &self.flex_data.attrs,
            r_effects,
            attr_consts,
            effect_consts,
        );
    }
}
