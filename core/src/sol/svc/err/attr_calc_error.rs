use crate::sol::err::basic::{AttrMetaFoundError, ItemFoundError, ItemLoadedError};

#[derive(Debug)]
pub(in crate::sol) enum AttrCalcError {
    ItemNotFound(ItemFoundError),
    ItemNotLoaded(ItemLoadedError),
    AttrMetaNotFound(AttrMetaFoundError),
}
impl From<ItemFoundError> for AttrCalcError {
    fn from(error: ItemFoundError) -> Self {
        Self::ItemNotFound(error)
    }
}
impl From<ItemLoadedError> for AttrCalcError {
    fn from(error: ItemLoadedError) -> Self {
        Self::ItemNotLoaded(error)
    }
}
impl From<AttrMetaFoundError> for AttrCalcError {
    fn from(error: AttrMetaFoundError) -> Self {
        Self::AttrMetaNotFound(error)
    }
}
impl std::error::Error for AttrCalcError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::ItemNotFound(e) => Some(e),
            Self::ItemNotLoaded(e) => Some(e),
            Self::AttrMetaNotFound(e) => Some(e),
        }
    }
}
impl std::fmt::Display for AttrCalcError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::ItemNotFound(e) => e.fmt(f),
            Self::ItemNotLoaded(e) => e.fmt(f),
            Self::AttrMetaNotFound(e) => e.fmt(f),
        }
    }
}
