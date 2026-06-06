use super::{
    accum::{SeqAccum, SeqInstanceAccum},
    proj_shared::{
        AggrProjInvData, get_proj_regular_output, get_proj_spool_cycle_output, get_proj_spool_part_str_mult,
    },
    traits::{HasImpact, InstanceLimit},
};
use crate::{
    misc::Spool,
    nd::NEffectOutputGetter,
    num::{Count, PValue, Value},
    rd::{REffect, REffectProjOpcSpec},
    svc::{
        SvcCtx,
        calc::Calc,
        cycle::{CSeqHardDtFull, CycleDataFull, CycleSeq, GetMainDuration},
        spool::ResolvedSpool,
    },
    ud::UItemId,
};

// Projected effects, considers only first cycle (for "burst" stats)
// Hard downtime is ignored, since burst cseqs are supposed not to have it
pub(in crate::svc::vast) fn aggr_proj_burst<BG, BX, I, IA>(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_uid: UItemId,
    effect: &REffect,
    cseq: &CycleSeq<CycleDataFull, CSeqHardDtFull>,
    ospec: &REffectProjOpcSpec<BG>,
    base_xargs: BX,
    projectee_uid: Option<UItemId>,
    spool: Option<Spool>,
    mut accum: SeqAccum<IA>,
) -> Option<SeqAccum<IA>>
where
    BG: NEffectOutputGetter<Instance = I, XArgs = BX>,
    I: Copy + std::ops::MulAssign<PValue> + HasImpact + InstanceLimit,
    IA: SeqInstanceAccum<I>,
{
    let Some(inv_proj) = AggrProjInvData::try_make(ctx, calc, projector_uid, effect, ospec, base_xargs, projectee_uid)
    else {
        return None;
    };
    let cycle_data = cseq.get_first_cycle();
    let cycle_output = if ospec.spoolable
        && let Some(spool_attrs) = effect.spool_attr_rids
        && let Some(resolved) = ResolvedSpool::try_build(ctx, calc, projector_uid, effect, spool, spool_attrs)
    {
        let part_str_mult = get_proj_spool_part_str_mult(
            ctx,
            calc,
            projector_uid,
            ospec,
            &inv_proj,
            cycle_data.active.chargedness,
        );
        get_proj_spool_cycle_output(&inv_proj, part_str_mult, resolved.mult - Value::ONE)
    } else {
        get_proj_regular_output(
            ctx,
            calc,
            projector_uid,
            ospec,
            &inv_proj,
            cycle_data.active.chargedness,
        )
    };
    accum.add_output_full(&cycle_output, inv_proj.chance_mult, Count::ONE);
    accum.time += cycle_data.get_main_duration();
    Some(accum)
}
