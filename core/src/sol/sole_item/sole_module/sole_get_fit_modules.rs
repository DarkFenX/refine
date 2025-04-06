use crate::{
    err::basic::FitFoundError,
    sol::{FitId, ModRack, SolarSystem, info::ModuleInfo},
};

impl SolarSystem {
    pub fn get_fit_modules(
        &self,
        fit_id: &FitId,
        rack: ModRack,
    ) -> Result<Vec<Option<ModuleInfo>>, GetFitModulesError> {
        let fit = self.uad.fits.get_fit(fit_id)?;
        let module_ids = match rack {
            ModRack::High => &fit.mods_high,
            ModRack::Mid => &fit.mods_mid,
            ModRack::Low => &fit.mods_low,
        };
        let module_infos = module_ids
            .iter_all()
            .map(|o| o.map(|v| self.make_module_info(self.uad.items.get_by_id(&v).unwrap().get_module().unwrap())))
            .collect();
        Ok(module_infos)
    }
}

#[derive(Debug)]
pub enum GetFitModulesError {
    FitNotFound(FitFoundError),
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
impl From<FitFoundError> for GetFitModulesError {
    fn from(error: FitFoundError) -> Self {
        Self::FitNotFound(error)
    }
}
