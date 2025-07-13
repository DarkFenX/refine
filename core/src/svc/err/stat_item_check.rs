use crate::svc::err::{KeyedItemKindVsStatError, KeyedItemLoadedError};

#[derive(thiserror::Error, Debug)]
pub enum StatItemCheckError {
    #[error("{0}")]
    ItemNotLoaded(#[from] KeyedItemLoadedError),
    #[error("{0}")]
    UnsupportedStat(#[from] KeyedItemKindVsStatError),
}
