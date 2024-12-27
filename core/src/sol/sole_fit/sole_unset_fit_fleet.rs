use crate::{
    defs::SolFitId,
    err::basic::{FitFleetAssignedError, FitFoundError},
    sol::{SolView, SolarSystem},
};

impl SolarSystem {
    pub fn unset_fit_fleet(&mut self, fit_id: &SolFitId) -> Result<(), UnsetFitFleetError> {
        let fit = self.fits.get_fit(fit_id)?;
        let fleet_id = match fit.fleet {
            Some(fleet_id) => fleet_id,
            None => return Err(FitFleetAssignedError::new(*fit_id).into()),
        };
        let fleet = self.fleets.get_fleet(&fleet_id).unwrap();
        self.svcs.remove_fit_from_fleet(
            &SolView::new(
                &self.src,
                &self.fleets,
                &self.fits,
                &self.items,
                &self.default_incoming_dmg,
            ),
            fleet,
            fit_id,
        );
        let fleet = self.fleets.get_fleet_mut(&fleet_id).unwrap();
        fleet.remove_fit(fit_id);
        let fit = self.fits.get_fit_mut(fit_id).unwrap();
        fit.fleet = None;
        Ok(())
    }
}

#[derive(Debug)]
pub enum UnsetFitFleetError {
    FitNotFound(FitFoundError),
    FitHasNoFleet(FitFleetAssignedError),
}
impl std::error::Error for UnsetFitFleetError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::FitNotFound(e) => Some(e),
            Self::FitHasNoFleet(e) => Some(e),
        }
    }
}
impl std::fmt::Display for UnsetFitFleetError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::FitNotFound(e) => e.fmt(f),
            Self::FitHasNoFleet(e) => e.fmt(f),
        }
    }
}
impl From<FitFoundError> for UnsetFitFleetError {
    fn from(error: FitFoundError) -> Self {
        Self::FitNotFound(error)
    }
}
impl From<FitFleetAssignedError> for UnsetFitFleetError {
    fn from(error: FitFleetAssignedError) -> Self {
        Self::FitHasNoFleet(error)
    }
}
