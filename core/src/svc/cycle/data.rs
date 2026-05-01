use crate::num::{PValue, UnitInterval};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Cycle data containers
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Copy, Clone)]
pub(in crate::svc) struct CycleDataFull {
    pub(in crate::svc) active: CycleActive,
    pub(in crate::svc) dt_soft: Option<CycleDtSoft>,
}
impl CycleDataFull {
    // Active duration and soft downtime duration combined
    pub(in crate::svc) fn get_main_duration(&self) -> PValue {
        let mut duration = self.active.duration;
        if let Some(dt_soft) = &self.dt_soft {
            duration += dt_soft.duration;
        }
        duration
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Detail fields
////////////////////////////////////////////////////////////////////////////////////////////////////
// Data about active part of cycle
#[derive(Copy, Clone)]
pub(in crate::svc) struct CycleActive {
    // Duration of effect cycle
    pub(in crate::svc) duration: PValue,
    // How charged cycle is
    pub(in crate::svc) chargedness: Option<UnitInterval>,
}

// Info about soft downtime between cycles (during which effects can apply their instances)
#[derive(Copy, Clone)]
pub(in crate::svc) struct CycleDtSoft {
    pub(in crate::svc) duration: PValue,
    pub(in crate::svc) reason: CycleDtSoftReason,
}
impl CycleDtSoft {
    pub(super) fn try_new(duration: PValue, cooldown: bool, reload: bool) -> Option<Self> {
        let reason = CycleDtSoftReason::try_new(cooldown, reload)?;
        Some(Self { duration, reason })
    }
}
#[derive(Copy, Clone)]
pub(in crate::svc) struct CycleDtSoftReason {
    pub(in crate::svc) cooldown: bool,
    pub(in crate::svc) reload: bool,
}
impl CycleDtSoftReason {
    fn try_new(cooldown: bool, reload: bool) -> Option<Self> {
        match cooldown || reload {
            true => Some(Self { cooldown, reload }),
            false => None,
        }
    }
}

// Info about hard downtime between cycles (during which effects cannot apply their instances)
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(in crate::svc) struct CycleDtHard {
    pub(in crate::svc) duration: PValue,
    pub(in crate::svc) reason: CycleDtHardReason,
}
impl CycleDtHard {
    pub(super) fn try_new(duration: PValue, rearm: bool) -> Option<Self> {
        let reason = CycleDtHardReason::try_new(rearm)?;
        Some(Self { duration, reason })
    }
}
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(in crate::svc) struct CycleDtHardReason {
    pub(in crate::svc) refuel: bool,
}
impl CycleDtHardReason {
    fn try_new(refuel: bool) -> Option<Self> {
        match refuel {
            true => Some(Self { refuel }),
            false => None,
        }
    }
}
