use crate::{err::basic::AttrMetaFoundError, sol::err::KeyedItemLoadedError};

#[derive(thiserror::Error, Debug)]
pub(in crate::sol) enum AttrCalcError {
    #[error("{0}")]
    KeyedItemNotLoaded(#[from] KeyedItemLoadedError),
    #[error("{0}")]
    AttrMetaNotFound(#[from] AttrMetaFoundError),
}
