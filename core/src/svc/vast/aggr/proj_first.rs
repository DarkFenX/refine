use super::{
    accum::{SeqAccum, SeqInstanceAccum},
    proj_shared::{
        AggrProjInvData, get_proj_regular_output, get_proj_spool_cycle_output, get_proj_spool_part_str_mult,
    },
    traits::LimitInstance,
};
use crate::{
    misc::Spool,
    num::{PValue, Value},
    rd::{REffect, REffectProjOpcSpec},
    svc::{
        SvcCtx,
        calc::Calc,
        cycle::{CycleDataFull, CycleSeq},
        spool::ResolvedSpool,
    },
    ud::UItemId,
};

// Projected effects, considers only first cycle (for "burst" stats)
#[must_use]
pub(in crate::svc::vast) fn aggr_proj_first<T, A>(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_uid: UItemId,
    effect: &REffect,
    cseq: &CycleSeq<CycleDataFull>,
    ospec: &REffectProjOpcSpec<T>,
    projectee_uid: Option<UItemId>,
    spool: Option<Spool>,
    accum: &mut SeqAccum<A>,
) -> bool
where
    T: Copy + std::ops::MulAssign<PValue> + LimitInstance,
    A: SeqInstanceAccum<T>,
{
    let inv_proj = match AggrProjInvData::try_make(ctx, calc, projector_uid, effect, ospec, projectee_uid) {
        Some(inv_proj) => inv_proj,
        None => return false,
    };
    let cycle_data = cseq.get_first_cycle();
    let cycle_output = if ospec.spoolable
        && let Some(spool_attrs) = effect.spool_attr_rids
        && let Some(resolved) = ResolvedSpool::try_build(ctx, calc, projector_uid, effect, spool, spool_attrs)
    {
        let part_str_mult =
            get_proj_spool_part_str_mult(ctx, calc, projector_uid, ospec, &inv_proj, cycle_data.chargedness);
        get_proj_spool_cycle_output(&inv_proj, part_str_mult, resolved.mult - Value::ONE)
    } else {
        get_proj_regular_output(ctx, calc, projector_uid, ospec, &inv_proj, cycle_data.chargedness)
    };
    accum.add_instance(
        cycle_output.get_instance(),
        inv_proj.chance_mult,
        cycle_output.get_instance_count(),
    );
    accum.time += cycle_data.duration;
    true
}
