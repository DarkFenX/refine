use crate::num::{PValue, UnitInterval};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Details
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Copy, Clone)]
pub(in crate::svc) struct CycleDataFull {
    // Duration of effect cycle
    pub(in crate::svc) active_duration: PValue,
    // Duration during which item can still apply instances of previously cycled effects (cooldowns,
    // reloads)
    pub(in crate::svc) soft_dt_duration: PValue,
    // Duration during which none of item instances can apply (fighter rearm)
    pub(in crate::svc) hard_dt_duration: PValue,
    // What kind of interruptions happen after current cycle
    pub(in crate::svc) interrupt: Option<CycleInterrupt>,
    // How charged current cycle is
    pub(in crate::svc) chargedness: Option<UnitInterval>,
}
impl CycleDataFull {
    pub(in crate::svc) fn get_full_duration(&self) -> PValue {
        self.active_duration + self.soft_dt_duration + self.hard_dt_duration
    }
}

// Simplified cycle data types, they are useful mostly because they allow cycle optimizations during
// cycle conversion
#[derive(Copy, Clone, Eq, PartialEq)]
pub(in crate::svc) struct CycleDataDurCharge {
    pub(in crate::svc) active_duration: PValue,
    pub(in crate::svc) soft_dt_duration: PValue,
    pub(in crate::svc) hard_dt_duration: PValue,
    pub(in crate::svc) chargedness: Option<UnitInterval>,
}
impl From<CycleDataFull> for CycleDataDurCharge {
    fn from(details_full: CycleDataFull) -> Self {
        Self {
            active_duration: details_full.active_duration,
            soft_dt_duration: details_full.soft_dt_duration,
            hard_dt_duration: details_full.hard_dt_duration,
            chargedness: details_full.chargedness,
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(in crate::svc) struct CycleDataDur {
    pub(in crate::svc) active_duration: PValue,
    pub(in crate::svc) soft_dt_duration: PValue,
    pub(in crate::svc) hard_dt_duration: PValue,
}
impl CycleDataDur {
    pub(in crate::svc) fn get_full_duration(&self) -> PValue {
        self.active_duration + self.soft_dt_duration + self.hard_dt_duration
    }
}
impl From<CycleDataFull> for CycleDataDur {
    fn from(data_full: CycleDataFull) -> Self {
        Self {
            active_duration: data_full.active_duration,
            soft_dt_duration: data_full.soft_dt_duration,
            hard_dt_duration: data_full.hard_dt_duration,
        }
    }
}
impl From<CycleDataDurCharge> for CycleDataDur {
    fn from(data_duration_charge: CycleDataDurCharge) -> Self {
        Self {
            active_duration: data_duration_charge.active_duration,
            soft_dt_duration: data_duration_charge.soft_dt_duration,
            hard_dt_duration: data_duration_charge.hard_dt_duration,
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
