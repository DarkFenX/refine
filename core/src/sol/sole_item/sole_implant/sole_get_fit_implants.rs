use crate::{
    err::basic::FitFoundError,
    sol::{FitId, SolarSystem, info::ImplantInfo},
};

impl SolarSystem {
    pub fn get_fit_implants(&self, fit_id: &FitId) -> Result<Vec<ImplantInfo>, GetFitImplantsError> {
        let fit = self.uad.fits.get_fit(fit_id)?;
        let implant_infos = fit
            .implants
            .iter()
            .map(|item_key| self.get_implant_internal(*item_key).unwrap())
            .collect();
        Ok(implant_infos)
    }
}

#[derive(Debug)]
pub enum GetFitImplantsError {
    FitNotFound(FitFoundError),
}
impl std::error::Error for GetFitImplantsError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::FitNotFound(e) => Some(e),
        }
    }
}
impl std::fmt::Display for GetFitImplantsError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::FitNotFound(e) => e.fmt(f),
        }
    }
}
impl From<FitFoundError> for GetFitImplantsError {
    fn from(error: FitFoundError) -> Self {
        Self::FitNotFound(error)
    }
}
