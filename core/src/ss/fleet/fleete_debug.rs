use crate::{
    ss::{fleet::SsFleet, SsView},
    util::{DebugError, DebugResult},
};

impl SsFleet {
    pub(in crate::ss) fn debug_consistency_check(&self, ss_view: &SsView) -> DebugResult {
        // Every fit referenced by the fleet should exist, and refer back to the fleet
        for fit_id in self.iter_fits() {
            let fit = match ss_view.fits.get_fit(fit_id) {
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
