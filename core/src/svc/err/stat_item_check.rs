use crate::svc::err::{KeyedItemKindVsStatError, UItemLoadedError};

#[derive(thiserror::Error, Debug)]
pub enum StatItemCheckError {
    #[error("{0}")]
    ItemNotLoaded(#[from] UItemLoadedError),
    #[error("{0}")]
    UnsupportedStat(#[from] KeyedItemKindVsStatError),
}
