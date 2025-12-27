use super::shared::get_orps_cycle_options;
use crate::{
    def::{AttrVal, OF},
    misc::Spool,
    rd::{REffect, REffectProjOpcSpec},
    svc::{
        SvcCtx,
        aggr::{aggr_proj_first_per_second, aggr_proj_looped_per_second},
        calc::Calc,
        cycle::{CycleSeq, CyclingOptions, get_item_cseq_map},
        err::StatItemCheckError,
        spool::ResolvedSpool,
        vast::{StatTank, Vast, vaste_stats::item_checks::check_drone_fighter_module},
    },
    ud::UItemKey,
};

impl Vast {
    pub(in crate::svc) fn get_stat_item_outgoing_rps(
        ctx: SvcCtx,
        calc: &mut Calc,
        item_key: UItemKey,
        spool: Option<Spool>,
        ignore_state: bool,
    ) -> Result<StatTank<AttrVal>, StatItemCheckError> {
        check_drone_fighter_module(ctx.u_data, item_key)?;
        let orps = StatTank {
            shield: get_orr_item_key(ctx, calc, item_key, spool, ignore_state, get_getter_shield),
            armor: get_orr_item_key(ctx, calc, item_key, spool, ignore_state, get_getter_armor),
            hull: get_orr_item_key(ctx, calc, item_key, spool, ignore_state, get_getter_hull),
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
        let ocps = get_orr_item_key(ctx, calc, item_key, None, ignore_state, get_getter_cap);
        Ok(ocps)
    }
}

fn get_orr_item_key(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_key: UItemKey,
    spool: Option<Spool>,
    ignore_state: bool,
    rep_ospec_getter: fn(&REffect) -> Option<REffectProjOpcSpec<AttrVal>>,
) -> AttrVal {
    let mut item_orr = OF(0.0);
    let cycling_options = get_orps_cycle_options(false);
    let cseq_map = match get_item_cseq_map(ctx, calc, item_key, cycling_options, ignore_state) {
        Some(cseq_map) => cseq_map,
        None => return item_orr,
    };
    for (effect_key, cseq) in cseq_map {
        let effect = ctx.u_data.src.get_effect(effect_key);
        let ospec = match rep_ospec_getter(&effect) {
            Some(ospec) => ospec,
            None => continue,
        };
        match cycling_options {
            CyclingOptions::Burst => {
                if let Some(effect_orr) =
                    aggr_proj_first_per_second(ctx, calc, item_key, effect, &cseq, &ospec, None, spool)
                {
                    item_orr += effect_orr;
                }
            }
            CyclingOptions::Sim(_) => {
                if let Some(effect_orr) = aggr_proj_looped_per_second(ctx, calc, item_key, effect, &cseq, &ospec, None)
                {
                    item_orr += effect_orr;
                }
            }
        }
    }
    item_orr
}

fn get_orr_effect(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_key: UItemKey,
    effect: &REffect,
    effect_cycle: CycleSeq,
    spool: Option<Spool>,
    ospec_getter: fn(&REffect) -> Option<REffectProjOpcSpec<AttrVal>>,
) -> Option<AttrVal> {
    let ospec = ospec_getter(effect)?;
    let effect_cycle_loop = effect_cycle.to_time_charge().try_loop_cseq()?;
    let spool_mult = if ospec.spoolable
        && let Some(spool_attrs) = effect.spool_attr_keys
        && let Some(resolved) = ResolvedSpool::try_build(ctx, calc, item_key, effect, spool, spool_attrs)
    {
        Some(resolved.mult)
    } else {
        None
    };
    let mut rep_amount = OF(0.0);
    let mut time = OF(0.0);
    let invar_data = ospec.make_invar_data(ctx, calc, item_key, effect, None);
    for effect_cycle_part in effect_cycle_loop.iter_cseq_parts() {
        let chargedness = effect_cycle_part.data.chargedness;
        let cycle_rep_amount = ospec.get_total(ctx, calc, item_key, effect, chargedness, spool_mult, invar_data)?;
        rep_amount += cycle_rep_amount * effect_cycle_part.repeat_count as f64;
        time += effect_cycle_part.data.time * effect_cycle_part.repeat_count as f64;
    }
    Some(rep_amount / time)
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
