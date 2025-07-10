use crate::{
    def::{AttrVal, Count, OF},
    misc::Spool,
    util::{ceil_unerr, floor_unerr},
};

pub(in crate::svc) struct ResolvedSpool {
    pub(in crate::svc) value: AttrVal,
    pub(in crate::svc) cycles: Count,
}

pub(in crate::svc) fn resolve_spool(
    options: Spool,
    max: AttrVal,
    step: AttrVal,
    cycle_time: AttrVal,
) -> Option<ResolvedSpool> {
    // Step is used as divisor when calculating all spool types
    if step == OF(0.0) {
        return None;
    }
    let cycles = match options {
        Spool::Cycles(cycles_opt) => {
            // Limit requested count by max spool cycles
            let cycles_max = ceil_unerr(max / step) as Count;
            cycles_max.min(cycles_opt)
        }
        Spool::Time(time) => {
            if cycle_time == OF(0.0) {
                return None;
            }
            // Choose count of cycles finished by specified time, and limit by max spool cycles
            let cycles_max = ceil_unerr(max / step) as Count;
            let cycles_by_time = floor_unerr(time / cycle_time) as Count;
            cycles_max.min(cycles_by_time)
        }
        Spool::SpoolScale(range_value) => {
            let cycles = ceil_unerr(range_value.get_inner() * max / step) as Count;
            cycles
        }
        Spool::CycleScale(range_value) => {
            let cycles_max = ceil_unerr(max / step) as Count;
            let cycles = ceil_unerr(range_value.get_inner() * cycles_max as f64) as Count;
            cycles
        }
    };
    let value = max.min(step * cycles as f64);
    Some(ResolvedSpool { cycles, value })
}
