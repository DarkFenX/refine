use itertools::Itertools;

use crate::{
    ad,
    defs::{EAttrId, SolFitId, SolItemId},
    ec,
    sol::{
        fleet::SolFleet,
        item::SolItem,
        svc::{
            svce_calc::{get_proj_effect_resist_attr_id, SolFleetUpdates, SolModifier},
            SolSvcs,
        },
        SolView,
    },
    AttrVal,
};

impl SolSvcs {
    // Modification methods
    pub(in crate::sol::svc) fn calc_fit_added(&mut self, fit_id: &SolFitId) {
        self.calc_data.mods.reg_fit(fit_id)
    }
    pub(in crate::sol::svc) fn calc_fit_removed(&mut self, fit_id: &SolFitId) {
        self.calc_data.mods.unreg_fit(fit_id)
    }
    pub(in crate::sol::svc) fn calc_fit_added_to_fleet(
        &mut self,
        sol_view: &SolView,
        fleet: &SolFleet,
        fit_id: &SolFitId,
    ) {
        let updates = self.calc_data.mods.reg_fleet_for_fit(fleet, fit_id);
        self.process_fleet_updates(sol_view, fleet, fit_id, updates);
    }
    pub(in crate::sol::svc) fn calc_fit_removed_from_fleet(
        &mut self,
        sol_view: &SolView,
        fleet: &SolFleet,
        fit_id: &SolFitId,
    ) {
        let updates = self.calc_data.mods.unreg_fleet_for_fit(fleet, fit_id);
        self.process_fleet_updates(sol_view, fleet, fit_id, updates);
    }
    pub(in crate::sol::svc) fn calc_item_added(&mut self, sol_view: &SolView, item: &SolItem) {
        self.handle_location_owner_change(sol_view, item);
        // Custom modifiers
        let modifiers = self.calc_data.revs.get_mods_on_item_add();
        for modifier in modifiers.iter() {
            if modifier.revise_on_item_add(item, sol_view) {
                self.revise_modifier(sol_view, modifier);
            }
        }
    }
    pub(in crate::sol::svc) fn calc_item_removed(&mut self, sol_view: &SolView, item: &SolItem) {
        self.handle_location_owner_change(sol_view, item);
        // Custom modifiers
        let modifiers = self.calc_data.revs.get_mods_on_item_remove();
        for modifier in modifiers.iter() {
            if modifier.revise_on_item_remove(item, sol_view) {
                self.revise_modifier(sol_view, modifier);
            }
        }
    }
    pub(in crate::sol::svc) fn calc_item_loaded(&mut self, sol_view: &SolView, item: &SolItem) {
        self.calc_data.attrs.add_item(item.get_id());
        self.calc_data.afee.reg_affectee(sol_view, item);
    }
    pub(in crate::sol::svc) fn calc_item_unloaded(&mut self, sol_view: &SolView, item: &SolItem) {
        self.calc_data.afee.unreg_affectee(sol_view, item);
        let item_id = item.get_id();
        self.calc_data.attrs.remove_item(&item_id);
        self.calc_data.deps.remove_item(&item_id);
    }
    pub(in crate::sol::svc) fn calc_effects_started(
        &mut self,
        sol_view: &SolView,
        item: &SolItem,
        effects: &Vec<ad::ArcEffect>,
    ) {
        // Register new mods
        let modifiers = self.calc_generate_mods_for_effects(sol_view, item, effects);
        self.reg_mods(sol_view, item, &modifiers.all);
        // Buff maintenance - add info about effects/modifiers which use default buff attributes
        for effect in effects.iter() {
            self.calc_data.buffs.reg_effect(item.get_id(), effect);
        }
        for (buff_type_attr_id, dependent_mods) in modifiers.dependent_buffs.iter() {
            for dependent_mod in dependent_mods {
                self.calc_data
                    .buffs
                    .reg_dependent_mod(item.get_id(), *buff_type_attr_id, *dependent_mod);
            }
        }
    }
    pub(in crate::sol::svc) fn calc_effects_stopped(
        &mut self,
        sol_view: &SolView,
        item: &SolItem,
        effects: &Vec<ad::ArcEffect>,
    ) {
        // Unregister mods
        let modifiers = self.calc_generate_mods_for_effects(sol_view, item, effects);
        self.unreg_mods(sol_view, item, &modifiers.all);
        // Remove all ad-hoc attribute dependencies defined by effects being stopped. It is used by
        // various projection-related features (resistance to modification, projection range), and
        // custom propulsion module effect
        let item_id = item.get_id();
        for effect in effects.iter() {
            self.calc_data.deps.remove_by_source(&item_id, &effect.id);
        }
        // Buff maintenance - remove info about effects/modifiers which use default buff attributes
        for effect in effects.iter() {
            self.calc_data.buffs.unreg_effect(item.get_id(), effect);
        }
        for (buff_type_attr_id, dependent_mods) in modifiers.dependent_buffs.iter() {
            for dependent_mod in dependent_mods {
                self.calc_data
                    .buffs
                    .unreg_dependent_mod(&item.get_id(), buff_type_attr_id, dependent_mod);
            }
        }
    }
    pub(in crate::sol::svc) fn calc_effect_projected(
        &mut self,
        sol_view: &SolView,
        projector_item: &SolItem,
        effect: &ad::AEffect,
        projectee_item: &SolItem,
        range: Option<AttrVal>,
    ) {
        self.calc_data
            .projs
            .add_range(projector_item.get_id(), effect.id, projectee_item.get_id(), range);
        let projector_item_id = projector_item.get_id();
        let modifiers = self
            .calc_data
            .mods
            .iter_affector_item_mods(&projector_item_id)
            .filter(|v| v.effect_id == effect.id)
            .map(|v| *v)
            .collect_vec();
        if !modifiers.is_empty() {
            let mut affectee_item_ids = Vec::new();
            for modifier in modifiers.iter() {
                if self.calc_data.mods.add_mod_tgt(*modifier, projectee_item) {
                    affectee_item_ids.clear();
                    self.calc_data.afee.fill_affectees_for_tgt_item(
                        &mut affectee_item_ids,
                        sol_view,
                        modifier,
                        &projectee_item,
                    );
                    for affectee_item_id in affectee_item_ids.iter() {
                        self.calc_force_attr_recalc(sol_view, affectee_item_id, &modifier.affectee_attr_id);
                    }
                }
            }
        }
    }
    pub(in crate::sol::svc) fn calc_effect_tgt_removed(
        &mut self,
        sol_view: &SolView,
        item: &SolItem,
        effect: &ad::AEffect,
        tgt_item: &SolItem,
    ) {
        let item_id = item.get_id();
        let modifiers = self
            .calc_data
            .mods
            .iter_affector_item_mods(&item_id)
            .filter(|v| v.effect_id == effect.id)
            .map(|v| *v)
            .collect_vec();
        if !modifiers.is_empty() {
            let tgt_item_id = tgt_item.get_id();
            let resist_attr_id = get_proj_effect_resist_attr_id(item, effect);
            let optimal_attr_id = effect.range_attr_id;
            let falloff_attr_id = effect.falloff_attr_id;
            let mut affectee_item_ids = Vec::new();
            for modifier in modifiers.iter() {
                affectee_item_ids.clear();
                self.calc_data
                    .afee
                    .fill_affectees_for_tgt_item(&mut affectee_item_ids, sol_view, modifier, &tgt_item);
                for affectee_item_id in affectee_item_ids.iter() {
                    self.calc_force_attr_recalc(sol_view, &affectee_item_id, &modifier.affectee_attr_id);
                    // Remove dependencies which could've been added by the effect being unprojected
                    if let Some(resist_attr_id) = resist_attr_id {
                        self.calc_data.deps.remove_with_source(
                            &item_id,
                            &effect.id,
                            &tgt_item_id,
                            &resist_attr_id,
                            affectee_item_id,
                            &modifier.affectee_attr_id,
                        );
                    }
                    if let Some(optimal_attr_id) = optimal_attr_id {
                        self.calc_data.deps.remove_with_source(
                            &item_id,
                            &effect.id,
                            &item_id,
                            &optimal_attr_id,
                            affectee_item_id,
                            &modifier.affectee_attr_id,
                        );
                    }
                    if let Some(falloff_attr_id) = falloff_attr_id {
                        self.calc_data.deps.remove_with_source(
                            &item_id,
                            &effect.id,
                            &item_id,
                            &falloff_attr_id,
                            affectee_item_id,
                            &modifier.affectee_attr_id,
                        );
                    }
                }
                self.calc_data.mods.rm_mod_tgt(modifier, tgt_item);
            }
        }
        self.calc_data
            .projs
            .remove_range(item.get_id(), effect.id, tgt_item.get_id());
    }
    pub(in crate::sol::svc) fn calc_attr_value_changed(
        &mut self,
        sol_view: &SolView,
        item_id: &SolItemId,
        attr_id: &EAttrId,
    ) {
        let item = sol_view.items.get_item(item_id).unwrap();
        // Clear up attribute values which rely on passed attribute as an upper cap
        let attr_specs = self
            .calc_data
            .deps
            .get_affectee_attr_specs(item_id, attr_id)
            .map(|v| *v)
            .collect_vec();
        for attr_spec in attr_specs.iter() {
            self.calc_force_attr_recalc(sol_view, &attr_spec.item_id, &attr_spec.attr_id);
        }
        // Clear up attribute values which rely on passed attribute as a modification source
        let mods = self
            .calc_data
            .mods
            .iter_affector_item_mods(item_id)
            .filter(|v| v.get_affector_attr_id() == Some(*attr_id))
            .map(|v| *v)
            .collect_vec();
        if !mods.is_empty() {
            let mut affectees = Vec::new();
            for modifier in mods.iter() {
                affectees.clear();
                self.calc_data
                    .afee
                    .fill_affectees(&mut affectees, sol_view, item, &modifier);
                for tgt_item_id in affectees.iter() {
                    self.calc_force_attr_recalc(sol_view, tgt_item_id, &modifier.affectee_attr_id);
                }
            }
        }
        // Process buffs which rely on attribute being modified
        if ec::extras::BUFF_STDATTR_IDS.contains(attr_id) {
            // Remove modifiers of buffs which rely on the attribute
            if let Some(modifiers) = self.calc_data.buffs.extract_dependent_mods(item_id, attr_id) {
                let modifiers = modifiers.collect();
                self.unreg_mods(sol_view, item, &modifiers);
            }
            // Generate new modifiers using new value and apply them
            let effect_ids = self.calc_data.buffs.get_effects(item_id);
            if !effect_ids.is_empty() {
                let effect_ids = effect_ids.map(|v| *v).collect_vec();
                let modifiers = self.calc_generate_dependent_buff_mods(sol_view, item, effect_ids.iter(), attr_id);
                for modifier in modifiers.iter() {
                    self.calc_data.buffs.reg_dependent_mod(*item_id, *attr_id, *modifier);
                }
                self.reg_mods(sol_view, item, &modifiers);
            }
        }
    }
    pub(in crate::sol) fn calc_force_attr_recalc(
        &mut self,
        sol_view: &SolView,
        item_id: &SolItemId,
        attr_id: &EAttrId,
    ) {
        match self.calc_data.attrs.get_item_attrs_mut(item_id) {
            Ok(item_attrs) => {
                if item_attrs.remove(attr_id).is_some() {
                    self.notify_attr_val_changed(sol_view, item_id, attr_id);
                }
            }
            _ => return,
        }
    }
    // Private methods
    fn reg_mods(&mut self, sol_view: &SolView, item: &SolItem, modifiers: &Vec<SolModifier>) {
        if modifiers.is_empty() {
            return;
        }
        // Regular modifiers
        let mut fit_ids = Vec::new();
        let mut affectees = Vec::new();
        for modifier in modifiers.iter() {
            // Modifications have to be added before target attributes are cleared, because for case
            // of fleet buff ID attributes new value will be fetched instantly after cleanup, and
            // that value has to be new
            if self.calc_data.mods.reg_mod(&mut fit_ids, sol_view, item, *modifier) {
                affectees.clear();
                self.calc_data
                    .afee
                    .fill_affectees(&mut affectees, sol_view, item, modifier);
                for tgt_item_id in affectees.iter() {
                    self.calc_force_attr_recalc(sol_view, tgt_item_id, &modifier.affectee_attr_id);
                }
            }
        }
        // Revisions
        for modifier in modifiers.iter() {
            self.calc_data.revs.reg_mod(*modifier);
        }
    }
    fn unreg_mods(&mut self, sol_view: &SolView, item: &SolItem, modifiers: &Vec<SolModifier>) {
        if modifiers.is_empty() {
            return;
        }
        // Regular modifiers
        let mut fit_ids = Vec::new();
        let mut affectees = Vec::new();
        for modifier in modifiers.iter() {
            // Modifications have to be removed before target attributes are cleared, because for
            // case of fleet buff ID attributes new value will be fetched instantly after cleanup,
            // and that value has to be new
            if self.calc_data.mods.unreg_mod(&mut fit_ids, sol_view, item, modifier) {
                affectees.clear();
                self.calc_data
                    .afee
                    .fill_affectees(&mut affectees, sol_view, item, modifier);
                for tgt_item_id in affectees.iter() {
                    self.calc_force_attr_recalc(sol_view, tgt_item_id, &modifier.affectee_attr_id);
                }
            }
        }
        // Revisions
        for modifier in modifiers.iter() {
            self.calc_data.revs.unreg_mod(modifier);
        }
    }
    fn handle_location_owner_change(&mut self, sol_view: &SolView, item: &SolItem) {
        if item.get_root_loc_kind().is_some() {
            let fit_id = match item.get_fit_id() {
                Some(fit_id) => fit_id,
                _ => return,
            };
            let fit = match sol_view.fits.get_fit(&fit_id) {
                Ok(fit) => fit,
                _ => return,
            };
            let mut affectees = Vec::new();
            for modifier in self.calc_data.mods.get_mods_for_changed_root(sol_view, item) {
                affectees.clear();
                self.calc_data
                    .afee
                    .fill_affectees_for_fit(&mut affectees, &modifier, fit);
                for item_id in affectees.iter() {
                    self.calc_force_attr_recalc(sol_view, item_id, &modifier.affectee_attr_id);
                }
            }
        }
    }
    fn process_fleet_updates(
        &mut self,
        sol_view: &SolView,
        fleet: &SolFleet,
        fit_id: &SolFitId,
        updates: SolFleetUpdates,
    ) {
        let mut affectees = Vec::new();
        if !updates.incoming.is_empty() {
            let tgt_fit = sol_view.fits.get_fit(fit_id).unwrap();
            for modifier in updates.incoming.iter() {
                affectees.clear();
                self.calc_data
                    .afee
                    .fill_affectees_for_fit(&mut affectees, modifier, tgt_fit);
                for tgt_item_id in affectees.iter() {
                    self.calc_force_attr_recalc(sol_view, &tgt_item_id, &modifier.affectee_attr_id);
                }
            }
        }
        if !updates.outgoing.is_empty() {
            for tgt_fit in fleet
                .iter_fits()
                .filter(|v| *v != fit_id)
                .map(|v| sol_view.fits.get_fit(v).unwrap())
            {
                for modifier in updates.outgoing.iter() {
                    affectees.clear();
                    self.calc_data
                        .afee
                        .fill_affectees_for_fit(&mut affectees, modifier, tgt_fit);
                    for tgt_item_id in affectees.iter() {
                        self.calc_force_attr_recalc(sol_view, &tgt_item_id, &modifier.affectee_attr_id);
                    }
                }
            }
        }
    }
    fn revise_modifier(&mut self, sol_view: &SolView, modifier: &SolModifier) {
        let affector_item = sol_view.items.get_item(&modifier.affector_item_id).unwrap();
        let mut affectees = Vec::new();
        self.calc_data
            .afee
            .fill_affectees(&mut affectees, sol_view, affector_item, modifier);
        for tgt_item_id in affectees.iter() {
            self.calc_force_attr_recalc(sol_view, tgt_item_id, &modifier.affectee_attr_id);
        }
    }
}
