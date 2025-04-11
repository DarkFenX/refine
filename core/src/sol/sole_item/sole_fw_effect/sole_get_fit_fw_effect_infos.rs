use crate::{
    err::basic::FitFoundError,
    sol::{FitId, FitKey, SolarSystem, info::FwEffectInfo},
};

impl SolarSystem {
    pub fn get_fit_fw_effect_infos(&self, fit_id: &FitId) -> Result<Vec<FwEffectInfo>, GetFitFwEffectInfosError> {
        let fit_key = self.uad.fits.key_by_id_err(fit_id)?;
        Ok(self.get_fit_fw_effect_infos_internal(fit_key))
    }
    pub(in crate::sol) fn get_fit_fw_effect_infos_internal(&self, fit_key: FitKey) -> Vec<FwEffectInfo> {
        self.uad
            .fits
            .get(fit_key)
            .fw_effects
            .iter()
            .map(|item_key| self.get_fw_effect_info_internal(*item_key).unwrap())
            .collect()
    }
}

#[derive(thiserror::Error, Debug)]
pub enum GetFitFwEffectInfosError {
    #[error("{0}")]
    FitNotFound(#[from] FitFoundError),
}
