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
    pub breacher: Option<StatDmgEntryBreacher>,
}

pub struct StatDmgEntryApplied {
    pub em: PValue,
    pub thermal: PValue,
    pub kinetic: PValue,
    pub explosive: PValue,
    pub breacher: Option<PValue>,
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
    pub(in crate::svc::vast) fn nullified(self) -> Option<Self> {
        match self.absolute_max > PValue::FLOAT_TOLERANCE && self.relative_max > PValue::FLOAT_TOLERANCE {
            true => Some(self),
            false => None,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl StatDmgEntry {
    pub(in crate::svc::vast) fn from_dmgs(normal: DmgKinds<PValue>, breacher: Option<StatDmgEntryBreacher>) -> Self {
        Self {
            em: normal.em,
            thermal: normal.thermal,
            kinetic: normal.kinetic,
            explosive: normal.explosive,
            breacher: match breacher {
                Some(breacher) => breacher.nullified(),
                _ => None,
            },
        }
    }
}

impl StatDmgEntryApplied {
    pub(in crate::svc::vast) fn from_dmgs(normal: DmgKinds<PValue>, breacher: Option<PValue>) -> Self {
        Self {
            em: normal.em,
            thermal: normal.thermal,
            kinetic: normal.kinetic,
            explosive: normal.explosive,
            breacher,
        }
    }
}
