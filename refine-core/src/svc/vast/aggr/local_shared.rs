use super::{
    shared::{AggrPartData, AggrPartDataTail, get_cycle_tail_duration, get_item_ship_limit},
    traits::{HasImpact, InstanceDuration, InstanceLimit},
};
use crate::{
    nd::NEffectOutputGetter,
    num::{Count, PValue, UnitInterval},
    rd::{REffect, REffectLocalOpcSpec},
    svc::{
        SvcCtx,
        calc::Calc,
        cycle::{CycleDataFull, GetMainDuration},
        output::Output,
    },
    ud::UItemId,
    util::LibConverter,
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Data which stays the same through local effect cycles
////////////////////////////////////////////////////////////////////////////////////////////////////
pub(super) struct AggrLocalInvData<I> {
    // Fields are private intentionally; they are supposed to be processed to get an output usable
    // elsewhere
    output: Output<I>,
    instance_limit: Option<PValue>,
}
impl<I> AggrLocalInvData<I> {
    pub(super) fn try_make<BG, BX>(
        ctx: SvcCtx,
        calc: &mut Calc,
        item_uid: UItemId,
        effect: &REffect,
        ospec: &REffectLocalOpcSpec<BG>,
        base_xargs: BX,
    ) -> Option<Self>
    where
        I: HasImpact,
        BG: NEffectOutputGetter<Instance = I, XArgs = BX>,
    {
        let output = ospec.base.get(ctx, calc, item_uid, effect, base_xargs)?;
        if !output.has_impact() || output.get_instance_count() == Count::ZERO {
            return None;
        }
        Some(AggrLocalInvData {
            output,
            instance_limit: get_item_ship_limit(ctx, calc, item_uid, ospec.limit_attr_rid),
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Converter
////////////////////////////////////////////////////////////////////////////////////////////////////
pub(super) struct LocalConverter<'u, 'p, 'c, 'o, 'i, BG, I>
where
    BG: NEffectOutputGetter,
{
    ctx: SvcCtx<'u, 'p>,
    calc: &'c mut Calc,
    item_uid: UItemId,
    ospec: &'o REffectLocalOpcSpec<BG>,
    inv_local: &'i AggrLocalInvData<I>,
}
impl<'u, 'p, 'c, 'o, 'i, BG, I> LocalConverter<'u, 'p, 'c, 'o, 'i, BG, I>
where
    BG: NEffectOutputGetter,
{
    pub(super) fn new(
        ctx: SvcCtx<'u, 'p>,
        calc: &'c mut Calc,
        item_uid: UItemId,
        ospec: &'o REffectLocalOpcSpec<BG>,
        inv_local: &'i AggrLocalInvData<I>,
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
impl<BG, I> LibConverter<CycleDataFull, AggrPartData<I>> for LocalConverter<'_, '_, '_, '_, '_, BG, I>
where
    BG: NEffectOutputGetter,
    I: Copy + std::ops::MulAssign<PValue> + InstanceLimit,
{
    fn lib_convert(&mut self, input: CycleDataFull) -> AggrPartData<I> {
        let output = get_local_output(
            self.ctx,
            self.calc,
            self.item_uid,
            self.ospec,
            self.inv_local,
            input.active.chargedness,
        );
        AggrPartData {
            cycle_main_duration: input.get_main_duration(),
            output,
        }
    }
}
impl<BG, I> LibConverter<CycleDataFull, AggrPartDataTail<I>> for LocalConverter<'_, '_, '_, '_, '_, BG, I>
where
    BG: NEffectOutputGetter,
    I: Copy + std::ops::MulAssign<PValue> + InstanceDuration + InstanceLimit,
{
    fn lib_convert(&mut self, input: CycleDataFull) -> AggrPartDataTail<I> {
        let output = get_local_output(
            self.ctx,
            self.calc,
            self.item_uid,
            self.ospec,
            self.inv_local,
            input.active.chargedness,
        );
        let cycle_main_duration = input.get_main_duration();
        let cycle_tail_duration = get_cycle_tail_duration(cycle_main_duration, output.get_completion_duration());
        AggrPartDataTail {
            cycle_main_duration,
            cycle_tail_duration,
            output,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Helper functions
////////////////////////////////////////////////////////////////////////////////////////////////////
fn get_local_output<BG, I>(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_uid: UItemId,
    ospec: &REffectLocalOpcSpec<BG>,
    inv_local: &AggrLocalInvData<I>,
    chargeness: Option<UnitInterval>,
) -> Output<I>
where
    BG: NEffectOutputGetter,
    I: Copy + std::ops::MulAssign<PValue> + InstanceLimit,
{
    let mut output = inv_local.output;
    // Chargedness
    if let Some(charge_mult_getter) = ospec.charge_mult
        && let Some(chargedness) = chargeness
        && let Some(charge_mult) = charge_mult_getter.get(ctx, calc, item_uid, chargedness)
    {
        output.instance_mul_assign(charge_mult);
    }
    // Limit
    if let Some(limit) = inv_local.instance_limit {
        output.instance_limit(limit);
    }
    output
}
