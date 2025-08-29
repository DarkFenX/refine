use crate::{
    err::basic::{FitHasCharacterError, FitHasShipError, ItemLoadedError, SupportedStatError},
    sol::api::ItemStatError,
};

#[derive(thiserror::Error, Debug)]
pub enum FitShipStatError {
    #[error("{0}")]
    NoShip(#[from] FitHasShipError),
    #[error("{0}")]
    ItemNotLoaded(#[from] ItemLoadedError),
    #[error("{0}")]
    UnsupportedStat(#[from] SupportedStatError),
}
impl From<ItemStatError> for FitShipStatError {
    fn from(item_err: ItemStatError) -> Self {
        match item_err {
            ItemStatError::ItemNotLoaded(e) => e.into(),
            ItemStatError::UnsupportedStat(e) => e.into(),
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum FitCharacterStatError {
    #[error("{0}")]
    NoCharacter(#[from] FitHasCharacterError),
    #[error("{0}")]
    ItemNotLoaded(#[from] ItemLoadedError),
    #[error("{0}")]
    UnsupportedStat(#[from] SupportedStatError),
}
impl From<ItemStatError> for FitCharacterStatError {
    fn from(item_err: ItemStatError) -> Self {
        match item_err {
            ItemStatError::ItemNotLoaded(e) => e.into(),
            ItemStatError::UnsupportedStat(e) => e.into(),
        }
    }
}
