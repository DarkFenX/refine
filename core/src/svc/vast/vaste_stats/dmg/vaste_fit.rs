use super::shared::{VOLLEY_CYCLE_OPTIONS, get_dps_cycle_options};
use crate::{
    def::AttrVal,
    misc::{DmgKinds, Spool},
    svc::{
        SvcCtx,
        calc::Calc,
        cycle::{CycleOptions, get_item_cycle_info},
        vast::{
            StatDmg, StatDmgApplied, StatDmgBreacher, StatDmgItemKinds, Vast, VastFitData,
            shared::{BreacherAccum, apply_breacher},
        },
    },
    ud::{UFitKey, UItemKey},
};

impl Vast {
    pub(in crate::svc) fn get_stat_fits_dps_raw(
        &self,
        ctx: SvcCtx,
        calc: &mut Calc,
        fit_keys: impl ExactSizeIterator<Item = UFitKey>,
        item_kinds: StatDmgItemKinds,
        reload: bool,
        spool: Option<Spool>,
    ) -> StatDmg {
        let (dps_normal, breacher_accum) =
            self.internal_get_stat_fits_dps(ctx, calc, fit_keys, item_kinds, reload, spool, None);
        StatDmg::from((dps_normal, breacher_accum.get_dps()))
    }
    pub(in crate::svc) fn get_stat_fits_dps_applied(
        &self,
        ctx: SvcCtx,
        calc: &mut Calc,
        fit_keys: impl ExactSizeIterator<Item = UFitKey>,
        item_kinds: StatDmgItemKinds,
        reload: bool,
        spool: Option<Spool>,
        projectee_key: UItemKey,
    ) -> StatDmgApplied {
        let (dps_normal, breacher_accum) =
            self.internal_get_stat_fits_dps(ctx, calc, fit_keys, item_kinds, reload, spool, Some(projectee_key));
        StatDmgApplied::from((
            dps_normal,
            breacher_accum
                .get_dps()
                .map(|breacher_raw| apply_breacher(ctx, calc, breacher_raw, projectee_key)),
        ))
    }
    fn internal_get_stat_fits_dps(
        &self,
        ctx: SvcCtx,
        calc: &mut Calc,
        fit_keys: impl ExactSizeIterator<Item = UFitKey>,
        item_kinds: StatDmgItemKinds,
        reload: bool,
        spool: Option<Spool>,
        projectee_key: Option<UItemKey>,
    ) -> (DmgKinds<AttrVal>, BreacherAccum) {
        let cycle_options = get_dps_cycle_options(reload);
        let mut dps_normal = DmgKinds::new();
        let mut breacher_accum = BreacherAccum::new();
        for fit_key in fit_keys {
            self.get_fit_data(&fit_key).fill_stat_dps(
                ctx,
                calc,
                &mut dps_normal,
                &mut breacher_accum,
                item_kinds,
                cycle_options,
                spool,
                projectee_key,
            );
        }
        (dps_normal, breacher_accum)
    }
    pub(in crate::svc) fn get_stat_fit_dps_raw(
        &self,
        ctx: SvcCtx,
        calc: &mut Calc,
        fit_key: UFitKey,
        item_kinds: StatDmgItemKinds,
        reload: bool,
        spool: Option<Spool>,
    ) -> StatDmg {
        let (dps_normal, breacher_accum) =
            self.internal_get_stat_fit_dps(ctx, calc, fit_key, item_kinds, reload, spool, None);
        StatDmg::from((dps_normal, breacher_accum.get_dps()))
    }
    pub(in crate::svc) fn get_stat_fit_dps_applied(
        &self,
        ctx: SvcCtx,
        calc: &mut Calc,
        fit_key: UFitKey,
        item_kinds: StatDmgItemKinds,
        reload: bool,
        spool: Option<Spool>,
        projectee_key: UItemKey,
    ) -> StatDmgApplied {
        let (dps_normal, breacher_accum) =
            self.internal_get_stat_fit_dps(ctx, calc, fit_key, item_kinds, reload, spool, Some(projectee_key));
        StatDmgApplied::from((
            dps_normal,
            breacher_accum
                .get_dps()
                .map(|breacher_raw| apply_breacher(ctx, calc, breacher_raw, projectee_key)),
        ))
    }
    fn internal_get_stat_fit_dps(
        &self,
        ctx: SvcCtx,
        calc: &mut Calc,
        fit_key: UFitKey,
        item_kinds: StatDmgItemKinds,
        reload: bool,
        spool: Option<Spool>,
        projectee_key: Option<UItemKey>,
    ) -> (DmgKinds<AttrVal>, BreacherAccum) {
        let mut dps_normal = DmgKinds::new();
        let mut breacher_accum = BreacherAccum::new();
        let cycle_options = get_dps_cycle_options(reload);
        self.get_fit_data(&fit_key).fill_stat_dps(
            ctx,
            calc,
            &mut dps_normal,
            &mut breacher_accum,
            item_kinds,
            cycle_options,
            spool,
            projectee_key,
        );
        (dps_normal, breacher_accum)
    }
    pub(in crate::svc) fn get_stat_fits_volley_raw(
        &self,
        ctx: SvcCtx,
        calc: &mut Calc,
        fit_keys: impl ExactSizeIterator<Item = UFitKey>,
        item_kinds: StatDmgItemKinds,
        spool: Option<Spool>,
    ) -> StatDmg {
        let (volley_normal, volley_breacher) =
            self.internal_get_stat_fits_volley(ctx, calc, fit_keys, item_kinds, spool, None);
        StatDmg::from((volley_normal, Some(volley_breacher)))
    }
    pub(in crate::svc) fn get_stat_fits_volley_applied(
        &self,
        ctx: SvcCtx,
        calc: &mut Calc,
        fit_keys: impl ExactSizeIterator<Item = UFitKey>,
        item_kinds: StatDmgItemKinds,
        spool: Option<Spool>,
        projectee_key: UItemKey,
    ) -> StatDmgApplied {
        let (volley_normal, volley_breacher) =
            self.internal_get_stat_fits_volley(ctx, calc, fit_keys, item_kinds, spool, Some(projectee_key));
        StatDmgApplied::from((
            volley_normal,
            volley_breacher
                .nullified()
                .map(|breacher_raw| apply_breacher(ctx, calc, breacher_raw, projectee_key)),
        ))
    }
    fn internal_get_stat_fits_volley(
        &self,
        ctx: SvcCtx,
        calc: &mut Calc,
        fit_keys: impl ExactSizeIterator<Item = UFitKey>,
        item_kinds: StatDmgItemKinds,
        spool: Option<Spool>,
        projectee_key: Option<UItemKey>,
    ) -> (DmgKinds<AttrVal>, StatDmgBreacher) {
        let mut volley_normal = DmgKinds::new();
        let mut volley_breacher = StatDmgBreacher::new();
        for fit_key in fit_keys {
            self.get_fit_data(&fit_key).fill_stat_volley(
                ctx,
                calc,
                &mut volley_normal,
                &mut volley_breacher,
                item_kinds,
                spool,
                projectee_key,
            );
        }
        (volley_normal, volley_breacher)
    }
    pub(in crate::svc) fn get_stat_fit_volley_raw(
        &self,
        ctx: SvcCtx,
        calc: &mut Calc,
        fit_key: UFitKey,
        item_kinds: StatDmgItemKinds,
        spool: Option<Spool>,
    ) -> StatDmg {
        let (volley_normal, volley_breacher) =
            self.internal_get_stat_fit_volley(ctx, calc, fit_key, item_kinds, spool, None);
        StatDmg::from((volley_normal, Some(volley_breacher)))
    }
    pub(in crate::svc) fn get_stat_fit_volley_applied(
        &self,
        ctx: SvcCtx,
        calc: &mut Calc,
        fit_key: UFitKey,
        item_kinds: StatDmgItemKinds,
        spool: Option<Spool>,
        projectee_key: UItemKey,
    ) -> StatDmgApplied {
        let (volley_normal, volley_breacher) =
            self.internal_get_stat_fit_volley(ctx, calc, fit_key, item_kinds, spool, Some(projectee_key));
        StatDmgApplied::from((
            volley_normal,
            volley_breacher
                .nullified()
                .map(|breacher_raw| apply_breacher(ctx, calc, breacher_raw, projectee_key)),
        ))
    }
    fn internal_get_stat_fit_volley(
        &self,
        ctx: SvcCtx,
        calc: &mut Calc,
        fit_key: UFitKey,
        item_kinds: StatDmgItemKinds,
        spool: Option<Spool>,
        projectee_key: Option<UItemKey>,
    ) -> (DmgKinds<AttrVal>, StatDmgBreacher) {
        let mut volley_normal = DmgKinds::new();
        let mut volley_breacher = StatDmgBreacher::new();
        self.get_fit_data(&fit_key).fill_stat_volley(
            ctx,
            calc,
            &mut volley_normal,
            &mut volley_breacher,
            item_kinds,
            spool,
            projectee_key,
        );
        (volley_normal, volley_breacher)
    }
}

impl VastFitData {
    fn fill_stat_dps(
        &self,
        ctx: SvcCtx,
        calc: &mut Calc,
        dps_normal: &mut DmgKinds<AttrVal>,
        breacher_accum: &mut BreacherAccum,
        item_kinds: StatDmgItemKinds,
        cycle_options: CycleOptions,
        spool: Option<Spool>,
        projectee_key: Option<UItemKey>,
    ) {
        for (&item_key, item_data) in self.dmg_normal.iter() {
            let cycle_map = match get_item_cycle_info(ctx, calc, item_key, cycle_options, false) {
                Some(cycle_map) => cycle_map,
                None => continue,
            };
            let u_item = ctx.u_data.items.get(item_key);
            for (&effect_key, dmg_getter) in item_data.iter() {
                let r_effect = ctx.u_data.src.get_effect(effect_key);
                if !item_kinds.resolve(ctx, u_item, r_effect) {
                    continue;
                }
                let output_per_cycle = match dmg_getter(ctx, calc, item_key, r_effect, spool, projectee_key) {
                    Some(output_per_cycle) => output_per_cycle,
                    None => continue,
                };
                let effect_cycle_loop = match cycle_map.get(&effect_key).and_then(|v| v.try_get_loop()) {
                    Some(effect_cycle_loop) => effect_cycle_loop,
                    None => continue,
                };
                *dps_normal += output_per_cycle.get_total() / effect_cycle_loop.get_average_time();
            }
        }
        for (&item_key, item_data) in self.dmg_breacher.iter() {
            let cycle_map = match get_item_cycle_info(ctx, calc, item_key, cycle_options, false) {
                Some(cycle_map) => cycle_map,
                None => continue,
            };
            let u_item = ctx.u_data.items.get(item_key);
            for (&effect_key, dmg_getter) in item_data.iter() {
                let r_effect = ctx.u_data.src.get_effect(effect_key);
                if !item_kinds.resolve(ctx, u_item, r_effect) {
                    continue;
                }
                let output_per_cycle = match dmg_getter(ctx, calc, item_key, r_effect, projectee_key) {
                    Some(output_per_cycle) => output_per_cycle,
                    None => continue,
                };
                let effect_cycles = match cycle_map.get(&effect_key) {
                    Some(effect_cycles) => effect_cycles,
                    None => continue,
                };
                breacher_accum.add(output_per_cycle, *effect_cycles);
            }
        }
    }
    fn fill_stat_volley(
        &self,
        ctx: SvcCtx,
        calc: &mut Calc,
        volley_normal: &mut DmgKinds<AttrVal>,
        volley_breacher: &mut StatDmgBreacher,
        item_kinds: StatDmgItemKinds,
        spool: Option<Spool>,
        projectee_key: Option<UItemKey>,
    ) {
        for (&item_key, item_data) in self.dmg_normal.iter() {
            let cycle_map = match get_item_cycle_info(ctx, calc, item_key, VOLLEY_CYCLE_OPTIONS, false) {
                Some(cycle_map) => cycle_map,
                None => continue,
            };
            let u_item = ctx.u_data.items.get(item_key);
            for (&effect_key, dmg_getter) in item_data.iter() {
                let r_effect = ctx.u_data.src.get_effect(effect_key);
                if !item_kinds.resolve(ctx, u_item, r_effect) {
                    continue;
                }
                let output_per_cycle = match dmg_getter(ctx, calc, item_key, r_effect, spool, projectee_key) {
                    Some(output_per_cycle) => output_per_cycle,
                    None => continue,
                };
                if !cycle_map.contains_key(&effect_key) {
                    continue;
                };
                *volley_normal += output_per_cycle.get_amount();
            }
        }
        for (&item_key, item_data) in self.dmg_breacher.iter() {
            let cycle_map = match get_item_cycle_info(ctx, calc, item_key, VOLLEY_CYCLE_OPTIONS, false) {
                Some(cycle_map) => cycle_map,
                None => continue,
            };
            let u_item = ctx.u_data.items.get(item_key);
            for (&effect_key, dmg_getter) in item_data.iter() {
                let r_effect = ctx.u_data.src.get_effect(effect_key);
                if !item_kinds.resolve(ctx, u_item, r_effect) {
                    continue;
                }
                let output_per_cycle = match dmg_getter(ctx, calc, item_key, r_effect, projectee_key) {
                    Some(output_per_cycle) => output_per_cycle,
                    None => continue,
                };
                if !cycle_map.contains_key(&effect_key) {
                    continue;
                };
                volley_breacher.stack_instance_output(output_per_cycle);
            }
        }
    }
}
