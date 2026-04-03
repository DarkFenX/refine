use crate::{
    api::{CharacterMut, FitMut, ItemStatAppliedError, ItemStatError, ShipMut},
    err::basic::{
        FitHasCharacterError, FitHasShipError, ItemFoundError, ItemLoadedError, ItemReceiveProjError,
        SupportedStatError,
    },
    ud::ProjecteeUidError,
};

impl<'a> FitMut<'a> {
    pub(super) fn get_character_for_stats(&mut self) -> Result<CharacterMut<'_>, FitHasCharacterError> {
        let char_uid = match self.sol.u_data.fits.get(self.uid).character {
            Some(char_uid) => char_uid,
            None => {
                return Err(FitHasCharacterError {
                    fit_id: self.sol.u_data.fits.xid_by_iid(self.uid),
                });
            }
        };
        Ok(CharacterMut::new(self.sol, char_uid))
    }
    pub(super) fn get_ship_for_stats(&mut self) -> Result<ShipMut<'_>, FitHasShipError> {
        let ship_uid = match self.sol.u_data.fits.get(self.uid).ship {
            Some(ship_uid) => ship_uid,
            None => {
                return Err(FitHasShipError {
                    fit_id: self.sol.u_data.fits.xid_by_iid(self.uid),
                });
            }
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
impl From<ItemStatAppliedError> for FitShipAppliedStatError {
    fn from(item_err: ItemStatAppliedError) -> Self {
        match item_err {
            ItemStatAppliedError::ItemNotLoaded(e) => e.into(),
            ItemStatAppliedError::UnsupportedStat(e) => e.into(),
            ItemStatAppliedError::ProjecteeNotFound(e) => e.into(),
            ItemStatAppliedError::ProjecteeCantTakeProjs(e) => e.into(),
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
