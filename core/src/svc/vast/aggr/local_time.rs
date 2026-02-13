use super::{
    accum::{SeqAccum, SeqInstanceAccum},
    local_shared::{AggrLocalInvData, get_local_output},
    precalc::{AggrPartData, aggr_precalc_by_time},
    traits::{InstanceDuration, LimitInstance},
};
use crate::{
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
pub(in crate::svc::vast) fn aggr_local_time<T, A>(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_uid: UItemId,
    effect: &REffect,
    cseq: &CycleSeq<CycleDataFull>,
    ospec: &REffectLocalOpcSpec<T>,
    accum: &mut SeqAccum<A>,
    time: PValue,
) -> bool
where
    T: Copy + Eq + std::ops::MulAssign<PValue> + InstanceDuration + LimitInstance,
    A: SeqInstanceAccum<T>,
{
    let inv_local = match AggrLocalInvData::try_make(ctx, calc, item_uid, effect, ospec) {
        Some(inv_local) => inv_local,
        None => return false,
    };
    let mut converter = LocalConverter::new(ctx, calc, item_uid, ospec, &inv_local);
    let cseq_conv = cseq.convert_with_and_optimize(&mut converter);
    aggr_precalc_by_time(cseq_conv, None, &mut accum.instances, time);
    accum.time += time;
    true
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Converter
////////////////////////////////////////////////////////////////////////////////////////////////////
struct LocalConverter<'u, 'p, 'c, 'o, 'i, T>
where
    T: Copy,
{
    ctx: SvcCtx<'u, 'p>,
    calc: &'c mut Calc,
    item_uid: UItemId,
    ospec: &'o REffectLocalOpcSpec<T>,
    inv_local: &'i AggrLocalInvData<T>,
}
impl<'u, 'p, 'c, 'o, 'i, T> LocalConverter<'u, 'p, 'c, 'o, 'i, T>
where
    T: Copy,
{
    fn new(
        ctx: SvcCtx<'u, 'p>,
        calc: &'c mut Calc,
        item_uid: UItemId,
        ospec: &'o REffectLocalOpcSpec<T>,
        inv_local: &'i AggrLocalInvData<T>,
    ) -> Self {
        Self {
            ctx,
            calc,
            item_uid,
            ospec,
            inv_local,
        }
    }
}
impl<T> LibConverter<CycleDataFull, AggrPartData<T>> for LocalConverter<'_, '_, '_, '_, '_, T>
where
    T: Copy + std::ops::MulAssign<PValue> + InstanceDuration + LimitInstance,
{
    fn lib_convert(&mut self, input: CycleDataFull) -> AggrPartData<T> {
        let output = get_local_output(
            self.ctx,
            self.calc,
            self.item_uid,
            self.ospec,
            &self.inv_local,
            input.chargedness,
        );
        AggrPartData {
            cycle_duration: input.duration,
            cycle_tail_duration: PValue::from_value_clamped(output.get_completion_duration() - input.duration),
            output,
        }
    }
}
