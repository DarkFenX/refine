use crate::{
    defs::SolFitId,
    err::FitFoundError,
    sol::{item_info::SolStanceInfo, SolarSystem},
};

impl SolarSystem {
    pub fn get_fit_stance_info(&self, fit_id: &SolFitId) -> Result<Option<SolStanceInfo>, GetFitStanceInfoError> {
        let fit = self.fits.get_fit(&fit_id)?;
        Ok(fit
            .stance
            .map(|v| SolStanceInfo::from(self.items.get_item(&v).unwrap().get_stance().unwrap())))
    }
}

#[derive(Debug)]
pub enum GetFitStanceInfoError {
    FitNotFound(FitFoundError),
}
impl From<FitFoundError> for GetFitStanceInfoError {
    fn from(error: FitFoundError) -> Self {
        Self::FitNotFound(error)
    }
}
impl std::error::Error for GetFitStanceInfoError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::FitNotFound(e) => Some(e),
        }
    }
}
impl std::fmt::Display for GetFitStanceInfoError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::FitNotFound(e) => e.fmt(f),
        }
    }
}
