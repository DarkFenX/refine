use itertools::Itertools;

use crate::{
    ac,
    misc::{AttrSpec, DmgKinds},
    rd::RcEffect,
    svc::{
        SvcCtx,
        calc::{AttrValInfo, Calc, CalcAttrVals, ItemAttrPostprocs},
    },
    ud::{UFitId, UItem, UItemId},
};

impl Calc {
    pub(in crate::svc::calc) fn rah_item_loaded(&mut self, ctx: SvcCtx, item: &UItem) {
        if self.rah.sim_running {
            return;
        }
        if let UItem::Ship(ship) = item {
            self.clear_fit_rah_results(ctx, ship.get_fit_key());
        }
    }
    pub(in crate::svc::calc) fn rah_item_unloaded(&mut self, ctx: SvcCtx, item: &UItem) {
        if self.rah.sim_running {
            return;
        }
        if let UItem::Ship(ship) = item {
            self.clear_fit_rah_results(ctx, ship.get_fit_key());
        }
    }
    pub(in crate::svc::calc) fn rah_effects_started(
        &mut self,
        ctx: SvcCtx,
        item_key: UItemId,
        item: &UItem,
        effects: &[RcEffect],
    ) {
        if self.rah.sim_running {
            return;
        }
        if let UItem::Module(module) = item
            && let Some(rah_effect_key) = ctx.ec().adaptive_armor_hardener
            && effects.iter().any(|v| v.key == rah_effect_key)
        {
            let fit_key = module.get_fit_key();
            // Clear sim data for other RAHs on the same fit
            self.clear_fit_rah_results(ctx, fit_key);
            // Add sim data for RAH being started
            self.rah.resonances.insert(item_key, None);
            self.rah.by_fit.add_entry(fit_key, item_key);
            // Add postprocessors
            let attr_consts = ctx.ac();
            let item_attr_data = self.attrs.get_item_attr_data_mut(&item_key).unwrap();
            if let Some(em_attr_key) = attr_consts.armor_em_dmg_resonance {
                item_attr_data.reg_postproc(
                    em_attr_key,
                    ItemAttrPostprocs {
                        fast: rah_em_resonance_postproc_fast,
                        info: rah_em_resonance_postproc_info,
                    },
                );
            }
            if let Some(therm_attr_key) = attr_consts.armor_therm_dmg_resonance {
                item_attr_data.reg_postproc(
                    therm_attr_key,
                    ItemAttrPostprocs {
                        fast: rah_therm_resonance_postproc_fast,
                        info: rah_therm_resonance_postproc_info,
                    },
                );
            }
            if let Some(kin_attr_key) = attr_consts.armor_kin_dmg_resonance {
                item_attr_data.reg_postproc(
                    kin_attr_key,
                    ItemAttrPostprocs {
                        fast: rah_kin_resonance_postproc_fast,
                        info: rah_kin_resonance_postproc_info,
                    },
                );
            }
            if let Some(expl_attr_key) = attr_consts.armor_expl_dmg_resonance {
                item_attr_data.reg_postproc(
                    expl_attr_key,
                    ItemAttrPostprocs {
                        fast: rah_expl_resonance_postproc_fast,
                        info: rah_expl_resonance_postproc_info,
                    },
                );
            }
        }
    }
    pub(in crate::svc::calc) fn rah_effects_stopped(
        &mut self,
        ctx: SvcCtx,
        item_key: &UItemId,
        item: &UItem,
        effects: &[RcEffect],
    ) {
        if self.rah.sim_running {
            return;
        }
        if let UItem::Module(module) = item
            && let Some(rah_effect_key) = ctx.ec().adaptive_armor_hardener
            && effects.iter().any(|v| v.key == rah_effect_key)
        {
            let fit_key = module.get_fit_key();
            // Remove postprocessors
            let attr_consts = ctx.ac();
            let item_attr_data = self.attrs.get_item_attr_data_mut(item_key).unwrap();
            if let Some(em_attr_key) = attr_consts.armor_em_dmg_resonance {
                item_attr_data.unreg_postproc(em_attr_key);
            }
            if let Some(therm_attr_key) = attr_consts.armor_therm_dmg_resonance {
                item_attr_data.unreg_postproc(therm_attr_key);
            }
            if let Some(kin_attr_key) = attr_consts.armor_kin_dmg_resonance {
                item_attr_data.unreg_postproc(kin_attr_key);
            }
            if let Some(expl_attr_key) = attr_consts.armor_expl_dmg_resonance {
                item_attr_data.unreg_postproc(expl_attr_key);
            }
            // Remove sim data for RAH being stopped
            self.rah.resonances.remove(item_key);
            self.rah.by_fit.remove_entry(fit_key, item_key);
            // Clear sim data for other RAHs on the same fit
            self.clear_fit_rah_results(ctx, fit_key);
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
        let attr = ctx.u_data.src.get_attr(aspec.attr_key);
        match attr.id {
            // Ship armor resonances and RAH resonances
            ac::attrs::ARMOR_EM_DMG_RESONANCE
            | ac::attrs::ARMOR_THERM_DMG_RESONANCE
            | ac::attrs::ARMOR_KIN_DMG_RESONANCE
            | ac::attrs::ARMOR_EXPL_DMG_RESONANCE => match ctx.u_data.items.get(aspec.item_key) {
                UItem::Ship(ship) => self.clear_fit_rah_results(ctx, ship.get_fit_key()),
                UItem::Module(module) => {
                    if self.rah.resonances.contains_key(&aspec.item_key) {
                        self.clear_fit_rah_results(ctx, module.get_fit_key());
                    }
                }
                _ => (),
            },
            // RAH shift amount
            ac::attrs::RESIST_SHIFT_AMOUNT => {
                if self.rah.resonances.contains_key(&aspec.item_key) {
                    // Only modules should be registered in resonances container, and those are
                    // guaranteed to have fit ID
                    let fit_key = ctx.u_data.items.get(aspec.item_key).get_fit_key().unwrap();
                    self.clear_fit_rah_results(ctx, fit_key);
                }
            }
            // RAH cycle time
            _ if Some(aspec.attr_key) == ctx.u_data.src.get_rah_duration_attr_key() => {
                if self.rah.resonances.contains_key(&aspec.item_key) {
                    // Only modules should be registered in resonances container, and those are
                    // guaranteed to have fit ID
                    let fit_key = ctx.u_data.items.get(aspec.item_key).get_fit_key().unwrap();
                    // Clear only for fits with 2+ RAHs, since changing cycle time of 1 RAH does not
                    // change sim results
                    if self.rah.by_fit.get(&fit_key).len() >= 2 {
                        self.clear_fit_rah_results(ctx, fit_key);
                    }
                }
            }
            // Ship HP - need to clear results since breacher DPS depends on those
            ac::attrs::SHIELD_CAPACITY | ac::attrs::ARMOR_HP | ac::attrs::HP => {
                if let UItem::Ship(ship) = ctx.u_data.items.get(aspec.item_key) {
                    let fit_key = ship.get_fit_key();
                    if ctx.u_data.get_fit_key_rah_incoming_dps(fit_key).deals_breacher_dps() {
                        self.clear_fit_rah_results(ctx, fit_key);
                    }
                }
            }
            _ => (),
        }
    }
    pub(in crate::svc::calc) fn rah_fit_rah_dps_profile_changed(&mut self, ctx: SvcCtx, fit_key: UFitId) {
        self.clear_fit_rah_results(ctx, fit_key);
    }
    // Private methods
    fn clear_fit_rah_results(&mut self, ctx: SvcCtx, fit_key: UFitId) {
        let rah_keys = self.rah.by_fit.get(&fit_key).copied().collect_vec();
        for rah_key in rah_keys {
            self.clear_rah_result(ctx, rah_key);
        }
    }
    fn clear_rah_result(&mut self, ctx: SvcCtx, item_key: UItemId) {
        if self.rah.resonances.get_mut(&item_key).unwrap().take().is_some() {
            let attr_consts = ctx.ac();
            self.force_oattr_postproc_recalc(ctx, item_key, attr_consts.armor_em_dmg_resonance);
            self.force_oattr_postproc_recalc(ctx, item_key, attr_consts.armor_therm_dmg_resonance);
            self.force_oattr_postproc_recalc(ctx, item_key, attr_consts.armor_kin_dmg_resonance);
            self.force_oattr_postproc_recalc(ctx, item_key, attr_consts.armor_expl_dmg_resonance);
        }
    }
    fn get_rah_resonances(&mut self, ctx: SvcCtx, item_key: UItemId) -> DmgKinds<CalcAttrVals> {
        // Unwrap item, since method is supposed to be called only for registered RAHs
        if let Some(val) = self.rah.resonances.get(&item_key).unwrap() {
            return *val;
        }
        // Unwrap fit ID, since registered RAHs are supposed to be modules, which have fit ID
        let fit_key = ctx.u_data.items.get(item_key).get_fit_key().unwrap();
        self.rah.sim_running = true;
        self.rah_run_simulation(ctx, fit_key);
        self.rah.sim_running = false;
        // Unwrap value, since simulation is supposed to always set results for RAHs of requested
        // fit
        self.rah.resonances.get(&item_key).unwrap().unwrap()
    }
}

fn rah_em_resonance_postproc_fast(
    calc: &mut Calc,
    ctx: SvcCtx,
    item_key: UItemId,
    _cval: CalcAttrVals,
) -> CalcAttrVals {
    calc.get_rah_resonances(ctx, item_key).em
}
fn rah_therm_resonance_postproc_fast(
    calc: &mut Calc,
    ctx: SvcCtx,
    item_key: UItemId,
    _cval: CalcAttrVals,
) -> CalcAttrVals {
    calc.get_rah_resonances(ctx, item_key).thermal
}
fn rah_kin_resonance_postproc_fast(
    calc: &mut Calc,
    ctx: SvcCtx,
    item_key: UItemId,
    _cval: CalcAttrVals,
) -> CalcAttrVals {
    calc.get_rah_resonances(ctx, item_key).kinetic
}
fn rah_expl_resonance_postproc_fast(
    calc: &mut Calc,
    ctx: SvcCtx,
    item_key: UItemId,
    _cval: CalcAttrVals,
) -> CalcAttrVals {
    calc.get_rah_resonances(ctx, item_key).explosive
}

fn rah_em_resonance_postproc_info(
    calc: &mut Calc,
    ctx: SvcCtx,
    item_key: UItemId,
    mut info: AttrValInfo,
) -> AttrValInfo {
    info.value = calc.get_rah_resonances(ctx, item_key).em.extra;
    info
}
fn rah_therm_resonance_postproc_info(
    calc: &mut Calc,
    ctx: SvcCtx,
    item_key: UItemId,
    mut info: AttrValInfo,
) -> AttrValInfo {
    info.value = calc.get_rah_resonances(ctx, item_key).thermal.extra;
    info
}
fn rah_kin_resonance_postproc_info(
    calc: &mut Calc,
    ctx: SvcCtx,
    item_key: UItemId,
    mut info: AttrValInfo,
) -> AttrValInfo {
    info.value = calc.get_rah_resonances(ctx, item_key).kinetic.extra;
    info
}
fn rah_expl_resonance_postproc_info(
    calc: &mut Calc,
    ctx: SvcCtx,
    item_key: UItemId,
    mut info: AttrValInfo,
) -> AttrValInfo {
    info.value = calc.get_rah_resonances(ctx, item_key).explosive.extra;
    info
}
