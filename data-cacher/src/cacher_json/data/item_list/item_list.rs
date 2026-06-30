use crate::cacher_json::data::AdaptedConv;

#[serde_with::serde_as]
#[derive(serde_tuple::Serialize_tuple, serde_tuple::Deserialize_tuple)]
pub(in crate::cacher_json::data) struct CItemList {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    id: rc::ad::AItemListId,
    item_ids: Vec<i32>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl AdaptedConv for CItemList {
    type AEntity = rc::ad::AItemList;

    fn from_adapted(a_item_list: &Self::AEntity) -> Self {
        Self {
            id: a_item_list.id,
            item_ids: a_item_list.item_ids.iter().map(|v| v.into_i32()).collect(),
        }
    }

    fn into_adapted(self) -> Self::AEntity {
        Self::AEntity {
            id: self.id,
            item_ids: self.item_ids.into_iter().map(rc::ad::AItemId::from_i32).collect(),
        }
    }
}
