use itertools::Itertools;

use crate::{
    ad,
    defs::{AttrVal, EAttrId, SolFitId, SolItemId},
    ec,
    sol::{
        svc::calc::{SolAttrSpec, SolCalc, SolCtxModifier, SolModifierKind, SolRawModifier},
        uad::{fleet::SolFleet, item::SolItem, SolUad},
    },
    src::Src,
};

impl SolCalc {
    // Modification methods
    pub(in crate::sol::svc) fn src_changed(&mut self, src: &Src) {
        self.rah_src_changed(src);
    }
    pub(in crate::sol::svc) fn fit_added(&mut self, fit_id: &SolFitId) {
        self.std.reg_fit_for_sw(fit_id)
    }
    pub(in crate::sol::svc) fn fit_removed(&mut self, fit_id: &SolFitId) {
        self.std.unreg_fit_for_sw(fit_id)
    }
    pub(in crate::sol::svc) fn fit_added_to_fleet(&mut self, uad: &SolUad, fleet: &SolFleet, fit_id: &SolFitId) {
        let ctx_modifiers = self.std.reg_fleet_for_fit(fleet, fit_id);
        let mut affectees = Vec::new();
        for ctx_modifier in ctx_modifiers.iter() {
            self.force_mod_affectee_attr_recalc(&mut affectees, uad, ctx_modifier);
        }
    }
    pub(in crate::sol::svc) fn fit_removed_from_fleet(&mut self, uad: &SolUad, fleet: &SolFleet, fit_id: &SolFitId) {
        let ctx_modifiers = self.std.unreg_fleet_for_fit(fleet, fit_id);
        let mut affectees = Vec::new();
        for ctx_modifier in ctx_modifiers.iter() {
            self.force_mod_affectee_attr_recalc(&mut affectees, uad, ctx_modifier);
        }
    }
    pub(in crate::sol::svc) fn fit_rah_dmg_profile_changed(&mut self, uad: &SolUad, fit_id: &SolFitId) {
        self.rah_fit_rah_dmg_profile_changed(uad, fit_id);
    }
    pub(in crate::sol::svc) fn item_added(&mut self, uad: &SolUad, item: &SolItem) {
        // Custom modifiers
        let ctx_modifiers = self.revs.get_mods_on_item_add();
        for ctx_modifier in ctx_modifiers.iter() {
            if ctx_modifier.raw.revise_on_item_add(item, uad) {
                let mut util_items = Vec::new();
                self.force_mod_affectee_attr_recalc(&mut util_items, uad, ctx_modifier);
            }
        }
    }
    pub(in crate::sol::svc) fn item_removed(&mut self, uad: &SolUad, item: &SolItem) {
        // Custom modifiers
        let ctx_modifiers = self.revs.get_mods_on_item_remove();
        for ctx_modifier in ctx_modifiers.iter() {
            if ctx_modifier.raw.revise_on_item_remove(item, uad) {
                let mut util_items = Vec::new();
                self.force_mod_affectee_attr_recalc(&mut util_items, uad, ctx_modifier);
            }
        }
    }
    pub(in crate::sol::svc) fn item_loaded(&mut self, uad: &SolUad, item: &SolItem) {
        // Notify core calc services
        self.attrs.item_loaded(item);
        self.std.reg_affectee(uad, item);
        self.handle_location_owner_change(uad, item);
        // Notify RAH sim
        self.rah_item_loaded(uad, item);
    }
    pub(in crate::sol::svc) fn item_unloaded(&mut self, uad: &SolUad, item: &SolItem) {
        // Notify RAH sim
        self.rah_item_unloaded(uad, item);
        // Notify core calc services
        self.handle_location_owner_change(uad, item);
        self.std.unreg_affectee(uad, item);
        let item_id = item.get_id();
        self.deps.remove_item(&item_id);
        self.attrs.item_unloaded(&item_id);
    }
    pub(in crate::sol::svc) fn effects_started(&mut self, uad: &SolUad, item: &SolItem, effects: &[ad::ArcEffect]) {
        // Notify core calc services
        let item_id = item.get_id();
        let mut raw_modifiers = Vec::new();
        let mut util_items = Vec::new();
        let mut util_cmods = Vec::new();
        for effect in effects.iter() {
            self.generate_mods_for_effect(&mut raw_modifiers, uad, item, effect);
            for raw_modifier in raw_modifiers.iter() {
                self.reg_raw_mod(&mut util_items, &mut util_cmods, uad, item, raw_modifier);
            }
            // Buff maintenance - add info about effects which use default buff attributes
            self.buffs.reg_effect(item_id, effect);
        }
        // Notify RAH sim
        self.rah_effects_started(uad, item, effects);
    }
    pub(in crate::sol::svc) fn effects_stopped(&mut self, uad: &SolUad, item: &SolItem, effects: &[ad::ArcEffect]) {
        // Notify RAH sim
        self.rah_effects_stopped(uad, item, effects);
        // Notify core calc services
        let item_id = item.get_id();
        let mut raw_modifiers = Vec::new();
        let mut util_items = Vec::new();
        let mut util_cmods = Vec::new();
        for effect in effects.iter() {
            self.std
                .extract_raw_mods_for_effect(&mut raw_modifiers, item_id, effect.id);
            for raw_modifier in raw_modifiers.iter() {
                self.unreg_raw_mod(&mut util_items, &mut util_cmods, uad, item, raw_modifier)
            }
            // Buff maintenance - remove info about effects which use default buff attributes
            self.buffs.unreg_effect(item.get_id(), effect);
            // Remove all ad-hoc attribute dependencies defined by effects being stopped. It is used
            // by custom propulsion module effect
            self.deps.remove_by_source(&item_id, &effect.id);
        }
    }
    pub(in crate::sol::svc) fn effect_projected(
        &mut self,
        uad: &SolUad,
        projector_item: &SolItem,
        effect: &ad::AEffect,
        projectee_item: &SolItem,
        range: Option<AttrVal>,
    ) {
        self.projs
            .add_range(projector_item.get_id(), effect.id, projectee_item.get_id(), range);
        let ctx_modifiers = self
            .std
            .project_effect(&projector_item.get_id(), &effect.id, projectee_item);
        let mut affectees = Vec::new();
        for ctx_modifier in ctx_modifiers.iter() {
            self.force_mod_affectee_attr_recalc(&mut affectees, uad, ctx_modifier);
        }
    }
    pub(in crate::sol::svc) fn effect_proj_range_changed(
        &mut self,
        uad: &SolUad,
        projector_item: &SolItem,
        effect: &ad::AEffect,
        projectee_item: &SolItem,
        range: Option<AttrVal>,
    ) {
        self.projs
            .change_range(projector_item.get_id(), effect.id, projectee_item.get_id(), range);
        let ctx_modifiers = self
            .std
            .project_effect(&projector_item.get_id(), &effect.id, projectee_item);
        let mut affectees = Vec::new();
        for ctx_modifier in ctx_modifiers.iter() {
            self.force_mod_affectee_attr_recalc(&mut affectees, uad, ctx_modifier);
        }
    }
    pub(in crate::sol::svc) fn effect_unprojected(
        &mut self,
        uad: &SolUad,
        projector_item: &SolItem,
        effect: &ad::AEffect,
        projectee_item: &SolItem,
    ) {
        let ctx_modifiers = self
            .std
            .unproject_effect(&projector_item.get_id(), &effect.id, projectee_item);
        let mut affectees = Vec::new();
        for ctx_modifier in ctx_modifiers.iter() {
            self.force_mod_affectee_attr_recalc(&mut affectees, uad, ctx_modifier);
        }
        self.projs
            .remove_range(projector_item.get_id(), effect.id, projectee_item.get_id());
    }
    pub(in crate::sol::svc) fn attr_value_changed(&mut self, uad: &SolUad, item_id: &SolItemId, attr_id: &EAttrId) {
        // Clear up attribute values which rely on passed attribute as an upper/lower limit
        let attr_specs = self
            .deps
            .get_affectee_attr_specs(item_id, attr_id)
            .map(|v| *v)
            .collect_vec();
        for attr_spec in attr_specs.iter() {
            self.force_attr_value_recalc(uad, &attr_spec.item_id, &attr_spec.attr_id);
        }
        // Clear up attribute values which rely on passed attribute as a modification source
        let ctx_modifiers = self
            .std
            .iter_affector_spec_mods(&SolAttrSpec::new(*item_id, *attr_id))
            .map(|v| *v)
            .collect_vec();
        if !ctx_modifiers.is_empty() {
            let mut affectees = Vec::new();
            for ctx_modifier in ctx_modifiers.iter() {
                self.std.fill_affectees(&mut affectees, uad, &ctx_modifier);
                for projectee_item_id in affectees.iter() {
                    self.force_attr_value_recalc(uad, projectee_item_id, &ctx_modifier.raw.affectee_attr_id);
                }
            }
        }
        // Process buffs which rely on attribute being modified
        if ec::extras::BUFF_STDATTR_IDS.contains(attr_id) {
            let item = uad.items.get_item(item_id).unwrap();
            // Remove modifiers of buffs which rely on the attribute
            if let Some(raw_modifiers) = self.buffs.extract_dependent_mods(item_id, attr_id) {
                let mut util_items = Vec::new();
                let mut util_cmods = Vec::new();
                let raw_modifiers = raw_modifiers.collect_vec();
                for raw_modifier in raw_modifiers.iter() {
                    self.unreg_raw_mod(&mut util_items, &mut util_cmods, uad, item, raw_modifier);
                }
            }
            // Generate new modifiers using new value and apply them
            let effect_ids = self.buffs.get_effects(item_id);
            if !effect_ids.is_empty() {
                let effect_ids = effect_ids.map(|v| *v).collect_vec();
                let raw_modifiers = self.generate_dependent_buff_mods(uad, item, effect_ids.iter(), attr_id);
                for raw_modifier in raw_modifiers.iter() {
                    self.buffs.reg_dependent_mod(*item_id, *attr_id, *raw_modifier);
                }
                let mut util_items = Vec::new();
                let mut util_cmods = Vec::new();
                for raw_modifier in raw_modifiers.iter() {
                    self.reg_raw_mod(&mut util_items, &mut util_cmods, uad, item, raw_modifier);
                }
            }
        }
        // Notify RAH sim
        self.rah_attr_value_changed(uad, item_id, attr_id);
    }
    pub(in crate::sol::svc) fn force_attr_value_recalc(
        &mut self,
        uad: &SolUad,
        item_id: &SolItemId,
        attr_id: &EAttrId,
    ) {
        // Sometimes calc service receives requests to clear attributes it does not know yet; this
        // can happen in multiple cases, e.g. when adding module with charge, with "other" location
        // modifier on module. User data gets references between charge and module set right away,
        // but calculator registers module before charge, and attempts to clear charge attributes.
        // Due to cases like this, we cannot just unwrap item attribute data.
        if let Ok(item_attr_data) = self.attrs.get_item_attr_data_mut(item_id) {
            // No value calculated before that - there are no dependents to clear (dependents always
            // request dependencies while calculating their values). Removing attribute forces
            // recalculation
            if item_attr_data.values.remove(attr_id).is_some() {
                self.attr_value_changed(uad, item_id, attr_id);
            }
        }
    }
    pub(in crate::sol::svc) fn force_attr_postproc_recalc(
        &mut self,
        uad: &SolUad,
        item_id: &SolItemId,
        attr_id: &EAttrId,
    ) {
        // Almost-copy of force recalc method without attribute removal. When something that
        // installed a postprocessing function thinks its output can change, it can let calc service
        // know about it via this method.
        if let Ok(item_attr_data) = self.attrs.get_item_attr_data_mut(item_id) {
            // No value calculated before that - there are no dependents to clear (dependents always
            // request dependencies while calculating their values). In this case we do not remove
            // attribute, because only postprocessing output is supposed to change
            if item_attr_data.values.contains_key(attr_id) {
                self.attr_value_changed(uad, item_id, attr_id);
            }
        }
    }
    pub(in crate::sol::svc) fn skill_level_changed(&mut self, uad: &SolUad, item_id: &SolItemId) {
        self.force_attr_postproc_recalc(uad, item_id, &ec::attrs::SKILL_LEVEL)
    }
    // Private methods
    fn reg_raw_mod(
        &mut self,
        util_items: &mut Vec<SolItemId>,
        util_cmods: &mut Vec<SolCtxModifier>,
        uad: &SolUad,
        item: &SolItem,
        raw_modifier: &SolRawModifier,
    ) {
        match raw_modifier.kind {
            SolModifierKind::Local => {
                if let Some(ctx_modifier) = self.std.reg_local_mod(item, *raw_modifier) {
                    self.force_mod_affectee_attr_recalc(util_items, uad, &ctx_modifier);
                    // Revisions - we need those only for local modifiers for now
                    self.revs.reg_mod(&ctx_modifier);
                }
            }
            SolModifierKind::FleetBuff => {
                let registered = self.std.reg_fleet_buff_mod(util_cmods, uad, item, *raw_modifier);
                for ctx_modifier in util_cmods.iter() {
                    self.force_mod_affectee_attr_recalc(util_items, uad, &ctx_modifier);
                }
                if registered {
                    self.reg_raw_mod_for_buff(item, raw_modifier);
                }
            }
            SolModifierKind::System => match item {
                SolItem::SwEffect(_) => {
                    self.std.reg_sw_system_mod(util_cmods, uad, *raw_modifier);
                    for ctx_modifier in util_cmods.iter() {
                        self.force_mod_affectee_attr_recalc(util_items, uad, &ctx_modifier);
                    }
                }
                SolItem::FwEffect(fw_effect) => {
                    if let Some(ctx_modifier) = self.std.reg_fw_system_mod(fw_effect, *raw_modifier) {
                        self.force_mod_affectee_attr_recalc(util_items, uad, &ctx_modifier);
                    }
                }
                SolItem::ProjEffect(_) => self.std.reg_proj_mod(*raw_modifier),
                _ => (),
            },
            SolModifierKind::Buff => {
                let registered = match item {
                    SolItem::SwEffect(_) => {
                        let registered = self.std.reg_sw_buff_mod(util_cmods, uad, *raw_modifier);
                        for ctx_modifier in util_cmods.iter() {
                            self.force_mod_affectee_attr_recalc(util_items, uad, &ctx_modifier);
                        }
                        registered
                    }
                    SolItem::FwEffect(fw_effect) => {
                        let registered = self.std.reg_fw_buff_mod(util_cmods, uad, fw_effect, *raw_modifier);
                        for ctx_modifier in util_cmods.iter() {
                            self.force_mod_affectee_attr_recalc(util_items, uad, &ctx_modifier);
                        }
                        registered
                    }
                    _ => {
                        self.std.reg_proj_mod(*raw_modifier);
                        true
                    }
                };
                if registered {
                    self.reg_raw_mod_for_buff(item, raw_modifier);
                }
            }
            SolModifierKind::Targeted => self.std.reg_proj_mod(*raw_modifier),
        }
    }
    fn unreg_raw_mod(
        &mut self,
        util_items: &mut Vec<SolItemId>,
        util_cmods: &mut Vec<SolCtxModifier>,
        uad: &SolUad,
        item: &SolItem,
        raw_modifier: &SolRawModifier,
    ) {
        // Regular modifiers
        match raw_modifier.kind {
            SolModifierKind::Local => {
                if let Some(ctx_modifier) = self.std.unreg_local_mod(item, *raw_modifier) {
                    self.force_mod_affectee_attr_recalc(util_items, uad, &ctx_modifier);
                    // Revisions - we need those only for local modifiers for now
                    self.revs.unreg_mod(&ctx_modifier);
                }
            }
            SolModifierKind::FleetBuff => {
                self.std.unreg_fleet_buff_mod(util_cmods, uad, item, *raw_modifier);
                for ctx_modifier in util_cmods.iter() {
                    self.force_mod_affectee_attr_recalc(util_items, uad, &ctx_modifier);
                }
                self.unreg_raw_mod_for_buff(item, raw_modifier);
            }
            SolModifierKind::System => match item {
                SolItem::SwEffect(_) => {
                    self.std.unreg_sw_system_mod(util_cmods, uad, *raw_modifier);
                    for ctx_modifier in util_cmods.iter() {
                        self.force_mod_affectee_attr_recalc(util_items, uad, &ctx_modifier);
                    }
                }
                SolItem::FwEffect(fw_effect) => {
                    if let Some(ctx_modifier) = self.std.unreg_fw_system_mod(fw_effect, *raw_modifier) {
                        self.force_mod_affectee_attr_recalc(util_items, uad, &ctx_modifier);
                    }
                }
                // Don't need to do anything in this case, since projected effects were
                // removed during extraction earlier
                _ => (),
            },
            SolModifierKind::Buff => {
                match item {
                    SolItem::SwEffect(_) => {
                        self.std.unreg_sw_buff_mod(util_cmods, uad, raw_modifier);
                        for ctx_modifier in util_cmods.iter() {
                            self.force_mod_affectee_attr_recalc(util_items, uad, &ctx_modifier);
                        }
                    }
                    SolItem::FwEffect(fw_effect) => {
                        self.std.unreg_fw_buff_mod(util_cmods, uad, fw_effect, *raw_modifier);
                        for ctx_modifier in util_cmods.iter() {
                            self.force_mod_affectee_attr_recalc(util_items, uad, &ctx_modifier);
                        }
                    }
                    // Don't need to do anything in this case, since projected effects were
                    // removed during extraction earlier
                    SolItem::ProjEffect(_) => (),
                    _ => (),
                }
                self.unreg_raw_mod_for_buff(item, raw_modifier);
            }
            // Don't need to do anything in this case, since projected effects were
            // removed during extraction earlier
            SolModifierKind::Targeted => (),
        }
    }
    fn reg_raw_mod_for_buff(&mut self, item: &SolItem, raw_modifier: &SolRawModifier) {
        if let Some(buff_type_attr_id) = raw_modifier.buff_type_attr_id {
            self.buffs
                .reg_dependent_mod(item.get_id(), buff_type_attr_id, *raw_modifier);
        }
    }
    fn unreg_raw_mod_for_buff(&mut self, item: &SolItem, raw_modifier: &SolRawModifier) {
        if let Some(buff_type_attr_id) = raw_modifier.buff_type_attr_id {
            self.buffs
                .unreg_dependent_mod(&item.get_id(), &buff_type_attr_id, raw_modifier);
        }
    }
    fn force_mod_affectee_attr_recalc(
        &mut self,
        affectees: &mut Vec<SolItemId>,
        uad: &SolUad,
        modifier: &SolCtxModifier,
    ) {
        self.std.fill_affectees(affectees, uad, modifier);
        for projectee_item_id in affectees.iter() {
            self.force_attr_value_recalc(uad, projectee_item_id, &modifier.raw.affectee_attr_id);
        }
    }
    fn handle_location_owner_change(&mut self, uad: &SolUad, item: &SolItem) {
        if item.get_root_loc_kind().is_some() {
            let mut affectees = Vec::new();
            for ctx_modifier in self.std.get_mods_for_changed_root(item) {
                self.force_mod_affectee_attr_recalc(&mut affectees, uad, &ctx_modifier)
            }
        }
    }
}
