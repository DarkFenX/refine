#[serde_with::serde_as]
#[derive(serde_tuple::Serialize_tuple, serde_tuple::Deserialize_tuple)]
pub(in crate::cacher_json::data) struct CItemList {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    id: rc::ad::AItemListId,
    item_ids: Vec<i32>,
}
impl CItemList {
    pub(in crate::cacher_json::data) fn from_adapted(a_item_list: &rc::ad::AItemList) -> Self {
        Self {
            id: a_item_list.id,
            item_ids: a_item_list.item_ids.iter().map(|v| v.into_i32()).collect(),
        }
    }
    pub(in crate::cacher_json::data) fn into_adapted(self) -> rc::ad::AItemList {
        rc::ad::AItemList {
            id: self.id,
            item_ids: self
                .item_ids
                .into_iter()
                .map(|v| rc::ad::AItemId::from_i32(v))
                .collect(),
        }
    }
}
