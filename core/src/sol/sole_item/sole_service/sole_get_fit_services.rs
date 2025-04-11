use crate::{
    err::basic::FitFoundError,
    sol::{FitId, FitKey, SolarSystem, info::ServiceInfo},
};

impl SolarSystem {
    pub fn get_fit_service_infos(&self, fit_id: &FitId) -> Result<Vec<ServiceInfo>, GetFitServiceInfosError> {
        let fit_key = self.uad.fits.key_by_id_err(fit_id)?;
        Ok(self.get_fit_services_info_internal(fit_key))
    }
    pub(in crate::sol) fn get_fit_services_info_internal(&self, fit_key: FitKey) -> Vec<ServiceInfo> {
        self.uad
            .fits
            .get(fit_key)
            .services
            .iter()
            .map(|item_key| self.get_service_info_internal(*item_key).unwrap())
            .collect()
    }
}

#[derive(thiserror::Error, Debug)]
pub enum GetFitServiceInfosError {
    #[error("{0}")]
    FitNotFound(#[from] FitFoundError),
}
