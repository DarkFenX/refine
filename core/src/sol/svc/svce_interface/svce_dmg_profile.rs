use crate::{
    defs::SolFitId,
    sol::{svc::SolSvcs, SolView},
};

impl SolSvcs {
    pub(in crate::sol) fn default_incoming_dmg_profile_changed(&mut self, sol_view: &SolView) {
        for fit in sol_view.fits.iter_fits() {
            match fit.rah_incoming_dmg {
                Some(rah_incoming_dmg) => {
                    if sol_view.default_incoming_dmg != &rah_incoming_dmg {
                        self.notify_fit_rah_dmg_profile_changed(sol_view, &fit.id);
                    }
                }
                None => self.notify_fit_rah_dmg_profile_changed(sol_view, &fit.id),
            }
        }
    }
    pub(in crate::sol) fn fit_rah_dmg_profile_changed(&mut self, sol_view: &SolView, fit_id: &SolFitId) {
        self.notify_fit_rah_dmg_profile_changed(sol_view, fit_id)
    }
}
