use crate::svc::{err::KeyedItemLoadedError, vast::err::KeyedItemKindVsStatError};

#[derive(thiserror::Error, Debug)]
pub enum StatItemCheckError {
    #[error("{0}")]
    ItemNotLoaded(#[from] KeyedItemLoadedError),
    #[error("{0}")]
    UnsupportedStat(#[from] KeyedItemKindVsStatError),
}
