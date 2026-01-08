use crate::{
    api::FitMut,
    err::basic::FleetFoundError,
    sol::SolarSystem,
    ud::{FleetId, UFitId, UFleetId},
};

impl SolarSystem {
    pub(in crate::api) fn internal_set_fit_fleet(&mut self, fit_uid: UFitId, fleet_uid: UFleetId) {
        let u_fit = self.u_data.fits.get(fit_uid);
        self.u_data.fleets.get(fleet_uid);
        // Unassign from old fleet
        if let Some(old_fleet_uid) = u_fit.fleet {
            if old_fleet_uid == fleet_uid {
                return;
            }
            let old_u_fleet = self.u_data.fleets.get(old_fleet_uid);
            self.svc
                .notify_fit_removed_from_fleet(&self.u_data, old_u_fleet, fit_uid);
            let old_u_fleet = self.u_data.fleets.get_mut(old_fleet_uid);
            old_u_fleet.remove_fit(&fit_uid);
        }
        // Assign new fleet
        let u_fit = self.u_data.fits.get_mut(fit_uid);
        u_fit.fleet = Some(fleet_uid);
        let u_fleet = self.u_data.fleets.get_mut(fleet_uid);
        u_fleet.add_fit(fit_uid);
        let u_fleet = self.u_data.fleets.get(fleet_uid);
        self.svc.notify_fit_added_to_fleet(&self.u_data, u_fleet, fit_uid);
    }
}

impl<'a> FitMut<'a> {
    pub fn set_fleet(&mut self, fleet_id: &FleetId) -> Result<(), SetFitFleetError> {
        let fleet_uid = self.sol.u_data.fleets.iid_by_xid_err(fleet_id)?;
        self.sol.internal_set_fit_fleet(self.uid, fleet_uid);
        Ok(())
    }
}

#[derive(thiserror::Error, Debug)]
pub enum SetFitFleetError {
    #[error("{0}")]
    FleetNotFound(#[from] FleetFoundError),
}
