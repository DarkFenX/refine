use crate::{
    dbg::{DebugError, DebugResult},
    ud::{UData, fleet::UFleet},
};

impl UFleet {
    pub(in crate::ud) fn consistency_check(&self, u_data: &UData) -> DebugResult {
        let fleet_uid = match u_data.fleets.iid_by_xid(&self.id) {
            Some(fleet_uid) => fleet_uid,
            None => return Err(DebugError {}),
        };
        // Every fit referenced by the fleet should exist, and refer back to the fleet
        for fit_uid in self.iter_fits() {
            let fit = match u_data.fits.try_get(fit_uid) {
                Some(fit) => fit,
                _ => return Err(DebugError {}),
            };
            if fit.fleet != Some(fleet_uid) {
                return Err(DebugError {});
            }
        }
        Ok(())
    }
}
