use crate::{
    def::AttrVal,
    misc::{DmgKinds, Spool},
    svc::{
        SvcCtx,
        calc::Calc,
        cycle::{CycleOptionReload, CycleOptions, get_item_cycle_info},
        vast::VastFitData,
    },
};

const VOLLEY_CYCLE_OPTIONS: CycleOptions = CycleOptions {
    reload_mode: CycleOptionReload::Burst,
    reload_optionals: false,
};

impl VastFitData {
    pub(in crate::svc) fn get_stat_dps(
        &self,
        ctx: SvcCtx,
        calc: &mut Calc,
        reload: bool,
        spool: Option<Spool>,
    ) -> DmgKinds<AttrVal> {
        let options = CycleOptions {
            reload_mode: match reload {
                true => CycleOptionReload::Sim,
                false => CycleOptionReload::Burst,
            },
            reload_optionals: false,
        };
        let mut dps = DmgKinds::new();
        for (&item_key, item_data) in self.dmg_normal.iter() {
            let cycle_map = match get_item_cycle_info(ctx, calc, item_key, options, false) {
                Some(cycle_map) => cycle_map,
                None => continue,
            };
            for (a_effect_id, dmg_getter) in item_data.iter() {
                let r_effect = match ctx.u_data.src.get_r_effect(a_effect_id) {
                    Some(r_effect) => r_effect,
                    None => continue,
                };
                let output_per_cycle = match dmg_getter(ctx, calc, item_key, r_effect, spool, None) {
                    Some(output_per_cycle) => output_per_cycle,
                    None => continue,
                };
                let effect_cycles = match cycle_map.get(a_effect_id) {
                    Some(effect_cycles) => effect_cycles,
                    None => continue,
                };
                dps += output_per_cycle.get_total() / effect_cycles.get_average_cycle_time();
            }
        }
        dps
    }
    pub(in crate::svc) fn get_stat_volley(
        &self,
        ctx: SvcCtx,
        calc: &mut Calc,
        spool: Option<Spool>,
    ) -> DmgKinds<AttrVal> {
        let mut volley = DmgKinds::new();
        for (&item_key, item_data) in self.dmg_normal.iter() {
            let cycle_map = match get_item_cycle_info(ctx, calc, item_key, VOLLEY_CYCLE_OPTIONS, false) {
                Some(cycle_map) => cycle_map,
                None => continue,
            };
            for (a_effect_id, dmg_getter) in item_data.iter() {
                let r_effect = match ctx.u_data.src.get_r_effect(a_effect_id) {
                    Some(r_effect) => r_effect,
                    None => continue,
                };
                let output_per_cycle = match dmg_getter(ctx, calc, item_key, r_effect, spool, None) {
                    Some(output_per_cycle) => output_per_cycle,
                    None => continue,
                };
                if !cycle_map.contains_key(a_effect_id) {
                    continue;
                };
                volley += output_per_cycle.get_max();
            }
        }
        volley
    }
}
