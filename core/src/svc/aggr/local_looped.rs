use super::{local_inv_data::try_make_local_inv_data, traits::Aggregable};
use crate::{
    def::OF,
    nd::NChargeMultGetter,
    rd::{REffect, REffectLocalOpcSpec},
    svc::{
        SvcCtx,
        calc::Calc,
        cycle::{CycleDataTime, CycleDataTimeCharge, CycleSeq},
    },
    ud::UItemKey,
};

// Local effects, considers only infinite parts of cycles
pub(in crate::svc) fn aggr_local_looped_per_second<T>(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_key: UItemKey,
    effect: &REffect,
    cycle: &CycleSeq,
    ospec: &REffectLocalOpcSpec<T>,
) -> Option<T>
where
    T: Copy + Aggregable,
{
    match ospec.charge_mult {
        Some(charge_mult_getter) => {
            aggr_local_time_charge(ctx, calc, item_key, effect, cycle.into(), ospec, charge_mult_getter)
        }
        None => aggr_local_time(ctx, calc, item_key, effect, cycle.into(), ospec),
    }
}

fn aggr_local_time_charge<T>(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_key: UItemKey,
    effect: &REffect,
    cseq: CycleSeq<CycleDataTimeCharge>,
    ospec: &REffectLocalOpcSpec<T>,
    charge_mult_getter: NChargeMultGetter,
) -> Option<T>
where
    T: Copy + Aggregable,
{
    let cseq = cseq.try_loop_cseq()?;
    let inv_data = try_make_local_inv_data(ctx, calc, item_key, effect, ospec)?;
    let mut output = T::default();
    let mut time = OF(0.0);
    for cycle_part in cseq.iter_cseq_parts() {
        let mut part_output = inv_data.base.instance_sum() * OF::from(cycle_part.repeat_count);
        if let Some(chargedness) = cycle_part.data.chargedness
            && let Some(charge_mult) = charge_mult_getter(ctx, calc, item_key, chargedness)
        {
            part_output *= charge_mult;
        }
        output += part_output;
        time += cycle_part.data.time;
    }
    Some(output / time)
}

fn aggr_local_time<T>(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_key: UItemKey,
    effect: &REffect,
    cseq: CycleSeq<CycleDataTime>,
    ospec: &REffectLocalOpcSpec<T>,
) -> Option<T>
where
    T: Copy + Aggregable,
{
    let cseq = cseq.try_loop_cseq()?;
    let inv_data = try_make_local_inv_data(ctx, calc, item_key, effect, ospec)?;
    let mut output = T::default();
    let mut time = OF(0.0);
    for cycle_part in cseq.iter_cseq_parts() {
        output += inv_data.base.instance_sum() * OF::from(cycle_part.repeat_count);
        time += cycle_part.data.time;
    }
    Some(output / time)
}
