use crate::uad::UadItemKey;

#[derive(thiserror::Error, Debug)]
#[error("item with key {item_key} is not loaded")]
pub(crate) struct KeyedItemLoadedError {
    pub(crate) item_key: UadItemKey,
}
