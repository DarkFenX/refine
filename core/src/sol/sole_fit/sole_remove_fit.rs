use crate::{
    defs::SolFitId,
    sol::{err::basic::FitFoundError, SolarSystem},
};

impl SolarSystem {
    pub fn remove_fit(&mut self, fit_id: &SolFitId) -> Result<(), RemoveFitError> {
        let fit = self.fits.get_fit(fit_id)?;
        for item_id in fit.all_items().iter() {
            self.remove_item(item_id).unwrap();
        }
        self.svcs.remove_fit(&fit_id);
        let fit = self.fits.remove_fit(fit_id).unwrap();
        if let Some(fleet_id) = fit.fleet {
            let fleet = self.fleets.get_fleet_mut(&fleet_id).unwrap();
            fleet.remove_fit(fit_id);
        }
        Ok(())
    }
}

#[derive(Debug)]
pub enum RemoveFitError {
    FitNotFound(FitFoundError),
}
impl From<FitFoundError> for RemoveFitError {
    fn from(error: FitFoundError) -> Self {
        Self::FitNotFound(error)
    }
}
impl std::error::Error for RemoveFitError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::FitNotFound(e) => Some(e),
        }
    }
}
impl std::fmt::Display for RemoveFitError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::FitNotFound(e) => e.fmt(f),
        }
    }
}
