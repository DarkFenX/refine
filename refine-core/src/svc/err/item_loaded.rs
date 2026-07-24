use crate::ud::UItemId;

#[derive(Debug, thiserror::Error)]
#[error("item with UID {item_uid:?} is not loaded")]
pub(crate) struct UItemLoadedError {
    pub(crate) item_uid: UItemId,
}
