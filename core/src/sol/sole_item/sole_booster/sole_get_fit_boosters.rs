use crate::{
    err::basic::FitFoundError,
    sol::{FitId, SolarSystem, info::BoosterInfo},
};

impl SolarSystem {
    pub fn get_fit_boosters(&self, fit_id: &FitId) -> Result<Vec<BoosterInfo>, GetFitBoostersError> {
        let fit = self.uad.fits.get_fit(fit_id)?;
        let booster_infos = fit
            .boosters
            .iter()
            .map(|item_key| self.get_booster_internal(*item_key).unwrap())
            .collect();
        Ok(booster_infos)
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
