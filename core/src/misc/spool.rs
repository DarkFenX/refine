use crate::{
    def::{AttrVal, Count, OF},
    misc::UnitInterval,
    util::{ceil_unerr, floor_unerr},
};

pub(crate) struct ResolvedSpool {
    pub(crate) value: AttrVal,
    pub(crate) cycles: Count,
}

/// Controls on which spool cycle spoolable modules will be set.
#[derive(Copy, Clone)]
pub enum Spool {
    /// Module will use this number, or max spool cycles supported by module, whichever is lower.
    Cycles(Count),
    /// Module will use count of full cycles it finishes by this time, or max spool cycles supported
    /// by module, whichever is lower.
    Time(AttrVal),
    /// Specify a point on damage multiplier range, which is then used to choose count of cycles
    /// sufficient to reach it. For example, with max spool = 0.455 and spool step = 0.1, spool
    /// scale = 0.42:
    /// ⌈(0.455 ÷ 0.1) × 0.42⌉ = ⌈1.911⌉ = 2
    /// Result can be different from cycle scale only if max spool can be divided by spool step with
    /// remainder, due to cycle scale being wider. If there is no remainder, spool and cycle range
    /// effectively match.
    SpoolScale(UnitInterval),
    /// Specify a point on cycle number range, which is then used to choose count of cycles
    /// sufficient to reach it. For example, with max spool = 0.455 and spool step = 0.1, cycle
    /// scale = 0.42:
    /// ⌈⌈0.455 ÷ 0.1⌉ × 0.42⌉ = ⌈⌈4.55⌉ × 0.42⌉ = ⌈5 × 0.42⌉ = ⌈2.1⌉ = 3
    /// Result can be different from spool scale only if max spool can be divided by spool step with
    /// remainder, due to cycle scale being wider. If there is no remainder, spool and cycle range
    /// effectively match.
    CycleScale(UnitInterval),
}
impl Spool {
    pub(crate) fn resolve(&self, max: AttrVal, step: AttrVal, cycle_time: AttrVal) -> Option<ResolvedSpool> {
        // Step is used as divisor when calculating all spool types
        if step == OF(0.0) {
            return None;
        }
        let cycles = match self {
            Spool::Cycles(cycles_opt) => {
                // Limit requested count by max spool cycles
                let cycles_max = ceil_unerr(max / step) as Count;
                cycles_max.min(*cycles_opt)
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
}
