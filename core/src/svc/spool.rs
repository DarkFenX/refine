use crate::{
    misc::{Count, PValue, Spool, StOption, Value},
    rd::{REffect, RSpoolAttrs},
    svc::{SvcCtx, calc::Calc, funcs},
    ud::UItemId,
};

pub(super) struct ResolvedSpool {
    pub(super) cycles: Count,
    pub(super) cycles_max: Count,
    pub(super) mult: Value,
}
impl ResolvedSpool {
    pub(super) fn try_build(
        ctx: SvcCtx,
        calc: &mut Calc,
        item_uid: UItemId,
        effect: &REffect,
        spool: StOption<Spool>,
        spool_attrs: RSpoolAttrs,
    ) -> Option<Self> {
        let duration_s = funcs::get_effect_duration_s(ctx, calc, item_uid, effect)?;
        let spool_step = calc.get_item_attr_oextra(ctx, item_uid, spool_attrs.step_attr_rid)?;
        let spool_max = calc.get_item_attr_oextra(ctx, item_uid, spool_attrs.max_attr_rid)?;
        let spool = ctx.u_data.get_item_spool(item_uid, spool);
        resolve_spool(spool, spool_step, spool_max, duration_s)
    }
}

fn resolve_spool(spool: Spool, step: Value, max: Value, cycle_time: PValue) -> Option<ResolvedSpool> {
    // Step is used as divisor when calculating all spool types
    if step.abs() < PValue::FLOAT_TOLERANCE {
        return None;
    }
    let ratio = max / step;
    let ratio = match ratio > Value::ZERO {
        true => PValue::from_value_unchecked(ratio),
        false => return None,
    };
    let cycles_max = Count::from_pvalue_ceiled(ratio);
    let cycles = match spool {
        Spool::Cycles(cycles_opt) => {
            // Limit requested count by max spool cycles
            cycles_max.min(cycles_opt)
        }
        Spool::Time(time) => {
            // Choose count of cycles finished by specified time, and limit by max spool cycles
            let cycles_by_time = Count::from_pvalue_trunced(time / cycle_time);
            cycles_max.min(cycles_by_time)
        }
        Spool::SpoolScale(range_value) => Count::from_pvalue_ceiled(range_value.into_pvalue() * ratio),
        Spool::CycleScale(range_value) => {
            Count::from_pvalue_ceiled(range_value.into_pvalue() * cycles_max.into_pvalue())
        }
    };
    let mult = Value::ONE + max.min(step * cycles.into_value());
    Some(ResolvedSpool {
        cycles,
        cycles_max,
        mult,
    })
}
