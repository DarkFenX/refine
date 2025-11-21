use itertools::Itertools;

use crate::{
    ac,
    misc::{AttrSpec, EffectSpec},
    rd,
    svc::{
        SvcCtx,
        calc::{Calc, CtxModifier, FTR_COUNT_ATTR, ModifierKind, RawModifier, SEC_STATUS_ATTR, SKILL_LVL_ATTR},
    },
    ud::{UFitKey, UFleet, UItem, UItemKey},
};

impl Calc {
    // Modification methods
    pub(in crate::svc) fn fit_added(&mut self, fit_key: UFitKey) {
        self.std.reg_fit_for_sw(fit_key)
    }
    pub(in crate::svc) fn fit_removed(&mut self, fit_key: UFitKey) {
        self.std.unreg_fit_for_sw(fit_key)
    }
    pub(in crate::svc) fn fit_added_to_fleet(&mut self, ctx: SvcCtx, fleet: &UFleet, fit_key: UFitKey) {
        let cmods = self.std.reg_fleet_for_fit(fleet, fit_key);
        let mut reuse_affectees = Vec::new();
        for cmod in cmods.iter() {
            self.force_mod_affectee_attr_recalc(&mut reuse_affectees, ctx, cmod);
        }
    }
    pub(in crate::svc) fn fit_removed_from_fleet(&mut self, ctx: SvcCtx, fleet: &UFleet, fit_key: UFitKey) {
        let cmods = self.std.unreg_fleet_for_fit(fleet, fit_key);
        let mut reuse_affectees = Vec::new();
        for cmod in cmods.iter() {
            self.force_mod_affectee_attr_recalc(&mut reuse_affectees, ctx, cmod);
        }
    }
    pub(in crate::svc) fn fit_rah_dps_profile_changed(&mut self, ctx: SvcCtx, fit_key: UFitKey) {
        self.rah_fit_rah_dps_profile_changed(ctx, fit_key);
    }
    pub(in crate::svc) fn item_added(&mut self, ctx: SvcCtx, item_key: UItemKey, item: &UItem) {
        // Char/ship switches
        self.handle_location_root_add(ctx, item_key, item);
        // Custom modifiers
        let cmods = self
            .revs
            .iter_revs_on_item_add()
            .filter(|(cmod, reviser)| reviser(ctx, cmod.raw.affector_espec.item_key, item_key, item))
            .map(|(cmod, _reviser)| *cmod)
            .collect_vec();
        if !cmods.is_empty() {
            let mut reuse_items = Vec::new();
            for cmod in cmods {
                self.force_mod_affectee_attr_recalc(&mut reuse_items, ctx, &cmod);
            }
        }
    }
    pub(in crate::svc) fn item_removed(&mut self, ctx: SvcCtx, item_key: UItemKey, item: &UItem) {
        // Custom modifiers
        let cmods = self
            .revs
            .iter_revs_on_item_remove()
            .filter(|(cmod, reviser)| reviser(ctx, cmod.raw.affector_espec.item_key, item_key, item))
            .map(|(cmod, _reviser)| *cmod)
            .collect_vec();
        if !cmods.is_empty() {
            let mut reuse_items = Vec::new();
            for cmod in cmods {
                self.force_mod_affectee_attr_recalc(&mut reuse_items, ctx, &cmod);
            }
        }
        // Char/ship switches
        self.handle_location_root_remove(ctx, item_key, item);
    }
    pub(in crate::svc) fn item_loaded(&mut self, ctx: SvcCtx, item_key: UItemKey, item: &UItem) {
        // Notify core calc services
        self.attrs.item_loaded(item_key, item);
        self.std.reg_affectee(item_key, item);
        // Notify RAH sim
        self.rah_item_loaded(ctx, item);
    }
    pub(in crate::svc) fn item_unloaded(&mut self, ctx: SvcCtx, item_key: UItemKey, item: &UItem) {
        // Notify RAH sim
        self.rah_item_unloaded(ctx, item);
        // Notify core calc services
        self.std.unreg_affectee(item_key, item);
        self.deps.remove_item(item_key);
        self.attrs.item_unloaded(&item_key);
    }
    pub(in crate::svc) fn effects_started(
        &mut self,
        ctx: SvcCtx,
        item_key: UItemKey,
        item: &UItem,
        r_effects: &[rd::RcEffect],
    ) {
        // Notify core calc services
        let mut reuse_rmods = Vec::new();
        let mut reuse_items = Vec::new();
        let mut reuse_cmods = Vec::new();
        for r_effect in r_effects.iter() {
            self.generate_mods_for_effect(&mut reuse_rmods, ctx, item_key, item, r_effect);
            for &rmod in reuse_rmods.iter() {
                self.reg_raw_mod(&mut reuse_items, &mut reuse_cmods, ctx, item_key, item, rmod);
            }
            // Buff maintenance - add info about effects which use default buff attributes
            self.buffs.reg_effect(item_key, r_effect);
        }
        // Notify RAH sim
        self.rah_effects_started(ctx, item_key, item, r_effects);
    }
    pub(in crate::svc) fn effects_stopped(
        &mut self,
        ctx: SvcCtx,
        item_key: UItemKey,
        item: &UItem,
        r_effects: &[rd::RcEffect],
    ) {
        // Notify RAH sim
        self.rah_effects_stopped(ctx, &item_key, item, r_effects);
        // Notify core calc services
        let mut reuse_rmods = Vec::new();
        let mut reuse_items = Vec::new();
        let mut reuse_cmods = Vec::new();
        for r_effect in r_effects.iter() {
            let espec = EffectSpec::new(item_key, r_effect.get_key());
            self.std.extract_raw_mods_for_effect(&mut reuse_rmods, espec);
            for rmod in reuse_rmods.iter() {
                self.unreg_raw_mod(&mut reuse_items, &mut reuse_cmods, ctx, item_key, item, rmod)
            }
            // Buff maintenance - remove info about effects which use default buff attributes
            self.buffs.unreg_effect(item_key, r_effect);
            // Remove all ad-hoc attribute dependencies defined by effects being stopped. It is used
            // by e.g. custom propulsion module modifier
            self.deps.remove_by_source(&espec);
        }
    }
    pub(in crate::svc) fn effect_projected(
        &mut self,
        ctx: SvcCtx,
        projector_espec: EffectSpec,
        projectee_key: UItemKey,
        projectee_item: &UItem,
    ) {
        let cmods = self.std.project_effect(&projector_espec, projectee_key, projectee_item);
        let mut reuse_affectees = Vec::new();
        for cmod in cmods.iter() {
            self.force_mod_affectee_attr_recalc(&mut reuse_affectees, ctx, cmod);
        }
    }
    pub(in crate::svc) fn effect_proj_data_changed(
        &mut self,
        ctx: SvcCtx,
        projector_espec: EffectSpec,
        projectee_key: UItemKey,
        projectee_item: &UItem,
    ) {
        let cmods = self
            .std
            .query_projected_effect(&projector_espec, projectee_key, projectee_item);
        let mut reuse_affectees = Vec::new();
        for cmod in cmods.iter() {
            self.force_mod_affectee_attr_recalc(&mut reuse_affectees, ctx, cmod);
        }
    }
    pub(in crate::svc) fn effect_unprojected(
        &mut self,
        ctx: SvcCtx,
        projector_espec: EffectSpec,
        projectee_key: UItemKey,
        projectee_item: &UItem,
    ) {
        let cmods = self
            .std
            .unproject_effect(&projector_espec, projectee_key, projectee_item);
        let mut reuse_affectees = Vec::new();
        for cmod in cmods.iter() {
            self.force_mod_affectee_attr_recalc(&mut reuse_affectees, ctx, cmod);
        }
    }
    pub(in crate::svc) fn attr_value_changed(&mut self, ctx: SvcCtx, aspec: AttrSpec) {
        // Clear up attribute values which rely on passed attribute as an upper/lower limit
        let affectee_aspecs = self.deps.iter_affectee_aspecs(&aspec).copied().collect_vec();
        for affectee_aspec in affectee_aspecs.into_iter() {
            self.force_attr_value_recalc(ctx, affectee_aspec);
        }
        // Clear up attribute values which rely on passed attribute as a modification source
        let cmods = self.std.iter_affector_spec_cmods(&aspec).copied().collect_vec();
        if !cmods.is_empty() {
            let mut reuse_affectees = Vec::new();
            for cmod in cmods.iter() {
                self.std.fill_affectees(&mut reuse_affectees, ctx, cmod);
                for &affectee_key in reuse_affectees.iter() {
                    let projectee_aspec = AttrSpec::new(affectee_key, cmod.raw.affectee_attr_id);
                    self.force_attr_value_recalc(ctx, projectee_aspec);
                }
            }
        }
        // Process buffs which rely on attribute being modified
        if ac::extras::BUFF_STDATTR_IDS.contains(&aspec.a_attr_id) {
            let item = ctx.u_data.items.get(aspec.item_key);
            // Remove modifiers of buffs which rely on the attribute
            if let Some(rmods) = self.buffs.extract_dependent_mods(&aspec) {
                let mut reuse_items = Vec::new();
                let mut reuse_cmods = Vec::new();
                let rmods = rmods.collect_vec();
                for rmod in rmods.iter() {
                    self.unreg_raw_mod(&mut reuse_items, &mut reuse_cmods, ctx, aspec.item_key, item, rmod);
                }
            }
            // Generate new modifiers using new value and apply them
            let effect_keys = self.buffs.get_effects(&aspec.item_key);
            if !effect_keys.is_empty() {
                let effect_keys = effect_keys.collect_vec();
                let rmods =
                    self.generate_dependent_buff_mods(ctx, aspec.item_key, item, effect_keys.iter(), aspec.a_attr_id);
                for rmod in rmods.iter() {
                    self.buffs.reg_dependent_mod(aspec, *rmod);
                }
                let mut reuse_items = Vec::new();
                let mut reuse_cmods = Vec::new();
                for &rmod in rmods.iter() {
                    self.reg_raw_mod(&mut reuse_items, &mut reuse_cmods, ctx, aspec.item_key, item, rmod);
                }
            }
        }
        // Notify RAH sim
        self.rah_attr_value_changed(ctx, &aspec);
    }
    pub(in crate::svc) fn force_attr_value_recalc(&mut self, ctx: SvcCtx, aspec: AttrSpec) {
        // Sometimes calc service receives requests to clear attributes it does not know yet; this
        // can happen in multiple cases, e.g. when adding module with charge, with "other" location
        // modifier on module. User data gets references between charge and module set right away,
        // but calculator registers module before charge, and attempts to clear charge attributes.
        // Due to cases like this, we cannot just unwrap item attribute data.
        if let Some(item_attr_data) = self.attrs.get_item_attr_data_mut(&aspec.item_key) {
            // No value calculated before that - there are no dependents to clear (dependents always
            // request dependencies while calculating their values). Removing attribute forces
            // recalculation
            if item_attr_data.values.remove(&aspec.a_attr_id).is_some() {
                self.attr_value_changed(ctx, aspec);
            }
        }
    }
    pub(in crate::svc::calc) fn force_attr_postproc_recalc(&mut self, ctx: SvcCtx, aspec: AttrSpec) {
        // Almost-copy of force recalc method without attribute removal. When something that
        // installed a postprocessing function thinks its output can change, it can let calc service
        // know about it via this method.
        if let Some(item_attr_data) = self.attrs.get_item_attr_data_mut(&aspec.item_key) {
            // No value calculated before that - there are no dependents to clear (dependents always
            // request dependencies while calculating their values). In this case we do not remove
            // attribute, because only postprocessing output is supposed to change
            if item_attr_data.values.contains_key(&aspec.a_attr_id) {
                self.attr_value_changed(ctx, aspec);
            }
        }
    }
    pub(in crate::svc) fn sol_sec_zone_changed(&mut self, ctx: SvcCtx) {
        for item_key in ctx.u_data.items.keys() {
            self.force_attr_value_recalc(ctx, AttrSpec::new(item_key, ac::attrs::SECURITY_MODIFIER))
        }
    }
    pub(in crate::svc) fn fighter_count_changed(&mut self, ctx: SvcCtx, fighter_key: UItemKey) {
        self.force_attr_postproc_recalc(ctx, AttrSpec::new(fighter_key, FTR_COUNT_ATTR))
    }
    pub(in crate::svc) fn ship_sec_status_changed(&mut self, ctx: SvcCtx, ship_key: UItemKey) {
        self.force_attr_postproc_recalc(ctx, AttrSpec::new(ship_key, SEC_STATUS_ATTR))
    }
    pub(in crate::svc) fn skill_level_changed(&mut self, ctx: SvcCtx, skill_key: UItemKey) {
        self.force_attr_postproc_recalc(ctx, AttrSpec::new(skill_key, SKILL_LVL_ATTR))
    }
    // Private methods
    fn reg_raw_mod(
        &mut self,
        reuse_items: &mut Vec<UItemKey>,
        reuse_cmods: &mut Vec<CtxModifier>,
        ctx: SvcCtx,
        item_key: UItemKey,
        item: &UItem,
        rmod: RawModifier,
    ) {
        match rmod.kind {
            ModifierKind::Local => {
                if let Some(cmod) = self.std.reg_local_mod(item, rmod) {
                    self.force_mod_affectee_attr_recalc(reuse_items, ctx, &cmod);
                    // Revisions - we need those only for local modifiers for now
                    self.revs.reg_mod(&cmod);
                }
            }
            ModifierKind::FleetBuff => {
                let registered = self.std.reg_fleet_buff_mod(reuse_cmods, ctx, item, rmod);
                for cmod in reuse_cmods.iter() {
                    self.force_mod_affectee_attr_recalc(reuse_items, ctx, cmod);
                }
                if registered {
                    self.reg_raw_mod_for_buff(item_key, rmod);
                }
            }
            ModifierKind::System => match item {
                UItem::SwEffect(_) => {
                    self.std.reg_sw_system_mod(reuse_cmods, ctx, rmod);
                    for cmod in reuse_cmods.iter() {
                        self.force_mod_affectee_attr_recalc(reuse_items, ctx, cmod);
                    }
                }
                UItem::FwEffect(fw_effect) => {
                    if let Some(cmod) = self.std.reg_fw_system_mod(fw_effect, rmod) {
                        self.force_mod_affectee_attr_recalc(reuse_items, ctx, &cmod);
                    }
                }
                UItem::ProjEffect(_) => self.std.reg_proj_mod(rmod),
                _ => (),
            },
            ModifierKind::Buff => {
                let registered = match item {
                    UItem::SwEffect(_) => {
                        let registered = self.std.reg_sw_buff_mod(reuse_cmods, ctx, rmod);
                        for cmod in reuse_cmods.iter() {
                            self.force_mod_affectee_attr_recalc(reuse_items, ctx, cmod);
                        }
                        registered
                    }
                    UItem::FwEffect(fw_effect) => {
                        let registered = self.std.reg_fw_buff_mod(reuse_cmods, ctx, fw_effect, rmod);
                        for cmod in reuse_cmods.iter() {
                            self.force_mod_affectee_attr_recalc(reuse_items, ctx, cmod);
                        }
                        registered
                    }
                    _ => {
                        self.std.reg_proj_mod(rmod);
                        true
                    }
                };
                if registered {
                    self.reg_raw_mod_for_buff(item_key, rmod);
                }
            }
            ModifierKind::Targeted => self.std.reg_proj_mod(rmod),
        }
    }
    fn unreg_raw_mod(
        &mut self,
        reuse_items: &mut Vec<UItemKey>,
        reuse_cmods: &mut Vec<CtxModifier>,
        ctx: SvcCtx,
        item_key: UItemKey,
        item: &UItem,
        rmod: &RawModifier,
    ) {
        // Regular modifiers
        match rmod.kind {
            ModifierKind::Local => {
                if let Some(cmod) = self.std.unreg_local_mod(item, *rmod) {
                    self.force_mod_affectee_attr_recalc(reuse_items, ctx, &cmod);
                    // Revisions - we need those only for local modifiers for now
                    self.revs.unreg_mod(&cmod);
                }
            }
            ModifierKind::FleetBuff => {
                self.std.unreg_fleet_buff_mod(reuse_cmods, ctx, item, *rmod);
                for cmod in reuse_cmods.iter() {
                    self.force_mod_affectee_attr_recalc(reuse_items, ctx, cmod);
                }
                self.unreg_raw_mod_for_buff(item_key, rmod);
            }
            ModifierKind::System => match item {
                UItem::SwEffect(_) => {
                    self.std.unreg_sw_system_mod(reuse_cmods, ctx, *rmod);
                    for cmod in reuse_cmods.iter() {
                        self.force_mod_affectee_attr_recalc(reuse_items, ctx, cmod);
                    }
                }
                UItem::FwEffect(fw_effect) => {
                    if let Some(cmod) = self.std.unreg_fw_system_mod(fw_effect, *rmod) {
                        self.force_mod_affectee_attr_recalc(reuse_items, ctx, &cmod);
                    }
                }
                UItem::ProjEffect(_) => self.std.unreg_proj_mod(rmod),
                _ => (),
            },
            ModifierKind::Buff => {
                match item {
                    UItem::SwEffect(_) => {
                        self.std.unreg_sw_buff_mod(reuse_cmods, ctx, rmod);
                        for cmod in reuse_cmods.iter() {
                            self.force_mod_affectee_attr_recalc(reuse_items, ctx, cmod);
                        }
                    }
                    UItem::FwEffect(fw_effect) => {
                        self.std.unreg_fw_buff_mod(reuse_cmods, ctx, fw_effect, *rmod);
                        for cmod in reuse_cmods.iter() {
                            self.force_mod_affectee_attr_recalc(reuse_items, ctx, cmod);
                        }
                    }
                    _ => {
                        self.std.unreg_proj_mod(rmod);
                    }
                }
                self.unreg_raw_mod_for_buff(item_key, rmod);
            }
            ModifierKind::Targeted => self.std.unreg_proj_mod(rmod),
        }
    }
    fn reg_raw_mod_for_buff(&mut self, item_key: UItemKey, rmod: RawModifier) {
        if let Some(buff_type_attr_id) = rmod.buff_type_attr_id {
            self.buffs
                .reg_dependent_mod(AttrSpec::new(item_key, buff_type_attr_id), rmod);
        }
    }
    fn unreg_raw_mod_for_buff(&mut self, item_key: UItemKey, rmod: &RawModifier) {
        if let Some(buff_type_attr_id) = rmod.buff_type_attr_id {
            self.buffs
                .unreg_dependent_mod(&AttrSpec::new(item_key, buff_type_attr_id), rmod);
        }
    }
    fn force_mod_affectee_attr_recalc(&mut self, reuse_affectees: &mut Vec<UItemKey>, ctx: SvcCtx, cmod: &CtxModifier) {
        self.std.fill_affectees(reuse_affectees, ctx, cmod);
        for &affectee_key in reuse_affectees.iter() {
            self.force_attr_value_recalc(ctx, AttrSpec::new(affectee_key, cmod.raw.affectee_attr_id));
        }
    }
    fn handle_location_root_add(&mut self, ctx: SvcCtx, item_key: UItemKey, item: &UItem) {
        if matches!(item, UItem::Ship(_) | UItem::Character(_)) {
            let mut reuse_affectees = Vec::new();
            for cmod in self.std.get_mods_for_added_root(item_key, item) {
                self.force_mod_affectee_attr_recalc(&mut reuse_affectees, ctx, &cmod)
            }
        }
    }
    fn handle_location_root_remove(&mut self, ctx: SvcCtx, item_key: UItemKey, item: &UItem) {
        if matches!(item, UItem::Ship(_) | UItem::Character(_)) {
            let mut reuse_affectees = Vec::new();
            for cmod in self.std.get_mods_for_removed_root(item_key, item) {
                self.force_mod_affectee_attr_recalc(&mut reuse_affectees, ctx, &cmod)
            }
        }
    }
}
