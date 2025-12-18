use crate::{
    def::{AttrVal, Count},
    util::UnitInterval,
};

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
