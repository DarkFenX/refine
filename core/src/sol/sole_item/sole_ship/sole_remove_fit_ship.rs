use crate::{
    err::basic::{FitFoundError, FitHasItemKindError},
    sol::{FitId, FitKey, SolarSystem, uad::item::Ship},
    util::Named,
};

impl SolarSystem {
    pub fn remove_fit_ship(&mut self, fit_id: &FitId) -> Result<(), RemoveFitShipError> {
        let fit_key = self.uad.fits.key_by_id_err(fit_id)?;
        Ok(self.remove_fit_ship_internal(fit_key)?)
    }
    pub(in crate::sol) fn remove_fit_ship_internal(&mut self, fit_key: FitKey) -> Result<(), FitHasItemKindError> {
        let fit = self.uad.fits.get(fit_key);
        let item_key = match fit.ship {
            Some(item_key) => item_key,
            None => {
                return Err(FitHasItemKindError {
                    fit_id: fit.id,
                    item_kind: Ship::get_name(),
                });
            }
        };
        self.remove_ship_internal(item_key).unwrap();
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
