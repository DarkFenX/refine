use crate::ss::{fleet::SsFleet, SsView};

impl SsFleet {
    pub(in crate::ss) fn debug_consistency_check(&self, ss_view: &SsView) -> bool {
        // Every fit referenced by the fleet should exist
        for fit_id in self.fits.iter() {
            if let Err(_) = ss_view.fits.get_fit(fit_id) {
                return false;
            }
        }
        true
    }
}
