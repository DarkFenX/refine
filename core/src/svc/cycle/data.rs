use crate::{def::AttrVal, util::sig_round};

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
pub(in crate::svc) struct CycleDataTimeChargedness {
    pub(in crate::svc) time: AttrVal,
    pub(in crate::svc) chargedness: Option<AttrVal>,
}
impl From<&CycleDataFull> for CycleDataTimeChargedness {
    fn from(full: &CycleDataFull) -> Self {
        Self {
            time: full.time,
            chargedness: full.chargedness,
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
    fn from(full: &CycleDataFull) -> Self {
        Self { time: full.time }
    }
}
impl From<&CycleDataTimeChargedness> for CycleDataTime {
    fn from(time_charged: &CycleDataTimeChargedness) -> Self {
        Self {
            time: time_charged.time,
        }
    }
}
