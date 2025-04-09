use crate::sol::ItemKey;

#[derive(thiserror::Error, Debug)]
#[error("item with key {item_key} is not loaded")]
pub(in crate::sol) struct KeyedItemLoadedError {
    pub(in crate::sol) item_key: ItemKey,
}
