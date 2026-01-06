use crate::ud::ItemId;

#[derive(thiserror::Error, Debug)]
#[error("item {item_id} not found")]
pub struct ItemFoundError {
    pub item_id: ItemId,
}
