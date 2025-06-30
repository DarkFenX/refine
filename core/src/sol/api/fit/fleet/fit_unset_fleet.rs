use crate::{
    def::FitKey,
    err::basic::FitFleetAssignedError,
    sol::{SolarSystem, api::FitMut},
};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_unset_fit_fleet(
        &mut self,
        fit_key: FitKey,
    ) -> Result<(), FitFleetAssignedError> {
        let uad_fit = self.uad.fits.get(fit_key);
        let fleet_key = match uad_fit.fleet {
            Some(fleet_key) => fleet_key,
            None => return Err(FitFleetAssignedError { fit_id: uad_fit.id }),
        };
        let uad_fleet = self.uad.fleets.get(fleet_key);
        self.svc.notify_fit_removed_from_fleet(&self.uad, uad_fleet, &fit_key);
        let uad_fleet = self.uad.fleets.get_mut(fleet_key);
        uad_fleet.remove_fit(&fit_key);
        let uad_fit = self.uad.fits.get_mut(fit_key);
        uad_fit.fleet = None;
        Ok(())
    }
}

impl<'a> FitMut<'a> {
    pub fn unset_fleet(&mut self) -> Result<(), UnsetFitFleetError> {
        self.sol.internal_unset_fit_fleet(self.key)?;
        Ok(())
    }
}

#[derive(thiserror::Error, Debug)]
pub enum UnsetFitFleetError {
    #[error("{0}")]
    FitHasNoFleet(#[from] FitFleetAssignedError),
}
