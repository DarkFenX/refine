use itertools::Itertools;

use crate::{
    misc::{AttrSpec, EffectSpec},
    rd::{RAttrId, RData, REffect, REffectId},
    svc::{
        SvcCtx,
        calc::{
            Calc, CtxModifier, ItemAddRemoveReviser, ModifierKind, RawModId, RawModSlot, RawModTemplates, RawModifier,
            registers::{RegOp, StandardRegister},
        },
    },
    ud::{UFitId, UFleet, UItem, UItemId},
};

impl Calc {
    // Modification methods
    pub(in crate::svc) fn fit_added(&mut self, templates: &RawModTemplates, fit_uid: UFitId) {
        self.std
            .process_fit_for_sw::<{ RegOp::Reg }>(templates, &self.rmods, fit_uid)
    }
    pub(in crate::svc) fn fit_removed(&mut self, templates: &RawModTemplates, fit_uid: UFitId) {
        self.std
            .process_fit_for_sw::<{ RegOp::Unreg }>(templates, &self.rmods, fit_uid)
    }
    pub(in crate::svc) fn fit_added_to_fleet(&mut self, ctx: &SvcCtx, fleet: &UFleet, fit_uid: UFitId) {
        self.process_fleet_fit::<{ RegOp::Reg }>(ctx, fleet, fit_uid);
    }
    pub(in crate::svc) fn fit_removed_from_fleet(&mut self, ctx: &SvcCtx, fleet: &UFleet, fit_uid: UFitId) {
        self.process_fleet_fit::<{ RegOp::Unreg }>(ctx, fleet, fit_uid);
    }
    pub(in crate::svc) fn fit_rah_dps_profile_changed(&mut self, ctx: &SvcCtx, fit_uid: UFitId) {
        self.rah_fit_rah_dps_profile_changed(ctx, fit_uid);
    }
    pub(in crate::svc) fn item_added(&mut self, ctx: &SvcCtx, item_uid: UItemId, item: &UItem) {
        // Custom modifiers
        for reviser in ItemAddRemoveReviser::ALL {
            if !reviser.reacts_to(item) {
                continue;
            }
            let cmods = self
                .revs
                .iter_revs_on_item_add_remove(reviser)
                .filter_map(|ckey| {
                    let cmod = self.rmods.get_cmod(ctx.rmt(), ckey);
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
    }
    pub(in crate::svc) fn item_removed(&mut self, ctx: &SvcCtx, item_uid: UItemId, item: &UItem) {
        // Custom modifiers
        for reviser in ItemAddRemoveReviser::ALL {
            if !reviser.reacts_to(item) {
                continue;
            }
            let cmods = self
                .revs
                .iter_revs_on_item_add_remove(reviser)
                .filter_map(|ckey| {
                    let cmod = self.rmods.get_cmod(ctx.rmt(), ckey);
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
    }
    pub(in crate::svc) fn item_loaded(&mut self, ctx: &SvcCtx, item_uid: UItemId, item: &UItem) {
        // Notify core calc services
        self.attrs.item_loaded(ctx.u_data, item_uid, item);
        self.process_affectee::<{ RegOp::Reg }>(ctx, item_uid, item);
        // Notify RAH sim
        self.rah_item_loaded(ctx, item);
    }
    pub(in crate::svc) fn item_unloaded(&mut self, ctx: &SvcCtx, item_uid: UItemId, item: &UItem) {
        // Notify RAH sim
        self.rah_item_unloaded(ctx, item);
        // Notify core calc services
        self.process_affectee::<{ RegOp::Unreg }>(ctx, item_uid, item);
        self.deps.remove_item(item_uid);
        self.attrs.item_unloaded(item_uid);
    }
    pub(in crate::svc) fn effects_started(
        &mut self,
        ctx: &SvcCtx,
        item_uid: UItemId,
        item: &UItem,
        effect_rids: &[REffectId],
    ) {
        let r_data: &RData = &ctx.u_data.r_data;
        let effects = effect_rids
            .iter()
            .map(|&effect_rid| r_data.get_effect_by_rid(effect_rid));
        // Notify core calc services
        let mut reuse_items = self.reuse.items.take();
        let mut needs_rmod_gen = false;
        for effect in effects.clone() {
            for (rmod_id, rmod) in r_data.get_rmod_templates().iter_effect(effect, item_uid) {
                self.reg_raw_mod(&mut reuse_items, ctx, item_uid, item, rmod_id, rmod);
            }
            if effect.needs_rmod_gen {
                needs_rmod_gen = true;
                self.reg_stored_rmods(&mut reuse_items, ctx, item_uid, item, effect);
            }
        }
        // Effects with buffs which rely on on-item attributes always need modifier generation
        if needs_rmod_gen {
            self.reg_buff_effects(&mut reuse_items, ctx, item_uid, item, effects);
        }
        self.reuse.items.put(reuse_items);
        // Notify RAH sim
        self.rah_effects_started(ctx, item_uid, item, effect_rids);
    }
    pub(in crate::svc) fn effects_stopped(
        &mut self,
        ctx: &SvcCtx,
        item_uid: UItemId,
        item: &UItem,
        effect_rids: &[REffectId],
    ) {
        let r_data: &RData = &ctx.u_data.r_data;
        // Notify RAH sim
        self.rah_effects_stopped(ctx, item_uid, item, effect_rids);
        // Notify core calc services
        // Buff maintenance - remove info about effects which use default buff attributes before
        // removing any modifiers. Removal can change buff attribute values, and buff modifiers
        // would be re-generated for effects which are still registered
        if self.buffs.has_effects() {
            for &effect_rid in effect_rids {
                self.buffs.unreg_effect(item_uid, r_data.get_effect_by_rid(effect_rid));
            }
        }
        let mut reuse_items = self.reuse.items.take();
        for &effect_rid in effect_rids {
            let effect = r_data.get_effect_by_rid(effect_rid);
            // Includes modifiers which failed to register, which unregistration skips the same way
            for (rmod_id, rmod) in r_data.get_rmod_templates().iter_effect(effect, item_uid) {
                self.unreg_raw_mod(&mut reuse_items, ctx, item_uid, item, rmod_id, &rmod);
            }
            if effect.needs_rmod_gen {
                self.unreg_stored_rmods(&mut reuse_items, ctx, item_uid, item, effect_rid);
            }
        }
        self.reuse.items.put(reuse_items);
    }
    pub(in crate::svc) fn effect_projected(
        &mut self,
        ctx: &SvcCtx,
        projector_espec: EffectSpec,
        effect: &REffect,
        projectee_uid: UItemId,
        projectee_item: &UItem,
    ) {
        self.process_effect_projection::<{ RegOp::Reg }>(ctx, projector_espec, effect, projectee_uid, projectee_item);
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
            ctx.rmt(),
            &self.rmods,
            projector_espec,
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
        effect: &REffect,
        projectee_uid: UItemId,
        projectee_item: &UItem,
    ) {
        self.process_effect_projection::<{ RegOp::Unreg }>(ctx, projector_espec, effect, projectee_uid, projectee_item);
    }
    pub(in crate::svc) fn attr_value_changed(&mut self, ctx: &SvcCtx, aspec: AttrSpec) {
        // Clear up attribute values which rely on passed attribute directly, e.g. as an upper/lower
        // limit
        let attr = ctx.u_data.r_data.get_attr_by_rid(aspec.attr_rid);
        for &dependent_attr_rid in attr.dependent_attr_rids.iter() {
            self.force_attr_value_recalc(ctx, AttrSpec::new(aspec.item_uid, dependent_attr_rid));
        }
        // Clear up attribute values which rely on passed attribute via ad-hoc dependencies
        let affectee_aspecs = self.deps.iter_affectee_aspecs(aspec).collect_vec();
        for affectee_aspec in affectee_aspecs.into_iter() {
            self.force_attr_value_recalc(ctx, affectee_aspec);
        }
        // Clear up attribute values which rely on passed attribute as a modification source
        let mut cmods = self
            .std
            .iter_affector_spec_ckeys(aspec)
            .map(|ckey| self.rmods.get_cmod(ctx.rmt(), ckey))
            .collect_vec();
        // Local template modifiers are not registered by affector attribute, and have to be handled
        // here. They are found through the item's running effects, which take in an effect before
        // its modifiers register and let it go only after they unregister, so every registered one
        // is found; during a start or stop a few unregistered ones are too, which only adds
        // invalidations their registration or unregistration does anyway
        let item = ctx.u_data.items.get(aspec.item_uid);
        if attr.local_rmt_affector
            && let Some(item_rib) = item.get_r_item_base()
            && let Some(reffs) = item.get_reffs()
        {
            for &effect_rid in ctx
                .u_data
                .r_data
                .get_local_rmt_affector_effects(item_rib, aspec.attr_rid)
            {
                if !reffs.contains(effect_rid) {
                    continue;
                }
                let effect = ctx.u_data.r_data.get_effect_by_rid(effect_rid);
                for rmod in ctx
                    .rmt()
                    .iter_effect_local_by_affector_attr(effect, aspec.item_uid, aspec.attr_rid)
                {
                    if let Some(cmod) = StandardRegister::query_local_mod(item, rmod) {
                        cmods.push(cmod);
                    }
                }
            }
        }
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
            let rmod_ids = self.buffs.extract_dependent_mods(aspec);
            if rmod_ids.len() > 0 {
                let mut reuse_items = self.reuse.items.take();
                let mut reuse_cmods = self.reuse.cmods.take();
                let rmod_ids = rmod_ids.collect_vec();
                for rmod_id in rmod_ids.into_iter() {
                    let rmod = self.rmods.get_rmod(ctx.rmt(), rmod_id);
                    self.unproject_dependent_mod(&mut reuse_items, &mut reuse_cmods, ctx, rmod_id, rmod);
                    self.unreg_raw_mod(&mut reuse_items, ctx, aspec.item_uid, item, rmod_id, &rmod);
                    self.rmods.remove_full(rmod_id.expect_slot());
                }
                self.reuse.items.put(reuse_items);
                self.reuse.cmods.put(reuse_cmods);
            }
            // Generate new modifiers using new value and apply them
            let effect_rids = self.buffs.get_effects(aspec.item_uid);
            if effect_rids.len() > 0 {
                let effect_rids = effect_rids.collect_vec();
                let rmods = self.generate_dependent_buff_mods_for_attr(
                    ctx,
                    aspec.item_uid,
                    item,
                    effect_rids.into_iter(),
                    aspec.attr_rid,
                );
                let mut reuse_items = self.reuse.items.take();
                let mut reuse_cmods = self.reuse.cmods.take();
                for rmod in rmods.into_iter() {
                    let slot = self.rmods.add(rmod);
                    let rmod_id = RawModId::from_slot(slot);
                    if !self.reg_raw_mod(&mut reuse_items, ctx, aspec.item_uid, item, rmod_id, rmod) {
                        self.rmods.remove_full(slot);
                        continue;
                    }
                    self.project_dependent_mod(&mut reuse_items, &mut reuse_cmods, ctx, rmod_id, rmod);
                }
                self.reuse.items.put(reuse_items);
                self.reuse.cmods.put(reuse_cmods);
            }
        }
        // Notify RAH sim
        self.rah_attr_value_changed(ctx, aspec);
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
            if item_attr_data.has_value(aspec.attr_rid) {
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
    fn process_affectee<const OP: RegOp>(&mut self, ctx: &SvcCtx, item_uid: UItemId, item: &UItem) {
        // A ship gives its fit the location kind, under which items on ship or structure root are
        // registered: before its registration collects modifiers to invalidate, and until those
        // are invalidated on its unregistration
        let ship_fit_loc = match item {
            UItem::Ship(ship) => item.get_ship_loc_kind().map(|loc_kind| (ship.get_fit_uid(), loc_kind)),
            _ => None,
        };
        if OP == RegOp::Reg
            && let Some((fit_uid, loc_kind)) = ship_fit_loc
        {
            self.std.activate_fit_loc(ctx.u_data, fit_uid, loc_kind);
        }
        let mut reuse_cmods = self.reuse.cmods.take();
        self.std
            .process_affectee::<OP>(&mut reuse_cmods, ctx, &self.rmods, item_uid, item);
        if !reuse_cmods.is_empty() {
            let mut reuse_affectees = self.reuse.items.take();
            for cmod in reuse_cmods.iter() {
                self.force_mod_affectee_attr_recalc(&mut reuse_affectees, ctx, cmod)
            }
            self.reuse.items.put(reuse_affectees);
        }
        self.reuse.cmods.put(reuse_cmods);
        if OP == RegOp::Unreg
            && let Some((fit_uid, _)) = ship_fit_loc
        {
            self.std.deactivate_fit_loc(ctx.u_data, fit_uid);
        }
    }
    fn process_effect_projection<const OP: RegOp>(
        &mut self,
        ctx: &SvcCtx,
        projector_espec: EffectSpec,
        effect: &REffect,
        projectee_uid: UItemId,
        projectee_item: &UItem,
    ) {
        // Buff maintenance - record projection before projecting, and remove it before
        // unprojecting, so that buff modifiers re-generated during recalculations get projected
        // only while the effect is projected
        match OP {
            RegOp::Reg => self.buffs.reg_projection(projector_espec, effect, projectee_uid),
            RegOp::Unreg => self.buffs.unreg_projection(projector_espec, effect, projectee_uid),
        }
        let mut reuse_cmods = self.reuse.cmods.take();
        self.std.process_effect_projection::<OP>(
            &mut reuse_cmods,
            ctx.rmt(),
            &self.rmods,
            projector_espec,
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
    fn process_fleet_fit<const OP: RegOp>(&mut self, ctx: &SvcCtx, fleet: &UFleet, fit_uid: UFitId) {
        let mut reuse_cmods = self.reuse.cmods.take();
        self.std
            .process_fleet_for_fit::<OP>(&mut reuse_cmods, ctx, &self.rmods, fleet, fit_uid);
        let mut reuse_affectees = self.reuse.items.take();
        for cmod in reuse_cmods.iter() {
            self.force_mod_affectee_attr_recalc(&mut reuse_affectees, ctx, cmod);
        }
        self.reuse.items.put(reuse_affectees);
        self.reuse.cmods.put(reuse_cmods);
    }
    fn reg_stored_rmods(
        &mut self,
        reuse_items: &mut Vec<UItemId>,
        ctx: &SvcCtx,
        item_uid: UItemId,
        item: &UItem,
        effect: &REffect,
    ) {
        let mut reuse_rmod_slots = self.reuse.rmod_slots.take();
        let mut reuse_rmods = self.reuse.rmods.take();
        self.generate_mods_for_effect(&mut reuse_rmods, ctx.u_data, item_uid, item, effect);
        self.reg_raw_mods(
            &mut reuse_rmod_slots,
            reuse_items,
            ctx,
            item,
            EffectSpec::new(item_uid, effect.rid),
            &reuse_rmods,
        );
        self.reuse.rmod_slots.put(reuse_rmod_slots);
        self.reuse.rmods.put(reuse_rmods);
    }
    fn unreg_stored_rmods(
        &mut self,
        reuse_items: &mut Vec<UItemId>,
        ctx: &SvcCtx,
        item_uid: UItemId,
        item: &UItem,
        effect_rid: REffectId,
    ) {
        let mut reuse_rmod_slots = self.reuse.rmod_slots.take();
        let espec = EffectSpec::new(item_uid, effect_rid);
        self.rmods.remove_by_espec_p1(&mut reuse_rmod_slots, espec);
        for &slot in reuse_rmod_slots.iter() {
            // Attribute changes caused by unregistering a modifier can remove other modifiers of
            // the effect (e.g. buff modifiers which depend on attribute values), and their slots
            // can be reused by modifiers of other effects
            let Some(rmod) = self.rmods.get_rmod_for_espec(slot, espec) else {
                continue;
            };
            let rmod_id = RawModId::from_slot(slot);
            self.unreg_raw_mod(reuse_items, ctx, item_uid, item, rmod_id, &rmod);
            self.rmods.remove_by_espec_p2(slot);
        }
        self.reuse.rmod_slots.put(reuse_rmod_slots);
    }
    fn reg_buff_effects<'e>(
        &mut self,
        reuse_items: &mut Vec<UItemId>,
        ctx: &SvcCtx,
        item_uid: UItemId,
        item: &UItem,
        effects: impl Clone + Iterator<Item = &'e REffect>,
    ) {
        let mut reuse_rmod_slots = self.reuse.rmod_slots.take();
        let mut reuse_rmods = self.reuse.rmods.take();
        // Buffs which rely on on-item attributes are generated after all other modifiers of started
        // effects are registered, since those modifiers can change applied buff ID
        for effect in effects.clone() {
            if let Some(effect_buff) = &effect.buff
                && effect_buff.attr_merge.is_some()
            {
                self.generate_dependent_buff_mods_for_effect(&mut reuse_rmods, ctx, item_uid, item, effect);
                self.reg_raw_mods(
                    &mut reuse_rmod_slots,
                    reuse_items,
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
        for effect in effects {
            self.buffs.reg_effect(item_uid, effect);
        }
        self.reuse.rmod_slots.put(reuse_rmod_slots);
        self.reuse.rmods.put(reuse_rmods);
    }
    fn reg_raw_mods(
        &mut self,
        reuse_rmod_slots: &mut Vec<RawModSlot>,
        reuse_items: &mut Vec<UItemId>,
        ctx: &SvcCtx,
        item: &UItem,
        espec: EffectSpec,
        rmods: &[RawModifier],
    ) {
        // Add all modifiers of the effect in one go to avoid unnecessary allocations, and then
        // keep only those which were registered successfully
        self.rmods.add_for_espec(reuse_rmod_slots, espec, rmods);
        for (&slot, &rmod) in std::iter::zip(reuse_rmod_slots.iter(), rmods.iter()) {
            let rmod_id = RawModId::from_slot(slot);
            if !self.reg_raw_mod(reuse_items, ctx, espec.item_uid, item, rmod_id, rmod) {
                self.rmods.remove_full(slot);
            }
        }
    }
    fn reg_raw_mod(
        &mut self,
        reuse_items: &mut Vec<UItemId>,
        ctx: &SvcCtx,
        item_uid: UItemId,
        item: &UItem,
        rmod_id: RawModId,
        rmod: RawModifier,
    ) -> bool {
        self.process_raw_mod::<{ RegOp::Reg }>(reuse_items, ctx, item_uid, item, rmod_id, rmod)
    }
    fn unreg_raw_mod(
        &mut self,
        reuse_items: &mut Vec<UItemId>,
        ctx: &SvcCtx,
        item_uid: UItemId,
        item: &UItem,
        rmod_id: RawModId,
        rmod: &RawModifier,
    ) {
        self.process_raw_mod::<{ RegOp::Unreg }>(reuse_items, ctx, item_uid, item, rmod_id, *rmod);
    }
    fn process_raw_mod<const OP: RegOp>(
        &mut self,
        reuse_items: &mut Vec<UItemId>,
        ctx: &SvcCtx,
        item_uid: UItemId,
        item: &UItem,
        rmod_id: RawModId,
        rmod: RawModifier,
    ) -> bool {
        let processed = match rmod.kind {
            ModifierKind::Local => match self.std.process_local_mod::<OP>(item, rmod_id, rmod) {
                Some(cmod) => {
                    self.force_mod_affectee_attr_recalc(reuse_items, ctx, &cmod);
                    // Revisions - we need those only for local modifiers for now
                    match OP {
                        RegOp::Reg => self.revs.reg_mod(rmod_id, &cmod),
                        RegOp::Unreg => self.revs.unreg_mod(rmod_id, &cmod),
                    }
                    true
                }
                None => false,
            },
            ModifierKind::FleetBuff => {
                let mut reuse_cmods = self.reuse.cmods.take();
                let processed = self
                    .std
                    .process_fleet_buff_mod::<OP>(&mut reuse_cmods, ctx, item, rmod_id, rmod);
                for cmod in reuse_cmods.iter() {
                    self.force_mod_affectee_attr_recalc(reuse_items, ctx, cmod);
                }
                self.reuse.cmods.put(reuse_cmods);
                processed
            }
            ModifierKind::System => match item {
                UItem::SwEffect(..) => {
                    let mut reuse_cmods = self.reuse.cmods.take();
                    let processed = self
                        .std
                        .process_sw_system_mod::<OP>(&mut reuse_cmods, ctx, rmod_id, rmod);
                    for cmod in reuse_cmods.iter() {
                        self.force_mod_affectee_attr_recalc(reuse_items, ctx, cmod);
                    }
                    self.reuse.cmods.put(reuse_cmods);
                    processed
                }
                UItem::FwEffect(fw_effect) => match self.std.process_fw_system_mod::<OP>(fw_effect, rmod_id, rmod) {
                    Some(cmod) => {
                        self.force_mod_affectee_attr_recalc(reuse_items, ctx, &cmod);
                        true
                    }
                    None => false,
                },
                UItem::ProjEffect(..) => {
                    self.std.process_proj_mod::<OP>(rmod_id, &rmod);
                    true
                }
                _ => false,
            },
            ModifierKind::Buff => match item {
                UItem::SwEffect(..) => {
                    let mut reuse_cmods = self.reuse.cmods.take();
                    let processed = self.std.process_sw_buff_mod::<OP>(&mut reuse_cmods, ctx, rmod_id, rmod);
                    for cmod in reuse_cmods.iter() {
                        self.force_mod_affectee_attr_recalc(reuse_items, ctx, cmod);
                    }
                    self.reuse.cmods.put(reuse_cmods);
                    processed
                }
                UItem::FwEffect(fw_effect) => {
                    let mut reuse_cmods = self.reuse.cmods.take();
                    let processed = self
                        .std
                        .process_fw_buff_mod::<OP>(&mut reuse_cmods, ctx, fw_effect, rmod_id, rmod);
                    for cmod in reuse_cmods.iter() {
                        self.force_mod_affectee_attr_recalc(reuse_items, ctx, cmod);
                    }
                    self.reuse.cmods.put(reuse_cmods);
                    processed
                }
                _ => {
                    self.std.process_proj_mod::<OP>(rmod_id, &rmod);
                    true
                }
            },
            ModifierKind::Targeted => {
                self.std.process_proj_mod::<OP>(rmod_id, &rmod);
                true
            }
        };
        if processed {
            match OP {
                RegOp::Reg => self.reg_raw_mod_for_buff(item_uid, rmod_id, &rmod),
                RegOp::Unreg => {
                    // Ad-hoc attribute dependencies registered during calculation, e.g. by custom
                    // propulsion module modifier
                    if rmod.adds_deps_with_source() {
                        self.deps.remove_by_source(rmod.affector_espec);
                    }
                    self.unreg_raw_mod_for_buff(item_uid, rmod_id, &rmod);
                }
            }
        }
        processed
    }
    fn project_dependent_mod(
        &mut self,
        reuse_items: &mut Vec<UItemId>,
        reuse_cmods: &mut Vec<CtxModifier>,
        ctx: &SvcCtx,
        rmod_id: RawModId,
        rmod: RawModifier,
    ) {
        self.process_dependent_mod_projection::<{ RegOp::Reg }>(reuse_items, reuse_cmods, ctx, rmod_id, rmod);
    }
    fn unproject_dependent_mod(
        &mut self,
        reuse_items: &mut Vec<UItemId>,
        reuse_cmods: &mut Vec<CtxModifier>,
        ctx: &SvcCtx,
        rmod_id: RawModId,
        rmod: RawModifier,
    ) {
        self.process_dependent_mod_projection::<{ RegOp::Unreg }>(reuse_items, reuse_cmods, ctx, rmod_id, rmod);
    }
    fn process_dependent_mod_projection<const OP: RegOp>(
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
        for projectee_uid in self.buffs.iter_projectees(rmod.affector_espec) {
            let projectee_item = ctx.u_data.items.get(projectee_uid);
            if let Some(cmod) = self
                .std
                .process_mod_projection::<OP>(rmod_id, rmod, projectee_uid, projectee_item)
            {
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
                .unreg_dependent_mod(AttrSpec::new(item_uid, buff_type_attr_rid), rmod_id);
        }
    }
    fn force_mod_affectee_attr_recalc(&mut self, reuse_affectees: &mut Vec<UItemId>, ctx: &SvcCtx, cmod: &CtxModifier) {
        self.std.fill_affectees(reuse_affectees, ctx, cmod);
        for &affectee_uid in reuse_affectees.iter() {
            self.force_attr_value_recalc(ctx, AttrSpec::new(affectee_uid, cmod.rmod.affectee_attr_rid));
        }
    }
}
