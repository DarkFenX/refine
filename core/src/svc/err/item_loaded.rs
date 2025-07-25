use crate::ud::UItemKey;

#[derive(thiserror::Error, Debug)]
#[error("item with key {item_key} is not loaded")]
pub(crate) struct KeyedItemLoadedError {
    pub(crate) item_key: UItemKey,
}
