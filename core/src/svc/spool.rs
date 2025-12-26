use crate::{
    def::{AttrVal, Count, OF},
    misc::Spool,
    rd::{REffect, RSpoolAttrs},
    svc::{SvcCtx, calc::Calc, funcs},
    ud::UItemKey,
    util::{ceil_unerr, floor_unerr},
};

pub(super) struct ResolvedSpool {
    pub(super) cycles: Count,
    pub(super) cycles_max: Count,
    pub(super) mult: AttrVal,
}
impl ResolvedSpool {
    pub(super) fn try_build(
        ctx: SvcCtx,
        calc: &mut Calc,
        item_key: UItemKey,
        effect: &REffect,
        spool: Option<Spool>,
        spool_attrs: RSpoolAttrs,
    ) -> Option<Self> {
        let duration_s = funcs::get_effect_duration_s(ctx, calc, item_key, effect)?;
        let spool_step = calc.get_item_attr_oextra(ctx, item_key, spool_attrs.step)?;
        let spool_max = calc.get_item_attr_oextra(ctx, item_key, spool_attrs.max)?;
        let spool = ctx.u_data.get_item_key_spool(item_key, spool);
        resolve_spool(spool, spool_step, spool_max, duration_s)
    }
}

fn resolve_spool(spool: Spool, step: AttrVal, max: AttrVal, cycle_time: AttrVal) -> Option<ResolvedSpool> {
    // Step is used as divisor when calculating all spool types
    if step == OF(0.0) {
        return None;
    }
    let cycles_max = ceil_unerr(max / step).into_inner() as Count;
    let cycles = match spool {
        Spool::Cycles(cycles_opt) => {
            // Limit requested count by max spool cycles
            cycles_max.min(cycles_opt)
        }
        Spool::Time(time) => {
            // Choose count of cycles finished by specified time, and limit by max spool cycles
            let cycles_by_time = floor_unerr((time).max(OF(0.0)) / cycle_time).into_inner() as Count;
            cycles_max.min(cycles_by_time)
        }
        Spool::SpoolScale(range_value) => ceil_unerr(range_value.get_inner() * max / step).into_inner() as Count,
        Spool::CycleScale(range_value) => ceil_unerr(range_value.get_inner() * cycles_max as f64).into_inner() as Count,
    };
    let mult = OF(1.0) + max.min(step * cycles as f64);
    Some(ResolvedSpool {
        cycles,
        cycles_max,
        mult,
    })
}
