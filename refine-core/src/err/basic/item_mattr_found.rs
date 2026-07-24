use crate::{api::AttrId, ud::ItemId};

#[derive(Debug, thiserror::Error)]
#[error("attribute {attr_id} on item {item_id} contains no mutation data")]
pub struct ItemMAttrFoundError {
    pub item_id: ItemId,
    pub attr_id: AttrId,
}
