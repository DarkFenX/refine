use itertools::Itertools;

use crate::{
    ad,
    defs::{EAttrId, SsFitId, SsItemId},
    ss::{item::SsItem, svc::SsSvcs, SsView},
};

use super::misc::a_data_to_ss_mods;

impl SsSvcs {
    // Modification methods
    pub(in crate::ss::svc) fn calc_fit_added(&mut self, fit_id: &SsFitId) {}
    pub(in crate::ss::svc) fn calc_fit_removed(&mut self, fit_id: &SsFitId) {}
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
        let mods = a_data_to_ss_mods(item, effects);
        for ss_mod in mods.iter() {
            self.calc_data.mods.reg_mod(ss_view, item, *ss_mod);
            for tgt_item_id in self.calc_data.tgts.get_tgt_items(ss_view, item, ss_mod) {
                self.calc_force_attr_recalc(ss_view, &tgt_item_id, &ss_mod.tgt_attr_id);
            }
        }
        // Revisions
        for ss_mod in mods.iter() {
            self.calc_data.revs.reg_mod(*ss_mod);
        }
    }
    pub(in crate::ss::svc) fn calc_effects_stopped(
        &mut self,
        ss_view: &SsView,
        item: &SsItem,
        effects: &Vec<ad::ArcEffect>,
    ) {
        let mods = a_data_to_ss_mods(item, effects);
        for ss_mod in mods.iter() {
            for tgt_item_id in self.calc_data.tgts.get_tgt_items(ss_view, item, ss_mod) {
                self.calc_force_attr_recalc(ss_view, &tgt_item_id, &ss_mod.tgt_attr_id);
            }
            self.calc_data.mods.unreg_mod(ss_view, item, ss_mod);
        }
        // Revisions and effect-specific processing
        for ss_mod in mods.iter() {
            self.calc_data.revs.unreg_mod(ss_mod);
            // This bit is just for propulsion mode effect, so that when effect is not running (but
            // item is not removed), changes to parent attributes like ship mass do not clear the
            // child attribute - ship speed
            ss_mod.on_effect_stop(self, ss_view);
        }
    }
    pub(in crate::ss::svc) fn calc_attr_value_changed(
        &mut self,
        ss_view: &SsView,
        item_id: &SsItemId,
        attr_id: &EAttrId,
    ) {
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
        let mods = self
            .calc_data
            .mods
            .iter_mods_for_src(item_id)
            .filter(|v| v.get_src_attr_id() == Some(*attr_id))
            .map(|v| *v)
            .collect_vec();
        let item = ss_view.items.get_item(item_id).unwrap();
        for modifier in mods.iter() {
            for tgt_item_id in self.calc_data.tgts.get_tgt_items(ss_view, item, &modifier) {
                self.calc_force_attr_recalc(ss_view, &tgt_item_id, &modifier.tgt_attr_id);
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
    fn handle_location_owner_change(&mut self, ss_view: &SsView, item: &SsItem) {
        if item.get_root_loc_type().is_some() {
            for ss_mod in self
                .calc_data
                .mods
                .get_mods_for_changed_location_owner(item, ss_view.items)
            {
                if let Ok(src_item) = ss_view.items.get_item(&ss_mod.src_item_id) {
                    for item_id in self.calc_data.tgts.get_tgt_items(ss_view, src_item, &ss_mod) {
                        self.calc_force_attr_recalc(ss_view, &item_id, &ss_mod.tgt_attr_id);
                    }
                }
            }
        }
    }
}
