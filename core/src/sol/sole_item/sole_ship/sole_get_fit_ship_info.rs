use crate::{
    err::basic::FitFoundError,
    sol::{FitId, FitKey, SolarSystem, info::ShipInfo},
};

impl SolarSystem {
    pub fn get_fit_ship_info(&self, fit_id: &FitId) -> Result<Option<ShipInfo>, GetFitShipInfoError> {
        let fit_key = self.uad.fits.key_by_id_err(fit_id)?;
        Ok(self.get_fit_ship_info_internal(fit_key))
    }
    pub fn get_fit_ship_info_internal(&self, fit_key: FitKey) -> Option<ShipInfo> {
        self.uad
            .fits
            .get(fit_key)
            .ship
            .map(|item_key| self.get_ship_info_internal(item_key).unwrap())
    }
}

#[derive(thiserror::Error, Debug)]
pub enum GetFitShipInfoError {
    #[error("{0}")]
    FitNotFound(#[from] FitFoundError),
}
