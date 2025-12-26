use crate::{def::AttrVal, util::sig_round};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Details
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Copy, Clone)]
pub(in crate::svc) struct CycleDataFull {
    // Full time (active time with any downtimes combined)
    pub(in crate::svc) time: AttrVal,
    // What kind of interruptions happen after current cycle
    pub(in crate::svc) interrupt: Option<CycleInterrupt>,
    // How charged current cycle is
    pub(in crate::svc) chargedness: Option<AttrVal>,
}

// Simplified cycle data types, they are useful mostly because they allow cycle optimizations during
// cycle conversion
#[derive(Copy, Clone, Eq, PartialEq)]
pub(in crate::svc) struct CycleDataTimeCharge {
    pub(in crate::svc) time: AttrVal,
    pub(in crate::svc) chargedness: Option<AttrVal>,
}
impl From<&CycleDataFull> for CycleDataTimeCharge {
    fn from(details_full: &CycleDataFull) -> Self {
        Self {
            time: details_full.time,
            chargedness: details_full.chargedness,
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(in crate::svc) struct CycleDataTime {
    pub(in crate::svc) time: AttrVal,
}
impl CycleDataTime {
    pub(super) fn copy_rounded(&self) -> Self {
        Self {
            time: sig_round(self.time, 10),
        }
    }
}
impl From<&CycleDataFull> for CycleDataTime {
    fn from(data_full: &CycleDataFull) -> Self {
        Self { time: data_full.time }
    }
}
impl From<&CycleDataTimeCharge> for CycleDataTime {
    fn from(data_time_charge: &CycleDataTimeCharge) -> Self {
        Self {
            time: data_time_charge.time,
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
