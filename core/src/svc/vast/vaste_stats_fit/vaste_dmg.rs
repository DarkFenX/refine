use crate::{
    def::AttrVal,
    misc::{DmgKinds, Spool},
    svc::{
        SvcCtx,
        calc::Calc,
        cycle::{CycleOptionReload, CycleOptions, get_item_cycle_info},
        vast::{StatDmg, StatDmgItemKinds, Vast, VastFitData, shared::BreacherAccum},
    },
    ud::UFitKey,
};

const VOLLEY_CYCLE_OPTIONS: CycleOptions = CycleOptions {
    reload_mode: CycleOptionReload::Burst,
    charged_optionals: false,
};

impl Vast {
    pub(in crate::svc) fn get_stat_fit_dps(
        &self,
        ctx: SvcCtx,
        calc: &mut Calc,
        fit_key: UFitKey,
        item_kinds: StatDmgItemKinds,
        reload: bool,
        spool: Option<Spool>,
    ) -> StatDmg {
        let mut dps_normal = DmgKinds::new();
        let mut breacher_accum = BreacherAccum::new();
        let cycle_options = CycleOptions {
            reload_mode: match reload {
                true => CycleOptionReload::Sim,
                false => CycleOptionReload::Burst,
            },
            charged_optionals: false,
        };
        self.get_fit_data(&fit_key).fill_stat_dps(
            ctx,
            calc,
            &mut dps_normal,
            &mut breacher_accum,
            item_kinds,
            cycle_options,
            spool,
        );
        let mut dps = StatDmg::from(dps_normal);
        dps.breacher = breacher_accum.get_dps();
        dps
    }
    pub(in crate::svc) fn get_stat_fit_volley(
        &self,
        ctx: SvcCtx,
        calc: &mut Calc,
        fit_key: UFitKey,
        item_kinds: StatDmgItemKinds,
        spool: Option<Spool>,
    ) -> StatDmg {
        let mut volley = StatDmg::new();
        self.get_fit_data(&fit_key)
            .fill_stat_volley(ctx, calc, &mut volley, item_kinds, spool);
        volley
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
                let output_per_cycle = match dmg_getter(ctx, calc, item_key, r_effect, spool, None) {
                    Some(output_per_cycle) => output_per_cycle,
                    None => continue,
                };
                let effect_cycles = match cycle_map.get(&effect_key) {
                    Some(effect_cycles) => effect_cycles,
                    None => continue,
                };
                if !effect_cycles.is_infinite() {
                    continue;
                }
                *dps_normal += output_per_cycle.get_total() / effect_cycles.get_average_cycle_time();
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
                let output_per_cycle = match dmg_getter(ctx, calc, item_key, r_effect, None) {
                    Some(output_per_cycle) => output_per_cycle,
                    None => continue,
                };
                let effect_cycles = match cycle_map.get(&effect_key) {
                    Some(effect_cycles) => effect_cycles,
                    None => continue,
                };
                if !effect_cycles.is_infinite() {
                    continue;
                }
                breacher_accum.add(output_per_cycle, *effect_cycles);
            }
        }
    }
    fn fill_stat_volley(
        &self,
        ctx: SvcCtx,
        calc: &mut Calc,
        volley: &mut StatDmg,
        item_kinds: StatDmgItemKinds,
        spool: Option<Spool>,
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
                let output_per_cycle = match dmg_getter(ctx, calc, item_key, r_effect, spool, None) {
                    Some(output_per_cycle) => output_per_cycle,
                    None => continue,
                };
                if !cycle_map.contains_key(&effect_key) {
                    continue;
                };
                volley.stack_instance_normal(output_per_cycle.get_max());
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
                let output_per_cycle = match dmg_getter(ctx, calc, item_key, r_effect, None) {
                    Some(output_per_cycle) => output_per_cycle,
                    None => continue,
                };
                if !cycle_map.contains_key(&effect_key) {
                    continue;
                };
                volley.stack_instance_breacher_output(output_per_cycle);
            }
        }
    }
}
