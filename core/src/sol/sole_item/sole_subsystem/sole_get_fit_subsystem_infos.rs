use crate::{
    err::basic::FitFoundError,
    sol::{FitId, FitKey, SolarSystem, info::SubsystemInfo},
};

impl SolarSystem {
    pub fn get_fit_subsystems(&self, fit_id: &FitId) -> Result<Vec<SubsystemInfo>, GetFitSubsystemsError> {
        let fit_key = self.uad.fits.key_by_id_err(fit_id)?;
        Ok(self.get_fit_subsystems_internal(fit_key))
    }
    pub(in crate::sol) fn get_fit_subsystems_internal(&self, fit_key: FitKey) -> Vec<SubsystemInfo> {
        self.uad
            .fits
            .get(fit_key)
            .subsystems
            .iter()
            .map(|item_key| self.get_subsystem_info_internal(*item_key).unwrap())
            .collect()
    }
}

#[derive(thiserror::Error, Debug)]
pub enum GetFitSubsystemsError {
    #[error("{0}")]
    FitNotFound(#[from] FitFoundError),
}
