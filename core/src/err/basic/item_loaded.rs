use crate::sol::ItemId;

#[derive(thiserror::Error, Debug)]
#[error("item {item_id} is not loaded")]
pub struct ItemLoadedError {
    pub item_id: ItemId,
}
