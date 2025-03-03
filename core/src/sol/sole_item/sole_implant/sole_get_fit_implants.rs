use crate::{
    defs::SolFitId,
    err::basic::FitFoundError,
    sol::{SolarSystem, info::SolImplantInfo},
};

impl SolarSystem {
    pub fn get_fit_implants(&self, fit_id: &SolFitId) -> Result<Vec<SolImplantInfo>, GetFitImplantsError> {
        let fit = self.uad.fits.get_fit(fit_id)?;
        let implant_infos = fit
            .implants
            .iter()
            .map(|v| SolImplantInfo::from(self.uad.items.get_item(v).unwrap().get_implant().unwrap()))
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
