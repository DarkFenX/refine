use crate::sol::{FitKey, svc::Svc, uad::Uad};

impl Svc {
    pub(in crate::sol) fn default_incoming_dps_profile_changed(&mut self, uad: &Uad) {
        for (fit_key, fit) in uad.fits.iter() {
            match fit.rah_incoming_dps {
                Some(rah_incoming_dps) => {
                    if uad.default_incoming_dps != rah_incoming_dps {
                        self.notify_fit_rah_dps_profile_changed(uad, &fit_key);
                    }
                }
                None => self.notify_fit_rah_dps_profile_changed(uad, &fit_key),
            }
        }
    }
    pub(in crate::sol) fn fit_rah_dps_profile_changed(&mut self, uad: &Uad, fit_key: &FitKey) {
        self.notify_fit_rah_dps_profile_changed(uad, fit_key)
    }
}
