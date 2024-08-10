use crate::{
    defs::SolFitId,
    err::FitFoundError,
    sol::{item_info::SolImplantInfo, SolarSystem},
};

impl SolarSystem {
    pub fn get_fit_implant_infos(&self, fit_id: &SolFitId) -> Result<Vec<SolImplantInfo>, GetFitImplantInfosError> {
        let fit = self.fits.get_fit(fit_id)?;
        let implant_infos = fit
            .implants
            .iter()
            .map(|v| SolImplantInfo::from(self.items.get_item(v).unwrap().get_implant().unwrap()))
            .collect();
        Ok(implant_infos)
    }
}

#[derive(Debug)]
pub enum GetFitImplantInfosError {
    FitNotFound(FitFoundError),
}
impl From<FitFoundError> for GetFitImplantInfosError {
    fn from(error: FitFoundError) -> Self {
        Self::FitNotFound(error)
    }
}
impl std::error::Error for GetFitImplantInfosError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::FitNotFound(e) => Some(e),
        }
    }
}
impl std::fmt::Display for GetFitImplantInfosError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::FitNotFound(e) => e.fmt(f),
        }
    }
}
