use crate::{
    defs::SolFitId,
    err::basic::FitFoundError,
    sol::{item_info::SolModuleInfo, SolarSystem},
    SolModRack,
};

impl SolarSystem {
    pub fn get_fit_modules(
        &self,
        fit_id: &SolFitId,
        rack: SolModRack,
    ) -> Result<Vec<SolModuleInfo>, GetFitModulesError> {
        let fit = self.fits.get_fit(fit_id)?;
        Ok(self.int_get_fit_module_infos(fit, rack))
    }
}

#[derive(Debug)]
pub enum GetFitModulesError {
    FitNotFound(FitFoundError),
}
impl From<FitFoundError> for GetFitModulesError {
    fn from(error: FitFoundError) -> Self {
        Self::FitNotFound(error)
    }
}
impl std::error::Error for GetFitModulesError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::FitNotFound(e) => Some(e),
        }
    }
}
impl std::fmt::Display for GetFitModulesError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::FitNotFound(e) => e.fmt(f),
        }
    }
}
