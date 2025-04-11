use crate::{
    err::basic::FitFoundError,
    sol::{FitId, FitKey, ModRack, SolarSystem, info::ModuleInfo},
};

impl SolarSystem {
    pub fn get_fit_module_infos(
        &self,
        fit_id: &FitId,
        rack: ModRack,
    ) -> Result<Vec<Option<ModuleInfo>>, GetFitModuleInfosError> {
        let fit_key = self.uad.fits.key_by_id_err(fit_id)?;
        Ok(self.get_fit_module_infos_internal(fit_key, rack))
    }
    pub(in crate::sol) fn get_fit_module_infos_internal(
        &self,
        fit_key: FitKey,
        rack: ModRack,
    ) -> Vec<Option<ModuleInfo>> {
        let fit = self.uad.fits.get(fit_key);
        let module_ids = match rack {
            ModRack::High => &fit.mods_high,
            ModRack::Mid => &fit.mods_mid,
            ModRack::Low => &fit.mods_low,
        };
        module_ids
            .iter_all()
            .map(|item_key_opt| item_key_opt.map(|item_key| self.get_module_info_internal(item_key).unwrap()))
            .collect()
    }
}

#[derive(thiserror::Error, Debug)]
pub enum GetFitModuleInfosError {
    #[error("{0}")]
    FitNotFound(#[from] FitFoundError),
}
