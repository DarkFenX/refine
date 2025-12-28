use super::shared::get_mps_cycle_options;
use crate::{
    def::OF,
    misc::MiningAmount,
    rd::{REffectKey, REffectProjOpcSpec},
    svc::{
        SvcCtx,
        aggr::{aggr_proj_first_ps, aggr_proj_looped_ps},
        calc::Calc,
        cycle::get_item_cseq_map,
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
    fit_data: &RMapRMap<UItemKey, REffectKey, REffectProjOpcSpec<MiningAmount>>,
) -> MiningAmount {
    let cycle_options = get_mps_cycle_options(reload);
    let mut mps = MiningAmount::new(OF(0.0), OF(0.0));
    for (&item_key, item_data) in fit_data.iter() {
        let cseq_map = match get_item_cseq_map(ctx, calc, item_key, cycle_options, false) {
            Some(cseq_map) => cseq_map,
            None => continue,
        };
        let u_item = ctx.u_data.items.get(item_key);
        if !item_kinds.resolve(u_item) {
            continue;
        }
        for (&effect_key, ospec) in item_data.iter() {
            let cseq = match cseq_map.get(&effect_key) {
                Some(cseq) => cseq,
                None => continue,
            };
            let effect = ctx.u_data.src.get_effect(effect_key);
            match reload {
                true => {
                    if let Some(effect_mps) = aggr_proj_looped_ps(ctx, calc, item_key, effect, cseq, ospec, None) {
                        mps += effect_mps;
                    }
                }
                false => {
                    if let Some(effect_mps) = aggr_proj_first_ps(ctx, calc, item_key, effect, cseq, ospec, None, None) {
                        mps += effect_mps;
                    }
                }
            }
        }
    }
    mps
}
