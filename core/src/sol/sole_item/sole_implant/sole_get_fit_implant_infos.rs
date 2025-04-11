use crate::{
    err::basic::FitFoundError,
    sol::{FitId, FitKey, SolarSystem, info::ImplantInfo},
};

impl SolarSystem {
    pub fn get_fit_implant_infos(&self, fit_id: &FitId) -> Result<Vec<ImplantInfo>, GetFitImplantInfosError> {
        let fit_key = self.uad.fits.key_by_id_err(fit_id)?;
        Ok(self.get_fit_implant_infos_internal(fit_key))
    }
    pub(in crate::sol) fn get_fit_implant_infos_internal(&self, fit_key: FitKey) -> Vec<ImplantInfo> {
        self.uad
            .fits
            .get(fit_key)
            .implants
            .iter()
            .map(|item_key| self.get_implant_info_internal(*item_key).unwrap())
            .collect()
    }
}

#[derive(thiserror::Error, Debug)]
pub enum GetFitImplantInfosError {
    #[error("{0}")]
    FitNotFound(#[from] FitFoundError),
}
