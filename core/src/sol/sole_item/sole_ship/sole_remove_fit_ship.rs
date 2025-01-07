use crate::{
    defs::SolFitId,
    err::basic::{FitFoundError, FitHasItemKindError},
    sol::{uad::item::SolShip, SolarSystem},
    util::Named,
};

impl SolarSystem {
    pub fn remove_fit_ship(&mut self, fit_id: &SolFitId) -> Result<(), RemoveFitShipError> {
        let fit = self.uad.fits.get_fit(fit_id)?;
        let item_id = match fit.ship {
            Some(item_id) => item_id,
            None => return Err(FitHasItemKindError::new(*fit_id, SolShip::get_name()).into()),
        };
        self.remove_ship(&item_id).unwrap();
        Ok(())
    }
}

#[derive(Debug)]
pub enum RemoveFitShipError {
    FitNotFound(FitFoundError),
    FitHasNoShip(FitHasItemKindError),
}
impl std::error::Error for RemoveFitShipError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::FitNotFound(e) => Some(e),
            Self::FitHasNoShip(e) => Some(e),
        }
    }
}
impl std::fmt::Display for RemoveFitShipError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::FitNotFound(e) => e.fmt(f),
            Self::FitHasNoShip(e) => e.fmt(f),
        }
    }
}
impl From<FitFoundError> for RemoveFitShipError {
    fn from(error: FitFoundError) -> Self {
        Self::FitNotFound(error)
    }
}
impl From<FitHasItemKindError> for RemoveFitShipError {
    fn from(error: FitHasItemKindError) -> Self {
        Self::FitHasNoShip(error)
    }
}
