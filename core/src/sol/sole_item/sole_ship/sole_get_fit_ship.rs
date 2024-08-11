use crate::{
    defs::SolFitId,
    err::basic::FitFoundError,
    sol::{item_info::SolShipInfo, SolarSystem},
};

impl SolarSystem {
    pub fn get_fit_ship(&self, fit_id: &SolFitId) -> Result<Option<SolShipInfo>, GetFitShipError> {
        let fit = self.fits.get_fit(&fit_id)?;
        Ok(fit
            .ship
            .map(|v| SolShipInfo::from(self.items.get_item(&v).unwrap().get_ship().unwrap())))
    }
}

#[derive(Debug)]
pub enum GetFitShipError {
    FitNotFound(FitFoundError),
}
impl From<FitFoundError> for GetFitShipError {
    fn from(error: FitFoundError) -> Self {
        Self::FitNotFound(error)
    }
}
impl std::error::Error for GetFitShipError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::FitNotFound(e) => Some(e),
        }
    }
}
impl std::fmt::Display for GetFitShipError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::FitNotFound(e) => e.fmt(f),
        }
    }
}
