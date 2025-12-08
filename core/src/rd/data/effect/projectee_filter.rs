use crate::{
    ad::{AAttrId, AItemListId},
    nd::NEffectProjecteeFilter,
    rd::{RAttrKey, RItemListKey},
    util::RMap,
};

pub(crate) enum REffectProjecteeFilter {
    ItemList(RItemListKey),
    ItemListAttr(RAttrKey),
}
impl REffectProjecteeFilter {
    pub(in crate::rd::data::effect) fn try_from_n_projectee_filter(
        n_projectee_filter: &NEffectProjecteeFilter,
        item_list_id_key_map: &RMap<AItemListId, RItemListKey>,
        attr_id_key_map: &RMap<AAttrId, RAttrKey>,
    ) -> Option<Self> {
        match n_projectee_filter {
            NEffectProjecteeFilter::ItemList(item_list_id) => {
                let item_list_key = *item_list_id_key_map.get(item_list_id)?;
                Some(Self::ItemList(item_list_key))
            }
            NEffectProjecteeFilter::ItemListAttr(attr_id) => {
                let attr_key = *attr_id_key_map.get(attr_id)?;
                Some(Self::ItemListAttr(attr_key))
            }
        }
    }
    pub(crate) fn get_item_list_attr_key(&self) -> Option<RItemListKey> {
        match self {
            Self::ItemList(_) => None,
            Self::ItemListAttr(attr_key) => Some(*attr_key),
        }
    }
}
