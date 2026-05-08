use super::{
    accum::{SeqAccum, SeqInstanceAccum},
    local_shared::{AggrLocalInvData, LocalConverter, get_local_output},
    shared::{process_full_cycle_with_cutoff, process_full_loop_lim_sin_with_cutoff},
    traits::{HasImpact, InstanceDuration, InstanceLimit},
};
use crate::{
    nd::NEffectOutputGetter,
    num::{Count, PValue},
    rd::{REffect, REffectLocalOpcSpec},
    svc::{
        SvcCtx,
        calc::Calc,
        cycle::{CycleDataFull, CycleDtHard, CycleSeq, CycleSeqLooped},
    },
    ud::UItemId,
};

// Local effects, considers only infinite parts of cycles
#[must_use]
pub(in crate::svc::vast) fn aggr_local_looped<BG, BX, T, A>(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_uid: UItemId,
    effect: &REffect,
    cseq: &CycleSeq<CycleDataFull>,
    ospec: &REffectLocalOpcSpec<BG>,
    base_xargs: BX,
    accum: &mut SeqAccum<A>,
) -> bool
where
    BG: NEffectOutputGetter<Instance = T, XArgs = BX>,
    T: Copy + Eq + std::ops::MulAssign<PValue> + HasImpact + InstanceDuration + InstanceLimit,
    A: SeqInstanceAccum<T>,
{
    let cseq = match cseq.try_loop_cseq() {
        Some(cseq) => cseq,
        None => return false,
    };
    let inv_local = match AggrLocalInvData::try_make(ctx, calc, item_uid, effect, ospec, base_xargs) {
        Some(inv_local) => inv_local,
        None => return false,
    };
    match cseq.get_hard_dt() {
        Some(hard_dt) => process_hard_dt(ctx, calc, item_uid, cseq, ospec, accum, inv_local, hard_dt),
        None => process_regular(ctx, calc, item_uid, cseq, ospec, accum, inv_local),
    }
    true
}

fn process_regular<BG, T, A>(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_uid: UItemId,
    cseq: CycleSeqLooped<CycleDataFull>,
    ospec: &REffectLocalOpcSpec<BG>,
    accum: &mut SeqAccum<A>,
    inv_local: AggrLocalInvData<T>,
) where
    BG: NEffectOutputGetter,
    T: Copy + std::ops::MulAssign<PValue> + InstanceLimit,
    A: SeqInstanceAccum<T>,
{
    for cycle_part in cseq.iter_cseq_parts() {
        if cycle_part.repeat_count == Count::ZERO {
            continue;
        }
        let cycle_output = get_local_output(
            ctx,
            calc,
            item_uid,
            ospec,
            &inv_local,
            cycle_part.data.active.chargedness,
        );
        accum.add_instance(
            cycle_output.get_instance(),
            None,
            cycle_output.get_instance_count() * cycle_part.repeat_count,
        );
        accum.time += cycle_part.data.get_main_duration() * cycle_part.repeat_count.into_pvalue();
    }
}

fn process_hard_dt<BG, T, A>(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_uid: UItemId,
    cseq: CycleSeqLooped<CycleDataFull>,
    ospec: &REffectLocalOpcSpec<BG>,
    accum: &mut SeqAccum<A>,
    inv_local: AggrLocalInvData<T>,
    hard_dt: CycleDtHard,
) where
    BG: NEffectOutputGetter,
    T: Copy + Eq + std::ops::MulAssign<PValue> + InstanceDuration + InstanceLimit,
    A: SeqInstanceAccum<T>,
{
    let mut converter = LocalConverter::new(ctx, calc, item_uid, ospec, &inv_local);
    let cseq_conv = cseq.convert_with_and_optimize(&mut converter);
    match cseq_conv {
        CycleSeqLooped::Inf(inner) => {
            process_full_cycle_with_cutoff(&mut accum.instances, &inner.data, None, Count::ONE);
            let loop_full_duration = inner.data.cycle_main_duration + hard_dt.duration;
            accum.time += loop_full_duration;
        }
        CycleSeqLooped::LoopLimSin(inner) => {
            let loop_inner_duration = inner.p1_data.cycle_main_duration * inner.p1_repeat_count.into_pvalue()
                + inner.p2_data.cycle_main_duration;
            process_full_loop_lim_sin_with_cutoff(
                &mut accum.instances,
                &inner.p1_data,
                inner.p1_repeat_count,
                &inner.p2_data,
                None,
                loop_inner_duration,
                Count::ONE,
            );
            let loop_full_duration = loop_inner_duration + hard_dt.duration;
            accum.time += loop_full_duration;
        }
    }
}
