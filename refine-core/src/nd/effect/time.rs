use crate::num::PValue;

/// Time relatively effect cycle start when some event happens.
///
/// Information specified in this enum is ignored, just its presence is noted. Might change in the
/// future.
pub(crate) enum NEffectTime {
    /// Event is triggered at the end of effect cycle.
    CycleEnd,
    /// Event is triggered at hardcoded point of time (in seconds).
    Hardcoded(PValue),
}

/// Duration of some effect's impact relatively effect cycle start (e.g. disallow-cloak duration).
///
/// Information specified in this enum is ignored, just its presence is noted. Might change in the
/// future.
pub(crate) enum NEffectDuration {
    /// Impactful as long as effect is in active part of its cycle.
    Effect,
}
