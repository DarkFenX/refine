use crate::ad::{AItemListId, AItemListItemIds};

#[cfg_attr(
    feature = "serde-ad",
    derive(serde_tuple::Serialize_tuple, serde_tuple::Deserialize_tuple)
)]
pub struct AItemList {
    pub id: AItemListId,
    pub item_ids: AItemListItemIds,
}
