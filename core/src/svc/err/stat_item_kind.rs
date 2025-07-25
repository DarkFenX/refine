use crate::ud::UItemKey;

#[derive(thiserror::Error, Debug)]
#[error("item with key {item_key} does not support requested stat")]
pub(crate) struct KeyedItemKindVsStatError {
    pub(crate) item_key: UItemKey,
}
