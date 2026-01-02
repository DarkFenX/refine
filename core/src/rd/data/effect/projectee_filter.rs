use crate::{
    ad::{AAttrId, AItemListId},
    nd::NEffectProjecteeFilter,
    rd::{RAttrId, RItemListId},
    util::RMap,
};

pub(crate) enum REffectProjecteeFilter {
    ItemList(RItemListId),
    ItemListAttr(RAttrId),
}
impl REffectProjecteeFilter {
    pub(in crate::rd::data::effect) fn try_from_n_projectee_filter(
        n_projectee_filter: &NEffectProjecteeFilter,
        item_list_id_key_map: &RMap<AItemListId, RItemListId>,
        attr_id_key_map: &RMap<AAttrId, RAttrId>,
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
    pub(crate) fn get_item_list_attr_r_id(&self) -> Option<RAttrId> {
        match self {
            Self::ItemList(_) => None,
            Self::ItemListAttr(attr_r_id) => Some(*attr_r_id),
        }
    }
}
