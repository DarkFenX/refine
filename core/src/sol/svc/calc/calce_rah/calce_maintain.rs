use itertools::Itertools;

use crate::{
    ad,
    defs::{EAttrId, SolFitId, SolItemId},
    sol::{
        svc::calc::{SolAttrVal, SolAttrValInfo, SolCalc, SolItemAttrPostprocs},
        uad::{item::SolItem, SolUad},
        SolDmgKinds,
    },
    src::Src,
};

use super::shared::{EM_ATTR_ID, EXPL_ATTR_ID, KIN_ATTR_ID, RAH_EFFECT_ID, SHIFT_ATTR_ID, THERM_ATTR_ID};

impl SolCalc {
    pub(in crate::sol::svc::calc) fn rah_item_loaded(&mut self, uad: &SolUad, item: &SolItem) {
        if self.rah.sim_running {
            return;
        }
        if let SolItem::Ship(ship) = item {
            self.clear_fit_rah_results(uad, &ship.get_fit_id());
        }
    }
    pub(in crate::sol::svc::calc) fn rah_item_unloaded(&mut self, uad: &SolUad, item: &SolItem) {
        if self.rah.sim_running {
            return;
        }
        if let SolItem::Ship(ship) = item {
            self.clear_fit_rah_results(uad, &ship.get_fit_id());
        }
    }
    pub(in crate::sol::svc::calc) fn rah_effects_started(
        &mut self,
        uad: &SolUad,
        item: &SolItem,
        effects: &[ad::ArcEffect],
    ) {
        if self.rah.sim_running {
            return;
        }
        if let SolItem::Module(module) = item {
            if effects.iter().any(|v| v.id == RAH_EFFECT_ID) {
                let item_id = module.get_id();
                let fit_id = module.get_fit_id();
                // Clear sim data for other RAHs on the same fit
                self.clear_fit_rah_results(uad, &fit_id);
                // Add sim data for RAH being started
                self.rah.resonances.insert(item_id, None);
                self.rah.by_fit.add_entry(fit_id, item_id);
                // Add postprocessors
                let item_attr_data = self.attrs.get_item_attr_data_mut(&item_id).unwrap();
                item_attr_data.postprocs.insert(
                    EM_ATTR_ID,
                    SolItemAttrPostprocs::new(rah_em_resonance_postproc_fast, rah_em_resonance_postproc_info),
                );
                item_attr_data.postprocs.insert(
                    THERM_ATTR_ID,
                    SolItemAttrPostprocs::new(rah_therm_resonance_postproc_fast, rah_therm_resonance_postproc_info),
                );
                item_attr_data.postprocs.insert(
                    KIN_ATTR_ID,
                    SolItemAttrPostprocs::new(rah_kin_resonance_postproc_fast, rah_kin_resonance_postproc_info),
                );
                item_attr_data.postprocs.insert(
                    EXPL_ATTR_ID,
                    SolItemAttrPostprocs::new(rah_expl_resonance_postproc_fast, rah_expl_resonance_postproc_info),
                );
            }
        }
    }
    pub(in crate::sol::svc::calc) fn rah_effects_stopped(
        &mut self,
        uad: &SolUad,
        item: &SolItem,
        effects: &[ad::ArcEffect],
    ) {
        if self.rah.sim_running {
            return;
        }
        if let SolItem::Module(module) = item {
            if effects.iter().any(|v| v.id == RAH_EFFECT_ID) {
                let item_id = module.get_id();
                let fit_id = module.get_fit_id();
                // Remove postprocessors
                let item_attr_data = self.attrs.get_item_attr_data_mut(&item_id).unwrap();
                item_attr_data.postprocs.remove(&EM_ATTR_ID);
                item_attr_data.postprocs.remove(&THERM_ATTR_ID);
                item_attr_data.postprocs.remove(&KIN_ATTR_ID);
                item_attr_data.postprocs.remove(&EXPL_ATTR_ID);
                // Remove sim data for RAH being stopped
                self.rah.resonances.remove(&item_id);
                self.rah.by_fit.remove_entry(&module.get_fit_id(), &item_id);
                // Clear sim data for other RAHs on the same fit
                self.clear_fit_rah_results(uad, &fit_id);
            }
        }
    }
    pub(in crate::sol::svc::calc) fn rah_attr_value_changed(
        &mut self,
        uad: &SolUad,
        item_id: &SolItemId,
        attr_id: &EAttrId,
    ) {
        if self.rah.sim_running {
            return;
        }
        match *attr_id {
            // Ship armor resonances and RAH resonances
            EM_ATTR_ID | THERM_ATTR_ID | KIN_ATTR_ID | EXPL_ATTR_ID => match uad.items.get_item(item_id).unwrap() {
                SolItem::Ship(ship) => self.clear_fit_rah_results(uad, &ship.get_fit_id()),
                SolItem::Module(module) => {
                    if self.rah.resonances.contains_key(&item_id) {
                        self.clear_fit_rah_results(uad, &module.get_fit_id());
                    }
                }
                _ => (),
            },
            // RAH shift amount
            SHIFT_ATTR_ID => {
                if self.rah.resonances.contains_key(&item_id) {
                    // Only modules should be registered in resonances container, and those are
                    // guaranteed to have fit ID
                    let fit_id = uad.items.get_item(item_id).unwrap().get_fit_id().unwrap();
                    self.clear_fit_rah_results(uad, &fit_id);
                }
            }
            // RAH cycle time
            a if Some(a) == self.rah.cycle_time_attr_id => {
                if self.rah.resonances.contains_key(&item_id) {
                    // Only modules should be registered in resonances container, and those are
                    // guaranteed to have fit ID
                    let fit_id = uad.items.get_item(item_id).unwrap().get_fit_id().unwrap();
                    // Clear only for fits with 2+ RAHs, since changing cycle time of 1 RAH does not
                    // change sim results
                    if self.rah.by_fit.get(&fit_id).len() >= 2 {
                        self.clear_fit_rah_results(uad, &fit_id);
                    }
                }
            }
            _ => (),
        }
    }
    pub(in crate::sol::svc::calc) fn rah_src_changed(&mut self, src: &Src) {
        self.rah.cycle_time_attr_id = src.get_a_effect(&RAH_EFFECT_ID).map(|v| v.duration_attr_id).flatten();
    }
    pub(in crate::sol::svc::calc) fn rah_fit_rah_dmg_profile_changed(&mut self, uad: &SolUad, fit_id: &SolFitId) {
        self.clear_fit_rah_results(uad, fit_id);
    }
    // Private methods
    fn clear_fit_rah_results(&mut self, uad: &SolUad, fit_id: &SolFitId) {
        let other_item_ids = self.rah.by_fit.get(&fit_id).copied().collect_vec();
        for other_item_id in other_item_ids {
            self.clear_rah_result(uad, &other_item_id);
        }
    }
    fn clear_rah_result(&mut self, uad: &SolUad, item_id: &SolItemId) {
        if self.rah.resonances.get_mut(item_id).unwrap().take().is_some() {
            self.force_attr_postproc_recalc(uad, item_id, &EM_ATTR_ID);
            self.force_attr_postproc_recalc(uad, item_id, &THERM_ATTR_ID);
            self.force_attr_postproc_recalc(uad, item_id, &KIN_ATTR_ID);
            self.force_attr_postproc_recalc(uad, item_id, &EXPL_ATTR_ID);
        }
    }
    fn get_rah_resonances(&mut self, uad: &SolUad, item_id: &SolItemId) -> SolDmgKinds<SolAttrVal> {
        // Unwrap item, since method is supposed to be called only for registered RAHs
        if let Some(val) = self.rah.resonances.get(item_id).unwrap() {
            return *val;
        }
        // Unwrap item and its fit ID, since registered RAHs are supposed to be modules, which have
        // fit ID
        let fit_id = uad.items.get_item(item_id).unwrap().get_fit_id().unwrap();
        self.rah.sim_running = true;
        self.rah_run_simulation(uad, &fit_id);
        self.rah.sim_running = false;
        // Unwrap value, since simulation is supposed to always set results for requested RAH
        self.rah.resonances.get(item_id).unwrap().unwrap()
    }
}

fn rah_em_resonance_postproc_fast(
    calc: &mut SolCalc,
    uad: &SolUad,
    item_id: &SolItemId,
    _val: SolAttrVal,
) -> SolAttrVal {
    calc.get_rah_resonances(uad, item_id).em
}

fn rah_therm_resonance_postproc_fast(
    calc: &mut SolCalc,
    uad: &SolUad,
    item_id: &SolItemId,
    _val: SolAttrVal,
) -> SolAttrVal {
    calc.get_rah_resonances(uad, item_id).thermal
}

fn rah_kin_resonance_postproc_fast(
    calc: &mut SolCalc,
    uad: &SolUad,
    item_id: &SolItemId,
    _val: SolAttrVal,
) -> SolAttrVal {
    calc.get_rah_resonances(uad, item_id).kinetic
}

fn rah_expl_resonance_postproc_fast(
    calc: &mut SolCalc,
    uad: &SolUad,
    item_id: &SolItemId,
    _val: SolAttrVal,
) -> SolAttrVal {
    calc.get_rah_resonances(uad, item_id).explosive
}

fn rah_em_resonance_postproc_info(
    calc: &mut SolCalc,
    uad: &SolUad,
    item_id: &SolItemId,
    mut info: SolAttrValInfo,
) -> SolAttrValInfo {
    info.value = calc.get_rah_resonances(uad, item_id).em.extra;
    info
}

fn rah_therm_resonance_postproc_info(
    calc: &mut SolCalc,
    uad: &SolUad,
    item_id: &SolItemId,
    mut info: SolAttrValInfo,
) -> SolAttrValInfo {
    info.value = calc.get_rah_resonances(uad, item_id).thermal.extra;
    info
}

fn rah_kin_resonance_postproc_info(
    calc: &mut SolCalc,
    uad: &SolUad,
    item_id: &SolItemId,
    mut info: SolAttrValInfo,
) -> SolAttrValInfo {
    info.value = calc.get_rah_resonances(uad, item_id).kinetic.extra;
    info
}

fn rah_expl_resonance_postproc_info(
    calc: &mut SolCalc,
    uad: &SolUad,
    item_id: &SolItemId,
    mut info: SolAttrValInfo,
) -> SolAttrValInfo {
    info.value = calc.get_rah_resonances(uad, item_id).explosive.extra;
    info
}
