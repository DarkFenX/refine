use itertools::Itertools;

use crate::{
    ad,
    defs::{EAttrId, SolFitId, SolItemId},
    ec,
    sol::{
        item::SolItem,
        svc::{svce_calc::SolAttrVal, SolSvcs},
        SolDmgTypes, SolView,
    },
};

use super::shared::{RAH_EFFECT_ID, RES_ATTR_IDS};

impl SolSvcs {
    pub(in crate::sol::svc::svce_calc) fn calc_rah_item_loaded(&mut self, sol_view: &SolView, item: &SolItem) {
        if self.calc_data.rah.sim_running {
            return;
        }
        if let SolItem::Ship(ship) = item {
            self.clear_fit_rah_results(sol_view, &ship.get_fit_id());
        }
    }
    pub(in crate::sol::svc::svce_calc) fn calc_rah_item_unloaded(&mut self, sol_view: &SolView, item: &SolItem) {
        if self.calc_data.rah.sim_running {
            return;
        }
        if let SolItem::Ship(ship) = item {
            self.clear_fit_rah_results(sol_view, &ship.get_fit_id());
        }
    }
    pub(in crate::sol::svc::svce_calc) fn calc_rah_effects_started(
        &mut self,
        sol_view: &SolView,
        item: &SolItem,
        effects: &Vec<ad::ArcEffect>,
    ) {
        if self.calc_data.rah.sim_running {
            return;
        }
        if let SolItem::Module(module) = item {
            if effects.iter().any(|v| v.id == RAH_EFFECT_ID) {
                let item_id = module.get_id();
                let fit_id = module.get_fit_id();
                // Clear sim data for other RAHs on the same fit
                self.clear_fit_rah_results(sol_view, &fit_id);
                // Add sim data for RAH being started
                self.calc_data.rah.resonances.insert(item_id, None);
                self.calc_data.rah.by_fit.add_entry(fit_id, item_id);
                // Add postprocessors
                let item_attr_data = self.calc_data.attrs.get_item_attr_data_mut(&item_id).unwrap();
                item_attr_data
                    .postprocessors
                    .insert(ec::attrs::ARMOR_EM_DMG_RESONANCE, rah_em_resonance_postprocessor);
                item_attr_data
                    .postprocessors
                    .insert(ec::attrs::ARMOR_THERM_DMG_RESONANCE, rah_therm_resonance_postprocessor);
                item_attr_data
                    .postprocessors
                    .insert(ec::attrs::ARMOR_KIN_DMG_RESONANCE, rah_kin_resonance_postprocessor);
                item_attr_data
                    .postprocessors
                    .insert(ec::attrs::ARMOR_EXPL_DMG_RESONANCE, rah_expl_resonance_postprocessor);
            }
        }
    }
    pub(in crate::sol::svc::svce_calc) fn calc_rah_effects_stopped(
        &mut self,
        sol_view: &SolView,
        item: &SolItem,
        effects: &Vec<ad::ArcEffect>,
    ) {
        if self.calc_data.rah.sim_running {
            return;
        }
        if let SolItem::Module(module) = item {
            if effects.iter().any(|v| v.id == RAH_EFFECT_ID) {
                let item_id = module.get_id();
                let fit_id = module.get_fit_id();
                // Remove postprocessors
                let item_attr_data = self.calc_data.attrs.get_item_attr_data_mut(&item_id).unwrap();
                item_attr_data.postprocessors.remove(&ec::attrs::ARMOR_EM_DMG_RESONANCE);
                item_attr_data
                    .postprocessors
                    .remove(&ec::attrs::ARMOR_THERM_DMG_RESONANCE);
                item_attr_data
                    .postprocessors
                    .remove(&ec::attrs::ARMOR_KIN_DMG_RESONANCE);
                item_attr_data
                    .postprocessors
                    .remove(&ec::attrs::ARMOR_EXPL_DMG_RESONANCE);
                // Remove sim data for RAH being stopped
                self.calc_data.rah.resonances.remove(&item_id);
                self.calc_data.rah.by_fit.remove_entry(&module.get_fit_id(), &item_id);
                // Clear sim data for other RAHs on the same fit
                self.clear_fit_rah_results(sol_view, &fit_id);
            }
        }
    }
    fn calc_rah_attr_value_changed(&mut self, sol_view: &SolView, item_id: &SolItemId, attr_id: &EAttrId) {
        if self.calc_data.rah.sim_running {
            return;
        }
        match *attr_id {
            // Ship armor resonances and RAH resonances
            ec::attrs::ARMOR_EM_DMG_RESONANCE
            | ec::attrs::ARMOR_THERM_DMG_RESONANCE
            | ec::attrs::ARMOR_KIN_DMG_RESONANCE
            | ec::attrs::ARMOR_EXPL_DMG_RESONANCE => match sol_view.items.get_item(item_id).unwrap() {
                SolItem::Ship(ship) => self.clear_fit_rah_results(sol_view, &ship.get_fit_id()),
                SolItem::Module(module) => {
                    if self.calc_data.rah.resonances.contains_key(&item_id) {
                        self.clear_fit_rah_results(sol_view, &module.get_fit_id());
                    }
                }
                _ => (),
            },
            // RAH shift amount
            ec::attrs::RESIST_SHIFT_AMOUNT => {
                if self.calc_data.rah.resonances.contains_key(&item_id) {
                    // Only modules should be registered in resonances container, and those are
                    // guaranteed to have fit ID
                    let fit_id = sol_view.items.get_item(item_id).unwrap().get_fit_id().unwrap();
                    self.clear_fit_rah_results(sol_view, &fit_id);
                }
            }
            // TODO: RAH cycle time
            _ => (),
        }
    }
    fn calc_rah_dmg_profile_changed(&mut self, sol_view: &SolView, fit_id: &SolFitId) {
        if self.calc_data.rah.sim_running {
            return;
        }
        self.clear_fit_rah_results(sol_view, fit_id);
    }
    // Private methods
    fn clear_fit_rah_results(&mut self, sol_view: &SolView, fit_id: &SolFitId) {
        let other_item_ids = self.calc_data.rah.by_fit.get(&fit_id).map(|v| *v).collect_vec();
        for other_item_id in other_item_ids {
            self.clear_rah_result(sol_view, &other_item_id);
        }
    }
    fn clear_rah_result(&mut self, sol_view: &SolView, item_id: &SolItemId) {
        if self.calc_data.rah.resonances.get_mut(item_id).unwrap().take().is_some() {
            for attr_id in RES_ATTR_IDS.iter() {
                self.notify_attr_val_changed(sol_view, item_id, attr_id)
            }
        }
    }
    fn get_rah_resonances(&mut self, sol_view: &SolView, item_id: &SolItemId) -> SolDmgTypes<SolAttrVal> {
        // Unwrap item, since method is supposed to be called only for registered RAHs
        if let Some(val) = self.calc_data.rah.resonances.get(item_id).unwrap() {
            return *val;
        }
        // Unwrap item and its fit ID, since registered RAHs are supposed to be modules, which have
        // fit ID
        let fit_id = sol_view.items.get_item(item_id).unwrap().get_fit_id().unwrap();
        self.calc_data.rah.sim_running = true;
        self.calc_rah_run_simulation(sol_view, &fit_id);
        self.calc_data.rah.sim_running = false;
        // Unwrap value, since simulation is supposed to always set results for requested RAH
        self.calc_data.rah.resonances.get(item_id).unwrap().unwrap()
    }
}

fn rah_em_resonance_postprocessor(
    svcs: &mut SolSvcs,
    sol_view: &SolView,
    item_id: &SolItemId,
    _: SolAttrVal,
) -> SolAttrVal {
    svcs.get_rah_resonances(sol_view, item_id).em
}

fn rah_therm_resonance_postprocessor(
    svcs: &mut SolSvcs,
    sol_view: &SolView,
    item_id: &SolItemId,
    _: SolAttrVal,
) -> SolAttrVal {
    svcs.get_rah_resonances(sol_view, item_id).thermal
}

fn rah_kin_resonance_postprocessor(
    svcs: &mut SolSvcs,
    sol_view: &SolView,
    item_id: &SolItemId,
    _: SolAttrVal,
) -> SolAttrVal {
    svcs.get_rah_resonances(sol_view, item_id).kinetic
}

fn rah_expl_resonance_postprocessor(
    svcs: &mut SolSvcs,
    sol_view: &SolView,
    item_id: &SolItemId,
    _: SolAttrVal,
) -> SolAttrVal {
    svcs.get_rah_resonances(sol_view, item_id).explosive
}
