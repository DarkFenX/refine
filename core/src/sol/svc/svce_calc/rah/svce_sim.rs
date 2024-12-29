use itertools::Itertools;

use crate::{
    defs::{AttrVal, SolFitId, SolItemId, OF},
    ec,
    sol::{
        svc::{svce_calc::SolAttrVal, SolSvcs},
        SolDmgTypes, SolView,
    },
    util::StMap,
};

use super::{
    info::SolRahSimRahData,
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
        let mut rah_datas = self.get_fit_rah_datas(sol_view, fit_id);
        // If the map is empty, no setting fallbacks needed, they were set in the info getter
        if rah_datas.is_empty() {
            return;
        }
        let dmg_profile = match sol_view.fits.get_fit(fit_id).unwrap().rah_incoming_dmg {
            Some(dmg_profile) => dmg_profile,
            None => *sol_view.default_incoming_dmg,
        };
        if dmg_profile.em <= OF(0.0)
            && dmg_profile.thermal <= OF(0.0)
            && dmg_profile.kinetic <= OF(0.0)
            && dmg_profile.explosive <= OF(0.0)
        {
            for item_id in rah_datas.keys() {
                self.set_rah_fallback(sol_view, item_id);
            }
            return;
        }
        // Run "zero" simulation tick - write initial results and TODO: record initial state in history
        for (item_id, rah_data) in rah_datas.iter() {
            self.set_rah_result(sol_view, item_id, rah_data.resos, false)
        }
        for tick_data in SolRahSimTickIter::new(&rah_datas) {
            // For each RAH, calculate damage received during this tick
            let ship_resos = match self.get_ship_resonances(sol_view, &ship_id) {
                Some(ship_resos) => ship_resos,
                None => {
                    for item_id in rah_datas.keys() {
                        self.set_rah_fallback(sol_view, item_id);
                    }
                    return;
                }
            };
            for rah_cycle_dmg_data in rah_datas.values_mut() {
                rah_cycle_dmg_data.taken_dmg.em += dmg_profile.em * ship_resos.em * tick_data.time_passed;
                rah_cycle_dmg_data.taken_dmg.thermal +=
                    dmg_profile.thermal * ship_resos.thermal * tick_data.time_passed;
                rah_cycle_dmg_data.taken_dmg.kinetic +=
                    dmg_profile.kinetic * ship_resos.kinetic * tick_data.time_passed;
                rah_cycle_dmg_data.taken_dmg.explosive +=
                    dmg_profile.explosive * ship_resos.explosive * tick_data.time_passed;
            }
            // If RAH just finished its cycle, make resist switch
            for cycled_item_id in tick_data.cycled {
                let rah_info = rah_datas.get_mut(&cycled_item_id).unwrap();
                let mut taken_dmg = SolDmgTypes::new(OF(0.0), OF(0.0), OF(0.0), OF(0.0));
                // Extract damage ship taken during RAH cycle, replacing it with 0's
                std::mem::swap(&mut taken_dmg, &mut rah_info.taken_dmg);
                let next_resos = get_next_resonances(
                    self.calc_data.rah.resonances.get(&cycled_item_id).unwrap().unwrap(),
                    taken_dmg,
                    rah_info.shift_amount,
                );
                // Write new resonances to results, letting everyone know about the changes. This is
                // needed to get updated ship resonances next tick.
                self.set_rah_result(sol_view, &cycled_item_id, next_resos, true);
            }
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
    fn get_fit_rah_datas(&mut self, sol_view: &SolView, fit_id: &SolFitId) -> StMap<SolItemId, SolRahSimRahData> {
        let mut fit_rah_attrs = StMap::new();
        for item_id in self.calc_data.rah.by_fit.get(fit_id).map(|v| *v).collect_vec() {
            let rah_attrs = match self.get_rah_data(sol_view, &item_id) {
                Some(rah_attrs) => rah_attrs,
                // Whenever a RAH has unacceptable for sim attributes, set fallback values and don't
                // add it to the map
                None => {
                    self.set_rah_fallback(sol_view, &item_id);
                    continue;
                }
            };
            fit_rah_attrs.insert(item_id, rah_attrs);
        }
        fit_rah_attrs
    }
    fn get_rah_data(&mut self, sol_view: &SolView, item_id: &SolItemId) -> Option<SolRahSimRahData> {
        // Get resonances through postprocessing functions, since we already installed them for RAHs
        let res_em = self
            .calc_get_item_attr_val_no_pp(sol_view, item_id, &ec::attrs::ARMOR_EM_DMG_RESONANCE)
            .ok()?;
        let res_therm = self
            .calc_get_item_attr_val_no_pp(sol_view, item_id, &ec::attrs::ARMOR_THERM_DMG_RESONANCE)
            .ok()?;
        let res_kin = self
            .calc_get_item_attr_val_no_pp(sol_view, item_id, &ec::attrs::ARMOR_KIN_DMG_RESONANCE)
            .ok()?;
        let res_expl = self
            .calc_get_item_attr_val_no_pp(sol_view, item_id, &ec::attrs::ARMOR_EXPL_DMG_RESONANCE)
            .ok()?;
        if res_em.dogma == OF(1.0)
            && res_therm.dogma == OF(1.0)
            && res_kin.dogma == OF(1.0)
            && res_expl.dogma == OF(1.0)
        {
            return None;
        }
        // Other attributes using regular getters
        let shift_amount = self
            .calc_get_item_attr_val(sol_view, item_id, &ec::attrs::RESIST_SHIFT_AMOUNT)
            .ok()?
            .dogma;
        if shift_amount == OF(0.0) {
            return None;
        }
        let cycle_ms = self.get_item_effect_id_duration(sol_view, &item_id, &RAH_EFFECT_ID)?;
        if cycle_ms <= OF(0.0) {
            return None;
        }
        let rah_info = SolRahSimRahData::new(
            res_em,
            res_therm,
            res_kin,
            res_expl,
            // Raw form of cycle time is defined in milliseconds (we don't really care in RAH sim,
            // just to be more intuitive during debugging)
            cycle_ms / OF(1000.0),
            // Raw form of shift amount is defined in percentages, while resonances are in
            // absolute form
            shift_amount / OF(100.0),
        );
        Some(rah_info)
    }
    // Set resonances to unadapted values in sim storage for all RAHs of requested fit
    fn set_fit_rah_fallbacks(&mut self, sol_view: &SolView, fit_id: &SolFitId) {
        for item_id in self.calc_data.rah.by_fit.get(fit_id).map(|v| *v).collect_vec() {
            self.set_rah_fallback(sol_view, &item_id);
        }
    }
    fn set_rah_fallback(&mut self, sol_view: &SolView, item_id: &SolItemId) {
        let em = self
            .calc_get_item_attr_val_no_pp(sol_view, item_id, &ec::attrs::ARMOR_EM_DMG_RESONANCE)
            .unwrap_or(SolAttrVal::new(OF(1.0), OF(1.0), OF(1.0)));
        let therm = self
            .calc_get_item_attr_val_no_pp(sol_view, item_id, &ec::attrs::ARMOR_THERM_DMG_RESONANCE)
            .unwrap_or(SolAttrVal::new(OF(1.0), OF(1.0), OF(1.0)));
        let kin = self
            .calc_get_item_attr_val_no_pp(sol_view, item_id, &ec::attrs::ARMOR_KIN_DMG_RESONANCE)
            .unwrap_or(SolAttrVal::new(OF(1.0), OF(1.0), OF(1.0)));
        let expl = self
            .calc_get_item_attr_val_no_pp(sol_view, item_id, &ec::attrs::ARMOR_EXPL_DMG_RESONANCE)
            .unwrap_or(SolAttrVal::new(OF(1.0), OF(1.0), OF(1.0)));
        let rah_resos = SolDmgTypes::new(em, therm, kin, expl);
        // Fallback is just unsimulated RAH attribs, and fallback is supposed to be called before
        // any simulated results were written, so no notification about changed attribs needed
        self.set_rah_result(sol_view, item_id, rah_resos, false);
    }
    fn set_rah_result(
        &mut self,
        sol_view: &SolView,
        item_id: &SolItemId,
        resos: SolDmgTypes<SolAttrVal>,
        notify: bool,
    ) {
        self.calc_data.rah.resonances.get_mut(item_id).unwrap().replace(resos);
        if notify {
            for attr_id in RES_ATTR_IDS.iter() {
                self.notify_attr_val_changed(sol_view, item_id, attr_id)
            }
        }
    }
}

fn get_next_resonances(
    mut resonances: SolDmgTypes<SolAttrVal>,
    taken_dmg: SolDmgTypes<AttrVal>,
    shift_amount: AttrVal,
) -> SolDmgTypes<SolAttrVal> {
    // We borrow resistances from at least 2 resist types, possibly more if ship didn't take any
    // damage of those types
    let donors = taken_dmg.iter().filter(|v| **v == OF(0.0)).count().max(2);
    let recipients = 4 - donors as u8;
    // Indices are against damage type container, i.e. order is EM, explosive, kinetic, explosive.
    // When equal damage is received across several damage types, those which come earlier in this
    // list will be picked as donors. In EVE, it's this way probably due to backing attribute IDs,
    // since the list is in attribute ID ascending order.
    let mut sorted_indices: [usize; 4] = [0, 3, 2, 1];
    sorted_indices.sort_by(|a, b| taken_dmg[*a].partial_cmp(&taken_dmg[*b]).unwrap());
    let mut donated_amount = OF(0.0);
    // Donate
    for index in sorted_indices[..donors].iter() {
        let current_value = resonances[*index];
        // Can't borrow more than it has
        let to_donate = shift_amount.min(OF(1.0) - current_value.dogma);
        donated_amount += to_donate;
        let new_value = current_value.dogma + to_donate;
        resonances[*index] = SolAttrVal::new(current_value.base, new_value, new_value);
    }
    // Distribute
    for index in sorted_indices[donors..].iter() {
        let current_value = resonances[*index];
        let new_value = current_value.dogma - donated_amount / AttrVal::from(recipients);
        resonances[*index] = SolAttrVal::new(current_value.base, new_value, new_value);
    }
    resonances
}
