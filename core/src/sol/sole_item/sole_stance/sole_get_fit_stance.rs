use crate::{
    err::basic::FitFoundError,
    sol::{FitId, FitKey, SolarSystem, info::StanceInfo},
};

impl SolarSystem {
    pub fn get_fit_stance_info(&self, fit_id: &FitId) -> Result<Option<StanceInfo>, GetFitStanceInfoError> {
        let fit_key = self.uad.fits.key_by_id_err(fit_id)?;
        Ok(self.get_fit_stance_info_internal(fit_key))
    }
    pub(in crate::sol) fn get_fit_stance_info_internal(&self, fit_key: FitKey) -> Option<StanceInfo> {
        self.uad
            .fits
            .get(fit_key)
            .stance
            .map(|item_key| self.get_stance_info_internal(item_key).unwrap())
    }
}

#[derive(thiserror::Error, Debug)]
pub enum GetFitStanceInfoError {
    #[error("{0}")]
    FitNotFound(#[from] FitFoundError),
}
