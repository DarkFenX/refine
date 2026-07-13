use crate::{
    api::{CharacterMut, FitMut, ItemAppliedStatError, ItemStatError, ShipMut},
    err::basic::{
        FitHasCharacterError, FitHasShipError, ItemFoundError, ItemLoadedError, ItemReceiveProjError,
        SupportedStatError,
    },
    ud::ProjecteeUidError,
};

impl<'s> FitMut<'s> {
    pub(super) fn get_character_for_stats(&mut self) -> Result<CharacterMut<'_>, FitHasCharacterError> {
        let Some(char_uid) = self.sol.u_data.fits.get(self.uid).character else {
            return Err(FitHasCharacterError {
                fit_id: self.sol.u_data.fits.ext_id_by_int_id(self.uid),
            });
        };
        Ok(CharacterMut::new(self.sol, char_uid))
    }
    pub(super) fn get_ship_for_stats(&mut self) -> Result<ShipMut<'_>, FitHasShipError> {
        let Some(ship_uid) = self.sol.u_data.fits.get(self.uid).ship else {
            return Err(FitHasShipError {
                fit_id: self.sol.u_data.fits.ext_id_by_int_id(self.uid),
            });
        };
        Ok(ShipMut::new(self.sol, ship_uid))
    }
}

#[derive(thiserror::Error, Debug)]
pub enum FitAppliedStatError {
    #[error("{0}")]
    ProjecteeNotFound(#[from] ItemFoundError),
    #[error("{0}")]
    ProjecteeCantTakeProjs(#[from] ItemReceiveProjError),
}
impl From<ProjecteeUidError> for FitAppliedStatError {
    fn from(uid_err: ProjecteeUidError) -> Self {
        match uid_err {
            ProjecteeUidError::ProjecteeNotFound(e) => e.into(),
            ProjecteeUidError::ProjecteeCantTakeProjs(e) => e.into(),
        }
    }
}

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
pub enum FitShipAppliedStatError {
    #[error("{0}")]
    NoShip(#[from] FitHasShipError),
    #[error("{0}")]
    ItemNotLoaded(#[from] ItemLoadedError),
    #[error("{0}")]
    UnsupportedStat(#[from] SupportedStatError),
    #[error("{0}")]
    ProjecteeNotFound(#[from] ItemFoundError),
    #[error("{0}")]
    ProjecteeCantTakeProjs(#[from] ItemReceiveProjError),
}
impl From<ItemAppliedStatError> for FitShipAppliedStatError {
    fn from(item_err: ItemAppliedStatError) -> Self {
        match item_err {
            ItemAppliedStatError::ItemNotLoaded(e) => e.into(),
            ItemAppliedStatError::UnsupportedStat(e) => e.into(),
            ItemAppliedStatError::ProjecteeNotFound(e) => e.into(),
            ItemAppliedStatError::ProjecteeCantTakeProjs(e) => e.into(),
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
