use crate::{
    defs::SolFitId,
    err::FitFoundError,
    sol::{item_info::SolDroneInfo, SolarSystem},
};

impl SolarSystem {
    pub fn get_fit_drone_infos(&self, fit_id: &SolFitId) -> Result<Vec<SolDroneInfo>, GetFitDroneInfosError> {
        let fit = self.fits.get_fit(fit_id)?;
        let drone_infos = fit
            .drones
            .iter()
            .map(|v| SolDroneInfo::from(self.items.get_item(v).unwrap().get_drone().unwrap()))
            .collect();
        Ok(drone_infos)
    }
}

#[derive(Debug)]
pub enum GetFitDroneInfosError {
    FitNotFound(FitFoundError),
}
impl From<FitFoundError> for GetFitDroneInfosError {
    fn from(error: FitFoundError) -> Self {
        Self::FitNotFound(error)
    }
}
impl std::error::Error for GetFitDroneInfosError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::FitNotFound(e) => Some(e),
        }
    }
}
impl std::fmt::Display for GetFitDroneInfosError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::FitNotFound(e) => e.fmt(f),
        }
    }
}
