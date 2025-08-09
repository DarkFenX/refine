use crate::{
    misc::Spool,
    svc::{
        SvcCtx,
        calc::Calc,
        cycle::{CycleOptionReload, CycleOptions, get_item_cycle_info},
        vast::{StatDmg, VastFitData},
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
        reload: bool,
        spool: Option<Spool>,
    ) -> StatDmg {
        let options = CycleOptions {
            reload_mode: match reload {
                true => CycleOptionReload::Sim,
                false => CycleOptionReload::Burst,
            },
            charged_optionals: false,
        };
        let mut dps = StatDmg::new();
        for (&item_key, item_data) in self.dmg_normal.iter() {
            let cycle_map = match get_item_cycle_info(ctx, calc, item_key, options, false) {
                Some(cycle_map) => cycle_map,
                None => continue,
            };
            for (&effect_key, dmg_getter) in item_data.iter() {
                let r_effect = ctx.u_data.src.get_effect(effect_key);
                let output_per_cycle = match dmg_getter(ctx, calc, item_key, r_effect, spool, None) {
                    Some(output_per_cycle) => output_per_cycle,
                    None => continue,
                };
                let effect_cycles = match cycle_map.get(&effect_key) {
                    Some(effect_cycles) => effect_cycles,
                    None => continue,
                };
                dps.stack_normal_div(output_per_cycle.get_total(), effect_cycles.get_average_cycle_time());
            }
        }
        dps
    }
    pub(in crate::svc) fn get_stat_volley(&self, ctx: SvcCtx, calc: &mut Calc, spool: Option<Spool>) -> StatDmg {
        let mut volley = StatDmg::new();
        for (&item_key, item_data) in self.dmg_normal.iter() {
            let cycle_map = match get_item_cycle_info(ctx, calc, item_key, VOLLEY_CYCLE_OPTIONS, false) {
                Some(cycle_map) => cycle_map,
                None => continue,
            };
            for (&effect_key, dmg_getter) in item_data.iter() {
                let r_effect = ctx.u_data.src.get_effect(effect_key);
                let output_per_cycle = match dmg_getter(ctx, calc, item_key, r_effect, spool, None) {
                    Some(output_per_cycle) => output_per_cycle,
                    None => continue,
                };
                if !cycle_map.contains_key(&effect_key) {
                    continue;
                };
                volley.stack_normal(output_per_cycle.get_max());
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
                volley.stack_breacher_output(output_per_cycle);
            }
        }
        volley
    }
}
