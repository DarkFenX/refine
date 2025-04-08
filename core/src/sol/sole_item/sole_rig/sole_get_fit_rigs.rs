use crate::{
    err::basic::FitFoundError,
    sol::{FitId, FitKey, SolarSystem, info::RigInfo},
};

impl SolarSystem {
    pub fn get_fit_rigs(&self, fit_id: &FitId) -> Result<Vec<RigInfo>, GetFitRigsError> {
        let fit_key = self.uad.fits.key_by_id_err(fit_id)?;
        Ok(self.get_fit_rigs_internal(fit_key))
    }
    pub(in crate::sol) fn get_fit_rigs_internal(&self, fit_key: FitKey) -> Vec<RigInfo> {
        self.uad
            .fits
            .get(fit_key)
            .rigs
            .iter()
            .map(|item_key| self.get_rig_internal(*item_key).unwrap())
            .collect()
    }
}

#[derive(Debug)]
pub enum GetFitRigsError {
    FitNotFound(FitFoundError),
}
impl std::error::Error for GetFitRigsError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::FitNotFound(e) => Some(e),
        }
    }
}
impl std::fmt::Display for GetFitRigsError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::FitNotFound(e) => e.fmt(f),
        }
    }
}
impl From<FitFoundError> for GetFitRigsError {
    fn from(error: FitFoundError) -> Self {
        Self::FitNotFound(error)
    }
}
