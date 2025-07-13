use crate::{
    err::basic::{FitHasShipError, ItemLoadedError},
    sol::api::ItemStatError,
};

#[derive(thiserror::Error, Debug)]
pub enum FitShipStatError {
    #[error("{0}")]
    NoShip(#[from] FitHasShipError),
    #[error("{0}")]
    ItemNotLoaded(#[from] ItemLoadedError),
}
impl From<ItemStatError> for FitShipStatError {
    fn from(item_err: ItemStatError) -> Self {
        match item_err {
            ItemStatError::ItemNotLoaded(e) => e.into(),
            // All stats exposed on fit are supposed to be fetchable from ship
            ItemStatError::UnsupportedStat(_) => unreachable!(),
        }
    }
}
