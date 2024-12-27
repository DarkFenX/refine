use itertools::Itertools;

use crate::{
    defs::{AttrVal, SolFitId},
    sol::{svc::SolSvcs, SolView},
};

use super::shared::RES_ATTR_IDS;

struct SolRahAttrs {
    em: AttrVal,
    therm: AttrVal,
    kin: AttrVal,
    expl: AttrVal,
    cycle_time: AttrVal,
    shift_amount: AttrVal,
}

impl SolSvcs {
    pub(super) fn calc_rah_run_simulation(&mut self, sol_view: &SolView, fit_id: &SolFitId) {
        self.calc_data.rah.sim_running = true;

        let dmg_profile = match sol_view.fits.get_fit(fit_id).unwrap().rah_incoming_dmg {
            Some(dmg_profile) => dmg_profile,
            None => *sol_view.default_incoming_dmg,
        };
        self.set_fallback_results(sol_view, fit_id);
        self.calc_data.rah.sim_running = false;
    }
    // Set resonances to unadapted values in sim storage for all RAHs of requested fit
    fn set_fallback_results(&mut self, sol_view: &SolView, fit_id: &SolFitId) {
        let item_ids = self.calc_data.rah.by_fit.get(fit_id).map(|v| *v).collect_vec();
        for item_id in item_ids {
            for attr_id in RES_ATTR_IDS {
                let val = match self.calc_get_item_attr_val_no_postprocessing(sol_view, &item_id, &attr_id) {
                    Ok(val) => val,
                    Err(_) => continue,
                };
                self.calc_data
                    .rah
                    .resonances
                    .get_mut(&item_id)
                    .unwrap()
                    .insert(attr_id, val);
            }
        }
    }
}
