use crate::{api::AttrId, ud::ItemId};

#[derive(Debug, thiserror::Error)]
#[error("attribute {attr_id} has no base value on item {item_id}")]
pub struct ItemMAttrValueError {
    pub item_id: ItemId,
    pub attr_id: AttrId,
}
