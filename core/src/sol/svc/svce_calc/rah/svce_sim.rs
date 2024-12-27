use itertools::Itertools;

use crate::{
    defs::{AttrVal, SolFitId, SolItemId},
    ec,
    sol::{svc::SolSvcs, SolView},
    util::StMap,
};

use super::shared::{RAH_EFFECT_ID, RES_ATTR_IDS};

struct SolRahAttrs {
    em: AttrVal,
    therm: AttrVal,
    kin: AttrVal,
    expl: AttrVal,
    cycle_time: AttrVal,
    shift_amount: AttrVal,
}
impl SolRahAttrs {
    fn new(
        em: AttrVal,
        therm: AttrVal,
        kin: AttrVal,
        expl: AttrVal,
        cycle_time: AttrVal,
        shift_amount: AttrVal,
    ) -> Self {
        Self {
            em,
            therm,
            kin,
            expl,
            cycle_time,
            shift_amount,
        }
    }
}

impl SolSvcs {
    pub(super) fn calc_rah_run_simulation(&mut self, sol_view: &SolView, fit_id: &SolFitId) {
        let fit_rah_attrs = self.get_fit_rah_attrs(sol_view, fit_id);
        if fit_rah_attrs.values().all(|v| v.is_none()) {
            self.set_fallback_results(sol_view, fit_id);
            return;
        }
        let dmg_profile = match sol_view.fits.get_fit(fit_id).unwrap().rah_incoming_dmg {
            Some(dmg_profile) => dmg_profile,
            None => *sol_view.default_incoming_dmg,
        };
        self.set_fallback_results(sol_view, fit_id);
    }
    fn get_fit_rah_attrs(&mut self, sol_view: &SolView, fit_id: &SolFitId) -> StMap<SolItemId, Option<SolRahAttrs>> {
        let mut fit_rah_attrs = StMap::new();
        for item_id in self.calc_data.rah.by_fit.get(fit_id).map(|v| *v).collect_vec() {
            let rah_attrs = self.get_rah_attrs(sol_view, &item_id);
            fit_rah_attrs.insert(item_id, rah_attrs);
        }
        fit_rah_attrs
    }
    fn get_rah_attrs(&mut self, sol_view: &SolView, item_id: &SolItemId) -> Option<SolRahAttrs> {
        // Get resonances through postprocessing functions, since we already installed them for RAHs
        let res_em = self
            .calc_get_item_attr_val_no_pp(sol_view, item_id, &ec::attrs::ARMOR_EM_DMG_RESONANCE)
            .ok()?
            .dogma;
        let res_therm = self
            .calc_get_item_attr_val_no_pp(sol_view, item_id, &ec::attrs::ARMOR_THERM_DMG_RESONANCE)
            .ok()?
            .dogma;
        let res_kin = self
            .calc_get_item_attr_val_no_pp(sol_view, item_id, &ec::attrs::ARMOR_KIN_DMG_RESONANCE)
            .ok()?
            .dogma;
        let res_expl = self
            .calc_get_item_attr_val_no_pp(sol_view, item_id, &ec::attrs::ARMOR_EXPL_DMG_RESONANCE)
            .ok()?
            .dogma;
        if res_em == 1.0 && res_therm == 1.0 && res_kin == 1.0 && res_expl == 1.0 {
            return None;
        }
        // Other attributes using regular getters
        let shift_amount = self
            .calc_get_item_attr_val(sol_view, item_id, &ec::attrs::RESIST_SHIFT_AMOUNT)
            .ok()?
            .dogma;
        if shift_amount == 0.0 {
            return None;
        }
        let cycle_time = self.get_item_effect_id_duration(sol_view, &item_id, &RAH_EFFECT_ID)?;
        if cycle_time <= 0.0 {
            return None;
        }
        let rah_attrs = SolRahAttrs::new(res_em, res_therm, res_kin, res_expl, cycle_time, shift_amount);
        Some(rah_attrs)
    }
    // Set resonances to unadapted values in sim storage for all RAHs of requested fit
    fn set_fallback_results(&mut self, sol_view: &SolView, fit_id: &SolFitId) {
        for item_id in self.calc_data.rah.by_fit.get(fit_id).map(|v| *v).collect_vec() {
            for attr_id in RES_ATTR_IDS {
                let val = match self.calc_get_item_attr_val_no_pp(sol_view, &item_id, &attr_id) {
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
