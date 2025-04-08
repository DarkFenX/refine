use crate::{
    err::basic::FitFoundError,
    sol::{FitId, SolarSystem, info::RigInfo},
};

impl SolarSystem {
    pub fn get_fit_rigs(&self, fit_id: &FitId) -> Result<Vec<RigInfo>, GetFitRigsError> {
        let fit = self.uad.fits.get_fit(fit_id)?;
        let rig_infos = fit
            .rigs
            .iter()
            .map(|item_key| self.get_rig_internal(*item_key).unwrap())
            .collect();
        Ok(rig_infos)
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
