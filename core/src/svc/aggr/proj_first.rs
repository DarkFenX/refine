use super::{
    proj_inv_data::ProjInvariantData,
    shared::{AggrAmount, AggrOutput},
    traits::LimitAmount,
};
use crate::{
    AttrVal,
    misc::Spool,
    rd::{REffect, REffectProjOpcSpec},
    svc::{SvcCtx, calc::Calc, cycle::CycleSeq, spool::ResolvedSpool},
    ud::UItemKey,
};

// Projected effects, considers only first cycle (for "burst" stats)
pub(in crate::svc) fn aggr_proj_first_ps<T>(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_key: UItemKey,
    effect: &REffect,
    cseq: &CycleSeq,
    ospec: &REffectProjOpcSpec<T>,
    projectee_key: Option<UItemKey>,
    spool: Option<Spool>,
) -> Option<T>
where
    T: Copy
        + std::ops::Mul<AttrVal, Output = T>
        + std::ops::MulAssign<AttrVal>
        + std::ops::Div<AttrVal, Output = T>
        + LimitAmount,
{
    aggr_proj_first_amount(ctx, calc, projector_key, effect, cseq, ospec, projectee_key, spool)
        .and_then(|aggr_amount| aggr_amount.get_ps())
}

pub(in crate::svc) fn aggr_proj_first_max<T>(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_key: UItemKey,
    effect: &REffect,
    cseq: &CycleSeq,
    ospec: &REffectProjOpcSpec<T>,
    projectee_key: Option<UItemKey>,
    spool: Option<Spool>,
) -> Option<T>
where
    T: Copy + std::ops::Mul<AttrVal, Output = T> + std::ops::MulAssign<AttrVal> + LimitAmount,
{
    aggr_proj_first_output(ctx, calc, projector_key, effect, cseq, ospec, projectee_key, spool)
        .map(|output_data| output_data.output.get_max_amount())
}

pub(in crate::svc) fn aggr_proj_first_amount<T>(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_key: UItemKey,
    effect: &REffect,
    cseq: &CycleSeq,
    ospec: &REffectProjOpcSpec<T>,
    projectee_key: Option<UItemKey>,
    spool: Option<Spool>,
) -> Option<AggrAmount<T>>
where
    T: Copy + std::ops::Mul<AttrVal, Output = T> + std::ops::MulAssign<AttrVal> + LimitAmount,
{
    aggr_proj_first_output(ctx, calc, projector_key, effect, cseq, ospec, projectee_key, spool).map(|output_data| {
        AggrAmount {
            amount: output_data.output.amount_sum(),
            time: output_data.time,
        }
    })
}

pub(in crate::svc) fn aggr_proj_first_output<T>(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_key: UItemKey,
    effect: &REffect,
    cseq: &CycleSeq,
    ospec: &REffectProjOpcSpec<T>,
    projectee_key: Option<UItemKey>,
    spool: Option<Spool>,
) -> Option<AggrOutput<T>>
where
    T: Copy + std::ops::MulAssign<AttrVal> + LimitAmount,
{
    let cycle = cseq.get_first_cycle();
    let inv_proj = ProjInvariantData::try_make(ctx, calc, projector_key, effect, ospec, projectee_key)?;
    let mut output = inv_proj.output;
    // Chargedness
    if let Some(charge_mult_getter) = ospec.charge_mult
        && let Some(chargedness) = cycle.chargedness
        && let Some(charge_mult) = charge_mult_getter(ctx, calc, projector_key, chargedness)
    {
        output *= charge_mult;
    }
    // Spool
    if ospec.spoolable
        && let Some(spool_attrs) = effect.spool_attr_keys
        && let Some(resolved) = ResolvedSpool::try_build(ctx, calc, projector_key, effect, spool, spool_attrs)
    {
        output *= resolved.mult;
    }
    // Limit
    if let Some(limit) = inv_proj.amount_limit {
        output.limit_amount(limit);
    }
    // Chance-based multipliers
    if let Some(mult_post) = inv_proj.mult_post {
        output *= mult_post;
    }
    Some(AggrOutput {
        output,
        time: cycle.time,
    })
}
