use crate::sol::ItemId;

#[derive(thiserror::Error, Debug)]
#[error("item {item_id} is not mutated")]
pub(in crate::sol) struct ItemMutatedError {
    pub(in crate::sol) item_id: ItemId,
}
