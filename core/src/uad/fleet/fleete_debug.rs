use crate::{
    dbg::{DebugError, DebugResult},
    uad::{Uad, fleet::UadFleet},
};

impl UadFleet {
    pub(in crate::uad) fn consistency_check(&self, uad: &Uad) -> DebugResult {
        let fleet_key = match uad.fleets.key_by_id(&self.id) {
            Some(fleet_key) => fleet_key,
            None => return Err(DebugError {}),
        };
        // Every fit referenced by the fleet should exist, and refer back to the fleet
        for &fit_key in self.iter_fits() {
            let fit = match uad.fits.try_get(fit_key) {
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
