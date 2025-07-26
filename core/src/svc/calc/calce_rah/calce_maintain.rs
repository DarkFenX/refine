use itertools::Itertools;

use super::shared::{
    ARMOR_EM_ATTR_ID, ARMOR_EXPL_ATTR_ID, ARMOR_HP_ATTR_ID, ARMOR_KIN_ATTR_ID, ARMOR_THERM_ATTR_ID, HULL_HP_ATTR_ID,
    RAH_EFFECT_ID, RAH_SHIFT_ATTR_ID, SHIELD_HP_ATTR_ID,
};
use crate::{
    misc::{AttrSpec, DmgKinds},
    rd,
    svc::{
        SvcCtx,
        calc::{AttrValInfo, Calc, CalcAttrVal, ItemAttrPostprocs},
    },
    ud::{UFitKey, UItem, UItemKey},
    util::GetId,
};

impl Calc {
    pub(in crate::svc::calc) fn rah_item_loaded(&mut self, ctx: SvcCtx, item: &UItem) {
        if self.rah.sim_running {
            return;
        }
        if let UItem::Ship(ship) = item {
            self.clear_fit_rah_results(ctx, &ship.get_fit_key());
        }
    }
    pub(in crate::svc::calc) fn rah_item_unloaded(&mut self, ctx: SvcCtx, item: &UItem) {
        if self.rah.sim_running {
            return;
        }
        if let UItem::Ship(ship) = item {
            self.clear_fit_rah_results(ctx, &ship.get_fit_key());
        }
    }
    pub(in crate::svc::calc) fn rah_effects_started(
        &mut self,
        ctx: SvcCtx,
        item_key: UItemKey,
        item: &UItem,
        r_effects: &[rd::RcEffect],
    ) {
        if self.rah.sim_running {
            return;
        }
        if let UItem::Module(module) = item
            && r_effects.iter().any(|v| v.get_id() == RAH_EFFECT_ID)
        {
            let fit_key = module.get_fit_key();
            // Clear sim data for other RAHs on the same fit
            self.clear_fit_rah_results(ctx, &fit_key);
            // Add sim data for RAH being started
            self.rah.resonances.insert(item_key, None);
            self.rah.by_fit.add_entry(fit_key, item_key);
            // Add postprocessors
            let item_attr_data = self.attrs.get_item_attr_data_mut(&item_key).unwrap();
            item_attr_data.postprocs.insert(
                ARMOR_EM_ATTR_ID,
                ItemAttrPostprocs {
                    fast: rah_em_resonance_postproc_fast,
                    info: rah_em_resonance_postproc_info,
                },
            );
            item_attr_data.postprocs.insert(
                ARMOR_THERM_ATTR_ID,
                ItemAttrPostprocs {
                    fast: rah_therm_resonance_postproc_fast,
                    info: rah_therm_resonance_postproc_info,
                },
            );
            item_attr_data.postprocs.insert(
                ARMOR_KIN_ATTR_ID,
                ItemAttrPostprocs {
                    fast: rah_kin_resonance_postproc_fast,
                    info: rah_kin_resonance_postproc_info,
                },
            );
            item_attr_data.postprocs.insert(
                ARMOR_EXPL_ATTR_ID,
                ItemAttrPostprocs {
                    fast: rah_expl_resonance_postproc_fast,
                    info: rah_expl_resonance_postproc_info,
                },
            );
        }
    }
    pub(in crate::svc::calc) fn rah_effects_stopped(
        &mut self,
        ctx: SvcCtx,
        item_key: &UItemKey,
        item: &UItem,
        r_effects: &[rd::RcEffect],
    ) {
        if self.rah.sim_running {
            return;
        }
        if let UItem::Module(module) = item
            && r_effects.iter().any(|v| v.get_id() == RAH_EFFECT_ID)
        {
            let fit_key = module.get_fit_key();
            // Remove postprocessors
            let item_attr_data = self.attrs.get_item_attr_data_mut(item_key).unwrap();
            item_attr_data.postprocs.remove(&ARMOR_EM_ATTR_ID);
            item_attr_data.postprocs.remove(&ARMOR_THERM_ATTR_ID);
            item_attr_data.postprocs.remove(&ARMOR_KIN_ATTR_ID);
            item_attr_data.postprocs.remove(&ARMOR_EXPL_ATTR_ID);
            // Remove sim data for RAH being stopped
            self.rah.resonances.remove(item_key);
            self.rah.by_fit.remove_entry(&fit_key, item_key);
            // Clear sim data for other RAHs on the same fit
            self.clear_fit_rah_results(ctx, &fit_key);
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
        match aspec.a_attr_id {
            // Ship armor resonances and RAH resonances
            ARMOR_EM_ATTR_ID | ARMOR_THERM_ATTR_ID | ARMOR_KIN_ATTR_ID | ARMOR_EXPL_ATTR_ID => {
                match ctx.u_data.items.get(aspec.item_key) {
                    UItem::Ship(ship) => self.clear_fit_rah_results(ctx, &ship.get_fit_key()),
                    UItem::Module(module) => {
                        if self.rah.resonances.contains_key(&aspec.item_key) {
                            self.clear_fit_rah_results(ctx, &module.get_fit_key());
                        }
                    }
                    _ => (),
                }
            }
            // RAH shift amount
            RAH_SHIFT_ATTR_ID => {
                if self.rah.resonances.contains_key(&aspec.item_key) {
                    // Only modules should be registered in resonances container, and those are
                    // guaranteed to have fit ID
                    let fit_key = ctx.u_data.items.get(aspec.item_key).get_fit_key().unwrap();
                    self.clear_fit_rah_results(ctx, &fit_key);
                }
            }
            // RAH cycle time
            a_attr_id if Some(a_attr_id) == ctx.u_data.src.get_rah_duration_attr_id() => {
                if self.rah.resonances.contains_key(&aspec.item_key) {
                    // Only modules should be registered in resonances container, and those are
                    // guaranteed to have fit ID
                    let fit_key = ctx.u_data.items.get(aspec.item_key).get_fit_key().unwrap();
                    // Clear only for fits with 2+ RAHs, since changing cycle time of 1 RAH does not
                    // change sim results
                    if self.rah.by_fit.get(&fit_key).len() >= 2 {
                        self.clear_fit_rah_results(ctx, &fit_key);
                    }
                }
            }
            // Ship HP - need to clear results since breacher DPS depends on those
            SHIELD_HP_ATTR_ID | ARMOR_HP_ATTR_ID | HULL_HP_ATTR_ID => {
                if let UItem::Ship(ship) = ctx.u_data.items.get(aspec.item_key) {
                    let fit_key = ship.get_fit_key();
                    if ctx.u_data.get_fit_key_rah_incoming_dps(fit_key).deals_breacher_dps() {
                        self.clear_fit_rah_results(ctx, &fit_key);
                    }
                }
            }
            _ => (),
        }
    }
    pub(in crate::svc::calc) fn rah_fit_rah_dps_profile_changed(&mut self, ctx: SvcCtx, fit_key: &UFitKey) {
        self.clear_fit_rah_results(ctx, fit_key);
    }
    // Private methods
    fn clear_fit_rah_results(&mut self, ctx: SvcCtx, fit_key: &UFitKey) {
        let rah_keys = self.rah.by_fit.get(fit_key).copied().collect_vec();
        for rah_key in rah_keys {
            self.clear_rah_result(ctx, rah_key);
        }
    }
    fn clear_rah_result(&mut self, ctx: SvcCtx, item_key: UItemKey) {
        if self.rah.resonances.get_mut(&item_key).unwrap().take().is_some() {
            self.force_attr_postproc_recalc(ctx, AttrSpec::new(item_key, ARMOR_EM_ATTR_ID));
            self.force_attr_postproc_recalc(ctx, AttrSpec::new(item_key, ARMOR_THERM_ATTR_ID));
            self.force_attr_postproc_recalc(ctx, AttrSpec::new(item_key, ARMOR_KIN_ATTR_ID));
            self.force_attr_postproc_recalc(ctx, AttrSpec::new(item_key, ARMOR_EXPL_ATTR_ID));
        }
    }
    fn get_rah_resonances(&mut self, ctx: SvcCtx, item_key: UItemKey) -> DmgKinds<CalcAttrVal> {
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

fn rah_em_resonance_postproc_fast(calc: &mut Calc, ctx: SvcCtx, item_key: UItemKey, _cval: CalcAttrVal) -> CalcAttrVal {
    calc.get_rah_resonances(ctx, item_key).em
}

fn rah_therm_resonance_postproc_fast(
    calc: &mut Calc,
    ctx: SvcCtx,
    item_key: UItemKey,
    _cval: CalcAttrVal,
) -> CalcAttrVal {
    calc.get_rah_resonances(ctx, item_key).thermal
}

fn rah_kin_resonance_postproc_fast(
    calc: &mut Calc,
    ctx: SvcCtx,
    item_key: UItemKey,
    _cval: CalcAttrVal,
) -> CalcAttrVal {
    calc.get_rah_resonances(ctx, item_key).kinetic
}

fn rah_expl_resonance_postproc_fast(
    calc: &mut Calc,
    ctx: SvcCtx,
    item_key: UItemKey,
    _cval: CalcAttrVal,
) -> CalcAttrVal {
    calc.get_rah_resonances(ctx, item_key).explosive
}

fn rah_em_resonance_postproc_info(
    calc: &mut Calc,
    ctx: SvcCtx,
    item_key: UItemKey,
    mut info: AttrValInfo,
) -> AttrValInfo {
    info.value = calc.get_rah_resonances(ctx, item_key).em.extra;
    info
}

fn rah_therm_resonance_postproc_info(
    calc: &mut Calc,
    ctx: SvcCtx,
    item_key: UItemKey,
    mut info: AttrValInfo,
) -> AttrValInfo {
    info.value = calc.get_rah_resonances(ctx, item_key).thermal.extra;
    info
}

fn rah_kin_resonance_postproc_info(
    calc: &mut Calc,
    ctx: SvcCtx,
    item_key: UItemKey,
    mut info: AttrValInfo,
) -> AttrValInfo {
    info.value = calc.get_rah_resonances(ctx, item_key).kinetic.extra;
    info
}

fn rah_expl_resonance_postproc_info(
    calc: &mut Calc,
    ctx: SvcCtx,
    item_key: UItemKey,
    mut info: AttrValInfo,
) -> AttrValInfo {
    info.value = calc.get_rah_resonances(ctx, item_key).explosive.extra;
    info
}
