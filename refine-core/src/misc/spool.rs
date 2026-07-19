use crate::num::{Count, PValue, UnitInterval};

/// Controls on which spool cycle spoolable modules will be set.
#[derive(Copy, Clone)]
pub enum Spool {
    /// Module will use this number, or max spool cycles supported by module, whichever is lower.
    Cycles(Count),
    /// Module will use count of full cycles it finishes by this time, or max spool cycles supported
    /// by module, whichever is lower.
    Time(PValue),
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

#[derive(Copy, Clone)]
pub struct ItemSpoolInfo {
    /// Count of cycles at which effect reaches current spool setting.
    pub current: Count,
    /// Count of cycles at which effect reaches max spool.
    pub max: Count,
    /// True if spool parameters are defined directly on item, false if inherited from sol.
    pub overridden: bool,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Custom serialization/deserialization
////////////////////////////////////////////////////////////////////////////////////////////////////
const CYCLES_PREFIX: &str = "c";
const TIME_PREFIX: &str = "t";
const SPOOL_SCALE_PREFIX: &str = "ss";
const CYCLE_SCALE_PREFIX: &str = "cs";

impl std::fmt::Display for Spool {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Cycles(count) => write!(f, "{CYCLES_PREFIX}{count}"),
            Self::Time(time) => write!(f, "{TIME_PREFIX}{time}"),
            Self::SpoolScale(value) => write!(f, "{SPOOL_SCALE_PREFIX}{value}"),
            Self::CycleScale(value) => write!(f, "{CYCLE_SCALE_PREFIX}{value}"),
        }
    }
}
