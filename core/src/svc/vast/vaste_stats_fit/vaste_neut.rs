use crate::{
    def::{AttrVal, OF},
    nd::NNeutGetter,
    rd::REffectKey,
    svc::{
        SvcCtx,
        calc::Calc,
        cycle::{CycleOptionReload, CycleOptions, get_item_cycle_info},
        vast::{StatRemoteNpsItemKinds, Vast},
    },
    ud::{UFitKey, UItemKey},
    util::RMapRMap,
};

impl Vast {
    pub(in crate::svc) fn get_stat_fits_remote_nps(
        &self,
        ctx: SvcCtx,
        calc: &mut Calc,
        fit_keys: impl ExactSizeIterator<Item = UFitKey>,
        item_kinds: StatRemoteNpsItemKinds,
    ) -> AttrVal {
        fit_keys
            .map(|fit_key| get_nps(ctx, calc, item_kinds, &self.get_fit_data(&fit_key).neuts))
            .sum()
    }
    pub(in crate::svc) fn get_stat_fit_remote_nps(
        &self,
        ctx: SvcCtx,
        calc: &mut Calc,
        fit_key: UFitKey,
        item_kinds: StatRemoteNpsItemKinds,
    ) -> AttrVal {
        let fit_data = self.get_fit_data(&fit_key);
        get_nps(ctx, calc, item_kinds, &fit_data.neuts)
    }
}

const NEUT_CYCLE_OPTIONS: CycleOptions = CycleOptions {
    reload_mode: CycleOptionReload::Burst,
    charged_optionals: false,
};

fn get_nps(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_kinds: StatRemoteNpsItemKinds,
    fit_data: &RMapRMap<UItemKey, REffectKey, NNeutGetter>,
) -> AttrVal {
    let mut nps = OF(0.0);
    for (&item_key, item_data) in fit_data.iter() {
        let cycle_map = match get_item_cycle_info(ctx, calc, item_key, NEUT_CYCLE_OPTIONS, false) {
            Some(cycle_map) => cycle_map,
            None => continue,
        };
        let u_item = ctx.u_data.items.get(item_key);
        if !item_kinds.resolve(u_item) {
            continue;
        }
        for (&effect_key, neut_getter) in item_data.iter() {
            let r_effect = ctx.u_data.src.get_effect(effect_key);
            let output_per_cycle = match neut_getter(ctx, calc, item_key, r_effect, None) {
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
            nps += output_per_cycle.get_total() / effect_cycles.get_average_cycle_time();
        }
    }
    nps
}
