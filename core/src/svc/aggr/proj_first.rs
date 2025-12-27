use super::{proj_inv_data::try_make_proj_inv_data, shared::AggrData, traits::Aggregable};
use crate::{
    misc::Spool,
    rd::{REffect, REffectProjOpcSpec},
    svc::{SvcCtx, calc::Calc, cycle::CycleSeq, spool::ResolvedSpool},
    ud::UItemKey,
};

// Projected effects, considers only first cycle (for "burst" stats)
pub(in crate::svc) fn aggr_proj_first_per_second<T>(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_key: UItemKey,
    effect: &REffect,
    cseq: &CycleSeq,
    ospec: &REffectProjOpcSpec<T>,
    spool: Option<Spool>,
    projectee_key: Option<UItemKey>,
) -> Option<T>
where
    T: Copy + Aggregable,
{
    let aggr_data = aggr_proj_first(ctx, calc, projector_key, effect, cseq, ospec, spool, projectee_key)?;
    Some(aggr_data.get_per_second())
}

pub(in crate::svc) fn aggr_proj_first<T>(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_key: UItemKey,
    effect: &REffect,
    cseq: &CycleSeq,
    ospec: &REffectProjOpcSpec<T>,
    spool: Option<Spool>,
    projectee_key: Option<UItemKey>,
) -> Option<AggrData<T>>
where
    T: Copy + Aggregable,
{
    let cycle = cseq.get_first_cycle();
    let inv_data = try_make_proj_inv_data(ctx, calc, projector_key, effect, ospec, projectee_key)?;
    let mut output = inv_data.output;
    if let Some(charge_mult_getter) = ospec.charge_mult
        && let Some(chargedness) = cycle.chargedness
        && let Some(charge_mult) = charge_mult_getter(ctx, calc, projector_key, chargedness)
    {
        output *= charge_mult;
    }
    if ospec.spoolable
        && let Some(spool_attrs) = effect.spool_attr_keys
        && let Some(resolved) = ResolvedSpool::try_build(ctx, calc, projector_key, effect, spool, spool_attrs)
    {
        output *= resolved.mult;
    }
    if let Some(limit) = inv_data.amount_limit {
        output.limit_amount(limit);
    }
    if let Some(mult_post) = inv_data.mult_post {
        output *= mult_post;
    }
    Some(AggrData {
        amount: output.instance_sum(),
        time: cycle.time,
    })
}
