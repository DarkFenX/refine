use crate::cacher_json::data::{CItemId, CItemListId};

#[derive(serde_tuple::Serialize_tuple, serde_tuple::Deserialize_tuple)]
pub(in crate::cacher_json) struct CItemList {
    id: CItemListId,
    item_ids: Vec<CItemId>,
}
impl From<&rc::ad::AItemList> for CItemList {
    fn from(a_item_list: &rc::ad::AItemList) -> Self {
        CItemList {
            id: (&a_item_list.id).into(),
            item_ids: a_item_list.item_ids.iter().copied().collect(),
        }
    }
}
impl From<&CItemList> for rc::ad::AItemList {
    fn from(c_item_list: &CItemList) -> Self {
        Self {
            id: (&c_item_list.id).into(),
            item_ids: c_item_list.item_ids.iter().copied().collect(),
        }
    }
}
