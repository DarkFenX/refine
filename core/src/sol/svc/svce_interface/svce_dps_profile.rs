use crate::sol::{FitId, svc::Svc, uad::Uad};

impl Svc {
    pub(in crate::sol) fn default_incoming_dps_profile_changed(&mut self, uad: &Uad) {
        for fit in uad.fits.iter_fits() {
            match fit.rah_incoming_dps {
                Some(rah_incoming_dps) => {
                    if uad.default_incoming_dps != rah_incoming_dps {
                        self.notify_fit_rah_dps_profile_changed(uad, &fit.id);
                    }
                }
                None => self.notify_fit_rah_dps_profile_changed(uad, &fit.id),
            }
        }
    }
    pub(in crate::sol) fn fit_rah_dps_profile_changed(&mut self, uad: &Uad, fit_id: &FitId) {
        self.notify_fit_rah_dps_profile_changed(uad, fit_id)
    }
}
