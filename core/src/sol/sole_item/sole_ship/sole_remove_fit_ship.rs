use crate::{
    err::basic::{FitFoundError, FitHasItemKindError},
    sol::{FitId, FitKey, SolarSystem, uad::item::UadShip},
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
                    item_kind: UadShip::get_name(),
                });
            }
        };
        self.remove_ship_internal(item_key).unwrap();
        Ok(())
    }
}

#[derive(thiserror::Error, Debug)]
pub enum RemoveFitShipError {
    #[error("{0}")]
    FitNotFound(#[from] FitFoundError),
    #[error("{0}")]
    FitHasNoShip(#[from] FitHasItemKindError),
}
