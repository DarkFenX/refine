use super::shared::NEUT_CYCLE_OPTIONS;
use crate::{
    def::{AttrVal, OF},
    rd::{REffectKey, REffectProjOpcSpec},
    svc::{
        SvcCtx,
        aggr::aggr_proj_first_per_second,
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
    projectee_item_key: Option<UItemKey>,
    fit_data: &RMapRMap<UItemKey, REffectKey, REffectProjOpcSpec<AttrVal>>,
) -> AttrVal {
    let mut nps = OF(0.0);
    for (&item_key, item_data) in fit_data.iter() {
        let cseq_map = match get_item_cseq_map(ctx, calc, item_key, NEUT_CYCLE_OPTIONS, false) {
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
            if let Some(effect_nps) =
                aggr_proj_first_per_second(ctx, calc, item_key, effect, cseq, ospec, projectee_item_key, None)
            {
                nps += effect_nps;
            }
        }
    }
    nps
}
