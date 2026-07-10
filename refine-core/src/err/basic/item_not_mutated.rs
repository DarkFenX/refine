use crate::ud::ItemId;

#[derive(thiserror::Error, Debug)]
#[error("item {item_id} is mutated")]
pub struct ItemNotMutatedError {
    pub item_id: ItemId,
}
