use crate::{
    CharacterMut, FitMut, ShipMut,
    err::basic::{
        FitHasCharacterError, FitHasShipError, ItemFoundError, ItemLoadedError, ItemReceiveProjError,
        SupportedStatError,
    },
    stats::err::{StatItemAppliedError, StatItemError},
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
pub enum StatFitAppliedError {
    #[error(transparent)]
    ProjecteeNotFound(#[from] ItemFoundError),
    #[error(transparent)]
    ProjecteeCantTakeProjs(#[from] ItemReceiveProjError),
}
// Conversions
impl From<ProjecteeUidError> for StatFitAppliedError {
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
pub enum StatFitShipError<SS>
where
    SS: std::error::Error,
{
    #[error(transparent)]
    NoShip(#[from] FitHasShipError),
    #[error(transparent)]
    ItemNotLoaded(#[from] ItemLoadedError),
    #[error(transparent)]
    UnsupportedStat(#[from] SupportedStatError),
    #[error(transparent)]
    StatSpecific(SS),
}
// Conversions
impl<SS> From<StatItemError<SS>> for StatFitShipError<SS>
where
    SS: std::error::Error,
{
    fn from(item_err: StatItemError<SS>) -> Self {
        match item_err {
            StatItemError::ItemNotLoaded(err) => err.into(),
            StatItemError::UnsupportedStat(err) => err.into(),
            StatItemError::StatSpecific(err) => Self::StatSpecific(err),
        }
    }
}

#[derive(Clone, Debug, thiserror::Error)]
pub enum StatFitShipAppliedError<SS>
where
    SS: std::error::Error,
{
    #[error(transparent)]
    NoShip(#[from] FitHasShipError),
    #[error(transparent)]
    ItemNotLoaded(#[from] ItemLoadedError),
    #[error(transparent)]
    UnsupportedStat(#[from] SupportedStatError),
    #[error(transparent)]
    StatSpecific(SS),
    #[error(transparent)]
    ProjecteeNotFound(#[from] ItemFoundError),
    #[error(transparent)]
    ProjecteeCantTakeProjs(#[from] ItemReceiveProjError),
}
// Conversions
impl<SS> From<StatItemAppliedError<SS>> for StatFitShipAppliedError<SS>
where
    SS: std::error::Error,
{
    fn from(item_err: StatItemAppliedError<SS>) -> Self {
        match item_err {
            StatItemAppliedError::ItemNotLoaded(err) => err.into(),
            StatItemAppliedError::UnsupportedStat(err) => err.into(),
            StatItemAppliedError::StatSpecific(err) => Self::StatSpecific(err),
            StatItemAppliedError::ProjecteeNotFound(err) => err.into(),
            StatItemAppliedError::ProjecteeCantTakeProjs(err) => err.into(),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Fit character errors
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Clone, Debug, thiserror::Error)]
pub enum StatFitCharacterError<SS>
where
    SS: std::error::Error,
{
    #[error(transparent)]
    NoCharacter(#[from] FitHasCharacterError),
    #[error(transparent)]
    ItemNotLoaded(#[from] ItemLoadedError),
    #[error(transparent)]
    UnsupportedStat(#[from] SupportedStatError),
    #[error(transparent)]
    StatSpecific(SS),
}
// Conversions
impl<SS> From<StatItemError<SS>> for StatFitCharacterError<SS>
where
    SS: std::error::Error,
{
    fn from(item_err: StatItemError<SS>) -> Self {
        match item_err {
            StatItemError::ItemNotLoaded(err) => err.into(),
            StatItemError::UnsupportedStat(err) => err.into(),
            StatItemError::StatSpecific(err) => Self::StatSpecific(err),
        }
    }
}
