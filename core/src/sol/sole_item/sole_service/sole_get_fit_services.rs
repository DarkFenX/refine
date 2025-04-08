use crate::{
    err::basic::FitFoundError,
    sol::{FitId, FitKey, SolarSystem, info::ServiceInfo},
};

impl SolarSystem {
    pub fn get_fit_services(&self, fit_id: &FitId) -> Result<Vec<ServiceInfo>, GetFitServicesError> {
        let fit_key = self.uad.fits.key_by_id_err(fit_id)?;
        Ok(self.get_fit_services_internal(fit_key))
    }
    pub(in crate::sol) fn get_fit_services_internal(&self, fit_key: FitKey) -> Vec<ServiceInfo> {
        self.uad
            .fits
            .get(fit_key)
            .services
            .iter()
            .map(|item_key| self.get_service_internal(*item_key).unwrap())
            .collect()
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
