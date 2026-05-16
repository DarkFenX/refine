use super::traits::GetDuration;
use crate::num::{PValue, UnitInterval};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Cycle data containers
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Copy, Clone)]
pub(in crate::svc) struct CycleDataFull {
    pub(in crate::svc) active: CycleActive,
    pub(in crate::svc) soft_dt: Option<CycleSoftDt>,
}
impl CycleDataFull {
    // Active duration and soft downtime duration combined
    pub(in crate::svc) fn get_main_duration(&self) -> PValue {
        let mut duration = self.active.duration;
        if let Some(soft_dt) = &self.soft_dt {
            duration += soft_dt.duration;
        }
        duration
    }
}
impl GetDuration for CycleDataFull {
    fn get_duration(&self) -> PValue {
        self.get_main_duration()
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
pub(in crate::svc) struct CycleSoftDt {
    pub(in crate::svc) duration: PValue,
    pub(in crate::svc) reason: CycleSoftDtReason,
}
impl CycleSoftDt {
    pub(super) fn try_new(duration: PValue, cooldown: bool, reload: bool, pre_rearm_idling: bool) -> Option<Self> {
        let reason = CycleSoftDtReason::try_new(cooldown, reload, pre_rearm_idling)?;
        Some(Self { duration, reason })
    }
}
#[derive(Copy, Clone)]
pub(in crate::svc) struct CycleSoftDtReason {
    // Module reactivation delays & ability cooldowns
    pub(in crate::svc) cooldown: bool,
    // Module reloads only; fighter rearms are considered as a hard downtime
    pub(in crate::svc) reload: bool,
    // When there is some, but too little time to fit even partial cycle before fighter rearm, that
    // time is considered as pre-rearm-idling
    pub(in crate::svc) pre_rearm_idling: bool,
}
impl CycleSoftDtReason {
    fn try_new(cooldown: bool, reload: bool, pre_rearm_idling: bool) -> Option<Self> {
        match cooldown || reload || pre_rearm_idling {
            true => Some(Self {
                cooldown,
                reload,
                pre_rearm_idling,
            }),
            false => None,
        }
    }
}

// Info about hard downtime between cycles (during which effects cannot apply their instances)
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(in crate::svc) struct CycleHardDt {
    pub(in crate::svc) duration: PValue,
    pub(in crate::svc) reason: CycleHardDtReason,
}
impl CycleHardDt {
    pub(super) fn try_new(duration: PValue, rearm: bool) -> Option<Self> {
        let reason = CycleHardDtReason::try_new(rearm)?;
        Some(Self { duration, reason })
    }
}
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(in crate::svc) struct CycleHardDtReason {
    pub(in crate::svc) refuel: bool,
}
impl CycleHardDtReason {
    fn try_new(refuel: bool) -> Option<Self> {
        match refuel {
            true => Some(Self { refuel }),
            false => None,
        }
    }
}
