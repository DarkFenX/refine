use itertools::Itertools;

use crate::{
    ac, ad,
    sol::{
        FitKey, ItemKey,
        svc::{
            AttrSpec, EffectSpec, SvcCtx,
            calc::{Calc, CtxModifier, FTR_COUNT_ATTR, ModifierKind, RawModifier, SEC_STATUS_ATTR, SKILL_LVL_ATTR},
        },
        uad::{fleet::UadFleet, item::UadItem},
    },
    src::Src,
};

impl Calc {
    // Modification methods
    pub(in crate::sol::svc) fn src_changed(&mut self, src: &Src) {
        self.rah_src_changed(src);
    }
    pub(in crate::sol::svc) fn fit_added(&mut self, fit_key: FitKey) {
        self.std.reg_fit_for_sw(fit_key)
    }
    pub(in crate::sol::svc) fn fit_removed(&mut self, fit_key: FitKey) {
        self.std.unreg_fit_for_sw(fit_key)
    }
    pub(in crate::sol::svc) fn fit_added_to_fleet(&mut self, ctx: &SvcCtx, fleet: &UadFleet, fit_key: &FitKey) {
        let ctx_modifiers = self.std.reg_fleet_for_fit(fleet, fit_key);
        let mut affectees = Vec::new();
        for ctx_modifier in ctx_modifiers.iter() {
            self.force_mod_affectee_attr_recalc(&mut affectees, ctx, ctx_modifier);
        }
    }
    pub(in crate::sol::svc) fn fit_removed_from_fleet(&mut self, ctx: &SvcCtx, fleet: &UadFleet, fit_key: &FitKey) {
        let ctx_modifiers = self.std.unreg_fleet_for_fit(fleet, fit_key);
        let mut affectees = Vec::new();
        for ctx_modifier in ctx_modifiers.iter() {
            self.force_mod_affectee_attr_recalc(&mut affectees, ctx, ctx_modifier);
        }
    }
    pub(in crate::sol::svc) fn fit_rah_dps_profile_changed(&mut self, ctx: &SvcCtx, fit_key: &FitKey) {
        self.rah_fit_rah_dps_profile_changed(ctx, fit_key);
    }
    pub(in crate::sol::svc) fn item_added(&mut self, ctx: &SvcCtx, item_key: ItemKey, item: &UadItem) {
        // Char/ship switches
        self.handle_location_owner_add(ctx, item_key, item);
        // Custom modifiers
        let ctx_modifiers = self
            .revs
            .iter_mods_on_item_add()
            .filter(|v| v.raw.revise_on_item_add(ctx, item_key, item))
            .copied()
            .collect_vec();
        if !ctx_modifiers.is_empty() {
            let mut util_items = Vec::new();
            for ctx_modifier in ctx_modifiers {
                self.force_mod_affectee_attr_recalc(&mut util_items, ctx, &ctx_modifier);
            }
        }
    }
    pub(in crate::sol::svc) fn item_removed(&mut self, ctx: &SvcCtx, item_key: ItemKey, item: &UadItem) {
        // Custom modifiers
        let ctx_modifiers = self
            .revs
            .iter_mods_on_item_remove()
            .filter(|v| v.raw.revise_on_item_remove(ctx, item_key, item))
            .copied()
            .collect_vec();
        if !ctx_modifiers.is_empty() {
            let mut util_items = Vec::new();
            for ctx_modifier in ctx_modifiers {
                self.force_mod_affectee_attr_recalc(&mut util_items, ctx, &ctx_modifier);
            }
        }
        // Char/ship switches
        self.handle_location_owner_remove(ctx, item_key, item);
    }
    pub(in crate::sol::svc) fn item_loaded(&mut self, ctx: &SvcCtx, item_key: ItemKey, item: &UadItem) {
        // Notify core calc services
        self.attrs.item_loaded(item_key, item);
        self.std.reg_affectee(item_key, item);
        // Notify RAH sim
        self.rah_item_loaded(ctx, item);
    }
    pub(in crate::sol::svc) fn item_unloaded(&mut self, ctx: &SvcCtx, item_key: ItemKey, item: &UadItem) {
        // Notify RAH sim
        self.rah_item_unloaded(ctx, item);
        // Notify core calc services
        self.std.unreg_affectee(item_key, item);
        self.deps.remove_item(item_key);
        self.attrs.item_unloaded(&item_key);
    }
    pub(in crate::sol::svc) fn effects_started(
        &mut self,
        ctx: &SvcCtx,
        item_key: ItemKey,
        item: &UadItem,
        a_effects: &[ad::ArcEffectRt],
    ) {
        // Notify core calc services
        let mut raw_modifiers = Vec::new();
        let mut util_items = Vec::new();
        let mut util_cmods = Vec::new();
        for a_effect in a_effects.iter() {
            self.generate_mods_for_effect(&mut raw_modifiers, ctx, item_key, item, a_effect);
            for &raw_modifier in raw_modifiers.iter() {
                self.reg_raw_mod(&mut util_items, &mut util_cmods, ctx, item_key, item, raw_modifier);
            }
            // Buff maintenance - add info about effects which use default buff attributes
            self.buffs.reg_effect(item_key, a_effect);
        }
        // Notify RAH sim
        self.rah_effects_started(ctx, item_key, item, a_effects);
    }
    pub(in crate::sol::svc) fn effects_stopped(
        &mut self,
        ctx: &SvcCtx,
        item_key: ItemKey,
        item: &UadItem,
        a_effects: &[ad::ArcEffectRt],
    ) {
        // Notify RAH sim
        self.rah_effects_stopped(ctx, &item_key, item, a_effects);
        // Notify core calc services
        let mut raw_modifiers = Vec::new();
        let mut util_items = Vec::new();
        let mut util_cmods = Vec::new();
        for a_effect in a_effects.iter() {
            let espec = EffectSpec::new(item_key, a_effect.ae.id);
            self.std.extract_raw_mods_for_effect(&mut raw_modifiers, espec);
            for raw_modifier in raw_modifiers.iter() {
                self.unreg_raw_mod(&mut util_items, &mut util_cmods, ctx, item_key, item, raw_modifier)
            }
            // Buff maintenance - remove info about effects which use default buff attributes
            self.buffs.unreg_effect(item_key, a_effect);
            // Remove all ad-hoc attribute dependencies defined by effects being stopped. It is used
            // by e.g. custom propulsion module modifier
            self.deps.remove_by_source(&espec);
        }
    }
    pub(in crate::sol::svc) fn effect_projected(
        &mut self,
        ctx: &SvcCtx,
        projector_espec: EffectSpec,
        projectee_item_key: ItemKey,
        projectee_item: &UadItem,
    ) {
        let ctx_modifiers = self
            .std
            .project_effect(&projector_espec, projectee_item_key, projectee_item);
        let mut affectees = Vec::new();
        for ctx_modifier in ctx_modifiers.iter() {
            self.force_mod_affectee_attr_recalc(&mut affectees, ctx, ctx_modifier);
        }
    }
    pub(in crate::sol::svc) fn effect_proj_range_changed(
        &mut self,
        ctx: &SvcCtx,
        projector_espec: EffectSpec,
        projectee_item_key: ItemKey,
        projectee_item: &UadItem,
    ) {
        let ctx_modifiers = self
            .std
            .query_projected_effect(&projector_espec, projectee_item_key, projectee_item);
        let mut affectees = Vec::new();
        for ctx_modifier in ctx_modifiers.iter() {
            self.force_mod_affectee_attr_recalc(&mut affectees, ctx, ctx_modifier);
        }
    }
    pub(in crate::sol::svc) fn effect_unprojected(
        &mut self,
        ctx: &SvcCtx,
        projector_espec: EffectSpec,
        projectee_item_key: ItemKey,
        projectee_item: &UadItem,
    ) {
        let ctx_modifiers = self
            .std
            .unproject_effect(&projector_espec, projectee_item_key, projectee_item);
        let mut affectees = Vec::new();
        for ctx_modifier in ctx_modifiers.iter() {
            self.force_mod_affectee_attr_recalc(&mut affectees, ctx, ctx_modifier);
        }
    }
    pub(in crate::sol::svc) fn attr_value_changed(&mut self, ctx: &SvcCtx, aspec: AttrSpec) {
        // Clear up attribute values which rely on passed attribute as an upper/lower limit
        let affectee_aspecs = self.deps.get_affectee_attr_specs(&aspec).copied().collect_vec();
        for affectee_aspec in affectee_aspecs.into_iter() {
            self.force_attr_value_recalc(ctx, affectee_aspec);
        }
        // Clear up attribute values which rely on passed attribute as a modification source
        let ctx_modifiers = self.std.iter_affector_spec_mods(&aspec).copied().collect_vec();
        if !ctx_modifiers.is_empty() {
            let mut affectees = Vec::new();
            for ctx_modifier in ctx_modifiers.iter() {
                self.std.fill_affectees(&mut affectees, ctx, ctx_modifier);
                for &affectee_item_key in affectees.iter() {
                    let projectee_aspec = AttrSpec::new(affectee_item_key, ctx_modifier.raw.affectee_a_attr_id);
                    self.force_attr_value_recalc(ctx, projectee_aspec);
                }
            }
        }
        // Process buffs which rely on attribute being modified
        if ac::extras::BUFF_STDATTR_IDS.contains(&aspec.a_attr_id) {
            let item = ctx.uad.items.get(aspec.item_key);
            // Remove modifiers of buffs which rely on the attribute
            if let Some(raw_modifiers) = self.buffs.extract_dependent_mods(&aspec) {
                let mut util_items = Vec::new();
                let mut util_cmods = Vec::new();
                let raw_modifiers = raw_modifiers.collect_vec();
                for raw_modifier in raw_modifiers.iter() {
                    self.unreg_raw_mod(
                        &mut util_items,
                        &mut util_cmods,
                        ctx,
                        aspec.item_key,
                        item,
                        raw_modifier,
                    );
                }
            }
            // Generate new modifiers using new value and apply them
            let a_effect_ids = self.buffs.get_effects(&aspec.item_key);
            if !a_effect_ids.is_empty() {
                let effect_ids = a_effect_ids.copied().collect_vec();
                let raw_modifiers =
                    self.generate_dependent_buff_mods(ctx, aspec.item_key, item, effect_ids.iter(), aspec.a_attr_id);
                for raw_modifier in raw_modifiers.iter() {
                    self.buffs.reg_dependent_mod(aspec, *raw_modifier);
                }
                let mut util_items = Vec::new();
                let mut util_cmods = Vec::new();
                for &raw_modifier in raw_modifiers.iter() {
                    self.reg_raw_mod(
                        &mut util_items,
                        &mut util_cmods,
                        ctx,
                        aspec.item_key,
                        item,
                        raw_modifier,
                    );
                }
            }
        }
        // Notify RAH sim
        self.rah_attr_value_changed(ctx, &aspec);
    }
    pub(in crate::sol::svc) fn force_attr_value_recalc(&mut self, ctx: &SvcCtx, aspec: AttrSpec) {
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
    pub(in crate::sol::svc::calc) fn force_attr_postproc_recalc(&mut self, ctx: &SvcCtx, aspec: AttrSpec) {
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
    pub(in crate::sol::svc) fn sol_sec_zone_changed(&mut self, ctx: &SvcCtx) {
        for item_key in ctx.uad.items.keys() {
            self.force_attr_value_recalc(ctx, AttrSpec::new(item_key, ac::attrs::SECURITY_MODIFIER))
        }
    }
    pub(in crate::sol::svc) fn fighter_count_changed(&mut self, ctx: &SvcCtx, fighter_key: ItemKey) {
        self.force_attr_postproc_recalc(ctx, AttrSpec::new(fighter_key, FTR_COUNT_ATTR))
    }
    pub(in crate::sol::svc) fn ship_sec_status_changed(&mut self, ctx: &SvcCtx, ship_key: ItemKey) {
        self.force_attr_postproc_recalc(ctx, AttrSpec::new(ship_key, SEC_STATUS_ATTR))
    }
    pub(in crate::sol::svc) fn skill_level_changed(&mut self, ctx: &SvcCtx, skill_key: ItemKey) {
        self.force_attr_postproc_recalc(ctx, AttrSpec::new(skill_key, SKILL_LVL_ATTR))
    }
    // Private methods
    fn reg_raw_mod(
        &mut self,
        util_items: &mut Vec<ItemKey>,
        util_cmods: &mut Vec<CtxModifier>,
        ctx: &SvcCtx,
        item_key: ItemKey,
        item: &UadItem,
        raw_modifier: RawModifier,
    ) {
        match raw_modifier.kind {
            ModifierKind::Local => {
                if let Some(ctx_modifier) = self.std.reg_local_mod(item, raw_modifier) {
                    self.force_mod_affectee_attr_recalc(util_items, ctx, &ctx_modifier);
                    // Revisions - we need those only for local modifiers for now
                    self.revs.reg_mod(&ctx_modifier);
                }
            }
            ModifierKind::FleetBuff => {
                let registered = self.std.reg_fleet_buff_mod(util_cmods, ctx, item, raw_modifier);
                for ctx_modifier in util_cmods.iter() {
                    self.force_mod_affectee_attr_recalc(util_items, ctx, ctx_modifier);
                }
                if registered {
                    self.reg_raw_mod_for_buff(item_key, raw_modifier);
                }
            }
            ModifierKind::System => match item {
                UadItem::SwEffect(_) => {
                    self.std.reg_sw_system_mod(util_cmods, ctx, raw_modifier);
                    for ctx_modifier in util_cmods.iter() {
                        self.force_mod_affectee_attr_recalc(util_items, ctx, ctx_modifier);
                    }
                }
                UadItem::FwEffect(fw_effect) => {
                    if let Some(ctx_modifier) = self.std.reg_fw_system_mod(fw_effect, raw_modifier) {
                        self.force_mod_affectee_attr_recalc(util_items, ctx, &ctx_modifier);
                    }
                }
                UadItem::ProjEffect(_) => self.std.reg_proj_mod(raw_modifier),
                _ => (),
            },
            ModifierKind::Buff => {
                let registered = match item {
                    UadItem::SwEffect(_) => {
                        let registered = self.std.reg_sw_buff_mod(util_cmods, ctx, raw_modifier);
                        for ctx_modifier in util_cmods.iter() {
                            self.force_mod_affectee_attr_recalc(util_items, ctx, ctx_modifier);
                        }
                        registered
                    }
                    UadItem::FwEffect(fw_effect) => {
                        let registered = self.std.reg_fw_buff_mod(util_cmods, ctx, fw_effect, raw_modifier);
                        for ctx_modifier in util_cmods.iter() {
                            self.force_mod_affectee_attr_recalc(util_items, ctx, ctx_modifier);
                        }
                        registered
                    }
                    _ => {
                        self.std.reg_proj_mod(raw_modifier);
                        true
                    }
                };
                if registered {
                    self.reg_raw_mod_for_buff(item_key, raw_modifier);
                }
            }
            ModifierKind::Targeted => self.std.reg_proj_mod(raw_modifier),
        }
    }
    fn unreg_raw_mod(
        &mut self,
        util_items: &mut Vec<ItemKey>,
        util_cmods: &mut Vec<CtxModifier>,
        ctx: &SvcCtx,
        item_key: ItemKey,
        item: &UadItem,
        raw_modifier: &RawModifier,
    ) {
        // Regular modifiers
        match raw_modifier.kind {
            ModifierKind::Local => {
                if let Some(ctx_modifier) = self.std.unreg_local_mod(item, *raw_modifier) {
                    self.force_mod_affectee_attr_recalc(util_items, ctx, &ctx_modifier);
                    // Revisions - we need those only for local modifiers for now
                    self.revs.unreg_mod(&ctx_modifier);
                }
            }
            ModifierKind::FleetBuff => {
                self.std.unreg_fleet_buff_mod(util_cmods, ctx, item, *raw_modifier);
                for ctx_modifier in util_cmods.iter() {
                    self.force_mod_affectee_attr_recalc(util_items, ctx, ctx_modifier);
                }
                self.unreg_raw_mod_for_buff(item_key, raw_modifier);
            }
            ModifierKind::System => match item {
                UadItem::SwEffect(_) => {
                    self.std.unreg_sw_system_mod(util_cmods, ctx, *raw_modifier);
                    for ctx_modifier in util_cmods.iter() {
                        self.force_mod_affectee_attr_recalc(util_items, ctx, ctx_modifier);
                    }
                }
                UadItem::FwEffect(fw_effect) => {
                    if let Some(ctx_modifier) = self.std.unreg_fw_system_mod(fw_effect, *raw_modifier) {
                        self.force_mod_affectee_attr_recalc(util_items, ctx, &ctx_modifier);
                    }
                }
                UadItem::ProjEffect(_) => self.std.unreg_proj_mod(raw_modifier),
                _ => (),
            },
            ModifierKind::Buff => {
                match item {
                    UadItem::SwEffect(_) => {
                        self.std.unreg_sw_buff_mod(util_cmods, ctx, raw_modifier);
                        for ctx_modifier in util_cmods.iter() {
                            self.force_mod_affectee_attr_recalc(util_items, ctx, ctx_modifier);
                        }
                    }
                    UadItem::FwEffect(fw_effect) => {
                        self.std.unreg_fw_buff_mod(util_cmods, ctx, fw_effect, *raw_modifier);
                        for ctx_modifier in util_cmods.iter() {
                            self.force_mod_affectee_attr_recalc(util_items, ctx, ctx_modifier);
                        }
                    }
                    _ => {
                        self.std.unreg_proj_mod(raw_modifier);
                    }
                }
                self.unreg_raw_mod_for_buff(item_key, raw_modifier);
            }
            ModifierKind::Targeted => self.std.unreg_proj_mod(raw_modifier),
        }
    }
    fn reg_raw_mod_for_buff(&mut self, item_key: ItemKey, raw_modifier: RawModifier) {
        if let Some(buff_type_attr_id) = raw_modifier.buff_type_a_attr_id {
            self.buffs
                .reg_dependent_mod(AttrSpec::new(item_key, buff_type_attr_id), raw_modifier);
        }
    }
    fn unreg_raw_mod_for_buff(&mut self, item_key: ItemKey, raw_modifier: &RawModifier) {
        if let Some(buff_type_attr_id) = raw_modifier.buff_type_a_attr_id {
            self.buffs
                .unreg_dependent_mod(&AttrSpec::new(item_key, buff_type_attr_id), raw_modifier);
        }
    }
    fn force_mod_affectee_attr_recalc(&mut self, affectees: &mut Vec<ItemKey>, ctx: &SvcCtx, modifier: &CtxModifier) {
        self.std.fill_affectees(affectees, ctx, modifier);
        for &affectee_item_key in affectees.iter() {
            self.force_attr_value_recalc(ctx, AttrSpec::new(affectee_item_key, modifier.raw.affectee_a_attr_id));
        }
    }
    fn handle_location_owner_add(&mut self, ctx: &SvcCtx, item_key: ItemKey, item: &UadItem) {
        if matches!(item, UadItem::Ship(_) | UadItem::Character(_)) {
            let mut affectees = Vec::new();
            for ctx_modifier in self.std.get_mods_for_added_root(item_key, item) {
                self.force_mod_affectee_attr_recalc(&mut affectees, ctx, &ctx_modifier)
            }
        }
    }
    fn handle_location_owner_remove(&mut self, ctx: &SvcCtx, item_key: ItemKey, item: &UadItem) {
        if matches!(item, UadItem::Ship(_) | UadItem::Character(_)) {
            let mut affectees = Vec::new();
            for ctx_modifier in self.std.get_mods_for_removed_root(item_key, item) {
                self.force_mod_affectee_attr_recalc(&mut affectees, ctx, &ctx_modifier)
            }
        }
    }
}
