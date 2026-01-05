use crate::ud::ItemId;

#[derive(thiserror::Error, Debug)]
#[error("item {item_id} is not mutated")]
pub(crate) struct ItemMutatedError {
    pub(crate) item_id: ItemId,
}
