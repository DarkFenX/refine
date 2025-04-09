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

#[derive(thiserror::Error, Debug)]
pub enum GetFitServicesError {
    #[error("{0}")]
    FitNotFound(#[from] FitFoundError),
}
