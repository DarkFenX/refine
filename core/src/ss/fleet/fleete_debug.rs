use crate::ss::{fleet::SsFleet, SsView};

impl SsFleet {
    pub(in crate::ss) fn debug_consistency_check(&self, ss_view: &SsView) -> bool {
        // Every fit referenced by the fleet should exist, and refer back to the fleet
        for fit_id in self.fits.iter() {
            let fit = match ss_view.fits.get_fit(fit_id) {
                Ok(fit) => fit,
                _ => return false,
            };
            if fit.fleet != Some(self.id) {
                return false;
            }
        }
        true
    }
}
