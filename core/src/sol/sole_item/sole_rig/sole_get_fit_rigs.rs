use crate::{
    defs::SolFitId,
    err::basic::FitFoundError,
    sol::{item_info::SolRigInfo, SolarSystem},
};

impl SolarSystem {
    pub fn get_fit_rigs(&self, fit_id: &SolFitId) -> Result<Vec<SolRigInfo>, GetFitRigsError> {
        let fit = self.fits.get_fit(fit_id)?;
        let rig_infos = fit
            .rigs
            .iter()
            .map(|v| SolRigInfo::from(self.items.get_item(v).unwrap().get_rig().unwrap()))
            .collect();
        Ok(rig_infos)
    }
}

#[derive(Debug)]
pub enum GetFitRigsError {
    FitNotFound(FitFoundError),
}
impl From<FitFoundError> for GetFitRigsError {
    fn from(error: FitFoundError) -> Self {
        Self::FitNotFound(error)
    }
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
