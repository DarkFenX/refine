use crate::{
    def::OF,
    misc::Mining,
    nd::NMiningGetter,
    rd::REffectKey,
    svc::{
        SvcCtx,
        calc::Calc,
        cycle::{CycleOptionReload, CycleOptions, get_item_cycle_info},
        vast::{StatMiningItemKinds, Vast},
    },
    ud::{UFitKey, UItemKey},
    util::RMapRMap,
};

impl Vast {
    pub(in crate::svc) fn get_stat_fits_mps_ore(
        &self,
        ctx: SvcCtx,
        calc: &mut Calc,
        fit_keys: impl ExactSizeIterator<Item = UFitKey>,
        item_kinds: StatMiningItemKinds,
    ) -> Mining {
        fit_keys
            .map(|fit_key| get_mps(ctx, calc, item_kinds, &self.get_fit_data(&fit_key).mining_ore))
            .sum()
    }
    pub(in crate::svc) fn get_stat_fits_mps_ice(
        &self,
        ctx: SvcCtx,
        calc: &mut Calc,
        fit_keys: impl ExactSizeIterator<Item = UFitKey>,
        item_kinds: StatMiningItemKinds,
    ) -> Mining {
        fit_keys
            .map(|fit_key| get_mps(ctx, calc, item_kinds, &self.get_fit_data(&fit_key).mining_ice))
            .sum()
    }
    pub(in crate::svc) fn get_stat_fits_mps_gas(
        &self,
        ctx: SvcCtx,
        calc: &mut Calc,
        fit_keys: impl ExactSizeIterator<Item = UFitKey>,
        item_kinds: StatMiningItemKinds,
    ) -> Mining {
        fit_keys
            .map(|fit_key| get_mps(ctx, calc, item_kinds, &self.get_fit_data(&fit_key).mining_gas))
            .sum()
    }
    pub(in crate::svc) fn get_stat_fit_mps_ore(
        &self,
        ctx: SvcCtx,
        calc: &mut Calc,
        fit_key: UFitKey,
        item_kinds: StatMiningItemKinds,
    ) -> Mining {
        let fit_data = self.get_fit_data(&fit_key);
        get_mps(ctx, calc, item_kinds, &fit_data.mining_ore)
    }
    pub(in crate::svc) fn get_stat_fit_mps_ice(
        &self,
        ctx: SvcCtx,
        calc: &mut Calc,
        fit_key: UFitKey,
        item_kinds: StatMiningItemKinds,
    ) -> Mining {
        let fit_data = self.get_fit_data(&fit_key);
        get_mps(ctx, calc, item_kinds, &fit_data.mining_ice)
    }
    pub(in crate::svc) fn get_stat_fit_mps_gas(
        &self,
        ctx: SvcCtx,
        calc: &mut Calc,
        fit_key: UFitKey,
        item_kinds: StatMiningItemKinds,
    ) -> Mining {
        let fit_data = self.get_fit_data(&fit_key);
        get_mps(ctx, calc, item_kinds, &fit_data.mining_gas)
    }
}

const MINING_CYCLE_OPTIONS: CycleOptions = CycleOptions {
    reload_mode: CycleOptionReload::Sim,
    reload_optionals: true,
};

fn get_mps(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_kinds: StatMiningItemKinds,
    fit_data: &RMapRMap<UItemKey, REffectKey, NMiningGetter>,
) -> Mining {
    let mut mps = Mining::new(OF(0.0), OF(0.0));
    for (&item_key, item_data) in fit_data.iter() {
        let cycle_map = match get_item_cycle_info(ctx, calc, item_key, MINING_CYCLE_OPTIONS, false) {
            Some(cycle_map) => cycle_map,
            None => continue,
        };
        let u_item = ctx.u_data.items.get(item_key);
        if !item_kinds.resolve(u_item) {
            continue;
        }
        for (&effect_key, neut_getter) in item_data.iter() {
            let output_per_cycle = match neut_getter(ctx, calc, item_key) {
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
            mps += output_per_cycle.get_total() / effect_cycles.get_average_cycle_time();
        }
    }
    mps
}
