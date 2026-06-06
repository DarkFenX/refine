use super::stat::StatOutReps;
use crate::{
    nd::NEffectGeneralOutputGetter,
    num::PValue,
    rd::{REffect, REffectProjOpcSpec},
    svc::{
        SvcCtx,
        calc::Calc,
        cycle::{CseqMap, CyclingOptions, get_item_cseq_map},
        err::StatItemCheckError,
        vast::{
            StatTimeOptions, Vast,
            aggr::{SeqAccum, aggr_proj_burst, aggr_proj_looped, aggr_proj_time},
            stats::item_checks::check_drone_fighter_module,
        },
    },
    ud::UItemId,
};

impl Vast {
    pub(in crate::svc) fn get_stat_item_outgoing_rps(
        reuse_cseq_map: &mut CseqMap,
        ctx: SvcCtx,
        calc: &mut Calc,
        item_uid: UItemId,
        time_options: StatTimeOptions,
        projectee_uid: Option<UItemId>,
    ) -> Result<StatOutReps, StatItemCheckError> {
        check_drone_fighter_module(ctx.u_data, item_uid)?;
        let orps = StatOutReps {
            shield: get_orps(
                reuse_cseq_map,
                ctx,
                calc,
                item_uid,
                time_options,
                projectee_uid,
                get_getter_shield,
            ),
            armor: get_orps(
                reuse_cseq_map,
                ctx,
                calc,
                item_uid,
                time_options,
                projectee_uid,
                get_getter_armor,
            ),
            hull: get_orps(
                reuse_cseq_map,
                ctx,
                calc,
                item_uid,
                time_options,
                projectee_uid,
                get_getter_hull,
            ),
        };
        Ok(orps)
    }
}

fn get_orps<F>(
    reuse_cseq_map: &mut CseqMap,
    ctx: SvcCtx,
    calc: &mut Calc,
    item_uid: UItemId,
    time_options: StatTimeOptions,
    projectee_uid: Option<UItemId>,
    rep_ospec_getter: F,
) -> PValue
where
    F: Fn(&REffect) -> Option<REffectProjOpcSpec<NEffectGeneralOutputGetter>>,
{
    let mut orps = PValue::ZERO;
    let cycling_options = CyclingOptions::from_time_options(time_options);
    if !get_item_cseq_map(reuse_cseq_map, ctx, calc, item_uid, cycling_options) {
        return orps;
    }
    for (&effect_rid, cseq) in reuse_cseq_map.iter() {
        let effect = ctx.u_data.src.get_effect_by_rid(effect_rid);
        let Some(ospec) = rep_ospec_getter(effect) else {
            continue;
        };
        if let Some(accum) = match time_options {
            StatTimeOptions::Burst(burst_opts) => aggr_proj_burst(
                ctx,
                calc,
                item_uid,
                effect,
                cseq,
                &ospec,
                (),
                projectee_uid,
                burst_opts.spool,
                SeqAccum::new_stack(),
            ),
            StatTimeOptions::Sim(sim_options) => match sim_options.time {
                Some(time) if time > PValue::ZERO => aggr_proj_time(
                    ctx,
                    calc,
                    item_uid,
                    effect,
                    cseq,
                    &ospec,
                    (),
                    projectee_uid,
                    SeqAccum::new_stack(),
                    time,
                ),
                _ => aggr_proj_looped(
                    ctx,
                    calc,
                    item_uid,
                    effect,
                    cseq,
                    &ospec,
                    (),
                    projectee_uid,
                    SeqAccum::new_stack(),
                ),
            },
        } {
            orps += accum.get_per_second();
        }
    }
    orps
}

fn get_getter_shield(effect: &REffect) -> Option<REffectProjOpcSpec<NEffectGeneralOutputGetter>> {
    effect.outgoing_shield_rep
}

fn get_getter_armor(effect: &REffect) -> Option<REffectProjOpcSpec<NEffectGeneralOutputGetter>> {
    effect.outgoing_armor_rep
}

fn get_getter_hull(effect: &REffect) -> Option<REffectProjOpcSpec<NEffectGeneralOutputGetter>> {
    effect.outgoing_hull_rep
}
