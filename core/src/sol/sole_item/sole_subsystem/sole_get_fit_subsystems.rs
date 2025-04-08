use crate::{
    err::basic::FitFoundError,
    sol::{FitId, SolarSystem, info::SubsystemInfo},
};

impl SolarSystem {
    pub fn get_fit_subsystems(&self, fit_id: &FitId) -> Result<Vec<SubsystemInfo>, GetFitSubsystemsError> {
        let fit = self.uad.fits.get_fit(fit_id)?;
        let subsystem_infos = fit
            .subsystems
            .iter()
            .map(|item_key| self.get_subsystem_internal(*item_key).unwrap())
            .collect();
        Ok(subsystem_infos)
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
