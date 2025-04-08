use crate::{
    err::basic::FitFoundError,
    sol::{FitId, FitKey, SolarSystem, info::StanceInfo},
};

impl SolarSystem {
    pub fn get_fit_stance(&self, fit_id: &FitId) -> Result<Option<StanceInfo>, GetFitStanceError> {
        let fit_key = self.uad.fits.key_by_id_err(fit_id)?;
        Ok(self.get_fit_stance_internal(fit_key))
    }
    pub(in crate::sol) fn get_fit_stance_internal(&self, fit_key: FitKey) -> Option<StanceInfo> {
        self.uad
            .fits
            .get(fit_key)
            .stance
            .map(|item_key| self.get_stance_internal(item_key).unwrap())
    }
}

#[derive(Debug)]
pub enum GetFitStanceError {
    FitNotFound(FitFoundError),
}
impl std::error::Error for GetFitStanceError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::FitNotFound(e) => Some(e),
        }
    }
}
impl std::fmt::Display for GetFitStanceError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::FitNotFound(e) => e.fmt(f),
        }
    }
}
impl From<FitFoundError> for GetFitStanceError {
    fn from(error: FitFoundError) -> Self {
        Self::FitNotFound(error)
    }
}
