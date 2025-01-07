use crate::sol::{
    uad::{fleet::SolFleet, SolUad},
    SolDebugError, SolDebugResult,
};

impl SolFleet {
    pub(in crate::sol) fn debug_consistency_check(&self, uad: &SolUad) -> SolDebugResult {
        // Every fit referenced by the fleet should exist, and refer back to the fleet
        for fit_id in self.iter_fits() {
            let fit = match uad.fits.get_fit(fit_id) {
                Ok(fit) => fit,
                _ => return Err(SolDebugError::new()),
            };
            if fit.fleet != Some(self.id) {
                return Err(SolDebugError::new());
            }
        }
        Ok(())
    }
}
