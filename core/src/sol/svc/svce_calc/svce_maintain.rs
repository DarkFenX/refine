use itertools::Itertools;

use crate::{
    ad,
    defs::{EAttrId, SolFitId, SolItemId},
    ec,
    sol::{
        fleet::SolFleet,
        item::SolItem,
        svc::{
            svce_calc::{SolAttrSpec, SolCtxModifier, SolModifierKind, SolRawModifier},
            SolSvcs,
        },
        SolView,
    },
    AttrVal,
};

impl SolSvcs {
    // Modification methods
    pub(in crate::sol::svc) fn calc_fit_added(&mut self, fit_id: &SolFitId) {
        self.calc_data.std.reg_fit_for_sw(fit_id)
    }
    pub(in crate::sol::svc) fn calc_fit_removed(&mut self, fit_id: &SolFitId) {
        self.calc_data.std.unreg_fit_for_sw(fit_id)
    }
    pub(in crate::sol::svc) fn calc_fit_added_to_fleet(
        &mut self,
        sol_view: &SolView,
        fleet: &SolFleet,
        fit_id: &SolFitId,
    ) {
        let ctx_modifiers = self.calc_data.std.reg_fleet_for_fit(fleet, fit_id);
        let mut affectees = Vec::new();
        for ctx_modifier in ctx_modifiers.iter() {
            self.force_mod_affectee_attr_recalc(&mut affectees, sol_view, ctx_modifier);
        }
    }
    pub(in crate::sol::svc) fn calc_fit_removed_from_fleet(
        &mut self,
        sol_view: &SolView,
        fleet: &SolFleet,
        fit_id: &SolFitId,
    ) {
        let ctx_modifiers = self.calc_data.std.unreg_fleet_for_fit(fleet, fit_id);
        let mut affectees = Vec::new();
        for ctx_modifier in ctx_modifiers.iter() {
            self.force_mod_affectee_attr_recalc(&mut affectees, sol_view, ctx_modifier);
        }
    }
    pub(in crate::sol::svc) fn calc_item_added(&mut self, sol_view: &SolView, item: &SolItem) {
        // Custom modifiers
        let ctx_modifiers = self.calc_data.revs.get_mods_on_item_add();
        for ctx_modifier in ctx_modifiers.iter() {
            if ctx_modifier.raw.revise_on_item_add(item, sol_view) {
                let mut util_items = Vec::new();
                self.force_mod_affectee_attr_recalc(&mut util_items, sol_view, ctx_modifier);
            }
        }
    }
    pub(in crate::sol::svc) fn calc_item_removed(&mut self, sol_view: &SolView, item: &SolItem) {
        // Custom modifiers
        let ctx_modifiers = self.calc_data.revs.get_mods_on_item_remove();
        for ctx_modifier in ctx_modifiers.iter() {
            if ctx_modifier.raw.revise_on_item_remove(item, sol_view) {
                let mut util_items = Vec::new();
                self.force_mod_affectee_attr_recalc(&mut util_items, sol_view, ctx_modifier);
            }
        }
    }
    pub(in crate::sol::svc) fn calc_item_loaded(&mut self, sol_view: &SolView, item: &SolItem) {
        self.calc_data.attrs.item_loaded(item);
        self.calc_data.std.reg_affectee(sol_view, item);
        self.handle_location_owner_change(sol_view, item);
    }
    pub(in crate::sol::svc) fn calc_item_unloaded(&mut self, sol_view: &SolView, item: &SolItem) {
        self.handle_location_owner_change(sol_view, item);
        self.calc_data.std.unreg_affectee(sol_view, item);
        let item_id = item.get_id();
        self.calc_data.deps.remove_item(&item_id);
        self.calc_data.attrs.item_unloaded(&item_id);
    }
    pub(in crate::sol::svc) fn calc_effects_started(
        &mut self,
        sol_view: &SolView,
        item: &SolItem,
        effects: &Vec<ad::ArcEffect>,
    ) {
        let item_id = item.get_id();
        let mut raw_modifiers = Vec::new();
        let mut util_items = Vec::new();
        let mut util_cmods = Vec::new();
        for effect in effects.iter() {
            self.calc_generate_mods_for_effect(&mut raw_modifiers, sol_view, item, effect);
            for raw_modifier in raw_modifiers.iter() {
                self.reg_raw_mod(&mut util_items, &mut util_cmods, sol_view, item, raw_modifier);
            }
            // Buff maintenance - add info about effects which use default buff attributes
            self.calc_data.buffs.reg_effect(item_id, effect);
        }
        self.calc_rah_effects_started(sol_view, item, effects);
    }
    pub(in crate::sol::svc) fn calc_effects_stopped(
        &mut self,
        sol_view: &SolView,
        item: &SolItem,
        effects: &Vec<ad::ArcEffect>,
    ) {
        self.calc_rah_effects_stopped(sol_view, item, effects);
        let item_id = item.get_id();
        let mut raw_modifiers = Vec::new();
        let mut util_items = Vec::new();
        let mut util_cmods = Vec::new();
        for effect in effects.iter() {
            self.calc_data
                .std
                .extract_raw_mods_for_effect(&mut raw_modifiers, item_id, effect.id);
            for raw_modifier in raw_modifiers.iter() {
                self.unreg_raw_mod(&mut util_items, &mut util_cmods, sol_view, item, raw_modifier)
            }
            // Buff maintenance - remove info about effects which use default buff attributes
            self.calc_data.buffs.unreg_effect(item.get_id(), effect);
            // Remove all ad-hoc attribute dependencies defined by effects being stopped. It is used
            // by custom propulsion module effect
            self.calc_data.deps.remove_by_source(&item_id, &effect.id);
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
        let ctx_modifiers = self
            .calc_data
            .std
            .project_effect(&projector_item.get_id(), &effect.id, projectee_item);
        let mut affectees = Vec::new();
        for ctx_modifier in ctx_modifiers.iter() {
            self.force_mod_affectee_attr_recalc(&mut affectees, sol_view, ctx_modifier);
        }
    }
    pub(in crate::sol::svc) fn calc_effect_proj_range_changed(
        &mut self,
        sol_view: &SolView,
        projector_item: &SolItem,
        effect: &ad::AEffect,
        projectee_item: &SolItem,
        range: Option<AttrVal>,
    ) {
        self.calc_data
            .projs
            .change_range(projector_item.get_id(), effect.id, projectee_item.get_id(), range);
        let ctx_modifiers = self
            .calc_data
            .std
            .project_effect(&projector_item.get_id(), &effect.id, projectee_item);
        let mut affectees = Vec::new();
        for ctx_modifier in ctx_modifiers.iter() {
            self.force_mod_affectee_attr_recalc(&mut affectees, sol_view, ctx_modifier);
        }
    }
    pub(in crate::sol::svc) fn calc_effect_unprojected(
        &mut self,
        sol_view: &SolView,
        projector_item: &SolItem,
        effect: &ad::AEffect,
        projectee_item: &SolItem,
    ) {
        let ctx_modifiers = self
            .calc_data
            .std
            .unproject_effect(&projector_item.get_id(), &effect.id, projectee_item);
        let mut affectees = Vec::new();
        for ctx_modifier in ctx_modifiers.iter() {
            self.force_mod_affectee_attr_recalc(&mut affectees, sol_view, ctx_modifier);
        }
        self.calc_data
            .projs
            .remove_range(projector_item.get_id(), effect.id, projectee_item.get_id());
    }
    pub(in crate::sol::svc) fn calc_attr_value_changed(
        &mut self,
        sol_view: &SolView,
        item_id: &SolItemId,
        attr_id: &EAttrId,
    ) {
        let item = sol_view.items.get_item(item_id).unwrap();
        // Clear up attribute values which rely on passed attribute as an upper/lower limit
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
        let ctx_modifiers = self
            .calc_data
            .std
            .iter_affector_spec_mods(&SolAttrSpec::new(*item_id, *attr_id))
            .map(|v| *v)
            .collect_vec();
        if !ctx_modifiers.is_empty() {
            let mut affectees = Vec::new();
            for ctx_modifier in ctx_modifiers.iter() {
                self.calc_data
                    .std
                    .fill_affectees(&mut affectees, sol_view, &ctx_modifier);
                for projectee_item_id in affectees.iter() {
                    self.calc_force_attr_recalc(sol_view, projectee_item_id, &ctx_modifier.raw.affectee_attr_id);
                }
            }
        }
        // Process buffs which rely on attribute being modified
        if ec::extras::BUFF_STDATTR_IDS.contains(attr_id) {
            // Remove modifiers of buffs which rely on the attribute
            if let Some(raw_modifiers) = self.calc_data.buffs.extract_dependent_mods(item_id, attr_id) {
                let mut util_items = Vec::new();
                let mut util_cmods = Vec::new();
                let raw_modifiers = raw_modifiers.collect_vec();
                for raw_modifier in raw_modifiers.iter() {
                    self.unreg_raw_mod(&mut util_items, &mut util_cmods, sol_view, item, raw_modifier);
                }
            }
            // Generate new modifiers using new value and apply them
            let effect_ids = self.calc_data.buffs.get_effects(item_id);
            if !effect_ids.is_empty() {
                let effect_ids = effect_ids.map(|v| *v).collect_vec();
                let raw_modifiers = self.calc_generate_dependent_buff_mods(sol_view, item, effect_ids.iter(), attr_id);
                for raw_modifier in raw_modifiers.iter() {
                    self.calc_data
                        .buffs
                        .reg_dependent_mod(*item_id, *attr_id, *raw_modifier);
                }
                let mut util_items = Vec::new();
                let mut util_cmods = Vec::new();
                for raw_modifier in raw_modifiers.iter() {
                    self.reg_raw_mod(&mut util_items, &mut util_cmods, sol_view, item, raw_modifier);
                }
            }
        }
    }
    pub(in crate::sol::svc) fn calc_force_attr_recalc(
        &mut self,
        sol_view: &SolView,
        item_id: &SolItemId,
        attr_id: &EAttrId,
    ) {
        if let Ok(item_attr_data) = self.calc_data.attrs.get_item_attr_data_mut(item_id) {
            if item_attr_data.values.remove(attr_id).is_some() {
                self.notify_attr_val_changed(sol_view, item_id, attr_id);
            }
        }
    }
    // Private methods
    fn reg_raw_mod(
        &mut self,
        util_items: &mut Vec<SolItemId>,
        util_cmods: &mut Vec<SolCtxModifier>,
        sol_view: &SolView,
        item: &SolItem,
        raw_modifier: &SolRawModifier,
    ) {
        match raw_modifier.kind {
            SolModifierKind::Local => {
                if let Some(ctx_modifier) = self.calc_data.std.reg_local_mod(item, *raw_modifier) {
                    self.force_mod_affectee_attr_recalc(util_items, sol_view, &ctx_modifier);
                    // Revisions - we need those only for local modifiers for now
                    self.calc_data.revs.reg_mod(&ctx_modifier);
                }
            }
            SolModifierKind::FleetBuff => {
                let registered = self
                    .calc_data
                    .std
                    .reg_fleet_buff_mod(util_cmods, sol_view, item, *raw_modifier);
                for ctx_modifier in util_cmods.iter() {
                    self.force_mod_affectee_attr_recalc(util_items, sol_view, &ctx_modifier);
                }
                if registered {
                    self.reg_raw_mod_for_buff(item, raw_modifier);
                }
            }
            SolModifierKind::System => match item {
                SolItem::SwEffect(_) => {
                    self.calc_data
                        .std
                        .reg_sw_system_mod(util_cmods, sol_view, *raw_modifier);
                    for ctx_modifier in util_cmods.iter() {
                        self.force_mod_affectee_attr_recalc(util_items, sol_view, &ctx_modifier);
                    }
                }
                SolItem::FwEffect(fw_effect) => {
                    if let Some(ctx_modifier) = self.calc_data.std.reg_fw_system_mod(fw_effect, *raw_modifier) {
                        self.force_mod_affectee_attr_recalc(util_items, sol_view, &ctx_modifier);
                    }
                }
                SolItem::ProjEffect(_) => self.calc_data.std.reg_proj_mod(*raw_modifier),
                _ => (),
            },
            SolModifierKind::Buff => {
                let registered = match item {
                    SolItem::SwEffect(_) => {
                        let registered = self.calc_data.std.reg_sw_buff_mod(util_cmods, sol_view, *raw_modifier);
                        for ctx_modifier in util_cmods.iter() {
                            self.force_mod_affectee_attr_recalc(util_items, sol_view, &ctx_modifier);
                        }
                        registered
                    }
                    SolItem::FwEffect(fw_effect) => {
                        let registered =
                            self.calc_data
                                .std
                                .reg_fw_buff_mod(util_cmods, sol_view, fw_effect, *raw_modifier);
                        for ctx_modifier in util_cmods.iter() {
                            self.force_mod_affectee_attr_recalc(util_items, sol_view, &ctx_modifier);
                        }
                        registered
                    }
                    _ => {
                        self.calc_data.std.reg_proj_mod(*raw_modifier);
                        true
                    }
                };
                if registered {
                    self.reg_raw_mod_for_buff(item, raw_modifier);
                }
            }
            SolModifierKind::Targeted => self.calc_data.std.reg_proj_mod(*raw_modifier),
        }
    }
    fn unreg_raw_mod(
        &mut self,
        util_items: &mut Vec<SolItemId>,
        util_cmods: &mut Vec<SolCtxModifier>,
        sol_view: &SolView,
        item: &SolItem,
        raw_modifier: &SolRawModifier,
    ) {
        // Regular modifiers
        match raw_modifier.kind {
            SolModifierKind::Local => {
                if let Some(ctx_modifier) = self.calc_data.std.unreg_local_mod(item, *raw_modifier) {
                    self.force_mod_affectee_attr_recalc(util_items, sol_view, &ctx_modifier);
                    // Revisions - we need those only for local modifiers for now
                    self.calc_data.revs.unreg_mod(&ctx_modifier);
                }
            }
            SolModifierKind::FleetBuff => {
                self.calc_data
                    .std
                    .unreg_fleet_buff_mod(util_cmods, sol_view, item, *raw_modifier);
                for ctx_modifier in util_cmods.iter() {
                    self.force_mod_affectee_attr_recalc(util_items, sol_view, &ctx_modifier);
                }
                self.unreg_raw_mod_for_buff(item, raw_modifier);
            }
            SolModifierKind::System => match item {
                SolItem::SwEffect(_) => {
                    self.calc_data
                        .std
                        .unreg_sw_system_mod(util_cmods, sol_view, *raw_modifier);
                    for ctx_modifier in util_cmods.iter() {
                        self.force_mod_affectee_attr_recalc(util_items, sol_view, &ctx_modifier);
                    }
                }
                SolItem::FwEffect(fw_effect) => {
                    if let Some(ctx_modifier) = self.calc_data.std.unreg_fw_system_mod(fw_effect, *raw_modifier) {
                        self.force_mod_affectee_attr_recalc(util_items, sol_view, &ctx_modifier);
                    }
                }
                // Don't need to do anything in this case, since projected effects were
                // removed during extraction earlier
                _ => (),
            },
            SolModifierKind::Buff => {
                match item {
                    SolItem::SwEffect(_) => {
                        self.calc_data.std.unreg_sw_buff_mod(util_cmods, sol_view, raw_modifier);
                        for ctx_modifier in util_cmods.iter() {
                            self.force_mod_affectee_attr_recalc(util_items, sol_view, &ctx_modifier);
                        }
                    }
                    SolItem::FwEffect(fw_effect) => {
                        self.calc_data
                            .std
                            .unreg_fw_buff_mod(util_cmods, sol_view, fw_effect, *raw_modifier);
                        for ctx_modifier in util_cmods.iter() {
                            self.force_mod_affectee_attr_recalc(util_items, sol_view, &ctx_modifier);
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
            self.calc_data
                .buffs
                .reg_dependent_mod(item.get_id(), buff_type_attr_id, *raw_modifier);
        }
    }
    fn unreg_raw_mod_for_buff(&mut self, item: &SolItem, raw_modifier: &SolRawModifier) {
        if let Some(buff_type_attr_id) = raw_modifier.buff_type_attr_id {
            self.calc_data
                .buffs
                .unreg_dependent_mod(&item.get_id(), &buff_type_attr_id, raw_modifier);
        }
    }
    fn force_mod_affectee_attr_recalc(
        &mut self,
        affectees: &mut Vec<SolItemId>,
        sol_view: &SolView,
        modifier: &SolCtxModifier,
    ) {
        self.calc_data.std.fill_affectees(affectees, sol_view, modifier);
        for projectee_item_id in affectees.iter() {
            self.calc_force_attr_recalc(sol_view, projectee_item_id, &modifier.raw.affectee_attr_id);
        }
    }
    fn handle_location_owner_change(&mut self, sol_view: &SolView, item: &SolItem) {
        if item.get_root_loc_kind().is_some() {
            let mut affectees = Vec::new();
            for ctx_modifier in self.calc_data.std.get_mods_for_changed_root(item) {
                self.force_mod_affectee_attr_recalc(&mut affectees, sol_view, &ctx_modifier)
            }
        }
    }
}
