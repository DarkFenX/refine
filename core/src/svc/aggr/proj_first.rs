use super::{
    proj_shared::{AggrProjInvData, get_proj_output, get_proj_output_spool},
    shared::{AggrAmount, AggrOutput, calc_charge_mult},
    traits::LimitAmount,
};
use crate::{
    misc::Spool,
    num::{PValue, Value},
    rd::{REffect, REffectProjOpcSpec},
    svc::{SvcCtx, calc::Calc, cycle::CycleSeq, spool::ResolvedSpool},
    ud::UItemId,
};

// Projected effects, considers only first cycle (for "burst" stats)
pub(in crate::svc) fn aggr_proj_first_ps<T>(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_uid: UItemId,
    effect: &REffect,
    cseq: &CycleSeq,
    ospec: &REffectProjOpcSpec<T>,
    projectee_uid: Option<UItemId>,
    spool: Option<Spool>,
) -> Option<T>
where
    T: Copy
        + std::ops::Mul<PValue, Output = T>
        + std::ops::MulAssign<PValue>
        + std::ops::Div<PValue, Output = T>
        + LimitAmount,
{
    aggr_proj_first_amount(ctx, calc, projector_uid, effect, cseq, ospec, projectee_uid, spool)
        .and_then(|aggr_amount| aggr_amount.get_ps())
}

pub(in crate::svc) fn aggr_proj_first_max<T>(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_uid: UItemId,
    effect: &REffect,
    cseq: &CycleSeq,
    ospec: &REffectProjOpcSpec<T>,
    projectee_uid: Option<UItemId>,
    spool: Option<Spool>,
) -> Option<T>
where
    T: Copy + std::ops::Mul<PValue, Output = T> + std::ops::MulAssign<PValue> + LimitAmount,
{
    aggr_proj_first_output(ctx, calc, projector_uid, effect, cseq, ospec, projectee_uid, spool)
        .map(|output_data| output_data.output.get_max_amount())
}

pub(in crate::svc) fn aggr_proj_first_amount<T>(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_uid: UItemId,
    effect: &REffect,
    cseq: &CycleSeq,
    ospec: &REffectProjOpcSpec<T>,
    projectee_uid: Option<UItemId>,
    spool: Option<Spool>,
) -> Option<AggrAmount<T>>
where
    T: Copy + std::ops::Mul<PValue, Output = T> + std::ops::MulAssign<PValue> + LimitAmount,
{
    aggr_proj_first_output(ctx, calc, projector_uid, effect, cseq, ospec, projectee_uid, spool).map(|output_data| {
        AggrAmount {
            amount: output_data.output.get_amount_sum(),
            duration: output_data.duration,
        }
    })
}

pub(in crate::svc) fn aggr_proj_first_output<T>(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_uid: UItemId,
    effect: &REffect,
    cseq: &CycleSeq,
    ospec: &REffectProjOpcSpec<T>,
    projectee_uid: Option<UItemId>,
    spool: Option<Spool>,
) -> Option<AggrOutput<T>>
where
    T: Copy + std::ops::MulAssign<PValue> + LimitAmount,
{
    let cycle = cseq.get_first_cycle();
    let inv_proj = AggrProjInvData::try_make(ctx, calc, projector_uid, effect, ospec, projectee_uid)?;
    let output = if ospec.spoolable
        && let Some(spool_attrs) = effect.spool_attr_rids
        && let Some(resolved) = ResolvedSpool::try_build(ctx, calc, projector_uid, effect, spool, spool_attrs)
    {
        let charge_mult = calc_charge_mult(ctx, calc, projector_uid, ospec.charge_mult, cycle.chargedness);
        get_proj_output_spool(&inv_proj, charge_mult, resolved.mult - Value::ONE)
    } else {
        get_proj_output(ctx, calc, projector_uid, ospec, &inv_proj, cycle.chargedness)
    };
    Some(AggrOutput {
        output,
        duration: cycle.duration,
    })
}
