use super::{
    proj_shared::{
        AggrProjInvData, AggrSpoolInvData, ProjConverterRegular, get_proj_regular_output, get_proj_spool_part_str_mult,
    },
    shared_iter::{AggrIterData, AggrIterDataRegular, AggrIterDataSpool, AggrPartDataRegular, AggrPartDataSpool},
    traits::{HasImpact, InstanceDuration, InstanceLimit},
};
use crate::{
    nd::NEffectOutputGetter,
    num::{PValue, Value},
    rd::{REffect, REffectProjOpcSpec},
    svc::{
        SvcCtx,
        calc::Calc,
        cycle::{CSeqHardDtFull, CycleDataFull, CycleSeq},
        vast::aggr::proj_shared::get_proj_spool_cycle_output,
    },
    ud::UItemId,
    util::LibConverter,
};

// Projected effects, iterator over cycles (cycle time + instance iter)
pub(in crate::svc::vast) fn aggr_proj_iter<BG, BX, T>(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_uid: UItemId,
    effect: &REffect,
    cseq: &CycleSeq<CycleDataFull, CSeqHardDtFull>,
    ospec: &REffectProjOpcSpec<BG>,
    base_xargs: BX,
    projectee_uid: Option<UItemId>,
) -> Option<AggrIterData<T>>
where
    BG: NEffectOutputGetter<Instance = T, XArgs = BX>,
    T: Copy + Eq + std::ops::MulAssign<PValue> + HasImpact + InstanceDuration + InstanceLimit,
{
    let inv_proj = AggrProjInvData::try_make(ctx, calc, projector_uid, effect, ospec, base_xargs, projectee_uid)?;
    let aggr_iter = match AggrSpoolInvData::try_make(ctx, calc, projector_uid, effect, ospec) {
        Some(inv_spool) => aggr_spool(ctx, calc, projector_uid, cseq, ospec, inv_proj, inv_spool),
        None => aggr_regular(ctx, calc, projector_uid, cseq, ospec, inv_proj),
    };
    Some(aggr_iter)
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Non-spool
////////////////////////////////////////////////////////////////////////////////////////////////////
fn aggr_regular<BG, T>(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_uid: UItemId,
    cseq: &CycleSeq<CycleDataFull, CSeqHardDtFull>,
    ospec: &REffectProjOpcSpec<BG>,
    inv_proj: AggrProjInvData<T>,
) -> AggrIterData<T>
where
    BG: NEffectOutputGetter,
    T: Copy + Eq + std::ops::MulAssign<PValue> + InstanceDuration + InstanceLimit,
{
    let mut converter = ProjConverterRegular::new(ctx, calc, projector_uid, ospec, &inv_proj);
    let cseq_conv = cseq.convert_with_and_optimize(&mut converter);
    AggrIterData::Regular(AggrIterDataRegular::new(cseq_conv))
}

impl<BG, T> LibConverter<CycleDataFull, AggrPartDataRegular<T>> for ProjConverterRegular<'_, '_, '_, '_, '_, BG, T>
where
    BG: NEffectOutputGetter,
    T: Copy + std::ops::MulAssign<PValue> + InstanceDuration + InstanceLimit,
{
    fn lib_convert(&mut self, input: CycleDataFull) -> AggrPartDataRegular<T> {
        let output = get_proj_regular_output(
            self.ctx,
            self.calc,
            self.projector_uid,
            self.ospec,
            self.inv_proj,
            input.active.chargedness,
        );
        AggrPartDataRegular {
            cycle_main_duration: input.get_main_duration(),
            output,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Spool-specific
////////////////////////////////////////////////////////////////////////////////////////////////////
fn aggr_spool<BG, T>(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_uid: UItemId,
    cseq: &CycleSeq<CycleDataFull, CSeqHardDtFull>,
    ospec: &REffectProjOpcSpec<BG>,
    inv_proj: AggrProjInvData<T>,
    inv_spool: AggrSpoolInvData,
) -> AggrIterData<T>
where
    BG: NEffectOutputGetter,
    T: Copy + Eq + std::ops::MulAssign<PValue> + InstanceDuration + InstanceLimit,
{
    let mut converter = ProjConverterSpool::new(ctx, calc, projector_uid, ospec, &inv_proj, &inv_spool);
    let cseq_conv = cseq.convert_with_and_optimize(&mut converter);
    AggrIterData::Spool(AggrIterDataSpool::new(cseq_conv, inv_proj, inv_spool))
}

struct ProjConverterSpool<'sc1, 'sc2, 'calc, 'ospec, 'ip, 'is, BG, T>
where
    BG: NEffectOutputGetter,
    T: Copy,
{
    ctx: SvcCtx<'sc1, 'sc2>,
    calc: &'calc mut Calc,
    projector_uid: UItemId,
    ospec: &'ospec REffectProjOpcSpec<BG>,
    inv_proj: &'ip AggrProjInvData<T>,
    inv_spool: &'is AggrSpoolInvData,
}
impl<'sc1, 'sc2, 'calc, 'ospec, 'ip, 'is, BG, T> ProjConverterSpool<'sc1, 'sc2, 'calc, 'ospec, 'ip, 'is, BG, T>
where
    BG: NEffectOutputGetter,
    T: Copy,
{
    pub(super) fn new(
        ctx: SvcCtx<'sc1, 'sc2>,
        calc: &'calc mut Calc,
        projector_uid: UItemId,
        ospec: &'ospec REffectProjOpcSpec<BG>,
        inv_proj: &'ip AggrProjInvData<T>,
        inv_spool: &'is AggrSpoolInvData,
    ) -> Self {
        Self {
            ctx,
            calc,
            projector_uid,
            ospec,
            inv_proj,
            inv_spool,
        }
    }
}
impl<BG, T> LibConverter<CycleDataFull, AggrPartDataSpool<T>> for ProjConverterSpool<'_, '_, '_, '_, '_, '_, BG, T>
where
    BG: NEffectOutputGetter,
    T: Copy + std::ops::MulAssign<PValue> + InstanceDuration + InstanceLimit,
{
    fn lib_convert(&mut self, input: CycleDataFull) -> AggrPartDataSpool<T> {
        let part_str_mult = get_proj_spool_part_str_mult(
            self.ctx,
            self.calc,
            self.projector_uid,
            self.ospec,
            self.inv_proj,
            input.active.chargedness,
        );
        let output_zero_spool = get_proj_spool_cycle_output(self.inv_proj, part_str_mult, Value::ZERO);
        let output_max_spool = get_proj_spool_cycle_output(self.inv_proj, part_str_mult, self.inv_spool.max);
        AggrPartDataSpool {
            cycle_main_duration: input.get_main_duration(),
            interrupt: input.soft_dt.is_some(),
            str_mult: part_str_mult,
            output_zero_spool,
            output_max_spool,
        }
    }
}
