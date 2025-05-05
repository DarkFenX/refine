use crate::sol::{AttrId, ItemId};

#[derive(thiserror::Error, Debug)]
#[error("attribute {attr_id} has no base value on item {item_id}")]
pub struct ItemMAttrValueError {
    pub item_id: ItemId,
    pub attr_id: AttrId,
}
