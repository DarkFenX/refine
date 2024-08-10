use crate::{
    defs::SolFitId,
    err::FitFoundError,
    sol::{item_info::SolSubsystemInfo, SolarSystem},
};

impl SolarSystem {
    pub fn get_fit_subsystem_infos(
        &self,
        fit_id: &SolFitId,
    ) -> Result<Vec<SolSubsystemInfo>, GetFitSubsystemInfosError> {
        let fit = self.fits.get_fit(fit_id)?;
        let subsystem_infos = fit
            .subsystems
            .iter()
            .map(|v| SolSubsystemInfo::from(self.items.get_item(v).unwrap().get_subsystem().unwrap()))
            .collect();
        Ok(subsystem_infos)
    }
}

#[derive(Debug)]
pub enum GetFitSubsystemInfosError {
    FitNotFound(FitFoundError),
}
impl From<FitFoundError> for GetFitSubsystemInfosError {
    fn from(error: FitFoundError) -> Self {
        Self::FitNotFound(error)
    }
}
impl std::error::Error for GetFitSubsystemInfosError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::FitNotFound(e) => Some(e),
        }
    }
}
impl std::fmt::Display for GetFitSubsystemInfosError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::FitNotFound(e) => e.fmt(f),
        }
    }
}
