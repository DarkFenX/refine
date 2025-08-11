use crate::{
    misc::{DmgKinds, Spool},
    svc::{
        SvcCtx,
        calc::Calc,
        cycle::{CycleOptionReload, CycleOptions, get_item_cycle_info},
        vast::{StatDmg, StatDmgItemKinds, VastFitData, shared::BreacherAccum},
    },
};

const VOLLEY_CYCLE_OPTIONS: CycleOptions = CycleOptions {
    reload_mode: CycleOptionReload::Burst,
    charged_optionals: false,
};

impl VastFitData {
    pub(in crate::svc) fn get_stat_dps(
        &self,
        ctx: SvcCtx,
        calc: &mut Calc,
        item_kinds: StatDmgItemKinds,
        reload: bool,
        spool: Option<Spool>,
    ) -> StatDmg {
        let cycle_options = CycleOptions {
            reload_mode: match reload {
                true => CycleOptionReload::Sim,
                false => CycleOptionReload::Burst,
            },
            charged_optionals: false,
        };
        let mut dps_normal = DmgKinds::new();
        let mut breacher_accum = BreacherAccum::new();
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
                dps_normal += output_per_cycle.get_total() / effect_cycles.get_average_cycle_time();
            }
        }
        for (&item_key, item_data) in self.dmg_breacher.iter() {
            let cycle_map = match get_item_cycle_info(ctx, calc, item_key, cycle_options, false) {
                Some(cycle_map) => cycle_map,
                None => continue,
            };
            for (&effect_key, dmg_getter) in item_data.iter() {
                let r_effect = ctx.u_data.src.get_effect(effect_key);
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
        let mut dps = StatDmg::from(dps_normal);
        dps.breacher = breacher_accum.get_dps();
        dps
    }
    pub(in crate::svc) fn get_stat_volley(
        &self,
        ctx: SvcCtx,
        calc: &mut Calc,
        item_kinds: StatDmgItemKinds,
        spool: Option<Spool>,
    ) -> StatDmg {
        let mut volley = StatDmg::new();
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
            for (&effect_key, dmg_getter) in item_data.iter() {
                let r_effect = ctx.u_data.src.get_effect(effect_key);
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
        volley
    }
}
