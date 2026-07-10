use crate::ud::UItemId;

#[derive(thiserror::Error, Debug)]
#[error("item with UID {item_uid} is not loaded")]
pub(crate) struct UItemLoadedError {
    pub(crate) item_uid: UItemId,
}
