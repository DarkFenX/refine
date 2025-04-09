use crate::{
    err::basic::FitFoundError,
    sol::{FitId, FitKey, SolarSystem, info::BoosterInfo},
};

impl SolarSystem {
    pub fn get_fit_boosters(&self, fit_id: &FitId) -> Result<Vec<BoosterInfo>, GetFitBoostersError> {
        let fit_key = self.uad.fits.key_by_id_err(fit_id)?;
        Ok(self.get_fit_boosters_internal(fit_key))
    }
    pub(in crate::sol) fn get_fit_boosters_internal(&self, fit_key: FitKey) -> Vec<BoosterInfo> {
        self.uad
            .fits
            .get(fit_key)
            .boosters
            .iter()
            .map(|item_key| self.get_booster_internal(*item_key).unwrap())
            .collect()
    }
}

#[derive(thiserror::Error, Debug)]
pub enum GetFitBoostersError {
    #[error("{0}")]
    FitNotFound(#[from] FitFoundError),
}
