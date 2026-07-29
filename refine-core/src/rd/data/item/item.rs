use crate::{
    ad::{AAttrId, AEffectId, AItem, AItemId, AItemListId},
    rd::{RAttrConsts, RAttrId, REffectConsts, REffectId, RItemAttrData, RItemBase, RItemListId, RcEffect},
    util::{PSlab, RMap},
};

// Represents an item (or item type, according to EVE terminology).
//
// An item carries alot of info needed to calculate fit attributes, for example base attribute
// values.
pub(crate) struct RItem {
    pub(crate) base: RItemBase,
    pub(crate) attr_data: RItemAttrData,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl RItem {
    pub(in crate::rd) fn from_a_item(a_item: &AItem) -> Self {
        Self {
            base: RItemBase::from_a_item(a_item),
            attr_data: RItemAttrData::default(),
        }
    }
    pub(in crate::rd) fn fill_runtime(
        &mut self,
        a_items: &RMap<AItemId, AItem>,
        item_list_aid_rid_map: &RMap<AItemListId, RItemListId>,
        attr_aid_rid_map: &RMap<AAttrId, RAttrId>,
        effect_aid_rid_map: &RMap<AEffectId, REffectId>,
        attr_consts: &RAttrConsts,
        effect_consts: &REffectConsts,
        r_effects: &PSlab<REffectId, RcEffect>,
    ) {
        self.base.fill_runtime(
            a_items,
            item_list_aid_rid_map,
            attr_aid_rid_map,
            effect_aid_rid_map,
            r_effects,
        );
        self.attr_data.fill_runtime(
            &self.base,
            a_items,
            item_list_aid_rid_map,
            attr_aid_rid_map,
            attr_consts,
            effect_consts,
            r_effects,
        );
    }
}
