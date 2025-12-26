use super::shared::NEUT_CYCLE_OPTIONS;
use crate::{
    def::{AttrVal, OF},
    rd::{REffectKey, REffectProjOpcSpec},
    svc::{
        SvcCtx,
        calc::Calc,
        cycle::get_item_cseq_map,
        vast::{StatNeutItemKinds, Vast},
    },
    ud::{UFitKey, UItemKey},
    util::RMapRMap,
};

impl Vast {
    pub(in crate::svc) fn get_stat_fits_outgoing_nps(
        &self,
        ctx: SvcCtx,
        calc: &mut Calc,
        fit_keys: impl ExactSizeIterator<Item = UFitKey>,
        item_kinds: StatNeutItemKinds,
        projectee_key: Option<UItemKey>,
    ) -> AttrVal {
        fit_keys
            .map(|fit_key| {
                get_nps(
                    ctx,
                    calc,
                    item_kinds,
                    projectee_key,
                    &self.get_fit_data(&fit_key).out_neuts,
                )
            })
            .sum()
    }
    pub(in crate::svc) fn get_stat_fit_outgoing_nps(
        &self,
        ctx: SvcCtx,
        calc: &mut Calc,
        fit_key: UFitKey,
        item_kinds: StatNeutItemKinds,
        projectee_key: Option<UItemKey>,
    ) -> AttrVal {
        let fit_data = self.get_fit_data(&fit_key);
        get_nps(ctx, calc, item_kinds, projectee_key, &fit_data.out_neuts)
    }
}

fn get_nps(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_kinds: StatNeutItemKinds,
    projectee_key: Option<UItemKey>,
    fit_data: &RMapRMap<UItemKey, REffectKey, REffectProjOpcSpec<AttrVal>>,
) -> AttrVal {
    let mut nps = OF(0.0);
    for (&item_key, item_data) in fit_data.iter() {
        let cycle_map = match get_item_cseq_map(ctx, calc, item_key, NEUT_CYCLE_OPTIONS, false) {
            Some(cycle_map) => cycle_map,
            None => continue,
        };
        let u_item = ctx.u_data.items.get(item_key);
        if !item_kinds.resolve(u_item) {
            continue;
        }
        for (&effect_key, ospec) in item_data.iter() {
            let effect = ctx.u_data.src.get_effect(effect_key);
            let effect_cycle_loop = match cycle_map.get(&effect_key).and_then(|v| v.try_loop_cseq()) {
                Some(effect_cycle_loop) => effect_cycle_loop,
                None => continue,
            };
            let invar_data = ospec.make_invar_data(ctx, calc, item_key, effect, projectee_key);
            let output_per_cycle = match ospec.get_total(ctx, calc, item_key, effect, None, None, invar_data) {
                Some(output_per_cycle) => output_per_cycle,
                None => continue,
            };
            nps += output_per_cycle / effect_cycle_loop.get_average_time();
        }
    }
    nps
}
