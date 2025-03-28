use crate::sol::{
    debug::{DebugError, DebugResult},
    uad::{Uad, fleet::Fleet},
};

impl Fleet {
    pub(in crate::sol) fn debug_consistency_check(&self, uad: &Uad) -> DebugResult {
        // Every fit referenced by the fleet should exist, and refer back to the fleet
        for fit_id in self.iter_fits() {
            let fit = match uad.fits.get_fit(fit_id) {
                Ok(fit) => fit,
                _ => return Err(DebugError {}),
            };
            if fit.fleet != Some(self.id) {
                return Err(DebugError {});
            }
        }
        Ok(())
    }
}
