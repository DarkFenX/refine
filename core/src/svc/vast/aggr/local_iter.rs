use super::{
    local_shared::{AggrLocalInvData, LocalConverter, get_local_output},
    shared_iter::{AggrIterData, AggrIterDataRegular, AggrPartDataRegular},
    traits::{InstanceDuration, InstanceLimit},
};
use crate::{
    nd::NOutputGetter,
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
pub(in crate::svc::vast) fn aggr_local_iter<BG, BX, T, A>(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_uid: UItemId,
    effect: &REffect,
    cseq: &CycleSeq<CycleDataFull>,
    ospec: &REffectLocalOpcSpec<BG>,
    base_xargs: BX,
) -> Option<AggrIterData<T>>
where
    BG: NOutputGetter<Instance = T, Xargs = BX>,
    T: Copy + Eq + std::ops::MulAssign<PValue> + InstanceDuration + InstanceLimit,
{
    let inv_local = AggrLocalInvData::try_make(ctx, calc, item_uid, effect, ospec, base_xargs)?;
    let mut converter = LocalConverter::new(ctx, calc, item_uid, ospec, &inv_local);
    let cseq_conv = cseq.convert_with_and_optimize(&mut converter);
    Some(AggrIterData::Regular(AggrIterDataRegular::new(cseq_conv)))
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Converter
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<BG, T> LibConverter<CycleDataFull, AggrPartDataRegular<T>> for LocalConverter<'_, '_, '_, '_, '_, BG, T>
where
    BG: NOutputGetter<Instance = T>,
    T: Copy + std::ops::MulAssign<PValue> + InstanceDuration + InstanceLimit,
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
