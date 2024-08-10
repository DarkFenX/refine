use crate::{
    defs::SolFitId,
    err::FitFoundError,
    sol::{item_info::SolFwEffectInfo, SolarSystem},
};

impl SolarSystem {
    pub fn get_fit_fw_effect_infos(&self, fit_id: &SolFitId) -> Result<Vec<SolFwEffectInfo>, GetFitFwEffectInfosError> {
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
pub enum GetFitFwEffectInfosError {
    FitNotFound(FitFoundError),
}
impl From<FitFoundError> for GetFitFwEffectInfosError {
    fn from(error: FitFoundError) -> Self {
        Self::FitNotFound(error)
    }
}
impl std::error::Error for GetFitFwEffectInfosError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::FitNotFound(e) => Some(e),
        }
    }
}
impl std::fmt::Display for GetFitFwEffectInfosError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::FitNotFound(e) => e.fmt(f),
        }
    }
}
