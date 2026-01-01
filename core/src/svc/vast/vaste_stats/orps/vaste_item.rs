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
        projectee_key: Option<UItemKey>,
    ) -> Result<StatTank<AttrVal>, StatItemCheckError> {
        check_drone_fighter_module(ctx.u_data, item_key)?;
        let orps = StatTank {
            shield: get_orps(
                ctx,
                calc,
                item_key,
                time_options,
                ignore_state,
                projectee_key,
                get_getter_shield,
            ),
            armor: get_orps(
                ctx,
                calc,
                item_key,
                time_options,
                ignore_state,
                projectee_key,
                get_getter_armor,
            ),
            hull: get_orps(
                ctx,
                calc,
                item_key,
                time_options,
                ignore_state,
                projectee_key,
                get_getter_hull,
            ),
        };
        Ok(orps)
    }
}

fn get_orps(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_key: UItemKey,
    time_options: StatTimeOptions,
    ignore_state: bool,
    projectee_key: Option<UItemKey>,
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
                if let Some(effect_orps) = aggr_proj_first_ps(
                    ctx,
                    calc,
                    item_key,
                    effect,
                    &cseq,
                    &ospec,
                    projectee_key,
                    burst_opts.spool,
                ) {
                    orps += effect_orps;
                }
            }
            StatTimeOptions::Sim(sim_options) => match sim_options.time {
                Some(time) if time > OF(0.0) => {
                    if let Some(effect_orps) =
                        aggr_proj_time_ps(ctx, calc, item_key, effect, &cseq, &ospec, projectee_key, time)
                    {
                        orps += effect_orps;
                    }
                }
                _ => {
                    if let Some(effect_orps) =
                        aggr_proj_looped_ps(ctx, calc, item_key, effect, &cseq, &ospec, projectee_key)
                    {
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
