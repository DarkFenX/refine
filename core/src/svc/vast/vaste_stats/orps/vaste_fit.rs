use super::shared::get_orps_cycle_options;
use crate::{
    def::{AttrVal, OF},
    misc::Spool,
    rd::{REffectKey, REffectProjOpcSpec},
    svc::{
        SvcCtx,
        aggr::{aggr_proj_first_per_second, aggr_proj_looped_per_second},
        calc::Calc,
        cycle::{CyclingOptions, get_item_cseq_map},
        vast::{StatOutRepItemKinds, StatTank, Vast},
    },
    ud::{UFitKey, UItemKey},
    util::RMapRMap,
};

impl Vast {
    pub(in crate::svc) fn get_stat_fits_outgoing_rps(
        &self,
        ctx: SvcCtx,
        calc: &mut Calc,
        fit_keys: impl ExactSizeIterator<Item = UFitKey>,
        item_kinds: StatOutRepItemKinds,
        spool: Option<Spool>,
    ) -> StatTank<AttrVal> {
        let mut rps = StatTank {
            shield: OF(0.0),
            armor: OF(0.0),
            hull: OF(0.0),
        };
        for fit_key in fit_keys {
            let fit_data = self.get_fit_data(&fit_key);
            rps.shield += get_orrps(ctx, calc, item_kinds, spool, &fit_data.orr_shield);
            rps.armor += get_orrps(ctx, calc, item_kinds, spool, &fit_data.orr_armor);
            rps.hull += get_orrps(ctx, calc, item_kinds, spool, &fit_data.orr_hull);
        }
        rps
    }
    pub(in crate::svc) fn get_stat_fit_outgoing_rps(
        &self,
        ctx: SvcCtx,
        calc: &mut Calc,
        fit_key: UFitKey,
        item_kinds: StatOutRepItemKinds,
        spool: Option<Spool>,
    ) -> StatTank<AttrVal> {
        let fit_data = self.get_fit_data(&fit_key);
        StatTank {
            shield: get_orrps(ctx, calc, item_kinds, spool, &fit_data.orr_shield),
            armor: get_orrps(ctx, calc, item_kinds, spool, &fit_data.orr_armor),
            hull: get_orrps(ctx, calc, item_kinds, spool, &fit_data.orr_hull),
        }
    }
    pub(in crate::svc) fn get_stat_fits_outgoing_cps(
        &self,
        ctx: SvcCtx,
        calc: &mut Calc,
        fit_keys: impl ExactSizeIterator<Item = UFitKey>,
    ) -> AttrVal {
        fit_keys
            .map(|fit_key| {
                get_orrps(
                    ctx,
                    calc,
                    StatOutRepItemKinds::all_enabled(),
                    None,
                    &self.get_fit_data(&fit_key).out_cap,
                )
            })
            .sum()
    }
    pub(in crate::svc) fn get_stat_fit_outgoing_cps(&self, ctx: SvcCtx, calc: &mut Calc, fit_key: UFitKey) -> AttrVal {
        let fit_data = self.get_fit_data(&fit_key);
        get_orrps(ctx, calc, StatOutRepItemKinds::all_enabled(), None, &fit_data.out_cap)
    }
}

fn get_orrps(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_kinds: StatOutRepItemKinds,
    spool: Option<Spool>,
    fit_data: &RMapRMap<UItemKey, REffectKey, REffectProjOpcSpec<AttrVal>>,
) -> AttrVal {
    let mut rps = OF(0.0);
    let cycling_options = get_orps_cycle_options(false);
    for (&item_key, item_data) in fit_data.iter() {
        let cseq_map = match get_item_cseq_map(ctx, calc, item_key, cycling_options, false) {
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
            match cycling_options {
                CyclingOptions::Burst => {
                    if let Some(effect_rps) =
                        aggr_proj_first_per_second(ctx, calc, item_key, effect, cseq, ospec, None, spool)
                    {
                        rps += effect_rps;
                    }
                }
                CyclingOptions::Sim(_) => {
                    if let Some(effect_rps) =
                        aggr_proj_looped_per_second(ctx, calc, item_key, effect, cseq, ospec, None)
                    {
                        rps += effect_rps;
                    }
                }
            }
        }
    }
    rps
}
