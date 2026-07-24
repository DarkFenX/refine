use crate::{api::AttrId, ud::ItemId};

#[derive(Debug, thiserror::Error)]
#[error("attribute {attr_id} on item {item_id} already contains mutation data")]
pub struct ItemMAttrNotFoundError {
    pub item_id: ItemId,
    pub attr_id: AttrId,
}
