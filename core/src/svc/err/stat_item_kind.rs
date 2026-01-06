use crate::ud::UItemId;

#[derive(thiserror::Error, Debug)]
#[error("item with UID {item_uid} does not support requested stat")]
pub(crate) struct UItemKindVsStatError {
    pub(crate) item_uid: UItemId,
}
