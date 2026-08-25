use crate::{FitId, FitMut, SolarSystem, ud::UFitId};

impl SolarSystem {
    pub(in crate::api) fn internal_unset_fit_fleet(&mut self, fit_uid: UFitId) -> Result<(), FitFleetUnsetError> {
        let u_fit = self.u_data.fits.get(fit_uid);
        let Some(fleet_uid) = u_fit.fleet else {
            return Err(FitFleetUnsetError::FitHasNoFleet(u_fit.id));
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

impl<'s> FitMut<'s> {
    pub fn unset_fleet(&mut self) -> Result<(), FitFleetUnsetError> {
        self.sol.internal_unset_fit_fleet(self.uid)?;
        Ok(())
    }
}

#[derive(Debug, thiserror::Error)]
pub enum FitFleetUnsetError {
    #[error("fit {0} does not belong to any fleet")]
    FitHasNoFleet(FitId),
}
