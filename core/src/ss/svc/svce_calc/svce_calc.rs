use itertools::Itertools;

use crate::{
    ad,
    defs::{EAttrId, SsFitId, SsItemId},
    ec,
    ss::{
        fleet::SsFleet,
        item::SsItem,
        svc::{
            svce_calc::{SsAttrMod, SsFleetUpdates},
            SsSvcs,
        },
        SsView,
    },
};

impl SsSvcs {
    // Modification methods
    pub(in crate::ss::svc) fn calc_fit_added(&mut self, fit_id: &SsFitId) {
        self.calc_data.mods.reg_fit(fit_id)
    }
    pub(in crate::ss::svc) fn calc_fit_removed(&mut self, fit_id: &SsFitId) {
        self.calc_data.mods.unreg_fit(fit_id)
    }
    pub(in crate::ss::svc) fn calc_fit_added_to_fleet(&mut self, ss_view: &SsView, fleet: &SsFleet, fit_id: &SsFitId) {
        let updates = self.calc_data.mods.reg_fleet_for_fit(ss_view, fleet, fit_id);
        self.process_fleet_updates(ss_view, fleet, fit_id, updates);
    }
    pub(in crate::ss::svc) fn calc_fit_removed_from_fleet(
        &mut self,
        ss_view: &SsView,
        fleet: &SsFleet,
        fit_id: &SsFitId,
    ) {
        let updates = self.calc_data.mods.unreg_fleet_for_fit(ss_view, fleet, fit_id);
        self.process_fleet_updates(ss_view, fleet, fit_id, updates);
    }
    pub(in crate::ss::svc) fn calc_item_added(&mut self, ss_view: &SsView, item: &SsItem) {
        self.handle_location_owner_change(ss_view, item);
        // Custom modifiers
        let ss_mods = self.calc_data.revs.get_mods_on_item_add();
        if !ss_mods.is_empty() {
            let mut affectees = Vec::new();
            for ss_mod in ss_mods.iter() {
                if ss_mod.revise_on_item_add(item, ss_view) {
                    if let Ok(src_item) = ss_view.items.get_item(&ss_mod.affector_item_id) {
                        self.calc_data
                            .affectee
                            .fill_affectees(&mut affectees, ss_view, src_item, ss_mod);
                        for tgt_item_id in affectees.iter() {
                            self.calc_force_attr_recalc(ss_view, tgt_item_id, &ss_mod.affectee_attr_id);
                        }
                        affectees.clear();
                    }
                }
            }
        }
    }
    pub(in crate::ss::svc) fn calc_item_removed(&mut self, ss_view: &SsView, item: &SsItem) {
        self.handle_location_owner_change(ss_view, item);
        // Custom modifiers
        let ss_mods = self.calc_data.revs.get_mods_on_item_remove();
        if !ss_mods.is_empty() {
            let mut affectees = Vec::new();
            for ss_mod in ss_mods.iter() {
                if ss_mod.revise_on_item_remove(item, ss_view) {
                    if let Ok(src_item) = ss_view.items.get_item(&ss_mod.affector_item_id) {
                        self.calc_data
                            .affectee
                            .fill_affectees(&mut affectees, ss_view, src_item, ss_mod);
                        for tgt_item_id in affectees.iter() {
                            self.calc_force_attr_recalc(ss_view, tgt_item_id, &ss_mod.affectee_attr_id);
                        }
                        affectees.clear();
                    }
                }
            }
        }
    }
    pub(in crate::ss::svc) fn calc_item_loaded(&mut self, ss_view: &SsView, item: &SsItem) {
        self.calc_data.attrs.add_item(item.get_id());
        self.calc_data.affectee.reg_affectee(ss_view, item);
    }
    pub(in crate::ss::svc) fn calc_item_unloaded(&mut self, ss_view: &SsView, item: &SsItem) {
        self.calc_data.affectee.unreg_affectee(ss_view, item);
        let item_id = item.get_id();
        self.calc_data.attrs.remove_item(&item_id);
        self.calc_data.deps.clear_item_data(&item_id);
    }
    pub(in crate::ss::svc) fn calc_effects_started(
        &mut self,
        ss_view: &SsView,
        item: &SsItem,
        effects: &Vec<ad::ArcEffect>,
    ) {
        // Register new mods
        let mut affectees = Vec::new();
        let ss_mods = self.calc_generate_mods_for_effects(ss_view, item, effects);
        self.reg_mods(&mut affectees, ss_view, item, &ss_mods.all);
        // Apply mods to targets, if needed
        if let Some(tgt_item_ids) = item.iter_targets() {
            for tgt_item_id in tgt_item_ids {
                let tgt_item = ss_view.items.get_item(&tgt_item_id).unwrap();
                self.reg_mods_for_tgt(&mut affectees, ss_view, item, &ss_mods.all, tgt_item);
            }
        }
        // Buff maintenance - add info about effects/modifiers which use default buff attributes
        for effect in effects.iter() {
            self.calc_data.buffs.reg_effect(item.get_id(), effect);
        }
        for (buff_type_attr_id, dependent_mods) in ss_mods.dependent_buffs.iter() {
            for dependent_mod in dependent_mods {
                self.calc_data
                    .buffs
                    .reg_dependent_mod(item.get_id(), *buff_type_attr_id, *dependent_mod);
            }
        }
    }
    pub(in crate::ss::svc) fn calc_effects_stopped(
        &mut self,
        ss_view: &SsView,
        item: &SsItem,
        effects: &Vec<ad::ArcEffect>,
    ) {
        // Unregister mods
        let mut affectees = Vec::new();
        let ss_mods = self.calc_generate_mods_for_effects(ss_view, item, effects);
        self.unreg_mods(&mut affectees, ss_view, item, &ss_mods.all);
        // Remove mods from targets, if needed
        if let Some(tgt_item_ids) = item.iter_targets() {
            for tgt_item_id in tgt_item_ids {
                let tgt_item = ss_view.items.get_item(&tgt_item_id).unwrap();
                self.unreg_mods_for_tgt(&mut affectees, ss_view, item, &ss_mods.all, tgt_item);
            }
        }
        // This bit is just for propulsion mode effect, so that when effect is not running (but item
        // is not removed), changes to parent attributes like ship mass do not clear the child
        // attribute - ship speed
        for ss_mod in ss_mods.all.iter() {
            ss_mod.on_effect_stop(self, ss_view);
        }
        // Buff maintenance - remove info about effects/modifiers which use default buff attributes
        for effect in effects.iter() {
            self.calc_data.buffs.unreg_effect(item.get_id(), effect);
        }
        for (buff_type_attr_id, dependent_mods) in ss_mods.dependent_buffs.iter() {
            for dependent_mod in dependent_mods {
                self.calc_data
                    .buffs
                    .unreg_dependent_mod(&item.get_id(), buff_type_attr_id, dependent_mod);
            }
        }
    }
    pub(in crate::ss::svc) fn calc_item_tgt_added(&mut self, ss_view: &SsView, item: &SsItem, tgt_item_id: SsItemId) {
        let item_id = item.get_id();
        let ss_mods = self
            .calc_data
            .mods
            .iter_mods_for_src(&item_id)
            .map(|v| *v)
            .collect_vec();
        if !ss_mods.is_empty() {
            let tgt_item = ss_view.items.get_item(&tgt_item_id).unwrap();
            let mut affectees = Vec::new();
            self.reg_mods_for_tgt(&mut affectees, ss_view, item, &ss_mods, tgt_item);
        }
    }
    pub(in crate::ss::svc) fn calc_item_tgt_removed(
        &mut self,
        ss_view: &SsView,
        item: &SsItem,
        tgt_item_id: &SsItemId,
    ) {
        let item_id = item.get_id();
        let ss_mods = self
            .calc_data
            .mods
            .iter_mods_for_src(&item_id)
            .map(|v| *v)
            .collect_vec();
        if !ss_mods.is_empty() {
            let tgt_item = ss_view.items.get_item(&tgt_item_id).unwrap();
            let mut affectees = Vec::new();
            self.unreg_mods_for_tgt(&mut affectees, ss_view, item, &ss_mods, tgt_item);
        }
    }
    pub(in crate::ss::svc) fn calc_attr_value_changed(
        &mut self,
        ss_view: &SsView,
        item_id: &SsItemId,
        attr_id: &EAttrId,
    ) {
        let item = ss_view.items.get_item(item_id).unwrap();
        // Clear up attribute values which rely on passed attribute as an upper cap
        let attr_specs = self
            .calc_data
            .deps
            .get_tgt_attr_specs(item_id, attr_id)
            .map(|v| *v)
            .collect_vec();
        for attr_spec in attr_specs.iter() {
            self.calc_force_attr_recalc(ss_view, &attr_spec.item_id, &attr_spec.attr_id);
        }
        // Clear up attribute values which rely on passed attribute as a modification source
        let mods = self
            .calc_data
            .mods
            .iter_mods_for_src(item_id)
            .filter(|v| v.get_src_attr_id() == Some(*attr_id))
            .map(|v| *v)
            .collect_vec();
        if !mods.is_empty() {
            let mut affectees = Vec::new();
            for modifier in mods.iter() {
                self.calc_data
                    .affectee
                    .fill_affectees(&mut affectees, ss_view, item, &modifier);
                for tgt_item_id in affectees.iter() {
                    self.calc_force_attr_recalc(ss_view, tgt_item_id, &modifier.affectee_attr_id);
                }
                affectees.clear();
            }
        }
        // Process buffs which rely on attribute being modified
        if ec::attrs::BUFF_ID_ATTRS.contains(attr_id) {
            let mut affectees = Vec::new();
            // Remove modifiers of buffs which rely on the attribute
            if let Some(mods) = self.calc_data.buffs.extract_dependent_mods(item_id, attr_id) {
                let ss_mods = mods.collect();
                self.unreg_mods(&mut affectees, ss_view, item, &ss_mods);
            }
            // Generate new modifiers using new value and apply them
            let effect_ids = self.calc_data.buffs.get_effects(item_id);
            if !effect_ids.is_empty() {
                let effect_ids = effect_ids.map(|v| *v).collect_vec();
                let ss_mods = self.calc_generate_dependent_buff_mods(ss_view, item, effect_ids.iter(), attr_id);
                for ss_mod in ss_mods.iter() {
                    self.calc_data.buffs.reg_dependent_mod(*item_id, *attr_id, *ss_mod);
                }
                self.reg_mods(&mut affectees, ss_view, item, &ss_mods);
            }
        }
    }
    pub(in crate::ss) fn calc_force_attr_recalc(&mut self, ss_view: &SsView, item_id: &SsItemId, attr_id: &EAttrId) {
        match self.calc_data.attrs.get_item_attrs_mut(item_id) {
            Ok(item_attrs) => {
                if item_attrs.remove(attr_id).is_some() {
                    self.notify_attr_val_changed(ss_view, item_id, attr_id);
                }
            }
            _ => return,
        }
    }
    // Private methods
    fn reg_mods(&mut self, affectees: &mut Vec<SsItemId>, ss_view: &SsView, item: &SsItem, ss_mods: &Vec<SsAttrMod>) {
        // Regular modifiers
        for ss_mod in ss_mods.iter() {
            // Modifications have to be added before target attributes are cleared, because for case
            // of fleet buff ID attributes new value will be fetched instantly after cleanup, and
            // that value has to be new
            self.calc_data.mods.reg_mod(ss_view, item, *ss_mod);
            self.calc_data.affectee.fill_affectees(affectees, ss_view, item, ss_mod);
            for tgt_item_id in affectees.iter() {
                self.calc_force_attr_recalc(ss_view, tgt_item_id, &ss_mod.affectee_attr_id);
            }
            affectees.clear();
        }
        // Revisions
        for ss_mod in ss_mods.iter() {
            self.calc_data.revs.reg_mod(*ss_mod);
        }
    }
    fn unreg_mods(&mut self, affectees: &mut Vec<SsItemId>, ss_view: &SsView, item: &SsItem, ss_mods: &Vec<SsAttrMod>) {
        for ss_mod in ss_mods.iter() {
            // Modifications have to be removed before target attributes are cleared, because for
            // case of fleet buff ID attributes new value will be fetched instantly after cleanup,
            // and that value has to be new
            self.calc_data.mods.unreg_mod(ss_view, item, ss_mod);
            self.calc_data.affectee.fill_affectees(affectees, ss_view, item, ss_mod);
            for tgt_item_id in affectees.iter() {
                self.calc_force_attr_recalc(ss_view, tgt_item_id, &ss_mod.affectee_attr_id);
            }
            affectees.clear();
        }
        // Revisions and effect-specific processing
        for ss_mod in ss_mods.iter() {
            self.calc_data.revs.unreg_mod(ss_mod);
            // This bit is just for propulsion mode effect, so that when effect is not running (but
            // item is not removed), changes to parent attributes like ship mass do not clear the
            // child attribute - ship speed
            ss_mod.on_effect_stop(self, ss_view);
        }
    }
    fn reg_mods_for_tgt(
        &mut self,
        affectees: &mut Vec<SsItemId>,
        ss_view: &SsView,
        item: &SsItem,
        ss_mods: &Vec<SsAttrMod>,
        tgt_item: &SsItem,
    ) {
        for ss_mod in ss_mods.iter() {
            if self.calc_data.mods.add_mod_tgt(item, *ss_mod, tgt_item) {
                self.calc_data
                    .affectee
                    .fill_affectees_for_tgt_item(affectees, ss_view, ss_mod, &tgt_item);
                for tgt_item_id in affectees.iter() {
                    self.calc_force_attr_recalc(ss_view, tgt_item_id, &ss_mod.affectee_attr_id);
                }
                affectees.clear();
            }
        }
    }
    fn unreg_mods_for_tgt(
        &mut self,
        affectees: &mut Vec<SsItemId>,
        ss_view: &SsView,
        item: &SsItem,
        ss_mods: &Vec<SsAttrMod>,
        tgt_item: &SsItem,
    ) {
        for ss_mod in ss_mods.iter() {
            self.calc_data
                .affectee
                .fill_affectees_for_tgt_item(affectees, ss_view, ss_mod, &tgt_item);
            for tgt_item_id in affectees.iter() {
                self.calc_force_attr_recalc(ss_view, &tgt_item_id, &ss_mod.affectee_attr_id);
            }
            affectees.clear();
            self.calc_data.mods.rm_mod_tgt(item, ss_mod, tgt_item);
        }
    }
    fn handle_location_owner_change(&mut self, ss_view: &SsView, item: &SsItem) {
        if item.get_root_loc_type().is_some() {
            let fit_id = match item.get_fit_id() {
                Some(fit_id) => fit_id,
                _ => return,
            };
            let fit = match ss_view.fits.get_fit(&fit_id) {
                Ok(fit) => fit,
                _ => return,
            };
            let mut affectees = Vec::new();
            for ss_mod in self
                .calc_data
                .mods
                .get_mods_for_changed_location_owner(item, ss_view.items)
            {
                self.calc_data
                    .affectee
                    .fill_affectees_for_fit(&mut affectees, &ss_mod, fit);
                for item_id in affectees.iter() {
                    self.calc_force_attr_recalc(ss_view, item_id, &ss_mod.affectee_attr_id);
                }
                affectees.clear();
            }
        }
    }
    fn process_fleet_updates(&mut self, ss_view: &SsView, fleet: &SsFleet, fit_id: &SsFitId, updates: SsFleetUpdates) {
        let mut affectees = Vec::new();
        if !updates.incoming.is_empty() {
            let tgt_fit = ss_view.fits.get_fit(fit_id).unwrap();
            for ss_mod in updates.incoming.iter() {
                self.calc_data
                    .affectee
                    .fill_affectees_for_fit(&mut affectees, ss_mod, tgt_fit);
                for tgt_item_id in affectees.iter() {
                    self.calc_force_attr_recalc(ss_view, &tgt_item_id, &ss_mod.affectee_attr_id);
                }
                affectees.clear();
            }
        }
        if !updates.outgoing.is_empty() {
            for ss_mod in updates.outgoing.iter() {
                for tgt_fit in fleet
                    .iter_fits()
                    .filter(|v| *v != fit_id)
                    .map(|v| ss_view.fits.get_fit(v).unwrap())
                {
                    self.calc_data
                        .affectee
                        .fill_affectees_for_fit(&mut affectees, ss_mod, tgt_fit);
                }
                for tgt_item_id in affectees.iter() {
                    self.calc_force_attr_recalc(ss_view, &tgt_item_id, &ss_mod.affectee_attr_id);
                }
                affectees.clear();
            }
        }
    }
}
