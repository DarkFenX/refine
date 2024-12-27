use itertools::Itertools;

use crate::{
    defs::SolFleetId,
    err::basic::FleetFoundError,
    sol::{SolView, SolarSystem},
};

impl SolarSystem {
    pub fn remove_fleet(&mut self, fleet_id: &SolFleetId) -> Result<(), RemoveFleetError> {
        let fleet = self.fleets.get_fleet(fleet_id)?;
        let fit_ids = fleet.iter_fits().map(|v| *v).collect_vec();
        for fit_id in fit_ids.iter() {
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
            let fit = self.fits.get_fit_mut(fit_id).unwrap();
            fit.fleet = None;
        }
        self.fleets.remove_fleet(fleet_id).unwrap();
        Ok(())
    }
}

#[derive(Debug)]
pub enum RemoveFleetError {
    FleetNotFound(FleetFoundError),
}
impl std::error::Error for RemoveFleetError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::FleetNotFound(e) => Some(e),
        }
    }
}
impl std::fmt::Display for RemoveFleetError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::FleetNotFound(e) => e.fmt(f),
        }
    }
}
impl From<FleetFoundError> for RemoveFleetError {
    fn from(error: FleetFoundError) -> Self {
        Self::FleetNotFound(error)
    }
}
