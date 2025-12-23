use super::shared::get_mps_cycle_options;
use crate::{
    def::OF,
    misc::MiningAmount,
    nd::NMiningGetter,
    rd::REffectKey,
    svc::{
        SvcCtx,
        calc::Calc,
        cycle::get_item_cycle_info,
        vast::{StatMining, StatMiningItemKinds, Vast},
    },
    ud::{UFitKey, UItemKey},
    util::RMapRMap,
};

impl Vast {
    pub(in crate::svc) fn get_stat_fits_mps(
        &self,
        ctx: SvcCtx,
        calc: &mut Calc,
        fit_keys: impl ExactSizeIterator<Item = UFitKey>,
        item_kinds: StatMiningItemKinds,
        reload: bool,
    ) -> StatMining {
        fit_keys
            .map(|fit_key| StatMining {
                ore: get_mps(ctx, calc, item_kinds, reload, &self.get_fit_data(&fit_key).mining_ore),
                ice: get_mps(ctx, calc, item_kinds, reload, &self.get_fit_data(&fit_key).mining_ice),
                gas: get_mps(ctx, calc, item_kinds, reload, &self.get_fit_data(&fit_key).mining_gas),
            })
            .sum()
    }
    pub(in crate::svc) fn get_stat_fit_mps(
        &self,
        ctx: SvcCtx,
        calc: &mut Calc,
        fit_key: UFitKey,
        item_kinds: StatMiningItemKinds,
        reload: bool,
    ) -> StatMining {
        let fit_data = self.get_fit_data(&fit_key);
        StatMining {
            ore: get_mps(ctx, calc, item_kinds, reload, &fit_data.mining_ore),
            ice: get_mps(ctx, calc, item_kinds, reload, &fit_data.mining_ice),
            gas: get_mps(ctx, calc, item_kinds, reload, &fit_data.mining_gas),
        }
    }
}

fn get_mps(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_kinds: StatMiningItemKinds,
    reload: bool,
    fit_data: &RMapRMap<UItemKey, REffectKey, NMiningGetter>,
) -> MiningAmount {
    let cycle_options = get_mps_cycle_options(reload);
    let mut mps = MiningAmount::new(OF(0.0), OF(0.0));
    for (&item_key, item_data) in fit_data.iter() {
        let cycle_map = match get_item_cycle_info(ctx, calc, item_key, cycle_options, false) {
            Some(cycle_map) => cycle_map,
            None => continue,
        };
        let u_item = ctx.u_data.items.get(item_key);
        if !item_kinds.resolve(u_item) {
            continue;
        }
        for (&effect_key, neut_getter) in item_data.iter() {
            let effect = ctx.u_data.src.get_effect(effect_key);
            let output_per_cycle = match neut_getter(ctx, calc, item_key, effect) {
                Some(output_per_cycle) => output_per_cycle,
                None => continue,
            };
            let effect_cycle_loop = match cycle_map.get(&effect_key).and_then(|v| v.try_get_loop()) {
                Some(effect_cycle_loop) => effect_cycle_loop,
                None => continue,
            };
            mps += output_per_cycle.get_total() / effect_cycle_loop.get_average_time();
        }
    }
    mps
}
