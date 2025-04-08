use crate::{
    err::basic::FitFoundError,
    sol::{FitId, SolarSystem, info::StanceInfo},
};

impl SolarSystem {
    pub fn get_fit_stance(&self, fit_id: &FitId) -> Result<Option<StanceInfo>, GetFitStanceError> {
        let fit = self.uad.fits.get_fit(fit_id)?;
        Ok(fit.stance.map(|item_key| self.get_stance_internal(item_key).unwrap()))
    }
}

#[derive(Debug)]
pub enum GetFitStanceError {
    FitNotFound(FitFoundError),
}
impl std::error::Error for GetFitStanceError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::FitNotFound(e) => Some(e),
        }
    }
}
impl std::fmt::Display for GetFitStanceError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::FitNotFound(e) => e.fmt(f),
        }
    }
}
impl From<FitFoundError> for GetFitStanceError {
    fn from(error: FitFoundError) -> Self {
        Self::FitNotFound(error)
    }
}
