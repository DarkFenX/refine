use itertools::Itertools;

use crate::{
    ad,
    sol::{
        DmgKinds, FitId, ItemId,
        svc::calc::{AttrValInfo, Calc, CalcAttrVal, ItemAttrPostprocs},
        uad::{Uad, item::Item},
    },
    src::Src,
};

use super::shared::{
    ARMOR_EM_ATTR_ID, ARMOR_EXPL_ATTR_ID, ARMOR_HP_ATTR_ID, ARMOR_KIN_ATTR_ID, ARMOR_THERM_ATTR_ID, HULL_HP_ATTR_ID,
    RAH_EFFECT_ID, RAH_SHIFT_ATTR_ID, SHIELD_HP_ATTR_ID, get_fit_rah_incoming_dps,
};

impl Calc {
    pub(in crate::sol::svc::calc) fn rah_item_loaded(&mut self, uad: &Uad, item: &Item) {
        if self.rah.sim_running {
            return;
        }
        if let Item::Ship(ship) = item {
            self.clear_fit_rah_results(uad, &ship.get_fit_id());
        }
    }
    pub(in crate::sol::svc::calc) fn rah_item_unloaded(&mut self, uad: &Uad, item: &Item) {
        if self.rah.sim_running {
            return;
        }
        if let Item::Ship(ship) = item {
            self.clear_fit_rah_results(uad, &ship.get_fit_id());
        }
    }
    pub(in crate::sol::svc::calc) fn rah_effects_started(
        &mut self,
        uad: &Uad,
        item: &Item,
        a_effects: &[ad::ArcEffect],
    ) {
        if self.rah.sim_running {
            return;
        }
        if let Item::Module(module) = item {
            if a_effects.iter().any(|v| v.id == RAH_EFFECT_ID) {
                let item_id = module.get_item_id();
                let fit_id = module.get_fit_id();
                // Clear sim data for other RAHs on the same fit
                self.clear_fit_rah_results(uad, &fit_id);
                // Add sim data for RAH being started
                self.rah.resonances.insert(item_id, None);
                self.rah.by_fit.add_entry(fit_id, item_id);
                // Add postprocessors
                let item_attr_data = self.attrs.get_item_attr_data_mut(&item_id).unwrap();
                item_attr_data.postprocs.insert(
                    ARMOR_EM_ATTR_ID,
                    ItemAttrPostprocs {
                        fast: rah_em_resonance_postproc_fast,
                        info: rah_em_resonance_postproc_info,
                    },
                );
                item_attr_data.postprocs.insert(
                    ARMOR_THERM_ATTR_ID,
                    ItemAttrPostprocs {
                        fast: rah_therm_resonance_postproc_fast,
                        info: rah_therm_resonance_postproc_info,
                    },
                );
                item_attr_data.postprocs.insert(
                    ARMOR_KIN_ATTR_ID,
                    ItemAttrPostprocs {
                        fast: rah_kin_resonance_postproc_fast,
                        info: rah_kin_resonance_postproc_info,
                    },
                );
                item_attr_data.postprocs.insert(
                    ARMOR_EXPL_ATTR_ID,
                    ItemAttrPostprocs {
                        fast: rah_expl_resonance_postproc_fast,
                        info: rah_expl_resonance_postproc_info,
                    },
                );
            }
        }
    }
    pub(in crate::sol::svc::calc) fn rah_effects_stopped(
        &mut self,
        uad: &Uad,
        item: &Item,
        a_effects: &[ad::ArcEffect],
    ) {
        if self.rah.sim_running {
            return;
        }
        if let Item::Module(module) = item {
            if a_effects.iter().any(|v| v.id == RAH_EFFECT_ID) {
                let item_id = module.get_item_id();
                let fit_id = module.get_fit_id();
                // Remove postprocessors
                let item_attr_data = self.attrs.get_item_attr_data_mut(&item_id).unwrap();
                item_attr_data.postprocs.remove(&ARMOR_EM_ATTR_ID);
                item_attr_data.postprocs.remove(&ARMOR_THERM_ATTR_ID);
                item_attr_data.postprocs.remove(&ARMOR_KIN_ATTR_ID);
                item_attr_data.postprocs.remove(&ARMOR_EXPL_ATTR_ID);
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
        uad: &Uad,
        item_id: &ItemId,
        a_attr_id: &ad::AAttrId,
    ) {
        if self.rah.sim_running {
            return;
        }
        // This is going to be called very often, no need to figure out if we need to clear results
        // if we have no RAHs running
        if self.rah.resonances.is_empty() {
            return;
        }
        match *a_attr_id {
            // Ship armor resonances and RAH resonances
            ARMOR_EM_ATTR_ID | ARMOR_THERM_ATTR_ID | ARMOR_KIN_ATTR_ID | ARMOR_EXPL_ATTR_ID => {
                match uad.items.get_item(item_id).unwrap() {
                    Item::Ship(ship) => self.clear_fit_rah_results(uad, &ship.get_fit_id()),
                    Item::Module(module) => {
                        if self.rah.resonances.contains_key(item_id) {
                            self.clear_fit_rah_results(uad, &module.get_fit_id());
                        }
                    }
                    _ => (),
                }
            }
            // RAH shift amount
            RAH_SHIFT_ATTR_ID => {
                if self.rah.resonances.contains_key(item_id) {
                    // Only modules should be registered in resonances container, and those are
                    // guaranteed to have fit ID
                    let fit_id = uad.items.get_item(item_id).unwrap().get_fit_id().unwrap();
                    self.clear_fit_rah_results(uad, &fit_id);
                }
            }
            // RAH cycle time
            a_attr_id if Some(a_attr_id) == self.rah.cycle_time_a_attr_id => {
                if self.rah.resonances.contains_key(item_id) {
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
            // Ship HP - need to clear results since breacher DPS depends on those
            SHIELD_HP_ATTR_ID | ARMOR_HP_ATTR_ID | HULL_HP_ATTR_ID => {
                if let Item::Ship(ship) = uad.items.get_item(item_id).unwrap() {
                    let fit_id = ship.get_fit_id();
                    let fit = uad.fits.get_fit(&fit_id).unwrap();
                    if get_fit_rah_incoming_dps(uad, fit).deals_breacher_dps() {
                        self.clear_fit_rah_results(uad, &fit_id);
                    }
                }
            }
            _ => (),
        }
    }
    pub(in crate::sol::svc::calc) fn rah_src_changed(&mut self, src: &Src) {
        self.rah.cycle_time_a_attr_id = src.get_a_effect(&RAH_EFFECT_ID).and_then(|v| v.duration_attr_id);
    }
    pub(in crate::sol::svc::calc) fn rah_fit_rah_dps_profile_changed(&mut self, uad: &Uad, fit_id: &FitId) {
        self.clear_fit_rah_results(uad, fit_id);
    }
    // Private methods
    fn clear_fit_rah_results(&mut self, uad: &Uad, fit_id: &FitId) {
        let rah_item_ids = self.rah.by_fit.get(fit_id).copied().collect_vec();
        for rah_item_id in rah_item_ids {
            self.clear_rah_result(uad, &rah_item_id);
        }
    }
    fn clear_rah_result(&mut self, uad: &Uad, item_id: &ItemId) {
        if self.rah.resonances.get_mut(item_id).unwrap().take().is_some() {
            self.force_attr_postproc_recalc(uad, item_id, &ARMOR_EM_ATTR_ID);
            self.force_attr_postproc_recalc(uad, item_id, &ARMOR_THERM_ATTR_ID);
            self.force_attr_postproc_recalc(uad, item_id, &ARMOR_KIN_ATTR_ID);
            self.force_attr_postproc_recalc(uad, item_id, &ARMOR_EXPL_ATTR_ID);
        }
    }
    fn get_rah_resonances(&mut self, uad: &Uad, item_id: &ItemId) -> DmgKinds<CalcAttrVal> {
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

fn rah_em_resonance_postproc_fast(calc: &mut Calc, uad: &Uad, item_id: &ItemId, _cval: CalcAttrVal) -> CalcAttrVal {
    calc.get_rah_resonances(uad, item_id).em
}

fn rah_therm_resonance_postproc_fast(calc: &mut Calc, uad: &Uad, item_id: &ItemId, _cval: CalcAttrVal) -> CalcAttrVal {
    calc.get_rah_resonances(uad, item_id).thermal
}

fn rah_kin_resonance_postproc_fast(calc: &mut Calc, uad: &Uad, item_id: &ItemId, _cval: CalcAttrVal) -> CalcAttrVal {
    calc.get_rah_resonances(uad, item_id).kinetic
}

fn rah_expl_resonance_postproc_fast(calc: &mut Calc, uad: &Uad, item_id: &ItemId, _cval: CalcAttrVal) -> CalcAttrVal {
    calc.get_rah_resonances(uad, item_id).explosive
}

fn rah_em_resonance_postproc_info(calc: &mut Calc, uad: &Uad, item_id: &ItemId, mut info: AttrValInfo) -> AttrValInfo {
    info.value = calc.get_rah_resonances(uad, item_id).em.extra;
    info
}

fn rah_therm_resonance_postproc_info(
    calc: &mut Calc,
    uad: &Uad,
    item_id: &ItemId,
    mut info: AttrValInfo,
) -> AttrValInfo {
    info.value = calc.get_rah_resonances(uad, item_id).thermal.extra;
    info
}

fn rah_kin_resonance_postproc_info(calc: &mut Calc, uad: &Uad, item_id: &ItemId, mut info: AttrValInfo) -> AttrValInfo {
    info.value = calc.get_rah_resonances(uad, item_id).kinetic.extra;
    info
}

fn rah_expl_resonance_postproc_info(
    calc: &mut Calc,
    uad: &Uad,
    item_id: &ItemId,
    mut info: AttrValInfo,
) -> AttrValInfo {
    info.value = calc.get_rah_resonances(uad, item_id).explosive.extra;
    info
}
