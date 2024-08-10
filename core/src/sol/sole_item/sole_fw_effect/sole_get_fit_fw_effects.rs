use crate::{
    defs::SolFitId,
    sol::{err::basic::FitFoundError, item_info::SolFwEffectInfo, SolarSystem},
};

impl SolarSystem {
    pub fn get_fit_fw_effects(&self, fit_id: &SolFitId) -> Result<Vec<SolFwEffectInfo>, GetFitFwEffectsError> {
        let fit = self.fits.get_fit(fit_id)?;
        let fw_effect_infos = fit
            .fw_effects
            .iter()
            .map(|v| SolFwEffectInfo::from(self.items.get_item(v).unwrap().get_fw_effect().unwrap()))
            .collect();
        Ok(fw_effect_infos)
    }
}

#[derive(Debug)]
pub enum GetFitFwEffectsError {
    FitNotFound(FitFoundError),
}
impl From<FitFoundError> for GetFitFwEffectsError {
    fn from(error: FitFoundError) -> Self {
        Self::FitNotFound(error)
    }
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
