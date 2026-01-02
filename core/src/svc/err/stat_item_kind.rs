use crate::ud::UItemId;

#[derive(thiserror::Error, Debug)]
#[error("item with key {item_key} does not support requested stat")]
pub(crate) struct KeyedItemKindVsStatError {
    pub(crate) item_key: UItemId,
}
