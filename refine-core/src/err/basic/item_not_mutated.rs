use crate::ud::ItemId;

#[derive(Debug, thiserror::Error)]
#[error("item {item_id} is mutated")]
pub struct ItemNotMutatedError {
    pub item_id: ItemId,
}
