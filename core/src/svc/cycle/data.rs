use crate::{UnitInterval, num::PValue};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Details
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Copy, Clone)]
pub(in crate::svc) struct CycleDataFull {
    // Full duration (active duration with downtime duration combined)
    pub(in crate::svc) duration: PValue,
    // What kind of interruptions happen after current cycle
    pub(in crate::svc) interrupt: Option<CycleInterrupt>,
    // How charged current cycle is
    pub(in crate::svc) chargedness: Option<UnitInterval>,
}

// Simplified cycle data types, they are useful mostly because they allow cycle optimizations during
// cycle conversion
#[derive(Copy, Clone, Eq, PartialEq)]
pub(in crate::svc) struct CycleDataDurInt {
    pub(in crate::svc) duration: PValue,
    pub(in crate::svc) interrupt: bool,
}
impl From<CycleDataFull> for CycleDataDurInt {
    fn from(details_full: CycleDataFull) -> Self {
        Self {
            duration: details_full.duration,
            interrupt: details_full.interrupt.is_some(),
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub(in crate::svc) struct CycleDataDurCharge {
    pub(in crate::svc) duration: PValue,
    pub(in crate::svc) chargedness: Option<UnitInterval>,
}
impl From<CycleDataFull> for CycleDataDurCharge {
    fn from(details_full: CycleDataFull) -> Self {
        Self {
            duration: details_full.duration,
            chargedness: details_full.chargedness,
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(in crate::svc) struct CycleDataDur {
    pub(in crate::svc) duration: PValue,
}
impl CycleDataDur {
    pub(super) fn copy_rounded(&self) -> Self {
        Self {
            duration: self.duration.sig_rounded(10),
        }
    }
}
impl From<CycleDataFull> for CycleDataDur {
    fn from(data_full: CycleDataFull) -> Self {
        Self {
            duration: data_full.duration,
        }
    }
}
impl From<CycleDataDurCharge> for CycleDataDur {
    fn from(data_duration_charge: CycleDataDurCharge) -> Self {
        Self {
            duration: data_duration_charge.duration,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Detail fields
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Copy, Clone)]
pub(in crate::svc) struct CycleInterrupt {
    pub(in crate::svc) cooldown: bool,
    pub(in crate::svc) reload: bool,
}
impl CycleInterrupt {
    pub(in crate::svc) fn try_new(cooldown: bool, reload: bool) -> Option<Self> {
        match cooldown || reload {
            true => Some(Self { cooldown, reload }),
            false => None,
        }
    }
}
