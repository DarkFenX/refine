use crate::{
    err::basic::FitFoundError,
    sol::{FitId, FitKey, SolarSystem, info::FwEffectInfo},
};

impl SolarSystem {
    pub fn get_fit_fw_effects(&self, fit_id: &FitId) -> Result<Vec<FwEffectInfo>, GetFitFwEffectsError> {
        let fit_key = self.uad.fits.key_by_id_err(fit_id)?;
        Ok(self.get_fit_fw_effects_internal(fit_key))
    }
    pub(in crate::sol) fn get_fit_fw_effects_internal(&self, fit_key: FitKey) -> Vec<FwEffectInfo> {
        self.uad
            .fits
            .get(fit_key)
            .fw_effects
            .iter()
            .map(|item_key| self.get_fw_effect_internal(*item_key).unwrap())
            .collect()
    }
}

#[derive(thiserror::Error, Debug)]
pub enum GetFitFwEffectsError {
    #[error("{0}")]
    FitNotFound(#[from] FitFoundError),
}
