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
            .map(|item_key| self.get_subsystem_internal(*item_key).unwrap())
            .collect()
    }
}

#[derive(Debug)]
pub enum GetFitSubsystemsError {
    FitNotFound(FitFoundError),
}
impl std::error::Error for GetFitSubsystemsError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::FitNotFound(e) => Some(e),
        }
    }
}
impl std::fmt::Display for GetFitSubsystemsError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::FitNotFound(e) => e.fmt(f),
        }
    }
}
impl From<FitFoundError> for GetFitSubsystemsError {
    fn from(error: FitFoundError) -> Self {
        Self::FitNotFound(error)
    }
}
