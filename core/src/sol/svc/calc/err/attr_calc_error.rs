use crate::{err::basic::AttrMetaFoundError, sol::err::KeyedItemLoadedError};

#[derive(Debug)]
pub(in crate::sol) enum AttrCalcError {
    KeyedItemNotLoaded(KeyedItemLoadedError),
    AttrMetaNotFound(AttrMetaFoundError),
}
impl std::error::Error for AttrCalcError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::KeyedItemNotLoaded(e) => Some(e),
            Self::AttrMetaNotFound(e) => Some(e),
        }
    }
}
impl std::fmt::Display for AttrCalcError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::KeyedItemNotLoaded(e) => e.fmt(f),
            Self::AttrMetaNotFound(e) => e.fmt(f),
        }
    }
}
impl From<KeyedItemLoadedError> for AttrCalcError {
    fn from(error: KeyedItemLoadedError) -> Self {
        Self::KeyedItemNotLoaded(error)
    }
}
impl From<AttrMetaFoundError> for AttrCalcError {
    fn from(error: AttrMetaFoundError) -> Self {
        Self::AttrMetaNotFound(error)
    }
}
