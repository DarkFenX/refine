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
        item_list_aid_rid_map: &RMap<AItemListId, RItemListId>,
        attr_aid_rid_map: &RMap<AAttrId, RAttrId>,
    ) -> Option<Self> {
        match n_projectee_filter {
            NEffectProjecteeFilter::ItemList(item_list_aid) => {
                let item_list_rid = *item_list_aid_rid_map.get(item_list_aid)?;
                Some(Self::ItemList(item_list_rid))
            }
            NEffectProjecteeFilter::ItemListAttr(attr_aid) => {
                let attr_rid = *attr_aid_rid_map.get(attr_aid)?;
                Some(Self::ItemListAttr(attr_rid))
            }
        }
    }
    pub(crate) fn get_item_list_attr_rid(&self) -> Option<RAttrId> {
        match self {
            Self::ItemList(_) => None,
            Self::ItemListAttr(attr_rid) => Some(*attr_rid),
        }
    }
}
