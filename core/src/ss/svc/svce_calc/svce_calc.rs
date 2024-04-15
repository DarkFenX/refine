use itertools::Itertools;

use crate::{
    ad,
    defs::{EAttrId, SsFitId, SsItemId},
    ec,
    ss::{
        fleet::SsFleet,
        item::SsItem,
        svc::{
            svce_calc::{modifier::SsAttrMod, registers::FleetUpdates},
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
        for ss_mod in self.calc_data.revs.get_mods_on_item_add() {
            if ss_mod.revise_on_item_add(item, ss_view) {
                if let Ok(src_item) = ss_view.items.get_item(&ss_mod.src_item_id) {
                    for tgt_item_id in self.calc_data.tgts.get_tgt_items(ss_view, src_item, &ss_mod) {
                        self.calc_force_attr_recalc(ss_view, &tgt_item_id, &ss_mod.tgt_attr_id);
                    }
                }
            }
        }
    }
    pub(in crate::ss::svc) fn calc_item_removed(&mut self, ss_view: &SsView, item: &SsItem) {
        self.handle_location_owner_change(ss_view, item);
        // Custom modifiers
        for ss_mod in self.calc_data.revs.get_mods_on_item_remove() {
            if ss_mod.revise_on_item_remove(item, ss_view) {
                if let Ok(src_item) = ss_view.items.get_item(&ss_mod.src_item_id) {
                    for tgt_item_id in self.calc_data.tgts.get_tgt_items(ss_view, src_item, &ss_mod) {
                        self.calc_force_attr_recalc(ss_view, &tgt_item_id, &ss_mod.tgt_attr_id);
                    }
                }
            }
        }
    }
    pub(in crate::ss::svc) fn calc_item_loaded(&mut self, ss_view: &SsView, item: &SsItem) {
        self.calc_data.attrs.add_item(item.get_id());
        self.calc_data.tgts.reg_tgt(item, ss_view.fits);
    }
    pub(in crate::ss::svc) fn calc_item_unloaded(&mut self, ss_view: &SsView, item: &SsItem) {
        self.calc_data.tgts.unreg_tgt(item, ss_view.fits);
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
        // Buff maintenance
        for effect in effects.iter() {
            self.calc_data.buffs.reg_effect(item.get_id(), effect);
        }
        let ss_mods = self.calc_generate_mods_for_effects(ss_view, item, effects);
        self.reg_mods(ss_view, item, &ss_mods.all);
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
        let ss_mods = self.calc_generate_mods_for_effects(ss_view, item, effects);
        self.unreg_mods(ss_view, item, &ss_mods.all);
        // This bit is just for propulsion mode effect, so that when effect is not running (but item
        // is not removed), changes to parent attributes like ship mass do not clear the child
        // attribute - ship speed
        for ss_mod in ss_mods.all.iter() {
            ss_mod.on_effect_stop(self, ss_view);
        }
        // Buff maintenance
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
        let tgt_item = match ss_view.items.get_item(&tgt_item_id) {
            Ok(item) => item,
            _ => return,
        };
        let tgt_fit_id = match tgt_item.get_fit_id() {
            Some(fit_id) => fit_id,
            _ => return,
        };
        let tgt_fit = match ss_view.fits.get_fit(&tgt_fit_id) {
            Ok(fit) => fit,
            _ => return,
        };
        let tgt_fits = vec![tgt_fit];
        for ss_mod in ss_mods.iter() {
            if self.calc_data.mods.add_mod_tgt(item, *ss_mod, tgt_item) {
                let mod_item = match ss_view.items.get_item(&ss_mod.src_item_id) {
                    Ok(item) => item,
                    _ => continue,
                };
                for tgt_item_id in self.calc_data.tgts.get_tgt_items_for_fits(mod_item, ss_mod, &tgt_fits) {
                    self.calc_force_attr_recalc(ss_view, &tgt_item_id, &ss_mod.tgt_attr_id);
                }
            }
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
        let tgt_item = match ss_view.items.get_item(&tgt_item_id) {
            Ok(item) => item,
            _ => return,
        };
        let tgt_fit_id = match tgt_item.get_fit_id() {
            Some(fit_id) => fit_id,
            _ => return,
        };
        let tgt_fit = match ss_view.fits.get_fit(&tgt_fit_id) {
            Ok(fit) => fit,
            _ => return,
        };
        let tgt_fits = vec![tgt_fit];
        for ss_mod in ss_mods.iter() {
            let mod_item = match ss_view.items.get_item(&ss_mod.src_item_id) {
                Ok(item) => item,
                _ => continue,
            };
            for tgt_item_id in self.calc_data.tgts.get_tgt_items_for_fits(mod_item, ss_mod, &tgt_fits) {
                self.calc_force_attr_recalc(ss_view, &tgt_item_id, &ss_mod.tgt_attr_id);
            }
            self.calc_data.mods.rm_mod_tgt(item, ss_mod, tgt_item);
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
        let dependents = self
            .calc_data
            .deps
            .get_tgt_attr_specs(item_id, attr_id)
            .map(|v| v.iter().map(|v| *v).collect_vec());
        if let Some(attr_specs) = dependents {
            for attr_spec in attr_specs.iter() {
                self.calc_force_attr_recalc(ss_view, &attr_spec.item_id, &attr_spec.attr_id);
            }
        };
        // Clear up attribute values which rely on passed attribute as a modification source
        let mods = self
            .calc_data
            .mods
            .iter_mods_for_src(item_id)
            .filter(|v| v.get_src_attr_id() == Some(*attr_id))
            .map(|v| *v)
            .collect_vec();
        for modifier in mods.iter() {
            for tgt_item_id in self.calc_data.tgts.get_tgt_items(ss_view, item, &modifier) {
                self.calc_force_attr_recalc(ss_view, &tgt_item_id, &modifier.tgt_attr_id);
            }
        }
        // Process buffs which rely on attribute being modified
        if ec::attrs::BUFF_ID_ATTRS.contains(attr_id) {
            // Remove modifiers of buffs which rely on the attribute
            if let Some(mods) = self.calc_data.buffs.extract_dependent_mods(item_id, attr_id) {
                let ss_mods = mods.into_iter().collect();
                self.unreg_mods(ss_view, item, &ss_mods);
            }
            // Generate new modifiers using new value and apply them
            if let Some(effect_ids) = self.calc_data.buffs.get_effects(item_id) {
                let effect_ids = effect_ids.iter().map(|v| *v).collect();
                let ss_mods = self.calc_generate_dependent_buff_mods(ss_view, item, &effect_ids, attr_id);
                for ss_mod in ss_mods.iter() {
                    self.calc_data.buffs.reg_dependent_mod(*item_id, *attr_id, *ss_mod);
                }
                self.reg_mods(ss_view, item, &ss_mods);
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
    fn reg_mods(&mut self, ss_view: &SsView, item: &SsItem, ss_mods: &Vec<SsAttrMod>) {
        // Regular modifiers
        for ss_mod in ss_mods.iter() {
            // Modifications have to be added before target attributes are cleared, because for case
            // of fleet buff ID attributes new value will be fetched instantly after cleanup, and
            // that value has to be new
            self.calc_data.mods.reg_mod(ss_view, item, *ss_mod);
            for tgt_item_id in self.calc_data.tgts.get_tgt_items(ss_view, item, ss_mod) {
                self.calc_force_attr_recalc(ss_view, &tgt_item_id, &ss_mod.tgt_attr_id);
            }
        }
        // Revisions
        for ss_mod in ss_mods.iter() {
            self.calc_data.revs.reg_mod(*ss_mod);
        }
    }
    fn unreg_mods(&mut self, ss_view: &SsView, item: &SsItem, ss_mods: &Vec<SsAttrMod>) {
        // Regular modifiers
        for ss_mod in ss_mods.iter() {
            // Modifications have to be removed before target attributes are cleared, because for
            // case of fleet buff ID attributes new value will be fetched instantly after cleanup,
            // and that value has to be new
            self.calc_data.mods.unreg_mod(ss_view, item, ss_mod);
            for tgt_item_id in self.calc_data.tgts.get_tgt_items(ss_view, item, ss_mod) {
                self.calc_force_attr_recalc(ss_view, &tgt_item_id, &ss_mod.tgt_attr_id);
            }
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
            for ss_mod in self
                .calc_data
                .mods
                .get_mods_for_changed_location_owner(item, ss_view.items)
            {
                if let Ok(src_item) = ss_view.items.get_item(&ss_mod.src_item_id) {
                    for item_id in self
                        .calc_data
                        .tgts
                        .get_tgt_items_for_fits(src_item, &ss_mod, &vec![fit])
                    {
                        self.calc_force_attr_recalc(ss_view, &item_id, &ss_mod.tgt_attr_id);
                    }
                }
            }
        }
    }
    fn process_fleet_updates(&mut self, ss_view: &SsView, fleet: &SsFleet, fit_id: &SsFitId, updates: FleetUpdates) {
        if !updates.incoming.is_empty() {
            let tgt_fits = vec![ss_view.fits.get_fit(fit_id).unwrap()];
            for ss_mod in updates.incoming.iter() {
                let src_item = ss_view.items.get_item(&ss_mod.src_item_id).unwrap();
                for tgt_item_id in self.calc_data.tgts.get_tgt_items_for_fits(src_item, ss_mod, &tgt_fits) {
                    self.calc_force_attr_recalc(ss_view, &tgt_item_id, &ss_mod.tgt_attr_id);
                }
            }
        }
        if !updates.outgoing.is_empty() {
            let tgt_fits = fleet
                .fits
                .iter()
                .filter(|v| *v != fit_id)
                .map(|v| ss_view.fits.get_fit(v).unwrap())
                .collect();
            for ss_mod in updates.outgoing.iter() {
                let src_item = ss_view.items.get_item(&ss_mod.src_item_id).unwrap();
                for tgt_item_id in self.calc_data.tgts.get_tgt_items_for_fits(src_item, ss_mod, &tgt_fits) {
                    self.calc_force_attr_recalc(ss_view, &tgt_item_id, &ss_mod.tgt_attr_id);
                }
            }
        }
    }
}
