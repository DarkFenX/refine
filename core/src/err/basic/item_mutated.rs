use crate::sol::ItemId;

#[derive(thiserror::Error, Debug)]
#[error("item {item_id} is not mutated")]
pub struct ItemMutatedError {
    pub item_id: ItemId,
}
