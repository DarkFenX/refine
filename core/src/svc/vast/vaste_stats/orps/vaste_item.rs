use super::shared::CAP_TRANSFER_OPTIONS;
use crate::{
    def::{AttrVal, OF},
    rd::{REffect, REffectProjOpcSpec},
    svc::{
        SvcCtx,
        aggr::{aggr_proj_first_ps, aggr_proj_looped_ps, aggr_proj_time_ps},
        calc::Calc,
        cycle::get_item_cseq_map,
        err::StatItemCheckError,
        vast::{StatTank, StatTimeOptions, Vast, vaste_stats::item_checks::check_drone_fighter_module},
    },
    ud::UItemKey,
};

impl Vast {
    pub(in crate::svc) fn get_stat_item_outgoing_rps(
        ctx: SvcCtx,
        calc: &mut Calc,
        item_key: UItemKey,
        time_options: StatTimeOptions,
        ignore_state: bool,
    ) -> Result<StatTank<AttrVal>, StatItemCheckError> {
        check_drone_fighter_module(ctx.u_data, item_key)?;
        let orps = StatTank {
            shield: get_orr_item_key(ctx, calc, item_key, time_options, ignore_state, get_getter_shield),
            armor: get_orr_item_key(ctx, calc, item_key, time_options, ignore_state, get_getter_armor),
            hull: get_orr_item_key(ctx, calc, item_key, time_options, ignore_state, get_getter_hull),
        };
        Ok(orps)
    }
    pub(in crate::svc) fn get_stat_item_outgoing_cps(
        ctx: SvcCtx,
        calc: &mut Calc,
        item_key: UItemKey,
        ignore_state: bool,
    ) -> Result<AttrVal, StatItemCheckError> {
        check_drone_fighter_module(ctx.u_data, item_key)?;
        let ocps = get_orr_item_key(ctx, calc, item_key, CAP_TRANSFER_OPTIONS, ignore_state, get_getter_cap);
        Ok(ocps)
    }
}

fn get_orr_item_key(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_key: UItemKey,
    time_options: StatTimeOptions,
    ignore_state: bool,
    rep_ospec_getter: fn(&REffect) -> Option<REffectProjOpcSpec<AttrVal>>,
) -> AttrVal {
    let mut orps = OF(0.0);
    let cycling_options = time_options.into();
    let cseq_map = match get_item_cseq_map(ctx, calc, item_key, cycling_options, ignore_state) {
        Some(cseq_map) => cseq_map,
        None => return orps,
    };
    for (effect_key, cseq) in cseq_map {
        let effect = ctx.u_data.src.get_effect(effect_key);
        let ospec = match rep_ospec_getter(&effect) {
            Some(ospec) => ospec,
            None => continue,
        };
        match time_options {
            StatTimeOptions::Burst(burst_opts) => {
                if let Some(effect_orps) =
                    aggr_proj_first_ps(ctx, calc, item_key, effect, &cseq, &ospec, None, burst_opts.spool)
                {
                    orps += effect_orps;
                }
            }
            StatTimeOptions::Sim(sim_options) => match sim_options.time {
                Some(time) if time > OF(0.0) => {
                    if let Some(effect_orps) = aggr_proj_time_ps(ctx, calc, item_key, effect, &cseq, &ospec, None, time)
                    {
                        orps += effect_orps;
                    }
                }
                _ => {
                    if let Some(effect_orps) = aggr_proj_looped_ps(ctx, calc, item_key, effect, &cseq, &ospec, None) {
                        orps += effect_orps;
                    }
                }
            },
        }
    }
    orps
}

fn get_getter_shield(effect: &REffect) -> Option<REffectProjOpcSpec<AttrVal>> {
    effect.outgoing_shield_rep_opc_spec
}

fn get_getter_armor(effect: &REffect) -> Option<REffectProjOpcSpec<AttrVal>> {
    effect.outgoing_armor_rep_opc_spec
}

fn get_getter_hull(effect: &REffect) -> Option<REffectProjOpcSpec<AttrVal>> {
    effect.outgoing_hull_rep_opc_spec
}

fn get_getter_cap(effect_id: &REffect) -> Option<REffectProjOpcSpec<AttrVal>> {
    effect_id.outgoing_cap_opc_spec
}
