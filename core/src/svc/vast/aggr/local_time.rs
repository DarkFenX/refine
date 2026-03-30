use super::{
    accum::{SeqAccum, SeqInstanceAccum},
    local_shared::{AggrLocalInvData, LocalConverter, get_local_output},
    shared_time::{AggrPartDataTail, aggr_by_time},
    traits::{HasImpact, InstanceDuration, InstanceLimit},
};
use crate::{
    nd::NEffectOutputGetter,
    num::PValue,
    rd::{REffect, REffectLocalOpcSpec},
    svc::{
        SvcCtx,
        calc::Calc,
        cycle::{CycleDataFull, CycleSeq},
    },
    ud::UItemId,
    util::LibConverter,
};

// Local effects, aggregates total output by specified time
#[must_use]
pub(in crate::svc::vast) fn aggr_local_time<BG, BX, T, A>(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_uid: UItemId,
    effect: &REffect,
    cseq: &CycleSeq<CycleDataFull>,
    ospec: &REffectLocalOpcSpec<BG>,
    base_xargs: BX,
    accum: &mut SeqAccum<A>,
    time: PValue,
) -> bool
where
    BG: NEffectOutputGetter<Instance = T, Xargs = BX>,
    T: Copy + Eq + std::ops::MulAssign<PValue> + HasImpact + InstanceDuration + InstanceLimit,
    A: SeqInstanceAccum<T>,
{
    let inv_local = match AggrLocalInvData::try_make(ctx, calc, item_uid, effect, ospec, base_xargs) {
        Some(inv_local) => inv_local,
        None => return false,
    };
    let mut converter = LocalConverter::new(ctx, calc, item_uid, ospec, &inv_local);
    let cseq_conv = cseq.convert_with_and_optimize(&mut converter);
    aggr_by_time(cseq_conv, None, &mut accum.instances, time);
    accum.time += time;
    true
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Converter
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<BG, T> LibConverter<CycleDataFull, AggrPartDataTail<T>> for LocalConverter<'_, '_, '_, '_, '_, BG, T>
where
    BG: NEffectOutputGetter<Instance = T>,
    T: Copy + std::ops::MulAssign<PValue> + InstanceDuration + InstanceLimit,
{
    fn lib_convert(&mut self, input: CycleDataFull) -> AggrPartDataTail<T> {
        let output = get_local_output(
            self.ctx,
            self.calc,
            self.item_uid,
            self.ospec,
            self.inv_local,
            input.chargedness,
        );
        AggrPartDataTail {
            cycle_duration: input.duration,
            cycle_tail_duration: PValue::from_value_clamped(output.get_completion_duration() - input.duration),
            output,
        }
    }
}
