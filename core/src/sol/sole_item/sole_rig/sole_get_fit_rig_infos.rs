use crate::{
    defs::SolFitId,
    err::FitFoundError,
    sol::{item_info::SolRigInfo, SolarSystem},
};

impl SolarSystem {
    pub fn get_fit_rig_infos(&self, fit_id: &SolFitId) -> Result<Vec<SolRigInfo>, GetFitRigInfosError> {
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
pub enum GetFitRigInfosError {
    FitNotFound(FitFoundError),
}
impl From<FitFoundError> for GetFitRigInfosError {
    fn from(error: FitFoundError) -> Self {
        Self::FitNotFound(error)
    }
}
impl std::error::Error for GetFitRigInfosError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::FitNotFound(e) => Some(e),
        }
    }
}
impl std::fmt::Display for GetFitRigInfosError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::FitNotFound(e) => e.fmt(f),
        }
    }
}
