use std::collections::BTreeMap;

use itertools::Itertools;

use crate::{
    defs::{AttrVal, SolFitId, SolItemId, OF},
    ec,
    sol::{
        svc::{svce_calc::SolAttrVal, SolSvcs},
        SolDmgTypes, SolView,
    },
    util::StSet,
};

use super::{
    rah_data_sim::SolRahDataSim,
    rah_history_entry::SolRahSimHistoryEntry,
    rah_info::SolRahInfo,
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
        // Keys in this map have to be sorted, since it defines RAH order in simulation history,
        // which hashes vectors with history entries
        let mut sim_datas = self.get_fit_rah_sim_datas(sol_view, fit_id);
        // If the map is empty, no setting fallbacks needed, they were set in the data getter
        if sim_datas.is_empty() {
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
            for item_id in sim_datas.keys() {
                self.set_rah_fallback(sol_view, item_id);
            }
            return;
        }
        let mut history_entries_seen = StSet::new();
        let mut sim_history = Vec::new();
        // Run "zero" simulation tick - write initial results and record initial state in history
        let mut sim_history_entry = Vec::with_capacity(sim_datas.len());
        for (item_id, item_sim_data) in sim_datas.iter() {
            self.set_rah_result(sol_view, item_id, item_sim_data.info.resos, false);
            let item_history_entry = SolRahSimHistoryEntry::new(*item_id, OF(0.0), &item_sim_data.info.resos);
            sim_history_entry.push(item_history_entry);
        }
        history_entries_seen.insert(sim_history_entry.clone());
        sim_history.push(sim_history_entry);
        // Run simulation
        for tick_data in SolRahSimTickIter::new(sim_datas.iter()) {
            // For each RAH, calculate damage received during this tick
            let ship_resos = match self.get_ship_resonances(sol_view, &ship_id) {
                Some(ship_resos) => ship_resos,
                None => {
                    for item_id in sim_datas.keys() {
                        self.set_rah_fallback(sol_view, item_id);
                    }
                    return;
                }
            };
            for item_sim_data in sim_datas.values_mut() {
                item_sim_data.taken_dmg.em += dmg_profile.em * ship_resos.em * tick_data.time_passed;
                item_sim_data.taken_dmg.thermal += dmg_profile.thermal * ship_resos.thermal * tick_data.time_passed;
                item_sim_data.taken_dmg.kinetic += dmg_profile.kinetic * ship_resos.kinetic * tick_data.time_passed;
                item_sim_data.taken_dmg.explosive +=
                    dmg_profile.explosive * ship_resos.explosive * tick_data.time_passed;
            }
            // If RAH just finished its cycle, make resist switch
            for cycled_item_id in tick_data.cycled {
                let item_sim_data = sim_datas.get_mut(&cycled_item_id).unwrap();
                let mut taken_dmg = SolDmgTypes::new(OF(0.0), OF(0.0), OF(0.0), OF(0.0));
                // Extract damage ship taken during RAH cycle, replacing it with 0's
                std::mem::swap(&mut taken_dmg, &mut item_sim_data.taken_dmg);
                let next_resos = get_next_resonances(
                    self.calc_data.rah.resonances.get(&cycled_item_id).unwrap().unwrap(),
                    taken_dmg,
                    item_sim_data.info.shift_amount,
                );
                // Write new resonances to results, letting everyone know about the changes. This is
                // needed to get updated ship resonances next tick.
                self.set_rah_result(sol_view, &cycled_item_id, next_resos, true);
            }
            // Compose history entry of current tick
            let mut sim_history_entry = Vec::with_capacity(sim_datas.len());
            for item_id in sim_datas.keys() {
                let item_cycling_time = *tick_data.cycling_times.get(item_id).unwrap();
                let item_resos = self.calc_data.rah.resonances.get(item_id).unwrap().unwrap();
                let item_history_entry = SolRahSimHistoryEntry::new(*item_id, item_cycling_time, &item_resos);
                sim_history_entry.push(item_history_entry);
            }
            // See if we're in a loop, if we are - calculate average resists across tick states
            // which are within the loop
            if history_entries_seen.contains(&sim_history_entry) {
                // TODO: add loop processing
                self.set_fit_rah_fallbacks(sol_view, fit_id);
                return;
            }
            // Update history
            history_entries_seen.insert(sim_history_entry.clone());
            sim_history.push(sim_history_entry);
        }
        // If we didn't find any RAH state loops during specified quantity of sim ticks, calculate
        // average resonances based on whole history, excluding initial adaptation period
        // TODO: add non-loop processing and remove fallback
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
    fn get_fit_rah_sim_datas(&mut self, sol_view: &SolView, fit_id: &SolFitId) -> BTreeMap<SolItemId, SolRahDataSim> {
        let mut rah_datas = BTreeMap::new();
        for item_id in self.calc_data.rah.by_fit.get(fit_id).map(|v| *v).collect_vec() {
            let rah_attrs = match self.get_rah_sim_data(sol_view, &item_id) {
                Some(rah_attrs) => rah_attrs,
                // Whenever a RAH has unacceptable for sim attributes, set fallback values and don't
                // add it to the map
                None => {
                    self.set_rah_fallback(sol_view, &item_id);
                    continue;
                }
            };
            rah_datas.insert(item_id, rah_attrs);
        }
        rah_datas
    }
    fn get_rah_sim_data(&mut self, sol_view: &SolView, item_id: &SolItemId) -> Option<SolRahDataSim> {
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
        let rah_info = SolRahInfo::new(
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
        Some(SolRahDataSim::new(rah_info))
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
