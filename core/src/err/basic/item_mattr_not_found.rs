use crate::{api::AttrId, def::ItemId};

#[derive(thiserror::Error, Debug)]
#[error("attribute {attr_id} on item {item_id} already contains mutation data")]
pub struct ItemMAttrNotFoundError {
    pub item_id: ItemId,
    pub attr_id: AttrId,
}
