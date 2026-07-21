use crate::{svc::err::UItemLoadedError, ud::UItemId};

#[derive(thiserror::Error, Debug)]
#[error("item with UID {item_uid} does not support requested stat")]
pub(crate) struct UItemKindVsStatError {
    pub(crate) item_uid: UItemId,
}

#[derive(thiserror::Error, Debug)]
pub(crate) enum IntItemStatError<CE>
where
    CE: std::error::Error,
{
    #[error("{0}")]
    ItemNotLoaded(#[from] UItemLoadedError),
    #[error("{0}")]
    UnsupportedStat(#[from] UItemKindVsStatError),
    #[error("{0}")]
    StatSpecific(CE),
}
