use itertools::Itertools;

use crate::{
    defs::{AttrVal, SolFitId, SolItemId},
    ec,
    sol::{svc::SolSvcs, SolDmgTypes, SolView},
    util::StMap,
};

use super::{
    info::SolRahInfo,
    shared::{RAH_EFFECT_ID, RES_ATTR_IDS},
    tick_iter::SolRahSimTickIter,
};

impl SolSvcs {
    pub(super) fn calc_rah_run_simulation(&mut self, sol_view: &SolView, fit_id: &SolFitId) {
        let ship_id = match sol_view.fits.get_fit(fit_id).unwrap().ship {
            Some(ship_id) => ship_id,
            None => {
                self.set_fit_rah_fallbacks(sol_view, fit_id);
                return;
            }
        };
        let fit_rahs = self.get_fit_rah_infos(sol_view, fit_id);
        // If the map is empty, no setting fallbacks needed, they were set in the map getter
        if fit_rahs.is_empty() {
            return;
        }
        let dmg_profile = match sol_view.fits.get_fit(fit_id).unwrap().rah_incoming_dmg {
            Some(dmg_profile) => dmg_profile,
            None => *sol_view.default_incoming_dmg,
        };
        if dmg_profile.em <= 0.0
            && dmg_profile.thermal <= 0.0
            && dmg_profile.kinetic <= 0.0
            && dmg_profile.explosive <= 0.0
        {
            for item_id in fit_rahs.keys() {
                self.set_rah_fallbacks(sol_view, item_id);
            }
            return;
        }
        // Container for damage each RAH received during its cycle. May span across several
        // simulation ticks for multi-RAH setups
        let mut fit_dmg_data = StMap::with_capacity(fit_rahs.len());
        for item_id in fit_rahs.keys() {
            fit_dmg_data.insert(*item_id, SolDmgTypes::new(0.0, 0.0, 0.0, 0.0));
        }
        for tick_data in SolRahSimTickIter::new(&fit_rahs) {
            // For each RAH, calculate damage received during this tick
            let ship_resos = match self.get_ship_resonances(sol_view, &ship_id) {
                Some(ship_resos) => ship_resos,
                None => {
                    for item_id in fit_rahs.keys() {
                        self.set_rah_fallbacks(sol_view, item_id);
                    }
                    return;
                }
            };
            for rah_cycle_dmg_data in fit_dmg_data.values_mut() {
                rah_cycle_dmg_data.em += dmg_profile.em * ship_resos.em * tick_data.time_passed;
                rah_cycle_dmg_data.thermal += dmg_profile.thermal * ship_resos.thermal * tick_data.time_passed;
                rah_cycle_dmg_data.kinetic += dmg_profile.kinetic * ship_resos.kinetic * tick_data.time_passed;
                rah_cycle_dmg_data.explosive += dmg_profile.explosive * ship_resos.explosive * tick_data.time_passed;
            }
            // If RAH just finished its cycle, make resist switch
            for item_id in tick_data.cycled {}
        }
        self.set_fit_rah_fallbacks(sol_view, fit_id);
    }
    fn get_ship_resonances(&mut self, sol_view: &SolView, ship_id: &SolItemId) -> Option<SolDmgTypes<AttrVal>> {
        let em = self
            .calc_get_item_attr_val(sol_view, ship_id, &ec::attrs::ARMOR_EM_DMG_RESONANCE)
            .ok()?
            .dogma;
        let therm = self
            .calc_get_item_attr_val(sol_view, ship_id, &ec::attrs::ARMOR_THERM_DMG_RESONANCE)
            .ok()?
            .dogma;
        let kin = self
            .calc_get_item_attr_val(sol_view, ship_id, &ec::attrs::ARMOR_KIN_DMG_RESONANCE)
            .ok()?
            .dogma;
        let expl = self
            .calc_get_item_attr_val(sol_view, ship_id, &ec::attrs::ARMOR_EXPL_DMG_RESONANCE)
            .ok()?
            .dogma;
        Some(SolDmgTypes::new(em, therm, kin, expl))
    }
    fn get_fit_rah_infos(&mut self, sol_view: &SolView, fit_id: &SolFitId) -> StMap<SolItemId, SolRahInfo> {
        let mut fit_rah_attrs = StMap::new();
        for item_id in self.calc_data.rah.by_fit.get(fit_id).map(|v| *v).collect_vec() {
            let rah_attrs = match self.get_rah_info(sol_view, &item_id) {
                Some(rah_attrs) => rah_attrs,
                // Whenever a RAH has unacceptable for sim attributes, set fallback values and don't
                // add it to the map
                None => {
                    self.set_rah_fallbacks(sol_view, &item_id);
                    continue;
                }
            };
            fit_rah_attrs.insert(item_id, rah_attrs);
        }
        fit_rah_attrs
    }
    fn get_rah_info(&mut self, sol_view: &SolView, item_id: &SolItemId) -> Option<SolRahInfo> {
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
        let cycle_ms = self.get_item_effect_id_duration(sol_view, &item_id, &RAH_EFFECT_ID)?;
        if cycle_ms <= 0.0 {
            return None;
        }
        let rah_info = SolRahInfo::new(res_em, res_therm, res_kin, res_expl, cycle_ms / 1000.0, shift_amount);
        Some(rah_info)
    }
    // Set resonances to unadapted values in sim storage for all RAHs of requested fit
    fn set_fit_rah_fallbacks(&mut self, sol_view: &SolView, fit_id: &SolFitId) {
        for item_id in self.calc_data.rah.by_fit.get(fit_id).map(|v| *v).collect_vec() {
            self.set_rah_fallbacks(sol_view, &item_id);
        }
    }
    fn set_rah_fallbacks(&mut self, sol_view: &SolView, item_id: &SolItemId) {
        for attr_id in RES_ATTR_IDS {
            let val = match self.calc_get_item_attr_val_no_pp(sol_view, item_id, &attr_id) {
                Ok(val) => val,
                Err(_) => continue,
            };
            self.calc_data
                .rah
                .resonances
                .get_mut(item_id)
                .unwrap()
                .insert(attr_id, val);
        }
    }
}
