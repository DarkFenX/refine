use crate::{
    err::basic::FitFoundError,
    sol::{FitId, FitKey, SolarSystem, info::FighterInfo},
};

impl SolarSystem {
    pub fn get_fit_fighter_infos(&self, fit_id: &FitId) -> Result<Vec<FighterInfo>, GetFitFighterInfosError> {
        let fit_key = self.uad.fits.key_by_id_err(fit_id)?;
        Ok(self.get_fit_fighter_infos_internal(fit_key))
    }
    pub(in crate::sol) fn get_fit_fighter_infos_internal(&self, fit_key: FitKey) -> Vec<FighterInfo> {
        self.uad
            .fits
            .get(fit_key)
            .fighters
            .iter()
            .map(|item_key| self.get_fighter_info_internal(*item_key).unwrap())
            .collect()
    }
}

#[derive(thiserror::Error, Debug)]
pub enum GetFitFighterInfosError {
    #[error("{0}")]
    FitNotFound(#[from] FitFoundError),
}
