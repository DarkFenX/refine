use crate::{
    defs::SolFitId,
    err::FitFoundError,
    sol::{item_info::SolModuleInfo, SolarSystem},
    SolModRack,
};

impl SolarSystem {
    pub fn get_fit_module_infos(
        &self,
        fit_id: &SolFitId,
        rack: SolModRack,
    ) -> Result<Vec<SolModuleInfo>, GetFitModuleInfosError> {
        let fit = self.fits.get_fit(fit_id)?;
        Ok(self.int_get_fit_module_infos(fit, rack))
    }
}

#[derive(Debug)]
pub enum GetFitModuleInfosError {
    FitNotFound(FitFoundError),
}
impl From<FitFoundError> for GetFitModuleInfosError {
    fn from(error: FitFoundError) -> Self {
        Self::FitNotFound(error)
    }
}
impl std::error::Error for GetFitModuleInfosError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::FitNotFound(e) => Some(e),
        }
    }
}
impl std::fmt::Display for GetFitModuleInfosError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::FitNotFound(e) => e.fmt(f),
        }
    }
}
