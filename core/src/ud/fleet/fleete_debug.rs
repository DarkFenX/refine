use crate::{
    dbg::{DebugError, DebugResult},
    ud::{UData, fleet::UFleet},
};

impl UFleet {
    pub(in crate::ud) fn consistency_check(&self, u_data: &UData) -> DebugResult {
        let fleet_key = match u_data.fleets.int_id_by_ext_id(&self.id) {
            Some(fleet_key) => fleet_key,
            None => return Err(DebugError {}),
        };
        // Every fit referenced by the fleet should exist, and refer back to the fleet
        for fit_key in self.iter_fits() {
            let fit = match u_data.fits.try_get(fit_key) {
                Some(fit) => fit,
                _ => return Err(DebugError {}),
            };
            if fit.fleet != Some(fleet_key) {
                return Err(DebugError {});
            }
        }
        Ok(())
    }
}
