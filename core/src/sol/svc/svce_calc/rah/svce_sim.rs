use crate::{
    defs::SolFitId,
    sol::{svc::SolSvcs, SolView},
};

impl SolSvcs {
    pub(super) fn calc_rah_run_simulation(&mut self, sol_view: &SolView, fit_id: &SolFitId) {
        self.calc_data.rah.sim_running = true;
        let dmg_profile = match sol_view.fits.get_fit(fit_id).unwrap().rah_incoming_dmg {
            Some(dmg_profile) => dmg_profile,
            None => *sol_view.default_incoming_dmg,
        };
        self.calc_data.rah.sim_running = false;
    }
}
