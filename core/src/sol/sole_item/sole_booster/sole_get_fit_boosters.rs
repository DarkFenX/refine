use crate::{
    err::basic::FitFoundError,
    sol::{FitId, FitKey, SolarSystem, info::BoosterInfo},
};

impl SolarSystem {
    pub fn get_fit_boosters(&self, fit_id: &FitId) -> Result<Vec<BoosterInfo>, GetFitBoostersError> {
        let fit_key = self.uad.fits.key_by_id_err(fit_id)?;
        Ok(self.get_fit_boosters_internal(fit_key))
    }
    pub(in crate::sol) fn get_fit_boosters_internal(&self, fit_key: FitKey) -> Vec<BoosterInfo> {
        self.uad
            .fits
            .get(fit_key)
            .boosters
            .iter()
            .map(|item_key| self.get_booster_internal(*item_key).unwrap())
            .collect()
    }
}

#[derive(Debug)]
pub enum GetFitBoostersError {
    FitNotFound(FitFoundError),
}
impl std::error::Error for GetFitBoostersError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::FitNotFound(e) => Some(e),
        }
    }
}
impl std::fmt::Display for GetFitBoostersError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::FitNotFound(e) => e.fmt(f),
        }
    }
}
impl From<FitFoundError> for GetFitBoostersError {
    fn from(error: FitFoundError) -> Self {
        Self::FitNotFound(error)
    }
}
