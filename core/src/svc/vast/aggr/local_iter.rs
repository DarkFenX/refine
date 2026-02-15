use super::{
    local_shared::{AggrLocalInvData, LocalConverter, get_local_output},
    shared_iter::{AggrIter, AggrIterRegular, AggrPartDataRegular},
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

// Local effects, iterator over cycles (cycle time + instance iter)
pub(in crate::svc::vast) fn aggr_local_iter<T>(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_uid: UItemId,
    effect: &REffect,
    cseq: &CycleSeq<CycleDataFull>,
    ospec: &REffectLocalOpcSpec<T>,
) -> Option<AggrIter<T>>
where
    T: Copy + Eq + std::ops::MulAssign<PValue> + InstanceDuration + LimitInstance,
{
    let inv_local = AggrLocalInvData::try_make(ctx, calc, item_uid, effect, ospec)?;
    let mut converter = LocalConverter::new(ctx, calc, item_uid, ospec, &inv_local);
    let cseq_conv = cseq.convert_with_and_optimize(&mut converter);
    Some(AggrIter::Regular(AggrIterRegular::new(cseq_conv.iter_cycles())))
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Converter
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<T> LibConverter<CycleDataFull, AggrPartDataRegular<T>> for LocalConverter<'_, '_, '_, '_, '_, T>
where
    T: Copy + std::ops::MulAssign<PValue> + InstanceDuration + LimitInstance,
{
    fn lib_convert(&mut self, input: CycleDataFull) -> AggrPartDataRegular<T> {
        let output = get_local_output(
            self.ctx,
            self.calc,
            self.item_uid,
            self.ospec,
            &self.inv_local,
            input.chargedness,
        );
        AggrPartDataRegular {
            cycle_duration: input.duration,
            output,
        }
    }
}
