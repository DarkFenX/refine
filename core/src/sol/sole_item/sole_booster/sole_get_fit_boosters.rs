use crate::{
    defs::SolFitId,
    sol::{err::basic::FitFoundError, item_info::SolBoosterInfo, SolarSystem},
};

impl SolarSystem {
    pub fn get_fit_boosters(&self, fit_id: &SolFitId) -> Result<Vec<SolBoosterInfo>, GetFitBoostersError> {
        let fit = self.fits.get_fit(fit_id)?;
        let booster_infos = fit
            .boosters
            .iter()
            .map(|v| self.make_booster_info(self.items.get_item(v).unwrap().get_booster().unwrap()))
            .collect();
        Ok(booster_infos)
    }
}

#[derive(Debug)]
pub enum GetFitBoostersError {
    FitNotFound(FitFoundError),
}
impl From<FitFoundError> for GetFitBoostersError {
    fn from(error: FitFoundError) -> Self {
        Self::FitNotFound(error)
    }
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
