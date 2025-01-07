use crate::{
    defs::SolFitId,
    sol::{svc::SolSvc, uad::SolUad},
};

impl SolSvc {
    pub(in crate::sol) fn default_incoming_dmg_profile_changed(&mut self, uad: &SolUad) {
        for fit in uad.fits.iter_fits() {
            match fit.rah_incoming_dmg {
                Some(rah_incoming_dmg) => {
                    if uad.default_incoming_dmg != rah_incoming_dmg {
                        self.notify_fit_rah_dmg_profile_changed(uad, &fit.id);
                    }
                }
                None => self.notify_fit_rah_dmg_profile_changed(uad, &fit.id),
            }
        }
    }
    pub(in crate::sol) fn fit_rah_dmg_profile_changed(&mut self, uad: &SolUad, fit_id: &SolFitId) {
        self.notify_fit_rah_dmg_profile_changed(uad, fit_id)
    }
}
