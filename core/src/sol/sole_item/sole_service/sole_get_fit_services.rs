use crate::{
    err::basic::FitFoundError,
    sol::{FitId, SolarSystem, info::ServiceInfo},
};

impl SolarSystem {
    pub fn get_fit_services(&self, fit_id: &FitId) -> Result<Vec<ServiceInfo>, GetFitServicesError> {
        let fit = self.uad.fits.get_fit(fit_id)?;
        let service_infos = fit
            .services
            .iter()
            .map(|v| ServiceInfo::from(self.uad.items.get_item(v).unwrap().get_service().unwrap()))
            .collect();
        Ok(service_infos)
    }
}

#[derive(Debug)]
pub enum GetFitServicesError {
    FitNotFound(FitFoundError),
}
impl std::error::Error for GetFitServicesError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::FitNotFound(e) => Some(e),
        }
    }
}
impl std::fmt::Display for GetFitServicesError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::FitNotFound(e) => e.fmt(f),
        }
    }
}
impl From<FitFoundError> for GetFitServicesError {
    fn from(error: FitFoundError) -> Self {
        Self::FitNotFound(error)
    }
}
