use crate::sol::{fleet::SolFleet, SolDebugError, SolDebugResult, SolView};

impl SolFleet {
    pub(in crate::sol) fn debug_consistency_check(&self, sol_view: &SolView) -> SolDebugResult {
        // Every fit referenced by the fleet should exist, and refer back to the fleet
        for fit_id in self.iter_fits() {
            let fit = match sol_view.fits.get_fit(fit_id) {
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
