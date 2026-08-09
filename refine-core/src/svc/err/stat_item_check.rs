use crate::{svc::err::UItemLoadedError, ud::UItemId};

#[derive(Debug, thiserror::Error)]
#[error("item with UID {item_uid:?} does not support requested stat")]
pub(crate) struct UItemKindVsStatError {
    pub(crate) item_uid: UItemId,
}

#[derive(Debug, thiserror::Error)]
pub(crate) enum IntStatItemError<SS>
where
    SS: std::error::Error,
{
    #[error(transparent)]
    ItemNotLoaded(#[from] UItemLoadedError),
    #[error(transparent)]
    UnsupportedStat(#[from] UItemKindVsStatError),
    #[error(transparent)]
    StatSpecific(SS),
}
