use itertools::Itertools;

use crate::{
    ad::AAttrId,
    misc::{AttrSpec, DmgKinds},
    rd::RcEffect,
    svc::{
        SvcCtx,
        calc::{AttrValInfo, Calc, CalcAttrVals, ItemAttrPostproc},
    },
    ud::{UFitId, UItem, UItemId},
};

impl Calc {
    pub(in crate::svc::calc) fn rah_item_loaded(&mut self, ctx: SvcCtx, item: &UItem) {
        if self.rah.sim_running {
            return;
        }
        if let UItem::Ship(ship) = item {
            self.clear_fit_rah_results(ctx, ship.get_fit_uid());
        }
    }
    pub(in crate::svc::calc) fn rah_item_unloaded(&mut self, ctx: SvcCtx, item: &UItem) {
        if self.rah.sim_running {
            return;
        }
        if let UItem::Ship(ship) = item {
            self.clear_fit_rah_results(ctx, ship.get_fit_uid());
        }
    }
    pub(in crate::svc::calc) fn rah_effects_started(
        &mut self,
        ctx: SvcCtx,
        item_uid: UItemId,
        item: &UItem,
        effects: &[RcEffect],
    ) {
        if self.rah.sim_running {
            return;
        }
        if let UItem::Module(module) = item
            && let Some(rah_effect_rid) = ctx.ec().adaptive_armor_hardener
            && effects.iter().any(|v| v.rid == rah_effect_rid)
        {
            let fit_uid = module.get_fit_uid();
            // Clear sim data for other RAHs on the same fit
            self.clear_fit_rah_results(ctx, fit_uid);
            // Add sim data for RAH being started
            self.rah.resonances.insert(item_uid, None);
            self.rah.by_fit.add_entry(fit_uid, item_uid);
            // Add postprocessors
            let attr_consts = ctx.ac();
            let item_attr_data = self.attrs.get_item_attr_data_mut(&item_uid).unwrap();
            if let Some(em_attr_rid) = attr_consts.armor_em_dmg_resonance {
                item_attr_data.reg_postproc(em_attr_rid, ItemAttrPostproc::RahEm);
            }
            if let Some(therm_attr_rid) = attr_consts.armor_therm_dmg_resonance {
                item_attr_data.reg_postproc(therm_attr_rid, ItemAttrPostproc::RahThermal);
            }
            if let Some(kin_attr_rid) = attr_consts.armor_kin_dmg_resonance {
                item_attr_data.reg_postproc(kin_attr_rid, ItemAttrPostproc::RahKinetic);
            }
            if let Some(expl_attr_rid) = attr_consts.armor_expl_dmg_resonance {
                item_attr_data.reg_postproc(expl_attr_rid, ItemAttrPostproc::RahExplosive);
            }
        }
    }
    pub(in crate::svc::calc) fn rah_effects_stopped(
        &mut self,
        ctx: SvcCtx,
        item_uid: &UItemId,
        item: &UItem,
        effects: &[RcEffect],
    ) {
        if self.rah.sim_running {
            return;
        }
        if let UItem::Module(module) = item
            && let Some(rah_effect_rid) = ctx.ec().adaptive_armor_hardener
            && effects.iter().any(|v| v.rid == rah_effect_rid)
        {
            let fit_uid = module.get_fit_uid();
            // Remove postprocessors
            let attr_consts = ctx.ac();
            let item_attr_data = self.attrs.get_item_attr_data_mut(item_uid).unwrap();
            if let Some(em_attr_rid) = attr_consts.armor_em_dmg_resonance {
                item_attr_data.unreg_postproc(em_attr_rid);
            }
            if let Some(therm_attr_rid) = attr_consts.armor_therm_dmg_resonance {
                item_attr_data.unreg_postproc(therm_attr_rid);
            }
            if let Some(kin_attr_rid) = attr_consts.armor_kin_dmg_resonance {
                item_attr_data.unreg_postproc(kin_attr_rid);
            }
            if let Some(expl_attr_rid) = attr_consts.armor_expl_dmg_resonance {
                item_attr_data.unreg_postproc(expl_attr_rid);
            }
            // Remove sim data for RAH being stopped
            self.rah.resonances.remove(item_uid);
            self.rah.by_fit.remove_entry(fit_uid, item_uid);
            // Clear sim data for other RAHs on the same fit
            self.clear_fit_rah_results(ctx, fit_uid);
        }
    }
    pub(in crate::svc::calc) fn rah_attr_value_changed(&mut self, ctx: SvcCtx, aspec: &AttrSpec) {
        if self.rah.sim_running {
            return;
        }
        // This is going to be called very often, no need to figure out if we need to clear results
        // if we have no RAHs running
        if self.rah.resonances.is_empty() {
            return;
        }
        let attr = ctx.u_data.src.get_attr_by_rid(aspec.attr_rid);
        match attr.aid {
            // Ship armor resonances and RAH resonances
            AAttrId::ARMOR_EM_DMG_RESONANCE
            | AAttrId::ARMOR_THERM_DMG_RESONANCE
            | AAttrId::ARMOR_KIN_DMG_RESONANCE
            | AAttrId::ARMOR_EXPL_DMG_RESONANCE => match ctx.u_data.items.get(aspec.item_uid) {
                UItem::Ship(ship) => self.clear_fit_rah_results(ctx, ship.get_fit_uid()),
                UItem::Module(module) => {
                    if self.rah.resonances.contains_key(&aspec.item_uid) {
                        self.clear_fit_rah_results(ctx, module.get_fit_uid());
                    }
                }
                _ => (),
            },
            // RAH shift amount
            AAttrId::RESIST_SHIFT_AMOUNT => {
                if self.rah.resonances.contains_key(&aspec.item_uid) {
                    // Only modules should be registered in resonances container, and those are
                    // guaranteed to have fit ID
                    let fit_uid = ctx.u_data.items.get(aspec.item_uid).get_fit_uid().unwrap();
                    self.clear_fit_rah_results(ctx, fit_uid);
                }
            }
            // RAH cycle duration
            _ if Some(aspec.attr_rid) == ctx.u_data.src.get_rah_duration_attr_rid() => {
                if self.rah.resonances.contains_key(&aspec.item_uid) {
                    // Only modules should be registered in resonances container, and those are
                    // guaranteed to have fit ID
                    let fit_uid = ctx.u_data.items.get(aspec.item_uid).get_fit_uid().unwrap();
                    // Clear only for fits with 2+ RAHs, since changing cycle duration of 1 RAH does
                    // not change sim results
                    if self.rah.by_fit.get(&fit_uid).len() >= 2 {
                        self.clear_fit_rah_results(ctx, fit_uid);
                    }
                }
            }
            // Ship HP - need to clear results since breacher DPS depends on those
            AAttrId::SHIELD_CAPACITY | AAttrId::ARMOR_HP | AAttrId::HP => {
                if let UItem::Ship(ship) = ctx.u_data.items.get(aspec.item_uid) {
                    let fit_uid = ship.get_fit_uid();
                    if ctx.u_data.get_fit_uid_rah_incoming_dps(fit_uid).deals_breacher_dps() {
                        self.clear_fit_rah_results(ctx, fit_uid);
                    }
                }
            }
            _ => (),
        }
    }
    pub(in crate::svc::calc) fn rah_fit_rah_dps_profile_changed(&mut self, ctx: SvcCtx, fit_uid: UFitId) {
        self.clear_fit_rah_results(ctx, fit_uid);
    }
    // Private methods
    fn clear_fit_rah_results(&mut self, ctx: SvcCtx, fit_uid: UFitId) {
        let rah_uids = self.rah.by_fit.get(&fit_uid).copied().collect_vec();
        for rah_uid in rah_uids {
            self.clear_rah_result(ctx, rah_uid);
        }
    }
    fn clear_rah_result(&mut self, ctx: SvcCtx, item_uid: UItemId) {
        if self.rah.resonances.get_mut(&item_uid).unwrap().take().is_some() {
            let attr_consts = ctx.ac();
            self.force_oattr_postproc_recalc(ctx, item_uid, attr_consts.armor_em_dmg_resonance);
            self.force_oattr_postproc_recalc(ctx, item_uid, attr_consts.armor_therm_dmg_resonance);
            self.force_oattr_postproc_recalc(ctx, item_uid, attr_consts.armor_kin_dmg_resonance);
            self.force_oattr_postproc_recalc(ctx, item_uid, attr_consts.armor_expl_dmg_resonance);
        }
    }
}
