use crate::{
    err::basic::FitFoundError,
    sol::{FitId, FitKey, SolarSystem, info::DroneInfo},
};

impl SolarSystem {
    pub fn get_fit_drones(&self, fit_id: &FitId) -> Result<Vec<DroneInfo>, GetFitDronesError> {
        let fit_key = self.uad.fits.key_by_id_err(fit_id)?;
        Ok(self.get_fit_drones_internal(fit_key))
    }
    pub(in crate::sol) fn get_fit_drones_internal(&self, fit_key: FitKey) -> Vec<DroneInfo> {
        self.uad
            .fits
            .get(fit_key)
            .drones
            .iter()
            .map(|item_key| self.get_drone_internal(*item_key).unwrap())
            .collect()
    }
}

#[derive(thiserror::Error, Debug)]
pub enum GetFitDronesError {
    #[error("{0}")]
    FitNotFound(#[from] FitFoundError),
}
