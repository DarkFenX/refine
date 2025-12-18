use crate::{
    def::{AttrVal, Count, OF},
    misc::Spool,
    rd::{RAttrKey, REffect},
    svc::{SvcCtx, calc::Calc, eff_funcs},
    ud::UItemKey,
    util::{ceil_unerr, floor_unerr},
};

pub(crate) struct ResolvedSpool {
    pub(crate) cycles: Count,
    pub(crate) cycles_max: Count,
    pub(crate) mult: AttrVal,
}
impl ResolvedSpool {
    pub(super) fn try_build(
        ctx: SvcCtx,
        calc: &mut Calc,
        item_key: UItemKey,
        r_effect: &REffect,
        spool: Option<Spool>,
        step_attr_key: Option<RAttrKey>,
        max_attr_key: Option<RAttrKey>,
    ) -> Option<Self> {
        let duration_s = eff_funcs::get_effect_duration_s(ctx, calc, item_key, r_effect)?;
        let spool = ctx.u_data.get_item_key_spool(item_key, spool);
        let spool_step = calc.get_item_attr_oextra(ctx, item_key, step_attr_key?)?;
        let spool_max = calc.get_item_attr_oextra(ctx, item_key, max_attr_key?)?;
        resolve_spool(spool, spool_max, spool_step, duration_s)
    }
}

fn resolve_spool(spool: Spool, max: AttrVal, step: AttrVal, cycle_time: AttrVal) -> Option<ResolvedSpool> {
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
