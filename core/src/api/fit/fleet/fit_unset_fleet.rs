use crate::{api::FitMut, err::basic::FitFleetAssignedError, sol::SolarSystem, ud::UFitId};

impl SolarSystem {
    pub(in crate::api) fn internal_unset_fit_fleet(&mut self, fit_uid: UFitId) -> Result<(), FitFleetAssignedError> {
        let u_fit = self.u_data.fits.get(fit_uid);
        let fleet_uid = match u_fit.fleet {
            Some(fleet_uid) => fleet_uid,
            None => return Err(FitFleetAssignedError { fit_id: u_fit.id }),
        };
        let u_fleet = self.u_data.fleets.get(fleet_uid);
        self.svc.notify_fit_removed_from_fleet(&self.u_data, u_fleet, fit_uid);
        let u_fleet = self.u_data.fleets.get_mut(fleet_uid);
        u_fleet.remove_fit(&fit_uid);
        let u_fit = self.u_data.fits.get_mut(fit_uid);
        u_fit.fleet = None;
        Ok(())
    }
}

impl<'a> FitMut<'a> {
    pub fn unset_fleet(&mut self) -> Result<(), UnsetFitFleetError> {
        self.sol.internal_unset_fit_fleet(self.uid)?;
        Ok(())
    }
}

#[derive(thiserror::Error, Debug)]
pub enum UnsetFitFleetError {
    #[error("{0}")]
    FitHasNoFleet(#[from] FitFleetAssignedError),
}
