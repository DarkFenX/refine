use crate::{
    api::FitMut,
    err::basic::FleetFoundError,
    sol::{SolarSystem, UndoStep},
    ud::{FleetId, UFitId, UFleetId},
};

impl SolarSystem {
    // Compiler keeps it out of line with 6 callers; inlined, fleet changes are cheaper
    #[inline(always)]
    pub(in crate::api) fn internal_set_fit_fleet(&mut self, fit_uid: UFitId, fleet_uid: Option<UFleetId>) {
        let u_fit = self.u_data.fits.get(fit_uid);
        let old_fleet_uid = u_fit.fleet;
        if old_fleet_uid == fleet_uid {
            return;
        }
        if self.undo.is_recording() {
            self.undo.record(UndoStep::FitFleet(fit_uid, old_fleet_uid));
        }
        // Unassign from old fleet
        if let Some(old_fleet_uid) = old_fleet_uid {
            let old_u_fleet = self.u_data.fleets.get(old_fleet_uid);
            self.svc
                .notify_fit_removed_from_fleet(&self.u_data, old_u_fleet, fit_uid);
            let old_u_fleet = self.u_data.fleets.get_mut(old_fleet_uid);
            old_u_fleet.remove_fit(&fit_uid);
        }
        // Assign new fleet
        let u_fit = self.u_data.fits.get_mut(fit_uid);
        u_fit.fleet = fleet_uid;
        if let Some(fleet_uid) = fleet_uid {
            let u_fleet = self.u_data.fleets.get_mut(fleet_uid);
            u_fleet.add_fit(fit_uid);
            let u_fleet = self.u_data.fleets.get(fleet_uid);
            self.svc.notify_fit_added_to_fleet(&self.u_data, u_fleet, fit_uid);
        }
    }
}

impl<'s> FitMut<'s> {
    /// Set fleet, moving fit out of its current one.
    pub fn set_fleet(&mut self, fleet_id: Option<&FleetId>) -> Result<(), FitFleetSetError> {
        let fleet_uid = match fleet_id {
            Some(fleet_id) => Some(self.sol.u_data.fleets.int_id_by_ext_id_err(fleet_id)?),
            None => None,
        };
        self.sol.internal_set_fit_fleet(self.uid, fleet_uid);
        Ok(())
    }
}

#[derive(Debug, thiserror::Error)]
pub enum FitFleetSetError {
    #[error(transparent)]
    FleetNotFound(#[from] FleetFoundError),
}
