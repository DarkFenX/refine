use crate::{ad::AAttrId, num::PValue};

// Time relatively effect cycle start when some event happens
pub(crate) enum NEffectTime {
    // Event is triggered at the end of effect cycle
    CycleEnd,
    // Event is triggered at hardcoded point of time (in seconds)
    Hardcoded(PValue),
}

// Duration of some effect's impact relatively effect cycle start (e.g. disallow-cloak duration)
pub(crate) enum NEffectDuration {
    // Impactful as long as effect is in active duration of its cycle
    Effect,
    // Impactful for duration in milliseconds, amount is specified as value of on-item attribute
    AttrMs(AAttrId),
}
