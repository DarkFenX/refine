use itertools::Itertools;

use crate::{
    misc::{AttrSpec, EffectSpec},
    rd::{RAttrId, RcEffect},
    svc::{
        SvcCtx,
        calc::{Calc, CtxModifier, ModifierKind, RawModId, RawModifier},
    },
    ud::{UFitId, UFleet, UItem, UItemId},
};

impl Calc {
    // Modification methods
    pub(in crate::svc) fn fit_added(&mut self, fit_uid: UFitId) {
        self.std.reg_fit_for_sw(&self.rmods, fit_uid)
    }
    pub(in crate::svc) fn fit_removed(&mut self, fit_uid: UFitId) {
        self.std.unreg_fit_for_sw(&self.rmods, fit_uid)
    }
    pub(in crate::svc) fn fit_added_to_fleet(&mut self, ctx: &SvcCtx, fleet: &UFleet, fit_uid: UFitId) {
        let mut reuse_cmods = self.reuse.cmods.take();
        self.std
            .reg_fleet_for_fit(&mut reuse_cmods, ctx, &self.rmods, fleet, fit_uid);
        let mut reuse_affectees = self.reuse.items.take();
        for cmod in reuse_cmods.iter() {
            self.force_mod_affectee_attr_recalc(&mut reuse_affectees, ctx, cmod);
        }
        self.reuse.items.put(reuse_affectees);
        self.reuse.cmods.put(reuse_cmods);
    }
    pub(in crate::svc) fn fit_removed_from_fleet(&mut self, ctx: &SvcCtx, fleet: &UFleet, fit_uid: UFitId) {
        let mut reuse_cmods = self.reuse.cmods.take();
        self.std
            .unreg_fleet_for_fit(&mut reuse_cmods, ctx, &self.rmods, fleet, fit_uid);
        let mut reuse_affectees = self.reuse.items.take();
        for cmod in reuse_cmods.iter() {
            self.force_mod_affectee_attr_recalc(&mut reuse_affectees, ctx, cmod);
        }
        self.reuse.items.put(reuse_affectees);
        self.reuse.cmods.put(reuse_cmods);
    }
    pub(in crate::svc) fn fit_rah_dps_profile_changed(&mut self, ctx: &SvcCtx, fit_uid: UFitId) {
        self.rah_fit_rah_dps_profile_changed(ctx, fit_uid);
    }
    pub(in crate::svc) fn item_added(&mut self, ctx: &SvcCtx, item_uid: UItemId, item: &UItem) {
        // Custom modifiers
        let cmods = self
            .revs
            .iter_revs_on_item_add_remove()
            .filter_map(|(ckey, reviser)| {
                let cmod = self.rmods.get_cmod(ckey);
                match reviser.revise(ctx, cmod.rmod.affector_espec.item_uid, item_uid, item) {
                    true => Some(cmod),
                    false => None,
                }
            })
            .collect_vec();
        if !cmods.is_empty() {
            let mut reuse_items = self.reuse.items.take();
            for cmod in cmods.into_iter() {
                self.force_mod_affectee_attr_recalc(&mut reuse_items, ctx, &cmod);
            }
            self.reuse.items.put(reuse_items);
        }
    }
    pub(in crate::svc) fn item_removed(&mut self, ctx: &SvcCtx, item_uid: UItemId, item: &UItem) {
        // Custom modifiers
        let cmods = self
            .revs
            .iter_revs_on_item_add_remove()
            .filter_map(|(ckey, reviser)| {
                let cmod = self.rmods.get_cmod(ckey);
                match reviser.revise(ctx, cmod.rmod.affector_espec.item_uid, item_uid, item) {
                    true => Some(cmod),
                    false => None,
                }
            })
            .collect_vec();
        if !cmods.is_empty() {
            let mut reuse_items = self.reuse.items.take();
            for cmod in cmods.into_iter() {
                self.force_mod_affectee_attr_recalc(&mut reuse_items, ctx, &cmod);
            }
            self.reuse.items.put(reuse_items);
        }
    }
    pub(in crate::svc) fn item_loaded(&mut self, ctx: &SvcCtx, item_uid: UItemId, item: &UItem) {
        // Notify core calc services
        self.attrs.item_loaded(ctx.u_data, item_uid, item);
        let mut reuse_cmods = self.reuse.cmods.take();
        self.std
            .reg_affectee(&mut reuse_cmods, ctx, &self.rmods, item_uid, item);
        if !reuse_cmods.is_empty() {
            let mut reuse_affectees = self.reuse.items.take();
            for cmod in reuse_cmods.iter() {
                self.force_mod_affectee_attr_recalc(&mut reuse_affectees, ctx, cmod)
            }
            self.reuse.items.put(reuse_affectees);
        }
        self.reuse.cmods.put(reuse_cmods);
        // Notify RAH sim
        self.rah_item_loaded(ctx, item);
    }
    pub(in crate::svc) fn item_unloaded(&mut self, ctx: &SvcCtx, item_uid: UItemId, item: &UItem) {
        // Notify RAH sim
        self.rah_item_unloaded(ctx, item);
        // Notify core calc services
        let mut reuse_cmods = self.reuse.cmods.take();
        self.std
            .unreg_affectee(&mut reuse_cmods, ctx, &self.rmods, item_uid, item);
        if !reuse_cmods.is_empty() {
            let mut reuse_affectees = self.reuse.items.take();
            for cmod in reuse_cmods.iter() {
                self.force_mod_affectee_attr_recalc(&mut reuse_affectees, ctx, cmod)
            }
            self.reuse.items.put(reuse_affectees);
        }
        self.reuse.cmods.put(reuse_cmods);
        self.deps.remove_item(item_uid);
        self.attrs.item_unloaded(item_uid);
    }
    pub(in crate::svc) fn effects_started(
        &mut self,
        ctx: &SvcCtx,
        item_uid: UItemId,
        item: &UItem,
        effects: &[RcEffect],
    ) {
        // Notify core calc services
        let mut reuse_rmod_ids = self.reuse.rmod_ids.take();
        let mut reuse_rmods = self.reuse.rmods.take();
        let mut reuse_items = self.reuse.items.take();
        let mut reuse_cmods = self.reuse.cmods.take();
        for effect in effects.iter() {
            self.generate_mods_for_effect(&mut reuse_rmods, ctx.u_data, item_uid, item, effect);
            self.reg_raw_mods(
                &mut reuse_rmod_ids,
                &mut reuse_items,
                &mut reuse_cmods,
                ctx,
                item,
                EffectSpec::new(item_uid, effect.rid),
                &reuse_rmods,
            );
        }
        // Buffs which rely on on-item attributes are generated after all other modifiers of started
        // effects are registered, since those modifiers can change applied buff ID
        for effect in effects.iter() {
            if let Some(effect_buff) = &effect.buff
                && effect_buff.attr_merge.is_some()
            {
                self.generate_dependent_buff_mods_for_effect(&mut reuse_rmods, ctx, item_uid, item, effect);
                self.reg_raw_mods(
                    &mut reuse_rmod_ids,
                    &mut reuse_items,
                    &mut reuse_cmods,
                    ctx,
                    item,
                    EffectSpec::new(item_uid, effect.rid),
                    &reuse_rmods,
                );
            }
        }
        // Buff maintenance - add info about effects which use default buff attributes. Done after
        // all modifiers are registered, so that buff modifiers of started effects are not generated
        // again while starting them
        for effect in effects.iter() {
            self.buffs.reg_effect(item_uid, effect);
        }
        self.reuse.rmod_ids.put(reuse_rmod_ids);
        self.reuse.rmods.put(reuse_rmods);
        self.reuse.items.put(reuse_items);
        self.reuse.cmods.put(reuse_cmods);
        // Notify RAH sim
        self.rah_effects_started(ctx, item_uid, item, effects);
    }
    pub(in crate::svc) fn effects_stopped(
        &mut self,
        ctx: &SvcCtx,
        item_uid: UItemId,
        item: &UItem,
        effects: &[RcEffect],
    ) {
        // Notify RAH sim
        self.rah_effects_stopped(ctx, item_uid, item, effects);
        // Notify core calc services
        // Buff maintenance - remove info about effects which use default buff attributes before
        // removing any modifiers. Removal can change buff attribute values, and buff modifiers
        // would be re-generated for effects which are still registered
        for effect in effects.iter() {
            self.buffs.unreg_effect(item_uid, effect);
        }
        let mut reuse_rmod_ids = self.reuse.rmod_ids.take();
        let mut reuse_items = self.reuse.items.take();
        let mut reuse_cmods = self.reuse.cmods.take();
        for effect in effects.iter() {
            let espec = EffectSpec::new(item_uid, effect.rid);
            self.rmods.remove_by_espec_p1(&mut reuse_rmod_ids, &espec);
            for &rmod_id in reuse_rmod_ids.iter() {
                // Attribute changes caused by unregistering a modifier can remove other modifiers
                // of the effect (e.g. buff modifiers which depend on attribute values), and their
                // IDs can be reused by modifiers of other effects
                let Some(rmod) = self.rmods.get_rmod_for_espec(rmod_id, &espec) else {
                    continue;
                };
                self.unreg_raw_mod(&mut reuse_items, &mut reuse_cmods, ctx, item_uid, item, rmod_id, &rmod);
                self.rmods.remove_by_espec_p2(rmod_id);
            }
            // Remove all ad-hoc attribute dependencies defined by effects being stopped. It is used
            // by e.g. custom propulsion module modifier
            self.deps.remove_by_source(&espec);
        }
        self.reuse.rmod_ids.put(reuse_rmod_ids);
        self.reuse.items.put(reuse_items);
        self.reuse.cmods.put(reuse_cmods);
    }
    pub(in crate::svc) fn effect_projected(
        &mut self,
        ctx: &SvcCtx,
        projector_espec: EffectSpec,
        projectee_uid: UItemId,
        projectee_item: &UItem,
    ) {
        // Buff maintenance - record projection before projecting, so that buff modifiers
        // re-generated during recalculations get projected too
        let effect = ctx.u_data.r_data.get_effect_by_rid(projector_espec.effect_rid);
        self.buffs.reg_projection(projector_espec, effect, projectee_uid);
        let mut reuse_cmods = self.reuse.cmods.take();
        self.std.project_effect(
            &mut reuse_cmods,
            &self.rmods,
            &projector_espec,
            projectee_uid,
            projectee_item,
        );
        let mut reuse_affectees = self.reuse.items.take();
        for cmod in reuse_cmods.iter() {
            self.force_mod_affectee_attr_recalc(&mut reuse_affectees, ctx, cmod);
        }
        self.reuse.items.put(reuse_affectees);
        self.reuse.cmods.put(reuse_cmods);
    }
    pub(in crate::svc) fn effect_proj_data_changed(
        &mut self,
        ctx: &SvcCtx,
        projector_espec: EffectSpec,
        projectee_uid: UItemId,
        projectee_item: &UItem,
    ) {
        let mut reuse_cmods = self.reuse.cmods.take();
        self.std.query_projected_effect(
            &mut reuse_cmods,
            &self.rmods,
            &projector_espec,
            projectee_uid,
            projectee_item,
        );
        let mut reuse_affectees = self.reuse.items.take();
        for cmod in reuse_cmods.iter() {
            self.force_mod_affectee_attr_recalc(&mut reuse_affectees, ctx, cmod);
        }
        self.reuse.items.put(reuse_affectees);
        self.reuse.cmods.put(reuse_cmods);
    }
    pub(in crate::svc) fn effect_unprojected(
        &mut self,
        ctx: &SvcCtx,
        projector_espec: EffectSpec,
        projectee_uid: UItemId,
        projectee_item: &UItem,
    ) {
        // Buff maintenance - remove projection info before unprojecting, so that buff modifiers
        // re-generated during recalculations do not get projected
        let effect = ctx.u_data.r_data.get_effect_by_rid(projector_espec.effect_rid);
        self.buffs.unreg_projection(projector_espec, effect, projectee_uid);
        let mut reuse_cmods = self.reuse.cmods.take();
        self.std.unproject_effect(
            &mut reuse_cmods,
            &self.rmods,
            &projector_espec,
            projectee_uid,
            projectee_item,
        );
        let mut reuse_affectees = self.reuse.items.take();
        for cmod in reuse_cmods.iter() {
            self.force_mod_affectee_attr_recalc(&mut reuse_affectees, ctx, cmod);
        }
        self.reuse.items.put(reuse_affectees);
        self.reuse.cmods.put(reuse_cmods);
    }
    pub(in crate::svc) fn attr_value_changed(&mut self, ctx: &SvcCtx, aspec: AttrSpec) {
        // Clear up attribute values which rely on passed attribute directly, e.g. as an upper/lower
        // limit
        let attr = ctx.u_data.r_data.get_attr_by_rid(aspec.attr_rid);
        for &dependent_attr_rid in attr.dependent_attr_rids.iter() {
            self.force_attr_value_recalc(ctx, AttrSpec::new(aspec.item_uid, dependent_attr_rid));
        }
        // Clear up attribute values which rely on passed attribute via ad-hoc dependencies
        let affectee_aspecs = self.deps.iter_affectee_aspecs(&aspec).copied().collect_vec();
        for affectee_aspec in affectee_aspecs.into_iter() {
            self.force_attr_value_recalc(ctx, affectee_aspec);
        }
        // Clear up attribute values which rely on passed attribute as a modification source
        let cmods = self
            .std
            .iter_affector_spec_ckeys(&aspec)
            .map(|ckey| self.rmods.get_cmod(ckey))
            .collect_vec();
        if !cmods.is_empty() {
            let mut reuse_affectees = self.reuse.items.take();
            for cmod in cmods.into_iter() {
                self.std.fill_affectees(&mut reuse_affectees, ctx, &cmod);
                for &affectee_uid in reuse_affectees.iter() {
                    let affectee_aspec = AttrSpec::new(affectee_uid, cmod.rmod.affectee_attr_rid);
                    self.force_attr_value_recalc(ctx, affectee_aspec);
                }
            }
            self.reuse.items.put(reuse_affectees);
        }
        // Process buffs which rely on attribute being modified
        if ctx.ac().buff_merge_ids.contains(&aspec.attr_rid) {
            let item = ctx.u_data.items.get(aspec.item_uid);
            // Remove modifiers of buffs which rely on the attribute
            let rmod_ids = self.buffs.extract_dependent_mods(&aspec);
            if rmod_ids.len() > 0 {
                let mut reuse_items = self.reuse.items.take();
                let mut reuse_cmods = self.reuse.cmods.take();
                let rmod_ids = rmod_ids.collect_vec();
                for rmod_id in rmod_ids.into_iter() {
                    let rmod = *self.rmods.get_rmod(rmod_id);
                    self.unproject_dependent_mod(&mut reuse_items, &mut reuse_cmods, ctx, rmod_id, rmod);
                    self.unreg_raw_mod(
                        &mut reuse_items,
                        &mut reuse_cmods,
                        ctx,
                        aspec.item_uid,
                        item,
                        rmod_id,
                        &rmod,
                    );
                    self.rmods.remove_full(rmod_id);
                }
                self.reuse.items.put(reuse_items);
                self.reuse.cmods.put(reuse_cmods);
            }
            // Generate new modifiers using new value and apply them
            let effect_rids = self.buffs.get_effects(&aspec.item_uid);
            if effect_rids.len() > 0 {
                let effect_rids = effect_rids.collect_vec();
                let rmods = self.generate_dependent_buff_mods_for_attr(
                    ctx,
                    aspec.item_uid,
                    item,
                    effect_rids.iter(),
                    aspec.attr_rid,
                );
                let mut reuse_items = self.reuse.items.take();
                let mut reuse_cmods = self.reuse.cmods.take();
                for rmod in rmods.into_iter() {
                    let rmod_id = self.rmods.add(rmod);
                    if !self.reg_raw_mod(
                        &mut reuse_items,
                        &mut reuse_cmods,
                        ctx,
                        aspec.item_uid,
                        item,
                        rmod_id,
                        rmod,
                    ) {
                        self.rmods.remove_full(rmod_id);
                        continue;
                    }
                    self.project_dependent_mod(&mut reuse_items, &mut reuse_cmods, ctx, rmod_id, rmod);
                }
                self.reuse.items.put(reuse_items);
                self.reuse.cmods.put(reuse_cmods);
            }
        }
        // Notify RAH sim
        self.rah_attr_value_changed(ctx, &aspec);
    }
    pub(in crate::svc) fn force_attr_value_recalc(&mut self, ctx: &SvcCtx, aspec: AttrSpec) {
        // Sometimes calc service receives requests to clear attributes it does not know yet; this
        // can happen in multiple cases, e.g. when adding module with charge, with "other" location
        // modifier on module. User data gets references between charge and module set right away,
        // but calculator registers module before charge, and attempts to clear charge attributes.
        // Due to cases like this, we cannot just unwrap item attribute data.
        if let Some(item_attr_data) = self.attrs.get_item_attr_data_mut(aspec.item_uid) {
            // No value calculated before that - there are no dependents to clear (dependents always
            // request dependencies while calculating their values). Removing attribute forces
            // recalculation
            if item_attr_data.unset_value(aspec.attr_rid) {
                self.attr_value_changed(ctx, aspec);
            }
        }
    }
    pub(in crate::svc::calc) fn force_oattr_postproc_recalc(
        &mut self,
        ctx: &SvcCtx,
        item_uid: UItemId,
        attr_rid: Option<RAttrId>,
    ) {
        if let Some(attr_rid) = attr_rid {
            self.force_attr_postproc_recalc(ctx, AttrSpec::new(item_uid, attr_rid));
        }
    }
    fn force_attr_postproc_recalc(&mut self, ctx: &SvcCtx, aspec: AttrSpec) {
        // Almost-copy of force recalc method without attribute removal. When something that
        // installed a postprocessing function thinks its output can change, it can let calc service
        // know about it via this method.
        if let Some(item_attr_data) = self.attrs.get_item_attr_data_mut(aspec.item_uid) {
            // No value calculated before that - there are no dependents to clear (dependents always
            // request dependencies while calculating their values). In this case we do not remove
            // attribute, because only postprocessing output is supposed to change
            if item_attr_data.has_value(&aspec.attr_rid) {
                self.attr_value_changed(ctx, aspec);
            }
        }
    }
    pub(in crate::svc) fn sol_sec_zone_changed(&mut self, ctx: &SvcCtx) {
        if let Some(sec_mod_attr_rid) = ctx.ac().security_modifier {
            for item_uid in ctx.u_data.items.keys() {
                self.force_attr_value_recalc(ctx, AttrSpec::new(item_uid, sec_mod_attr_rid))
            }
        }
    }
    pub(in crate::svc) fn fighter_count_changed(&mut self, ctx: &SvcCtx, fighter_uid: UItemId) {
        self.force_oattr_postproc_recalc(ctx, fighter_uid, ctx.ac().ftr_sq_size);
    }
    pub(in crate::svc) fn ship_sec_status_changed(&mut self, ctx: &SvcCtx, ship_uid: UItemId) {
        self.force_oattr_postproc_recalc(ctx, ship_uid, ctx.ac().pilot_security_status);
    }
    pub(in crate::svc) fn skill_level_changed(&mut self, ctx: &SvcCtx, skill_uid: UItemId) {
        self.force_oattr_postproc_recalc(ctx, skill_uid, ctx.ac().skill_level);
    }
    // Private methods
    fn reg_raw_mods(
        &mut self,
        reuse_rmod_ids: &mut Vec<RawModId>,
        reuse_items: &mut Vec<UItemId>,
        reuse_cmods: &mut Vec<CtxModifier>,
        ctx: &SvcCtx,
        item: &UItem,
        espec: EffectSpec,
        rmods: &[RawModifier],
    ) {
        // Add all modifiers of the effect in one go to avoid unnecessary allocations, and then
        // keep only those which were registered successfully
        self.rmods.add_for_espec(reuse_rmod_ids, espec, rmods);
        for (&rmod_id, &rmod) in std::iter::zip(reuse_rmod_ids.iter(), rmods.iter()) {
            if !self.reg_raw_mod(reuse_items, reuse_cmods, ctx, espec.item_uid, item, rmod_id, rmod) {
                self.rmods.remove_full(rmod_id);
            }
        }
    }
    fn reg_raw_mod(
        &mut self,
        reuse_items: &mut Vec<UItemId>,
        reuse_cmods: &mut Vec<CtxModifier>,
        ctx: &SvcCtx,
        item_uid: UItemId,
        item: &UItem,
        rmod_id: RawModId,
        rmod: RawModifier,
    ) -> bool {
        let registered = match rmod.kind {
            ModifierKind::Local => match self.std.reg_local_mod(item, rmod_id, rmod) {
                Some(cmod) => {
                    self.force_mod_affectee_attr_recalc(reuse_items, ctx, &cmod);
                    // Revisions - we need those only for local modifiers for now
                    self.revs.reg_mod(rmod_id, &cmod);
                    true
                }
                None => false,
            },
            ModifierKind::FleetBuff => {
                let registered = self.std.reg_fleet_buff_mod(reuse_cmods, ctx, item, rmod_id, rmod);
                for cmod in reuse_cmods.iter() {
                    self.force_mod_affectee_attr_recalc(reuse_items, ctx, cmod);
                }
                registered
            }
            ModifierKind::System => match item {
                UItem::SwEffect(..) => {
                    let registered = self.std.reg_sw_system_mod(reuse_cmods, ctx, rmod_id, rmod);
                    for cmod in reuse_cmods.iter() {
                        self.force_mod_affectee_attr_recalc(reuse_items, ctx, cmod);
                    }
                    registered
                }
                UItem::FwEffect(fw_effect) => match self.std.reg_fw_system_mod(fw_effect, rmod_id, rmod) {
                    Some(cmod) => {
                        self.force_mod_affectee_attr_recalc(reuse_items, ctx, &cmod);
                        true
                    }
                    None => false,
                },
                UItem::ProjEffect(..) => {
                    self.std.reg_proj_mod(rmod_id, &rmod);
                    true
                }
                _ => false,
            },
            ModifierKind::Buff => match item {
                UItem::SwEffect(..) => {
                    let registered = self.std.reg_sw_buff_mod(reuse_cmods, ctx, rmod_id, rmod);
                    for cmod in reuse_cmods.iter() {
                        self.force_mod_affectee_attr_recalc(reuse_items, ctx, cmod);
                    }
                    registered
                }
                UItem::FwEffect(fw_effect) => {
                    let registered = self.std.reg_fw_buff_mod(reuse_cmods, ctx, fw_effect, rmod_id, rmod);
                    for cmod in reuse_cmods.iter() {
                        self.force_mod_affectee_attr_recalc(reuse_items, ctx, cmod);
                    }
                    registered
                }
                _ => {
                    self.std.reg_proj_mod(rmod_id, &rmod);
                    true
                }
            },
            ModifierKind::Targeted => {
                self.std.reg_proj_mod(rmod_id, &rmod);
                true
            }
        };
        if registered {
            self.reg_raw_mod_for_buff(item_uid, rmod_id, &rmod);
        }
        registered
    }
    fn unreg_raw_mod(
        &mut self,
        reuse_items: &mut Vec<UItemId>,
        reuse_cmods: &mut Vec<CtxModifier>,
        ctx: &SvcCtx,
        item_uid: UItemId,
        item: &UItem,
        rmod_id: RawModId,
        rmod: &RawModifier,
    ) {
        // Regular modifiers
        match rmod.kind {
            ModifierKind::Local => {
                if let Some(cmod) = self.std.unreg_local_mod(item, rmod_id, *rmod) {
                    self.force_mod_affectee_attr_recalc(reuse_items, ctx, &cmod);
                    // Revisions - we need those only for local modifiers for now
                    self.revs.unreg_mod(rmod_id, &cmod);
                }
            }
            ModifierKind::FleetBuff => {
                self.std.unreg_fleet_buff_mod(reuse_cmods, ctx, item, rmod_id, *rmod);
                for cmod in reuse_cmods.iter() {
                    self.force_mod_affectee_attr_recalc(reuse_items, ctx, cmod);
                }
            }
            ModifierKind::System => match item {
                UItem::SwEffect(..) => {
                    self.std.unreg_sw_system_mod(reuse_cmods, ctx, rmod_id, *rmod);
                    for cmod in reuse_cmods.iter() {
                        self.force_mod_affectee_attr_recalc(reuse_items, ctx, cmod);
                    }
                }
                UItem::FwEffect(fw_effect) => {
                    if let Some(cmod) = self.std.unreg_fw_system_mod(fw_effect, rmod_id, *rmod) {
                        self.force_mod_affectee_attr_recalc(reuse_items, ctx, &cmod);
                    }
                }
                UItem::ProjEffect(..) => self.std.unreg_proj_mod(rmod_id, rmod),
                _ => (),
            },
            ModifierKind::Buff => match item {
                UItem::SwEffect(..) => {
                    self.std.unreg_sw_buff_mod(reuse_cmods, ctx, rmod_id, rmod);
                    for cmod in reuse_cmods.iter() {
                        self.force_mod_affectee_attr_recalc(reuse_items, ctx, cmod);
                    }
                }
                UItem::FwEffect(fw_effect) => {
                    self.std.unreg_fw_buff_mod(reuse_cmods, ctx, fw_effect, rmod_id, *rmod);
                    for cmod in reuse_cmods.iter() {
                        self.force_mod_affectee_attr_recalc(reuse_items, ctx, cmod);
                    }
                }
                _ => {
                    self.std.unreg_proj_mod(rmod_id, rmod);
                }
            },
            ModifierKind::Targeted => self.std.unreg_proj_mod(rmod_id, rmod),
        }
        self.unreg_raw_mod_for_buff(item_uid, rmod_id, rmod);
    }
    fn project_dependent_mod(
        &mut self,
        reuse_items: &mut Vec<UItemId>,
        reuse_cmods: &mut Vec<CtxModifier>,
        ctx: &SvcCtx,
        rmod_id: RawModId,
        rmod: RawModifier,
    ) {
        // Buff modifiers re-generated for an already projected effect are not covered by effect
        // projection, so they are projected onto effect projectees here
        reuse_cmods.clear();
        for projectee_uid in self.buffs.iter_projectees(&rmod.affector_espec) {
            let projectee_item = ctx.u_data.items.get(projectee_uid);
            if let Some(cmod) = self.std.project_mod(rmod_id, rmod, projectee_uid, projectee_item) {
                reuse_cmods.push(cmod);
            }
        }
        for cmod in reuse_cmods.iter() {
            self.force_mod_affectee_attr_recalc(reuse_items, ctx, cmod);
        }
    }
    fn unproject_dependent_mod(
        &mut self,
        reuse_items: &mut Vec<UItemId>,
        reuse_cmods: &mut Vec<CtxModifier>,
        ctx: &SvcCtx,
        rmod_id: RawModId,
        rmod: RawModifier,
    ) {
        reuse_cmods.clear();
        for projectee_uid in self.buffs.iter_projectees(&rmod.affector_espec) {
            let projectee_item = ctx.u_data.items.get(projectee_uid);
            if let Some(cmod) = self.std.unproject_mod(rmod_id, rmod, projectee_uid, projectee_item) {
                reuse_cmods.push(cmod);
            }
        }
        for cmod in reuse_cmods.iter() {
            self.force_mod_affectee_attr_recalc(reuse_items, ctx, cmod);
        }
    }
    fn reg_raw_mod_for_buff(&mut self, item_uid: UItemId, rmod_id: RawModId, rmod: &RawModifier) {
        if let Some(buff_type_attr_rid) = rmod.buff_type_attr_rid {
            self.buffs
                .reg_dependent_mod(AttrSpec::new(item_uid, buff_type_attr_rid), rmod_id);
        }
    }
    fn unreg_raw_mod_for_buff(&mut self, item_uid: UItemId, rmod_id: RawModId, rmod: &RawModifier) {
        if let Some(buff_type_attr_rid) = rmod.buff_type_attr_rid {
            self.buffs
                .unreg_dependent_mod(AttrSpec::new(item_uid, buff_type_attr_rid), &rmod_id);
        }
    }
    fn force_mod_affectee_attr_recalc(&mut self, reuse_affectees: &mut Vec<UItemId>, ctx: &SvcCtx, cmod: &CtxModifier) {
        self.std.fill_affectees(reuse_affectees, ctx, cmod);
        for &affectee_uid in reuse_affectees.iter() {
            self.force_attr_value_recalc(ctx, AttrSpec::new(affectee_uid, cmod.rmod.affectee_attr_rid));
        }
    }
}
