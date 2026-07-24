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

////////////////////////////////////////////////////////////////////////////////////////////////////
// Fit errors
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Clone, Debug, thiserror::Error)]
pub enum FitAppliedStatError {
    #[error("{0}")]
    ProjecteeNotFound(#[from] ItemFoundError),
    #[error("{0}")]
    ProjecteeCantTakeProjs(#[from] ItemReceiveProjError),
}
// Conversions
impl From<ProjecteeUidError> for FitAppliedStatError {
    fn from(uid_err: ProjecteeUidError) -> Self {
        match uid_err {
            ProjecteeUidError::ProjecteeNotFound(e) => e.into(),
            ProjecteeUidError::ProjecteeCantTakeProjs(e) => e.into(),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Fit ship errors
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Clone, Debug, thiserror::Error)]
pub enum FitShipStatError<SS>
where
    SS: std::error::Error,
{
    #[error("{0}")]
    NoShip(#[from] FitHasShipError),
    #[error("{0}")]
    ItemNotLoaded(#[from] ItemLoadedError),
    #[error("{0}")]
    UnsupportedStat(#[from] SupportedStatError),
    #[error("{0}")]
    StatSpecific(#[source] SS),
}
// Conversions
impl<SS> From<ItemStatError<SS>> for FitShipStatError<SS>
where
    SS: std::error::Error,
{
    fn from(item_err: ItemStatError<SS>) -> Self {
        match item_err {
            ItemStatError::ItemNotLoaded(err) => err.into(),
            ItemStatError::UnsupportedStat(err) => err.into(),
            ItemStatError::StatSpecific(err) => Self::StatSpecific(err),
        }
    }
}

#[derive(Clone, Debug, thiserror::Error)]
pub enum FitShipAppliedStatError<SS>
where
    SS: std::error::Error,
{
    #[error("{0}")]
    NoShip(#[from] FitHasShipError),
    #[error("{0}")]
    ItemNotLoaded(#[from] ItemLoadedError),
    #[error("{0}")]
    UnsupportedStat(#[from] SupportedStatError),
    #[error("{0}")]
    StatSpecific(#[source] SS),
    #[error("{0}")]
    ProjecteeNotFound(#[from] ItemFoundError),
    #[error("{0}")]
    ProjecteeCantTakeProjs(#[from] ItemReceiveProjError),
}
// Conversions
impl<SS> From<ItemAppliedStatError<SS>> for FitShipAppliedStatError<SS>
where
    SS: std::error::Error,
{
    fn from(item_err: ItemAppliedStatError<SS>) -> Self {
        match item_err {
            ItemAppliedStatError::ItemNotLoaded(err) => err.into(),
            ItemAppliedStatError::UnsupportedStat(err) => err.into(),
            ItemAppliedStatError::StatSpecific(err) => Self::StatSpecific(err),
            ItemAppliedStatError::ProjecteeNotFound(err) => err.into(),
            ItemAppliedStatError::ProjecteeCantTakeProjs(err) => err.into(),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Fit character errors
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Clone, Debug, thiserror::Error)]
pub enum FitCharacterStatError<SS>
where
    SS: std::error::Error,
{
    #[error("{0}")]
    NoCharacter(#[from] FitHasCharacterError),
    #[error("{0}")]
    ItemNotLoaded(#[from] ItemLoadedError),
    #[error("{0}")]
    UnsupportedStat(#[from] SupportedStatError),
    #[error("{0}")]
    StatSpecific(#[source] SS),
}
// Conversions
impl<SS> From<ItemStatError<SS>> for FitCharacterStatError<SS>
where
    SS: std::error::Error,
{
    fn from(item_err: ItemStatError<SS>) -> Self {
        match item_err {
            ItemStatError::ItemNotLoaded(err) => err.into(),
            ItemStatError::UnsupportedStat(err) => err.into(),
            ItemStatError::StatSpecific(err) => Self::StatSpecific(err),
        }
    }
}
