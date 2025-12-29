use super::shared::get_orps_cycling_options;
use crate::{
    def::{AttrVal, OF},
    misc::Spool,
    rd::{REffect, REffectProjOpcSpec},
    svc::{
        SvcCtx,
        aggr::{aggr_proj_first_ps, aggr_proj_looped_ps},
        calc::Calc,
        cycle::{CyclingOptions, get_item_cseq_map},
        err::StatItemCheckError,
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
    let cycling_options = get_orps_cycling_options(false);
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
                if let Some(effect_orr) = aggr_proj_first_ps(ctx, calc, item_key, effect, &cseq, &ospec, None, spool) {
                    item_orr += effect_orr;
                }
            }
            CyclingOptions::Sim(_) => {
                if let Some(effect_orr) = aggr_proj_looped_ps(ctx, calc, item_key, effect, &cseq, &ospec, None) {
                    item_orr += effect_orr;
                }
            }
        }
    }
    item_orr
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
