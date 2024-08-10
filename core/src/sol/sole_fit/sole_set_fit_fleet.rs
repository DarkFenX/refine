use crate::{
    defs::{SolFitId, SolFleetId},
    sol::{
        err::basic::{FitFoundError, FleetFoundError},
        SolView, SolarSystem,
    },
};

impl SolarSystem {
    pub fn set_fit_fleet(&mut self, fit_id: &SolFitId, fleet_id: SolFleetId) -> Result<(), SetFitFleetError> {
        let fit = self.fits.get_fit(fit_id)?;
        self.fleets.get_fleet(&fleet_id)?;
        // Unassign from old fleet
        if let Some(old_fleet_id) = fit.fleet {
            if old_fleet_id == fleet_id {
                return Ok(());
            }
            let old_fleet = self.fleets.get_fleet(&old_fleet_id).unwrap();
            self.svcs.remove_fit_from_fleet(
                &SolView::new(&self.src, &self.fleets, &self.fits, &self.items),
                old_fleet,
                fit_id,
            );
            let old_fleet = self.fleets.get_fleet_mut(&fleet_id).unwrap();
            old_fleet.remove_fit(fit_id);
        }
        // Assign new fleet
        let fit = self.fits.get_fit_mut(fit_id).unwrap();
        fit.fleet = Some(fleet_id);
        let fleet = self.fleets.get_fleet_mut(&fleet_id).unwrap();
        fleet.add_fit(*fit_id);
        self.svcs.add_fit_to_fleet(
            &SolView::new(&self.src, &self.fleets, &self.fits, &self.items),
            self.fleets.get_fleet(&fleet_id).unwrap(),
            fit_id,
        );
        Ok(())
    }
}

#[derive(Debug)]
pub enum SetFitFleetError {
    FitNotFound(FitFoundError),
    FleetNotFound(FleetFoundError),
}
impl From<FitFoundError> for SetFitFleetError {
    fn from(error: FitFoundError) -> Self {
        Self::FitNotFound(error)
    }
}
impl From<FleetFoundError> for SetFitFleetError {
    fn from(error: FleetFoundError) -> Self {
        Self::FleetNotFound(error)
    }
}
impl std::error::Error for SetFitFleetError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::FitNotFound(e) => Some(e),
            Self::FleetNotFound(e) => Some(e),
        }
    }
}
impl std::fmt::Display for SetFitFleetError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::FitNotFound(e) => e.fmt(f),
            Self::FleetNotFound(e) => e.fmt(f),
        }
    }
}
