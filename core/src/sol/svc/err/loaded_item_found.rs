use crate::sol::err::basic::{ItemFoundError, ItemLoadedError};

#[derive(Debug)]
pub(in crate::sol) enum LoadedItemFoundError {
    ItemNotFound(ItemFoundError),
    ItemNotLoaded(ItemLoadedError),
}
impl From<ItemFoundError> for LoadedItemFoundError {
    fn from(error: ItemFoundError) -> Self {
        Self::ItemNotFound(error)
    }
}
impl From<ItemLoadedError> for LoadedItemFoundError {
    fn from(error: ItemLoadedError) -> Self {
        Self::ItemNotLoaded(error)
    }
}
impl std::error::Error for LoadedItemFoundError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::ItemNotFound(e) => Some(e),
            Self::ItemNotLoaded(e) => Some(e),
        }
    }
}
impl std::fmt::Display for LoadedItemFoundError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::ItemNotFound(e) => e.fmt(f),
            Self::ItemNotLoaded(e) => e.fmt(f),
        }
    }
}
