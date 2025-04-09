use crate::{
    err::basic::FitFoundError,
    sol::{FitId, FitKey, SolarSystem, info::ShipInfo},
};

impl SolarSystem {
    pub fn get_fit_ship(&self, fit_id: &FitId) -> Result<Option<ShipInfo>, GetFitShipError> {
        let fit_key = self.uad.fits.key_by_id_err(fit_id)?;
        Ok(self.get_fit_ship_internal(fit_key))
    }
    pub fn get_fit_ship_internal(&self, fit_key: FitKey) -> Option<ShipInfo> {
        self.uad
            .fits
            .get(fit_key)
            .ship
            .map(|item_key| self.get_ship_internal(item_key).unwrap())
    }
}

#[derive(thiserror::Error, Debug)]
pub enum GetFitShipError {
    #[error("{0}")]
    FitNotFound(#[from] FitFoundError),
}
