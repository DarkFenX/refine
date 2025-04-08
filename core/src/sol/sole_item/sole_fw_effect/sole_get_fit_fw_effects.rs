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

#[derive(Debug)]
pub enum GetFitFwEffectsError {
    FitNotFound(FitFoundError),
}
impl std::error::Error for GetFitFwEffectsError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::FitNotFound(e) => Some(e),
        }
    }
}
impl std::fmt::Display for GetFitFwEffectsError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::FitNotFound(e) => e.fmt(f),
        }
    }
}
impl From<FitFoundError> for GetFitFwEffectsError {
    fn from(error: FitFoundError) -> Self {
        Self::FitNotFound(error)
    }
}
