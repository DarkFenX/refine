use crate::{
    misc::PValue,
    rd::{REffect, REffectProjOpcSpec},
    svc::{
        SvcCtx,
        aggr::{aggr_proj_first_ps, aggr_proj_looped_ps, aggr_proj_time_ps},
        calc::Calc,
        cycle::{CyclingOptions, get_item_cseq_map},
        err::StatItemCheckError,
        vast::{StatTank, StatTimeOptions, Vast, vaste_stats::item_checks::check_drone_fighter_module},
    },
    ud::UItemId,
};

impl Vast {
    pub(in crate::svc) fn get_stat_item_outgoing_rps(
        ctx: SvcCtx,
        calc: &mut Calc,
        item_uid: UItemId,
        time_options: StatTimeOptions,
        ignore_state: bool,
        projectee_uid: Option<UItemId>,
    ) -> Result<StatTank<PValue>, StatItemCheckError> {
        check_drone_fighter_module(ctx.u_data, item_uid)?;
        let orps = StatTank {
            shield: get_orps(
                ctx,
                calc,
                item_uid,
                time_options,
                ignore_state,
                projectee_uid,
                get_getter_shield,
            ),
            armor: get_orps(
                ctx,
                calc,
                item_uid,
                time_options,
                ignore_state,
                projectee_uid,
                get_getter_armor,
            ),
            hull: get_orps(
                ctx,
                calc,
                item_uid,
                time_options,
                ignore_state,
                projectee_uid,
                get_getter_hull,
            ),
        };
        Ok(orps)
    }
}

fn get_orps(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_uid: UItemId,
    time_options: StatTimeOptions,
    ignore_state: bool,
    projectee_uid: Option<UItemId>,
    rep_ospec_getter: fn(&REffect) -> Option<REffectProjOpcSpec<PValue>>,
) -> PValue {
    let mut orps = PValue::ZERO;
    let cycling_options = CyclingOptions::from_time_options(time_options);
    let cseq_map = match get_item_cseq_map(ctx, calc, item_uid, cycling_options, ignore_state) {
        Some(cseq_map) => cseq_map,
        None => return orps,
    };
    for (effect_rid, cseq) in cseq_map {
        let effect = ctx.u_data.src.get_effect_by_rid(effect_rid);
        let ospec = match rep_ospec_getter(&effect) {
            Some(ospec) => ospec,
            None => continue,
        };
        match time_options {
            StatTimeOptions::Burst(burst_opts) => {
                if let Some(effect_orps) = aggr_proj_first_ps(
                    ctx,
                    calc,
                    item_uid,
                    effect,
                    &cseq,
                    &ospec,
                    projectee_uid,
                    burst_opts.spool,
                ) {
                    orps += effect_orps;
                }
            }
            StatTimeOptions::Sim(sim_options) => match sim_options.time {
                Some(time) if time > PValue::ZERO => {
                    if let Some(effect_orps) =
                        aggr_proj_time_ps(ctx, calc, item_uid, effect, &cseq, &ospec, projectee_uid, time)
                    {
                        orps += effect_orps;
                    }
                }
                _ => {
                    if let Some(effect_orps) =
                        aggr_proj_looped_ps(ctx, calc, item_uid, effect, &cseq, &ospec, projectee_uid)
                    {
                        orps += effect_orps;
                    }
                }
            },
        }
    }
    orps
}

fn get_getter_shield(effect: &REffect) -> Option<REffectProjOpcSpec<PValue>> {
    effect.outgoing_shield_rep_opc_spec
}

fn get_getter_armor(effect: &REffect) -> Option<REffectProjOpcSpec<PValue>> {
    effect.outgoing_armor_rep_opc_spec
}

fn get_getter_hull(effect: &REffect) -> Option<REffectProjOpcSpec<PValue>> {
    effect.outgoing_hull_rep_opc_spec
}
