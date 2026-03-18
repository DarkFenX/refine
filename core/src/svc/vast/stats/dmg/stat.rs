use crate::{misc::DmgKinds, num::PValue};

pub struct StatDmg {
    pub dps: StatDmgEntry,
    pub volley: StatDmgEntry,
}

pub struct StatDmgApplied {
    pub dps: StatDmgEntryApplied,
    pub volley: StatDmgEntryApplied,
}

pub struct StatDmgEntry {
    pub em: PValue,
    pub thermal: PValue,
    pub kinetic: PValue,
    pub explosive: PValue,
    pub breacher: StatDmgEntryBreacher,
}

pub struct StatDmgEntryApplied {
    pub em: PValue,
    pub thermal: PValue,
    pub kinetic: PValue,
    pub explosive: PValue,
    pub breacher: PValue,
}

pub struct StatDmgEntryBreacher {
    pub absolute_max: PValue,
    // This field is not unit interval since it is supposed to store breacher DPS as well, and DPS
    // can exceed value of 1 if server has more than 1 ticks per second
    pub relative_max: PValue,
}
impl StatDmgEntryBreacher {
    pub(in crate::svc::vast) fn new() -> Self {
        Self {
            absolute_max: PValue::ZERO,
            relative_max: PValue::ZERO,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl StatDmgEntry {
    pub(in crate::svc::vast) fn from_dmgs(normal: DmgKinds<PValue>, breacher: StatDmgEntryBreacher) -> Self {
        Self {
            em: normal.em,
            thermal: normal.thermal,
            kinetic: normal.kinetic,
            explosive: normal.explosive,
            breacher,
        }
    }
}

impl StatDmgEntryApplied {
    pub(in crate::svc::vast) fn from_dmgs(normal: DmgKinds<PValue>, breacher: PValue) -> Self {
        Self {
            em: normal.em,
            thermal: normal.thermal,
            kinetic: normal.kinetic,
            explosive: normal.explosive,
            breacher,
        }
    }
}
