use super::{
    accum::{SeqAccum, SeqInstanceAccum},
    local_shared::{AggrLocalInvData, LocalConverter, get_local_output},
    shared::AggrPartDataTail,
    shared_time::aggr_by_time,
    traits::{HasImpact, InstanceDuration, InstanceLimit},
};
use crate::{
    nd::NEffectOutputGetter,
    num::{PValue, Value},
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
    BG: NEffectOutputGetter<Instance = T, XArgs = BX>,
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
    BG: NEffectOutputGetter,
    T: Copy + std::ops::MulAssign<PValue> + InstanceDuration + InstanceLimit,
{
    fn lib_convert(&mut self, input: CycleDataFull) -> AggrPartDataTail<T> {
        let output = get_local_output(
            self.ctx,
            self.calc,
            self.item_uid,
            self.ospec,
            self.inv_local,
            input.active.chargedness,
        );
        let main_duration = input.get_main_duration();
        let tail_duration = output.get_completion_duration() - main_duration;
        let tail_duration = match tail_duration > Value::ZERO {
            true => Some(PValue::from_value_unchecked(tail_duration)),
            false => None,
        };
        AggrPartDataTail {
            cycle_main_duration: main_duration,
            cycle_tail_duration: tail_duration,
            output,
        }
    }
}
