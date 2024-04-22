use crate::{
    sol::{fleet::SolFleet, SolView},
    util::{DebugError, DebugResult},
};

impl SolFleet {
    pub(in crate::sol) fn debug_consistency_check(&self, sol_view: &SolView) -> DebugResult {
        // Every fit referenced by the fleet should exist, and refer back to the fleet
        for fit_id in self.iter_fits() {
            let fit = match sol_view.fits.get_fit(fit_id) {
                Ok(fit) => fit,
                _ => return Err(DebugError::new()),
            };
            if fit.fleet != Some(self.id) {
                return Err(DebugError::new());
            }
        }
        Ok(())
    }
}
