use crate::{
    defs::SolFitId,
    err::FitFoundError,
    sol::{item_info::SolShipInfo, SolarSystem},
};

impl SolarSystem {
    pub fn get_fit_ship_info(&self, fit_id: &SolFitId) -> Result<Option<SolShipInfo>, GetFitShipInfoError> {
        let fit = self.fits.get_fit(&fit_id)?;
        Ok(fit
            .ship
            .map(|v| SolShipInfo::from(self.items.get_item(&v).unwrap().get_ship().unwrap())))
    }
}

#[derive(Debug)]
pub enum GetFitShipInfoError {
    FitNotFound(FitFoundError),
}
impl From<FitFoundError> for GetFitShipInfoError {
    fn from(error: FitFoundError) -> Self {
        Self::FitNotFound(error)
    }
}
impl std::error::Error for GetFitShipInfoError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::FitNotFound(e) => Some(e),
        }
    }
}
impl std::fmt::Display for GetFitShipInfoError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::FitNotFound(e) => e.fmt(f),
        }
    }
}
